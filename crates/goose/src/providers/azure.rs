use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use serde_json::Value;
use std::time::Duration;
use serde::Serialize;
use tokio::time::sleep;

use super::azureauth::AzureAuth;
use super::base::{ConfigKey, Provider, ProviderMetadata, ProviderUsage, Usage};
use super::errors::ProviderError;
use super::formats::openai::{create_request, get_usage, response_to_message};
use super::utils::{emit_debug_trace, get_model, handle_response_openai_compat, ImageFormat};
use crate::message::Message;
use crate::model::ModelConfig;
use mcp_core::tool::Tool;

pub const AZURE_DEFAULT_MODEL: &str = "gpt-4o";
pub const AZURE_DOC_URL: &str =
    "https://learn.microsoft.com/en-us/azure/ai-services/openai/concepts/models";
pub const AZURE_DEFAULT_API_VERSION: &str = "2024-10-21";
pub const AZURE_OPENAI_KNOWN_MODELS: &[&str] = &["gpt-4o", "gpt-4o-mini", "gpt-4"];

// Default retry configuration
const DEFAULT_MAX_RETRIES: usize = 5;
const DEFAULT_INITIAL_RETRY_INTERVAL_MS: u64 = 1000; // Start with 1 second
const DEFAULT_MAX_RETRY_INTERVAL_MS: u64 = 32000; // Max 32 seconds
const DEFAULT_BACKOFF_MULTIPLIER: f64 = 2.0;

#[derive(Debug)]
pub struct AzureProvider {
    client: Client,
    auth: AzureAuth,
    endpoint: String,
    deployment_name: String,
    api_version: String,
}

impl Serialize for AzureProvider {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("AzureProvider", 3)?;
        state.serialize_field("endpoint", &self.endpoint)?;
        state.serialize_field("deployment_name", &self.deployment_name)?;
        state.serialize_field("api_version", &self.api_version)?;
        state.end()
    }
}

impl Default for AzureProvider {
    fn default() -> Self {
        let model = ModelConfig::new(AzureProvider::metadata().default_model);
        AzureProvider::from_env(model).expect("Failed to initialize Azure OpenAI provider")
    }
}

impl AzureProvider {
    pub fn from_env(model: ModelConfig) -> Result<Self> {
        let config = crate::config::Config::global();
        let endpoint: String = config.get_param("AZURE_OPENAI_ENDPOINT")?;
        let deployment_name: String = config.get_param("AZURE_OPENAI_DEPLOYMENT_NAME")?;
        let api_version: String = config
            .get_param("AZURE_OPENAI_API_VERSION")
            .unwrap_or_else(|_| AZURE_DEFAULT_API_VERSION.to_string());

        // Try to get API key first, if not found use Azure credential chain
        let api_key = config.get_secret("AZURE_OPENAI_API_KEY").ok();
        let auth = AzureAuth::new(api_key)?;

        let client = Client::builder()
            .timeout(Duration::from_secs(600))
            .build()?;

        Ok(Self {
            client,
            endpoint,
            auth,
            deployment_name,
            api_version,
        })
    }

    async fn post(&self, payload: Value) -> Result<Value, ProviderError> {
        let mut base_url = url::Url::parse(&self.endpoint)
            .map_err(|e| ProviderError::RequestFailed(format!("Invalid base URL: {e}")))?;

        // Get the existing path without trailing slashes
        let existing_path = base_url.path().trim_end_matches('/');
        let new_path = if existing_path.is_empty() {
            format!(
                "/openai/deployments/{}/chat/completions",
                self.deployment_name
            )
        } else {
            format!(
                "{}/openai/deployments/{}/chat/completions",
                existing_path, self.deployment_name
            )
        };

        base_url.set_path(&new_path);
        base_url.set_query(Some(&format!("api-version={}", self.api_version)));

        let mut attempts = 0;
        let mut last_error = None;
        let mut current_delay = DEFAULT_INITIAL_RETRY_INTERVAL_MS;

        loop {
            // Check if we've exceeded max retries
            if attempts > 0 && attempts > DEFAULT_MAX_RETRIES {
                let error_msg = format!(
                    "Exceeded maximum retry attempts ({}) for rate limiting",
                    DEFAULT_MAX_RETRIES
                );
                tracing::error!("{}", error_msg);
                return Err(last_error.unwrap_or(ProviderError::RateLimitExceeded(error_msg)));
            }

            // Get a fresh auth token for each attempt
            let auth_token = self.auth.get_token().await.map_err(|e| {
                tracing::error!("Authentication error: {:?}", e);
                ProviderError::RequestFailed(format!("Failed to get authentication token: {}", e))
            })?;

            let mut request_builder = self.client.post(base_url.clone());
            let token_value = auth_token.token_value.clone();

            // Set the correct header based on authentication type
            match self.auth.credential_type() {
                super::azureauth::AzureCredentials::ApiKey(_) => {
                    tracing::debug!("Using API key authentication");
                    request_builder = request_builder.header("api-key", token_value.clone());
                }
                super::azureauth::AzureCredentials::DefaultCredential => {
                    tracing::debug!("Using Azure default credential authentication");
                    request_builder = request_builder.header(
                        "Authorization",
                        format!("Bearer {}", token_value.clone()),
                    );
                }
            }

            tracing::debug!(
                "Sending request to Azure OpenAI (attempt {}): {} with payload: {:?}",
                attempts + 1,
                base_url,
                payload
            );

            // Log request details before sending
            tracing::debug!(
                "Request details:\n\
                 - URL: {}\n\
                 - Deployment: {}\n\
                 - API Version: {}\n\
                 - Auth Type: {:?}",
                base_url,
                self.deployment_name,
                self.api_version,
                self.auth.credential_type(),
            );

            // Log the raw HTTP request details
            tracing::warn!(
                "Raw HTTP Request (attempt {}):\nMethod: POST\nURL: {}\nHeaders:\n    Content-Type: application/json\n    {}: {}\nPayload: {:#}",
                attempts + 1,
                base_url,
                match self.auth.credential_type() {
                    super::azureauth::AzureCredentials::ApiKey(_) => "api-key",
                    super::azureauth::AzureCredentials::DefaultCredential => "Authorization",
                },
                match self.auth.credential_type() {
                    super::azureauth::AzureCredentials::ApiKey(_) => token_value,
                    super::azureauth::AzureCredentials::DefaultCredential => format!("Bearer {}", token_value),
                },
                serde_json::to_string_pretty(&payload).unwrap_or_else(|_| format!("{:?}", payload))
            );

            let response_result = request_builder.json(&payload).send().await;
            
            // Log the raw response result
            tracing::warn!(
                "Raw response result from Azure OpenAI (attempt {}): {:?}",
                attempts + 1,
                response_result
            );
            
            match response_result {
                Ok(response) => {
                    let status = response.status();
                    let headers = response.headers().clone();
                    
                    tracing::warn!(
                        "Raw response details:\nStatus: {}\nHeaders: {:?}\nResponse: {:?}",
                        status,
                        headers,
                        response
                    );

                    match handle_response_openai_compat(response).await {
                        Ok(result) => {
                            tracing::info!(
                                "Successfully received response from Azure OpenAI after {} attempts",
                                attempts + 1
                            );
                            tracing::debug!("Response content: {:?}", result);
                            return Ok(result);
                        }
                        Err(ProviderError::RateLimitExceeded(msg)) => {
                            attempts += 1;
                            last_error = Some(ProviderError::RateLimitExceeded(msg.clone()));

                            tracing::warn!(
                                "Rate limit error from Azure OpenAI (attempt {}/{}): {}",
                                attempts,
                                DEFAULT_MAX_RETRIES,
                                msg
                            );

                            let retry_after = if let Some(secs) = msg.to_lowercase().find("try again in ") {
                                msg[secs..]
                                    .split_whitespace()
                                    .nth(3)
                                    .and_then(|s| s.parse::<u64>().ok())
                                    .unwrap_or(0)
                            } else {
                                0
                            };

                            let delay = if retry_after > 0 {
                                tracing::info!(
                                    "Using server-provided retry-after value: {} seconds",
                                    retry_after
                                );
                                Duration::from_secs(retry_after)
                            } else {
                                let delay = current_delay.min(DEFAULT_MAX_RETRY_INTERVAL_MS);
                                current_delay = (current_delay as f64 * DEFAULT_BACKOFF_MULTIPLIER) as u64;
                                tracing::info!(
                                    "Using exponential backoff delay: {} ms (next delay will be {} ms)",
                                    delay,
                                    current_delay
                                );
                                Duration::from_millis(delay)
                            };

                            tracing::warn!(
                                "Rate limit exceeded (attempt {}/{}). Retrying after {:?}...",
                                attempts,
                                DEFAULT_MAX_RETRIES,
                                delay
                            );

                            sleep(delay).await;
                            continue;
                        }
                        Err(e) => {
                            tracing::error!(
                                "Error response from Azure OpenAI (attempt {}): {:?}\nStatus: {}\nHeaders: {:?}",
                                attempts + 1,
                                e,
                                status,
                                headers
                            );
                            return Err(e);
                        }
                    }
                }
                Err(e) => {
                    tracing::error!(
                        "Request failed (attempt {}): {:?}\nIs timeout: {}\nIs connect: {}\nIs request: {}",
                        attempts + 1,
                        e,
                        e.is_timeout(),
                        e.is_connect(),
                        e.is_request(),
                    );
                    
                    // For timeout errors, we should retry
                    if e.is_timeout() {
                        attempts += 1;
                        let delay = current_delay.min(DEFAULT_MAX_RETRY_INTERVAL_MS);
                        current_delay = (current_delay as f64 * DEFAULT_BACKOFF_MULTIPLIER) as u64;
                        
                        tracing::warn!(
                            "Request timeout (attempt {}/{}). Retrying after {} ms...",
                            attempts,
                            DEFAULT_MAX_RETRIES,
                            delay
                        );
                        
                        sleep(Duration::from_millis(delay)).await;
                        continue;
                    }
                    
                    return Err(ProviderError::RequestFailed(format!("Request failed: {}", e)));
                }
            }
        }
    }
}

#[async_trait]
impl Provider for AzureProvider {
    fn metadata() -> ProviderMetadata {
        ProviderMetadata::new(
            "azure_openai",
            "Azure OpenAI",
            "Models through Azure OpenAI Service",
            "gpt-4o",
            AZURE_OPENAI_KNOWN_MODELS
                .iter()
                .map(|s| s.to_string())
                .collect(),
            AZURE_DOC_URL,
            vec![
                ConfigKey::new("AZURE_OPENAI_API_KEY", false, true, None),
                ConfigKey::new("AZURE_OPENAI_ENDPOINT", true, false, None),
                ConfigKey::new("AZURE_OPENAI_DEPLOYMENT_NAME", true, false, None),
                ConfigKey::new("AZURE_OPENAI_API_VERSION", true, false, Some("2024-10-21")),
            ],
        )
    }

    fn get_model_config(&self) -> ModelConfig {
        ModelConfig::new(AZURE_DEFAULT_MODEL.to_string())
    }

    #[tracing::instrument(
        skip(self, system, messages, tools),
        fields(model_config, input, output, input_tokens, output_tokens, total_tokens)
    )]
    async fn complete(
        &self,
        system: &str,
        messages: &[Message],
        tools: &[Tool],
    ) -> Result<(Message, ProviderUsage), ProviderError> {
        tracing::info!("AzureProvider::complete called");
        let payload = create_request(&self.get_model_config(), system, messages, tools, &ImageFormat::OpenAi)?;
        tracing::info!("AzureProvider::complete: Payload created, calling post...");
        let response = self.post(payload.clone()).await?;
        tracing::info!("AzureProvider::complete: Post finished, processing response...");

        let message = response_to_message(response.clone())?;
        tracing::info!("AzureProvider::complete: Message extracted");
        let usage = match get_usage(&response) {
            Ok(usage) => usage,
            Err(ProviderError::UsageError(e)) => {
                tracing::debug!("Failed to get usage data: {}", e);
                Usage::default()
            }
            Err(e) => return Err(e),
        };
        tracing::info!("AzureProvider::complete: Usage extracted");
        let model = get_model(&response);
        emit_debug_trace(&self.get_model_config(), &payload, &response, &usage);
        tracing::info!("AzureProvider::complete: Returning Ok");
        Ok((message, ProviderUsage::new(model, usage)))
    }
}
