use serde_json::{json, Value};
use std::error::Error;

pub trait LLMClient {
    fn complete(&self, model: &str, prompt: &str) -> Result<String, Box<dyn Error>>;
}

pub struct OllamaClient {
    endpoint: String,
    client: reqwest::blocking::Client,
}

impl OllamaClient {
    pub fn new(endpoint: &str) -> Self {
        OllamaClient {
            endpoint: endpoint.to_string(),
            client: reqwest::blocking::Client::new(),
        }
    }
}

impl LLMClient for OllamaClient {
    fn complete(&self, model: &str, prompt: &str) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/api/generate", self.endpoint);
        let body = json!({
            "model": model,
            "prompt": prompt,
            "stream": false
        });

        let res = self.client.post(&url)
            .json(&body)
            .send()?;

        if !res.status().is_success() {
            return Err(format!("Ollama API Error: {}", res.status()).into());
        }

        let json: Value = res.json()?;
        let response = json["response"].as_str()
            .ok_or("Invalid response format")?
            .to_string();

        Ok(response)
    }
}

pub struct GeminiClient {
    endpoint: String,
    api_key: String,
}

impl GeminiClient {
    pub fn new(endpoint: &str, api_key: &str) -> Self {
        GeminiClient {
            endpoint: endpoint.to_string(),
            api_key: api_key.to_string(),
        }
    }
}

impl LLMClient for GeminiClient {
    fn complete(&self, model: &str, prompt: &str) -> Result<String, Box<dyn Error>> {
        log::info!("Gemini API call starting for model: {}", model);
        // Gemini API URL format: endpoint already includes /v1 or /v1beta
        let url = format!("{}/models/{}:generateContent?key={}", self.endpoint, model, self.api_key);
        let body = json!({
            "contents": [{
                "parts": [{ "text": prompt }]
            }]
        });

        log::info!("Sending request to Gemini API...");
        
        // Use ureq instead of reqwest to avoid async issues
        let response = ureq::post(&url)
            .timeout(std::time::Duration::from_secs(30))
            .send_json(&body)
            .map_err(|e| {
                log::error!("Gemini API request failed: {}", e);
                format!("Gemini API request failed: {}", e)
            })?;

        let status = response.status();
        log::info!("Received response with status: {}", status);
        
        if status != 200 {
            let error_text = response.into_string().unwrap_or_else(|_| "Unknown error".to_string());
            log::error!("Gemini API error response: {}", error_text);
            return Err(format!("Gemini API Error {}: {}", status, error_text).into());
        }

        let json: Value = response.into_json()?;
        log::info!("Parsing Gemini response...");
        
        let response_text = json["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .ok_or("Invalid Gemini response format")?
            .to_string();

        log::info!("Gemini response received successfully");
        Ok(response_text)
    }
}

pub struct OpenAIClient {
    endpoint: String,
    api_key: String,
}

impl OpenAIClient {
    pub fn new(endpoint: &str, api_key: &str) -> Self {
        OpenAIClient {
            endpoint: endpoint.to_string(),
            api_key: api_key.to_string(),
        }
    }
}

impl LLMClient for OpenAIClient {
    fn complete(&self, model: &str, prompt: &str) -> Result<String, Box<dyn Error>> {
        log::info!("OpenAI API call starting for model: {}", model);
        let url = format!("{}/chat/completions", self.endpoint);
        let body = json!({
            "model": model,
            "messages": [{"role": "user", "content": prompt}],
            "temperature": 0.7
        });

        log::info!("Sending request to OpenAI API...");
        let response = ureq::post(&url)
            .set("Authorization", &format!("Bearer {}", self.api_key))
            .timeout(std::time::Duration::from_secs(30))
            .send_json(&body)
            .map_err(|e| {
                log::error!("OpenAI API request failed: {}", e);
                format!("OpenAI API request failed: {}", e)
            })?;

        let status = response.status();
        log::info!("Received response with status: {}", status);
        
        if status != 200 {
            let error_text = response.into_string().unwrap_or_else(|_| "Unknown error".to_string());
            log::error!("OpenAI API error response: {}", error_text);
            return Err(format!("OpenAI API Error {}: {}", status, error_text).into());
        }

        let json: Value = response.into_json()?;
        let response_text = json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or("Invalid OpenAI response format")?
            .to_string();

        log::info!("OpenAI response received successfully");
        Ok(response_text)
    }
}

pub struct AnthropicClient {
    endpoint: String,
    api_key: String,
}

impl AnthropicClient {
    pub fn new(endpoint: &str, api_key: &str) -> Self {
        AnthropicClient {
            endpoint: endpoint.to_string(),
            api_key: api_key.to_string(),
        }
    }
}

impl LLMClient for AnthropicClient {
    fn complete(&self, model: &str, prompt: &str) -> Result<String, Box<dyn Error>> {
        log::info!("Anthropic API call starting for model: {}", model);
        let url = format!("{}/messages", self.endpoint);
        let body = json!({
            "model": model,
            "messages": [{"role": "user", "content": prompt}],
            "max_tokens": 1024
        });

        log::info!("Sending request to Anthropic API...");
        let response = ureq::post(&url)
            .set("x-api-key", &self.api_key)
            .set("anthropic-version", "2023-06-01")
            .timeout(std::time::Duration::from_secs(30))
            .send_json(&body)
            .map_err(|e| {
                log::error!("Anthropic API request failed: {}", e);
                format!("Anthropic API request failed: {}", e)
            })?;

        let status = response.status();
        log::info!("Received response with status: {}", status);
        
        if status != 200 {
            let error_text = response.into_string().unwrap_or_else(|_| "Unknown error".to_string());
            log::error!("Anthropic API error response: {}", error_text);
            return Err(format!("Anthropic API Error {}: {}", status, error_text).into());
        }

        let json: Value = response.into_json()?;
        let response_text = json["content"][0]["text"]
            .as_str()
            .ok_or("Invalid Anthropic response format")?
            .to_string();

        log::info!("Anthropic response received successfully");
        Ok(response_text)
    }
}

pub struct DeepSeekClient {
    endpoint: String,
    api_key: String,
}

impl DeepSeekClient {
    pub fn new(endpoint: &str, api_key: &str) -> Self {
        DeepSeekClient {
            endpoint: endpoint.to_string(),
            api_key: api_key.to_string(),
        }
    }
}

impl LLMClient for DeepSeekClient {
    fn complete(&self, model: &str, prompt: &str) -> Result<String, Box<dyn Error>> {
        log::info!("DeepSeek API call starting for model: {}", model);
        let url = format!("{}/chat/completions", self.endpoint);
        let body = json!({
            "model": model,
            "messages": [{"role": "user", "content": prompt}],
            "temperature": 0.7
        });

        log::info!("Sending request to DeepSeek API...");
        let response = ureq::post(&url)
            .set("Authorization", &format!("Bearer {}", self.api_key))
            .timeout(std::time::Duration::from_secs(30))
            .send_json(&body)
            .map_err(|e| {
                log::error!("DeepSeek API request failed: {}", e);
                format!("DeepSeek API request failed: {}", e)
            })?;

        let status = response.status();
        log::info!("Received response with status: {}", status);
        
        if status != 200 {
            let error_text = response.into_string().unwrap_or_else(|_| "Unknown error".to_string());
            log::error!("DeepSeek API error response: {}", error_text);
            return Err(format!("DeepSeek API Error {}: {}", status, error_text).into());
        }

        let json: Value = response.into_json()?;
        let response_text = json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or("Invalid DeepSeek response format")?
            .to_string();

        log::info!("DeepSeek response received successfully");
        Ok(response_text)
    }
}

pub struct OpenRouterClient {
    endpoint: String,
    api_key: String,
}

impl OpenRouterClient {
    pub fn new(endpoint: &str, api_key: &str) -> Self {
        OpenRouterClient {
            endpoint: endpoint.to_string(),
            api_key: api_key.to_string(),
        }
    }
}

impl LLMClient for OpenRouterClient {
    fn complete(&self, model: &str, prompt: &str) -> Result<String, Box<dyn Error>> {
        log::info!("OpenRouter API call starting for model: {}", model);
        let url = format!("{}/chat/completions", self.endpoint);
        let body = json!({
            "model": model,
            "messages": [{"role": "user", "content": prompt}],
            "temperature": 0.7
        });

        log::info!("Sending request to OpenRouter API...");
        let response = ureq::post(&url)
            .set("Authorization", &format!("Bearer {}", self.api_key))
            .set("HTTP-Referer", "http://localhost")
            .set("X-Title", "Sophia Desktop")
            .timeout(std::time::Duration::from_secs(30))
            .send_json(&body)
            .map_err(|e| {
                log::error!("OpenRouter API request failed: {}", e);
                format!("OpenRouter API request failed: {}", e)
            })?;

        let status = response.status();
        log::info!("Received response with status: {}", status);
        
        if status != 200 {
            let error_text = response.into_string().unwrap_or_else(|_| "Unknown error".to_string());
            log::error!("OpenRouter API error response: {}", error_text);
            return Err(format!("OpenRouter API Error {}: {}", status, error_text).into());
        }

        let json: Value = response.into_json()?;
        let response_text = json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or("Invalid OpenRouter response format")?
            .to_string();

        log::info!("OpenRouter response received successfully");
        Ok(response_text)
    }
}

// Mock Client for Testing
pub struct MockClient {
    pub response: String,
}

impl LLMClient for MockClient {
    fn complete(&self, _model: &str, _prompt: &str) -> Result<String, Box<dyn Error>> {
        Ok(self.response.clone())
    }
}
