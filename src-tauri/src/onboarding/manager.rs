use crate::storage::StorageManager;
use chrono::Utc;
use serde_json::json;
use std::sync::Arc;

pub struct OnboardingManager {
    storage: Arc<StorageManager>,
}

impl OnboardingManager {
    pub fn new(storage: Arc<StorageManager>) -> Self {
        OnboardingManager { storage }
    }

    pub fn has_completed_onboarding(&self) -> bool {
        // Check if "onboarding_completed" preference is set to true
        match self.storage.get_preference("onboarding_completed") {
            Ok(Some(val)) => val.as_bool().unwrap_or(false),
            _ => false,
        }
    }

    pub fn accept_contract(
        &self,
        contract_version: &str,
        contract_hash: &str,
        gemini_key_id: &str,
        network_egress_consent: bool,
    ) -> Result<(), String> {
        // 1. Store contract details
        self.storage.set_preference("contract_version", json!(contract_version))
            .map_err(|e| e.to_string())?;
            
        self.storage.set_preference("contract_hash", json!(contract_hash))
            .map_err(|e| e.to_string())?;
            
        self.storage.set_preference("contract_accepted_at", json!(Utc::now().to_rfc3339()))
            .map_err(|e| e.to_string())?;

        // 2. Ensure Gemini key was stored
        self.storage.set_preference("gemini_key_linked", json!(gemini_key_id))
            .map_err(|e| e.to_string())?;

        // 3. Network egress consent
        self.storage.set_preference("network_egress_consent", json!(network_egress_consent))
            .map_err(|e| e.to_string())?;

        self.storage.set_preference("primary_provider", json!("gemini"))
            .map_err(|e| e.to_string())?;

        // 4. Create Initial Snapshot (v1)
        let initial_snapshot = json!({
            "status": "initialized",
            "context": "fresh_install",
            "primary_provider": "gemini"
        });
        
        self.storage.save_snapshot(initial_snapshot)
            .map_err(|e| e.to_string())?;

        // 5. Mark complete
        self.storage.set_preference("onboarding_completed", json!(true))
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_onboarding_flow() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let storage = Arc::new(StorageManager::new_with_path(db_path));
        let manager = OnboardingManager::new(storage.clone());

        assert_eq!(manager.has_completed_onboarding(), false);

        manager.accept_contract("v1.0", "sha256:1234", "gemini_api_key", true).unwrap();

        assert_eq!(manager.has_completed_onboarding(), true);
        
        // Verify Snapshot created
        let snapshot = storage.get_active_snapshot().unwrap().unwrap();
        assert_eq!(snapshot.version, 1);
        
        // Verify Preferences
        let ver = storage.get_preference("contract_version").unwrap().unwrap();
        assert_eq!(ver, "v1.0");
    }
}
