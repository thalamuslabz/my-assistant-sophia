use crate::providers::{ProviderRegistry, ProviderType};
use crate::router::types::{ModelConfig, TaskType};
use crate::storage::StorageManager;
use std::sync::{Arc, Mutex};

pub struct ModelRouter {
    storage: Arc<StorageManager>,
    provider_registry: Arc<Mutex<ProviderRegistry>>,
}

impl ModelRouter {
    pub fn new(storage: Arc<StorageManager>, provider_registry: Arc<Mutex<ProviderRegistry>>) -> Self {
        let _config = Self::get_config(&storage);
        
        ModelRouter {
            storage,
            provider_registry,
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
        let task_type = self.classify_task(input);

        // Resolve provider (Gemini-first order, with primary override)
        let provider = if let Ok(Some(primary)) = self.storage.get_preference("primary_provider") {
            if let Some(p) = ProviderType::from_str(primary.as_str().unwrap_or("")) {
                p
            } else {
                ProviderType::Ollama
            }
        } else {
            let provider_order = self.provider_registry.lock().unwrap().get_active_provider_order();
            provider_order.first().cloned().unwrap_or(ProviderType::Ollama)
        };
        
        // Get the model from the provider config
        let registry = self.provider_registry.lock().unwrap();
        let model = registry.get_provider_config(&provider)
            .map(|c| c.model.clone())
            .unwrap_or_else(|| "gemini-1.5-flash".to_string());
        
        let client = registry.get_client(&provider);
        drop(registry); // Release lock before making API call

        // Log the routing decision (Audit)
        let _ = self.storage.record_decision(
            None,
            serde_json::json!({"input_length": input.len()}),
            serde_json::json!({"route": task_type, "model": &model, "provider": &provider}),
            Some("Routing decision".to_string())
        );

        client.complete(&model, input)
            .map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_classification_heuristics() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let storage = Arc::new(StorageManager::new_with_path(db_path));
        let registry = Arc::new(std::sync::Mutex::new(ProviderRegistry::new(Arc::new(crate::secret_store::SecretStore::new("test")))));
        let router = ModelRouter::new(storage, registry);

        assert_eq!(router.classify_task("write a function to add numbers"), TaskType::CodeAnalysis);
        assert_eq!(router.classify_task("create a plan for the project"), TaskType::Planning);
        assert_eq!(router.classify_task("hello world"), TaskType::GeneralChat);
    }

    #[test]
    fn test_routing_log() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let storage = Arc::new(StorageManager::new_with_path(db_path));
        let registry = Arc::new(std::sync::Mutex::new(ProviderRegistry::new(Arc::new(crate::secret_store::SecretStore::new("test")))));
        let router = ModelRouter::new(storage.clone(), registry);

        let _ = router.route_and_execute("write code");

        // Check if decision was logged
        let export = storage.export_all().unwrap();
        assert_eq!(export.decisions.len(), 1);
        
        let decision = &export.decisions[0];
        let output = &decision.decision_output;
        assert_eq!(output["route"].as_str().unwrap(), "CodeAnalysis");
    }
}
