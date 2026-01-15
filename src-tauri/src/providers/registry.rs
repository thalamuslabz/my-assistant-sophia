use crate::providers::types::{ProviderConfig, ProviderType};
use crate::router::client::{
    LLMClient, 
    OllamaClient, 
    GeminiClient, 
    OpenAIClient, 
    AnthropicClient, 
    DeepSeekClient, 
    OpenRouterClient, 
    MockClient
};
use crate::secret_store::SecretStore;
use std::collections::HashMap;
use std::sync::Arc;

pub struct ProviderRegistry {
    providers: HashMap<ProviderType, ProviderConfig>,
    secret_store: Arc<SecretStore>,
}

impl ProviderRegistry {
    pub fn new(secret_store: Arc<SecretStore>) -> Self {
        let mut providers = HashMap::new();
        providers.insert(ProviderType::Gemini, ProviderConfig::default_gemini());
        providers.insert(ProviderType::OpenAI, ProviderConfig::default_openai());
        providers.insert(ProviderType::Anthropic, ProviderConfig::default_anthropic());
        providers.insert(ProviderType::DeepSeek, ProviderConfig::default_deepseek());
        providers.insert(ProviderType::OpenRouter, ProviderConfig::default_openrouter());
        providers.insert(ProviderType::Ollama, ProviderConfig::default_ollama());

        ProviderRegistry { providers, secret_store }
    }

    pub fn get_provider_config(&self, provider: &ProviderType) -> Option<&ProviderConfig> {
        self.providers.get(provider)
    }

    pub fn get_provider_config_mut(&mut self, provider: &ProviderType) -> Option<&mut ProviderConfig> {
        self.providers.get_mut(provider)
    }

    pub fn set_provider_config(&mut self, config: ProviderConfig) {
        self.providers.insert(config.provider.clone(), config);
    }

    pub fn load_provider_config(&mut self, config: ProviderConfig) {
        self.providers.insert(config.provider.clone(), config);
    }

    pub fn set_api_key(&self, key_id: &str, value: &str) -> Result<(), String> {
        self.secret_store.set_secret(key_id, value)
    }

    pub fn get_api_key(&self, key_id: &str) -> Result<Option<String>, String> {
        self.secret_store.get_secret(key_id)
    }

    pub fn set_provider_enabled(&mut self, provider: &ProviderType, enabled: bool) {
        if let Some(config) = self.providers.get_mut(provider) {
            config.enabled = enabled;
        }
    }

    pub fn get_active_provider_order(&self) -> Vec<ProviderType> {
        // Gemini-first priority order
        let order = vec![
            ProviderType::Gemini,
            ProviderType::DeepSeek,
            ProviderType::OpenAI,
            ProviderType::Anthropic,
            ProviderType::OpenRouter,
            ProviderType::Ollama,
        ];

        let mut active = order.into_iter()
            .filter(|p| self.providers.get(p).map(|c| c.enabled).unwrap_or(false))
            .collect::<Vec<_>>();

        if active.is_empty() {
            active.push(ProviderType::Ollama);
        }

        active
    }

    pub fn get_client(&self, provider: &ProviderType) -> Box<dyn LLMClient + Send + Sync> {
        match provider {
            ProviderType::Ollama => Box::new(OllamaClient::new("http://localhost:11434")),
            ProviderType::Gemini => self.get_keyed_client(provider, |c, k| GeminiClient::new(&c.endpoint, &k), "Missing Gemini API key"),
            ProviderType::OpenAI => self.get_keyed_client(provider, |c, k| OpenAIClient::new(&c.endpoint, &k), "Missing OpenAI API key"),
            ProviderType::Anthropic => self.get_keyed_client(provider, |c, k| AnthropicClient::new(&c.endpoint, &k), "Missing Anthropic API key"),
            ProviderType::DeepSeek => self.get_keyed_client(provider, |c, k| DeepSeekClient::new(&c.endpoint, &k), "Missing DeepSeek API key"),
            ProviderType::OpenRouter => self.get_keyed_client(provider, |c, k| OpenRouterClient::new(&c.endpoint, &k), "Missing OpenRouter API key"),
        }
    }

    fn get_keyed_client<F, C>(&self, provider: &ProviderType, factory: F, missing_msg: &str) -> Box<dyn LLMClient + Send + Sync>
    where
        F: Fn(&ProviderConfig, String) -> C,
        C: LLMClient + Send + Sync + 'static,
    {
        if let Some(config) = self.providers.get(provider) {
            match self.get_api_key(&config.api_key_keychain_id) {
                Ok(Some(key)) => {
                    log::info!("Retrieved API key for provider {:?}", provider);
                    Box::new(factory(config, key))
                },
                Ok(None) => {
                    log::warn!("No API key found for provider {:?} (keychain_id: {})", provider, config.api_key_keychain_id);
                    Box::new(MockClient { response: format!("{} (key not found in keychain)", missing_msg) })
                },
                Err(e) => {
                    log::error!("Error retrieving API key for provider {:?}: {}", provider, e);
                    Box::new(MockClient { response: format!("{} (keychain error: {})", missing_msg, e) })
                }
            }
        } else {
            log::error!("Provider {:?} not configured in registry", provider);
            Box::new(MockClient { response: "Provider not configured".to_string() })
        }
    }
}
