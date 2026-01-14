use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskType {
    GeneralChat,
    CodeAnalysis,
    Planning,
    DataProcessing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub fast_model: String,
    pub complex_model: String,
    pub endpoint: String,
}

impl Default for ModelConfig {
    fn default() -> Self {
        ModelConfig {
            fast_model: "llama3.2:3b".to_string(),
            complex_model: "llama3.1:8b".to_string(),
            endpoint: "http://localhost:11434".to_string(),
        }
    }
}
