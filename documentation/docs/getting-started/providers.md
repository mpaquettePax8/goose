---
sidebar_position: 2
title: Configure LLM Provider
---

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

# Supported LLM Providers

Goose is compatible with a wide range of LLM providers, allowing you to choose and integrate your preferred model.

:::tip Model Selection
Goose relies heavily on tool calling capabilities and currently works best with Anthropic's Claude 3.5 Sonnet and OpenAI's GPT-4o (2024-11-20) model.
[Berkeley Function-Calling Leaderboard][function-calling-leaderboard] can be a good guide for selecting models.
:::

## Available Providers

| Provider                                                                    | Description                                                                                                                                                                                                               | Parameters                                                                                                                                                                          |
|-----------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| [Amazon Bedrock](https://aws.amazon.com/bedrock/)                           | Offers a variety of foundation models, including Claude, Jurassic-2, and others. **AWS environment variables must be set in advance, not configured through `goose configure`**                                           | `AWS_PROFILE`, or `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, `AWS_REGION`, ...                                                                                                   |
| [Anthropic](https://www.anthropic.com/)                                     | Offers Claude, an advanced AI model for natural language tasks.                                                                                                                                                           | `ANTHROPIC_API_KEY`, `ANTHROPIC_HOST` (optional)                                                                                                                                                                 |
| [Azure OpenAI](https://learn.microsoft.com/en-us/azure/ai-services/openai/) | Access Azure-hosted OpenAI models, including GPT-4 and GPT-3.5. Supports both API key and Azure credential chain authentication.                                                                                          | `AZURE_OPENAI_ENDPOINT`, `AZURE_OPENAI_DEPLOYMENT_NAME`, `AZURE_OPENAI_API_KEY` (optional)                                                                                           |
| [Databricks](https://www.databricks.com/)                                   | Unified data analytics and AI platform for building and deploying models.                                                                                                                                                 | `DATABRICKS_HOST`, `DATABRICKS_TOKEN`                                                                                                                                               |
| [Gemini](https://ai.google.dev/gemini-api/docs)                             | Advanced LLMs by Google with multimodal capabilities (text, images).                                                                                                                                                      | `GOOGLE_API_KEY`                                                                                                                                                                    |
| [GCP Vertex AI](https://cloud.google.com/vertex-ai)                         | Google Cloud's Vertex AI platform, supporting Gemini and Claude models. **Credentials must be configured in advance. Follow the instructions at https://cloud.google.com/vertex-ai/docs/authentication.**                 | `GCP_PROJECT_ID`, `GCP_LOCATION` and optional `GCP_MAX_RETRIES` (6), `GCP_INITIAL_RETRY_INTERVAL_MS` (5000), `GCP_BACKOFF_MULTIPLIER` (2.0), `GCP_MAX_RETRY_INTERVAL_MS` (320_000). |
| [Groq](https://groq.com/)                                                   | High-performance inference hardware and tools for LLMs.                                                                                                                                                                   | `GROQ_API_KEY`                                                                                                                                                                      |
| [Ollama](https://ollama.com/)                                               | Local model runner supporting Qwen, Llama, DeepSeek, and other open-source models. **Because this provider runs locally, you must first [download and run a model](/docs/getting-started/providers#local-llms-ollama).**  | `OLLAMA_HOST`                                                                                                                                                                       |
| [OpenAI](https://platform.openai.com/api-keys)                              | Provides gpt-4o, o1, and other advanced language models. Also supports OpenAI-compatible endpoints (e.g., self-hosted LLaMA, vLLM, KServe). **o1-mini and o1-preview are not supported because Goose uses tool calling.** | `OPENAI_API_KEY`, `OPENAI_HOST` (optional), `OPENAI_ORGANIZATION` (optional), `OPENAI_PROJECT` (optional), `OPENAI_CUSTOM_HEADERS` (optional)                                       |
| [OpenRouter](https://openrouter.ai/)                                        | API gateway for unified access to various models with features like rate-limiting management.                                                                                                                             | `OPENROUTER_API_KEY`                                                                                                                                                                |


   
## Configure Provider

To configure your chosen provider or see available options, run `goose configure` in the CLI or visit the `Provider Settings` page in the Goose Desktop.

<Tabs groupId="interface">
  <TabItem value="cli" label="Goose CLI" default>
    1. Run the following command: 

    ```sh
    goose configure
    ```

    2. Select `Configure Providers` from the menu and press Enter.

    ```
   ┌   goose-configure 
   │
   ◆  What would you like to configure?
   │  ● Configure Providers (Change provider or update credentials)
   │  ○ Toggle Extensions 
   │  ○ Add Extension 
   └  
   ```
   3. Choose a model provider and press Enter.

   ```
   ┌   goose-configure 
   │
   ◇  What would you like to configure?
   │  Configure Providers 
   │
   ◆  Which model provider should we use?
   │  ● Anthropic (Claude and other models from Anthropic)
   │  ○ Databricks 
   │  ○ Google Gemini 
   │  ○ Groq 
   │  ○ Ollama 
   │  ○ OpenAI 
   │  ○ OpenRouter 
   └  
   ```
   4. Enter your API key (and any other configuration details) when prompted

   ```
   ┌   goose-configure 
   │
   ◇  What would you like to configure?
   │  Configure Providers 
   │
   ◇  Which model provider should we use?
   │  Anthropic 
   │
   ◆  Provider Anthropic requires ANTHROPIC_API_KEY, please enter a value
   │   
   └  
```
  </TabItem>
  <TabItem value="ui" label="Goose Desktop">
  **To update your LLM provider and API key:** 
  1. Click `...` in the upper right corner
  2. Click `Settings`
  3. Next to `Models`, click `Browse`
  4. Click `Configure` in the upper right corner
  4. Press the `+` button next to the provider of your choice
  5. Add additional configurations (API key, host, etc) then press `submit`

  **To change provider model**
  1. Click `...` in the upper right corner
  2. Click `Settings`
  3. Next to `Models`, click `Browse`
  4. Scroll down to `Add Model` 
  5. Select a Provider from drop down menu
  6. Enter Model name
  7. Press `+ Add Model`

  You can explore more models by selecting a `provider` name under `Browse by Provider`. A link will appear, directing you to the provider's website. Once you've found the model you want, return to step 6 and paste the model name.
  </TabItem>

</Tabs>

## Using Custom OpenAI Endpoints

Goose supports using custom OpenAI-compatible endpoints, which is particularly useful for:
- Self-hosted LLMs (e.g., LLaMA, Mistral) using vLLM or KServe
- Private OpenAI-compatible API servers
- Enterprise deployments requiring data governance and security compliance
- OpenAI API proxies or gateways

### Configuration Parameters

| Parameter | Required | Description |
|-----------|----------|-------------|
| `OPENAI_API_KEY` | Yes | Authentication key for the API |
| `OPENAI_HOST` | No | Custom endpoint URL (defaults to api.openai.com) |
| `OPENAI_ORGANIZATION` | No | Organization ID for usage tracking and governance |
| `OPENAI_PROJECT` | No | Project identifier for resource management |
| `OPENAI_CUSTOM_HEADERS` | No | Additional headers to include in the request, in the format "HEADER_A=VALUE_A,HEADER_B=VALUE_B" |

### Example Configurations

<Tabs groupId="deployment">
  <TabItem value="vllm" label="vLLM Self-Hosted" default>
    If you're running LLaMA or other models using vLLM with OpenAI compatibility:
    ```sh
    OPENAI_HOST=https://your-vllm-endpoint.internal
    OPENAI_API_KEY=your-internal-api-key
    ```
  </TabItem>
  <TabItem value="kserve" label="KServe Deployment">
    For models deployed on Kubernetes using KServe:
    ```sh
    OPENAI_HOST=https://kserve-gateway.your-cluster
    OPENAI_API_KEY=your-kserve-api-key
    OPENAI_ORGANIZATION=your-org-id
    OPENAI_PROJECT=ml-serving
    ```
  </TabItem>
  <TabItem value="enterprise" label="Enterprise OpenAI">
    For enterprise OpenAI deployments with governance:
    ```sh
    OPENAI_API_KEY=your-api-key
    OPENAI_ORGANIZATION=org-id123
    OPENAI_PROJECT=compliance-approved
    ```
  </TabItem>
  <TabItem value="custom-headers" label="Custom Headers">
    For OpenAI-compatible endpoints that require custom headers:
    ```sh
    OPENAI_API_KEY=your-api-key
    OPENAI_ORGANIZATION=org-id123
    OPENAI_PROJECT=compliance-approved
    OPENAI_CUSTOM_HEADERS="X-Header-A=abc,X-Header-B=def"
    ```
  </TabItem>
</Tabs>

### Setup Instructions

<Tabs groupId="interface">
  <TabItem value="cli" label="Goose CLI" default>
    1. Run `goose configure`
    2. Select `Configure Providers`
    3. Choose `OpenAI` as the provider
    4. Enter your configuration when prompted:
       - API key
       - Host URL (if using custom endpoint)
       - Organization ID (if using organization tracking)
       - Project identifier (if using project management)
  </TabItem>
  <TabItem value="ui" label="Goose Desktop">
    1. Click `...` in the upper right corner
    2. Click `Settings`
    3. Next to `Models`, click the `browse` link
    4. Click the `configure` link in the upper right corner
    5. Press the `+` button next to OpenAI
    6. Fill in your configuration details:
       - API Key (required)
       - Host URL (for custom endpoints)
       - Organization ID (for usage tracking)
       - Project (for resource management)
    7. Press `submit`
  </TabItem>
</Tabs>

:::tip Enterprise Deployment
For enterprise deployments, you can pre-configure these values using environment variables or configuration files to ensure consistent governance across your organization.
:::

## Using Azure OpenAI with Credential Chain Authentication

Goose supports two authentication methods for Azure OpenAI:

1. **API Key Authentication**: Traditional authentication using an API key
2. **Azure Credential Chain**: Authenticate using Azure CLI credentials or other Azure identity providers

### Azure Credential Chain Benefits

- **Simplified Authentication**: No need to manage API keys
- **Enhanced Security**: Leverage Azure's robust identity and access management
- **Token Auto-renewal**: Credentials are automatically refreshed
- **Integrated Identity Management**: Works with Azure Active Directory roles and permissions

### Configuration Parameters

| Parameter | Required | Description |
|-----------|----------|-------------|
| `AZURE_OPENAI_ENDPOINT` | Yes | Your Azure OpenAI service endpoint URL |
| `AZURE_OPENAI_DEPLOYMENT_NAME` | Yes | The name of your model deployment |
| `AZURE_OPENAI_API_VERSION` | No | API version (defaults to "2024-10-21") |
| `AZURE_OPENAI_API_KEY` | No | API key (if not provided, Azure credential chain will be used) |

### Setup Instructions

<Tabs groupId="azure-auth">
  <TabItem value="credential-chain" label="Using Azure Credential Chain" default>
    
    **Prerequisites**:
    1. Ensure you have the Azure CLI installed: [Install Azure CLI](https://docs.microsoft.com/en-us/cli/azure/install-azure-cli)
    2. Log in to your Azure account:
       ```sh
       az login
       ```
    3. Verify you have appropriate permissions for the Azure OpenAI service
    
    **Configuration**:
    1. Run `goose configure`
    2. Select `Configure Providers`
    3. Choose `Azure OpenAI` as the provider
    4. Enter your Azure OpenAI endpoint and deployment name
    5. Leave the API key field empty to use Azure credential chain
    6. Select your model of choice
    
    ```bash
    # Example configuration
    AZURE_OPENAI_ENDPOINT=https://your-resource.openai.azure.com
    AZURE_OPENAI_DEPLOYMENT_NAME=your-deployment-name
    # No API key needed - will use Azure credential chain
    ```
  </TabItem>
  
  <TabItem value="api-key" label="Using API Key">
    
    **Configuration**:
    1. Obtain your API key from the Azure portal
    2. Run `goose configure`
    3. Select `Configure Providers`
    4. Choose `Azure OpenAI` as the provider
    5. Enter your Azure OpenAI endpoint, deployment name, and API key
    6. Select your model of choice
    
    ```bash
    # Example configuration
    AZURE_OPENAI_ENDPOINT=https://your-resource.openai.azure.com
    AZURE_OPENAI_DEPLOYMENT_NAME=your-deployment-name
    AZURE_OPENAI_API_KEY=your-api-key
    ```
  </TabItem>
</Tabs>

### Troubleshooting Azure Authentication

If you encounter authentication issues:

1. **For Credential Chain**:
   - Ensure you're logged in with `az login`
   - Verify your account has appropriate role assignments (e.g., "Cognitive Services OpenAI User")
   - Check that your Azure subscription has access to the Azure OpenAI service

2. **For API Key**:
   - Verify your API key is correct and has not expired
   - Ensure the API key has appropriate permissions for the deployment
   - Check if IP restrictions are in place for your Azure OpenAI service

:::tip
You can switch between authentication methods at any time by running `goose configure` again and either providing or omitting the API key.
:::

## Using Goose for Free

Goose is a free and open source AI agent that you can start using right away, but not all supported [LLM Providers][providers] provide a free tier. 

Below, we outline a couple of free options and how to get started with them.

:::warning Limitations
These free options are a great way to get started with Goose and explore its capabilities. However, you may need to upgrade your LLM for better performance.
:::


### Google Gemini
Google Gemini provides a free tier. To start using the Gemini API with Goose, you need an API Key from [Google AI studio](https://aistudio.google.com/app/apikey).

To set up Google Gemini with Goose, follow these steps:

<Tabs groupId="interface">
  <TabItem value="cli" label="Goose CLI" default>
    1. Run: 
    ```sh
    goose configure
    ```
    2. Select `Configure Providers` from the menu.
    3. Follow the prompts to choose `Google Gemini` as the provider.
    4. Enter your API key when prompted.
    5. Enter the Gemini model of your choice.

    ```
    ┌   goose-configure
    │
    ◇ What would you like to configure?
    │ Configure Providers
    │
    ◇ Which model provider should we use?
    │ Google Gemini
    │
    ◇ Provider Google Gemini requires GOOGLE_API_KEY, please enter a value
    │▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪▪
    │    
    ◇ Enter a model from that provider:
    │ gemini-2.0-flash-exp
    │
    ◇ Hello! You're all set and ready to go, feel free to ask me anything!
    │
    └ Configuration saved successfully
    ```
    
  </TabItem>
  <TabItem value="ui" label="Goose Desktop">
  **To update your LLM provider and API key:** 

    1. Click on the three dots in the top-right corner.
    2. Select `Provider Settings` from the menu.
    2. Choose `Google Gemini` as provider from the list.
    3. Click Edit, enter your API key, and click `Set as Active`.

  </TabItem>
</Tabs>


### Local LLMs (Ollama)

Ollama provides local LLMs, which requires a bit more set up before you can use it with Goose.

1. [Download Ollama](https://ollama.com/download). 
2. Run any [model supporting tool-calling](https://ollama.com/search?c=tools):

:::warning Limited Support for models without tool calling
Goose extensively uses tool calling, so models without it (e.g. `DeepSeek-r1`) can only do chat completion. If using models without tool calling, all Goose [extensions must be disabled](/docs/getting-started/using-extensions#enablingdisabling-extensions). As an alternative, you can use a [custom DeepSeek-r1 model](/docs/getting-started/providers#deepseek-r1) we've made specifically for Goose.
:::

Example:

```sh
ollama run qwen2.5
```