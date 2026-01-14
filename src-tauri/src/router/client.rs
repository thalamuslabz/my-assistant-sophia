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

// Mock Client for Testing
pub struct MockClient {
    pub response: String,
}

impl LLMClient for MockClient {
    fn complete(&self, _model: &str, _prompt: &str) -> Result<String, Box<dyn Error>> {
        Ok(self.response.clone())
    }
}
