use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProviderType {
    Gemini,
    DeepSeek,
    OpenAI,
    Anthropic,
    Ollama,
    OpenRouter,
}

impl ProviderType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProviderType::Gemini => "gemini",
            ProviderType::DeepSeek => "deepseek",
            ProviderType::OpenAI => "openai",
            ProviderType::Anthropic => "anthropic",
            ProviderType::Ollama => "ollama",
            ProviderType::OpenRouter => "openrouter",
        }
    }

    pub fn preference_key(&self) -> String {
        format!("provider_config_{}", self.as_str())
    }

    pub fn from_str(value: &str) -> Option<Self> {
        match value.to_lowercase().as_str() {
            "gemini" => Some(ProviderType::Gemini),
            "deepseek" => Some(ProviderType::DeepSeek),
            "openai" => Some(ProviderType::OpenAI),
            "anthropic" => Some(ProviderType::Anthropic),
            "ollama" => Some(ProviderType::Ollama),
            "openrouter" => Some(ProviderType::OpenRouter),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub provider: ProviderType,
    pub api_key_keychain_id: String,
    pub endpoint: String,
    pub model: String,
    pub enabled: bool,
}

impl ProviderConfig {
    pub fn default_gemini() -> Self {
        ProviderConfig {
            provider: ProviderType::Gemini,
            api_key_keychain_id: "gemini_api_key".to_string(),
            endpoint: "https://generativelanguage.googleapis.com/v1beta".to_string(),
            model: "gemini-2.5-flash-lite".to_string(),
            enabled: false,
        }
    }

    pub fn default_openai() -> Self {
        ProviderConfig {
            provider: ProviderType::OpenAI,
            api_key_keychain_id: "openai_api_key".to_string(),
            endpoint: "https://api.openai.com/v1".to_string(),
            model: "gpt-4o-mini".to_string(), // Cost-effective model
            enabled: false,
        }
    }

    pub fn default_anthropic() -> Self {
        ProviderConfig {
            provider: ProviderType::Anthropic,
            api_key_keychain_id: "anthropic_api_key".to_string(),
            endpoint: "https://api.anthropic.com/v1".to_string(),
            model: "claude-3-5-haiku-20241022".to_string(), // Fast, affordable model
            enabled: false,
        }
    }

    pub fn default_deepseek() -> Self {
        ProviderConfig {
            provider: ProviderType::DeepSeek,
            api_key_keychain_id: "deepseek_api_key".to_string(),
            endpoint: "https://api.deepseek.com/v1".to_string(),
            model: "deepseek-chat".to_string(),
            enabled: false,
        }
    }

    pub fn default_openrouter() -> Self {
        ProviderConfig {
            provider: ProviderType::OpenRouter,
            api_key_keychain_id: "openrouter_api_key".to_string(),
            endpoint: "https://openrouter.ai/api/v1".to_string(),
            model: "openai/gpt-4o".to_string(),
            enabled: false,
        }
    }

    pub fn default_ollama() -> Self {
        ProviderConfig {
            provider: ProviderType::Ollama,
            api_key_keychain_id: "".to_string(),
            endpoint: "http://localhost:11434".to_string(),
            model: "llama3.2:3b".to_string(),
            enabled: true,
        }
    }
}
