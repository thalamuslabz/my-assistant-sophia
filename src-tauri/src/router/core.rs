use crate::router::client::{LLMClient, OllamaClient};
use crate::router::types::{ModelConfig, TaskType};
use crate::storage::StorageManager;
use std::sync::Arc;
use serde_json::Value;

pub struct ModelRouter {
    storage: Arc<StorageManager>,
    // We use dynamic dispatch to swap clients (Mock vs Real)
    client: Box<dyn LLMClient + Send + Sync>, 
}

impl ModelRouter {
    pub fn new(storage: Arc<StorageManager>) -> Self {
        let config = Self::get_config(&storage);
        let client = Box::new(OllamaClient::new(&config.endpoint));
        
        ModelRouter {
            storage,
            client,
        }
    }

    // Constructor for testing with mock client
    pub fn new_with_client(storage: Arc<StorageManager>, client: Box<dyn LLMClient + Send + Sync>) -> Self {
        ModelRouter {
            storage,
            client,
        }
    }

    fn get_config(storage: &StorageManager) -> ModelConfig {
        match storage.get_preference("model_config") {
            Ok(Some(val)) => serde_json::from_value(val).unwrap_or_default(),
            _ => ModelConfig::default(),
        }
    }

    pub fn classify_task(&self, input: &str) -> TaskType {
        // Simple heuristic for v1 (to avoid model dependency loop)
        // In v2, this would be a "Fast" model call
        if input.contains("code") || input.contains("function") {
            TaskType::CodeAnalysis
        } else if input.contains("plan") || input.contains("todo") {
            TaskType::Planning
        } else if input.contains("json") || input.contains("data") {
            TaskType::DataProcessing
        } else {
            TaskType::GeneralChat
        }
    }

    pub fn route_and_execute(&self, input: &str) -> Result<String, String> {
        let config = Self::get_config(&self.storage);
        let task_type = self.classify_task(input);

        let model = match task_type {
            TaskType::GeneralChat | TaskType::DataProcessing => &config.fast_model,
            TaskType::CodeAnalysis | TaskType::Planning => &config.complex_model,
        };

        // Log the routing decision (Audit)
        let _ = self.storage.record_decision(
            None,
            serde_json::json!({"input_length": input.len()}),
            serde_json::json!({"route": task_type, "model": model}),
            Some("Routing decision".to_string())
        );

        self.client.complete(model, input)
            .map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::router::client::MockClient;
    use tempfile::tempdir;

    #[test]
    fn test_classification_heuristics() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let storage = Arc::new(StorageManager::new_with_path(db_path));
        let client = Box::new(MockClient { response: "ok".to_string() });
        let router = ModelRouter::new_with_client(storage, client);

        assert_eq!(router.classify_task("write a function to add numbers"), TaskType::CodeAnalysis);
        assert_eq!(router.classify_task("create a plan for the project"), TaskType::Planning);
        assert_eq!(router.classify_task("hello world"), TaskType::GeneralChat);
    }

    #[test]
    fn test_routing_log() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let storage = Arc::new(StorageManager::new_with_path(db_path));
        let client = Box::new(MockClient { response: "simulated_response".to_string() });
        let router = ModelRouter::new_with_client(storage.clone(), client);

        let _ = router.route_and_execute("write code");

        // Check if decision was logged
        let export = storage.export_all().unwrap();
        assert_eq!(export.decisions.len(), 1);
        
        let decision = &export.decisions[0];
        let output = &decision.decision_output;
        assert_eq!(output["route"].as_str().unwrap(), "CodeAnalysis");
    }
}
