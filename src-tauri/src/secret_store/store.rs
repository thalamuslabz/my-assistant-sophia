use std::collections::HashMap;
use std::sync::Mutex;

pub struct SecretStore {
    _service: String,
    // Temporary in-memory store until we implement proper encryption
    // TODO: Replace with encrypted file storage or fix keyring integration
    store: Mutex<HashMap<String, String>>,
}

impl SecretStore {
    pub fn new(service: &str) -> Self {
        log::warn!("SecretStore using in-memory storage (not persistent across restarts)");
        SecretStore {
            _service: service.to_string(),
            store: Mutex::new(HashMap::new()),
        }
    }

    pub fn set_secret(&self, key: &str, value: &str) -> Result<(), String> {
        log::info!("Storing secret for key: {}", key);
        let mut store = self.store.lock().map_err(|e| e.to_string())?;
        store.insert(key.to_string(), value.to_string());
        log::info!("Secret stored successfully for key: {}", key);
        Ok(())
    }

    pub fn get_secret(&self, key: &str) -> Result<Option<String>, String> {
        log::info!("Retrieving secret for key: {}", key);
        let store = self.store.lock().map_err(|e| e.to_string())?;
        let result = store.get(key).cloned();
        if result.is_some() {
            log::info!("Secret found for key: {}", key);
        } else {
            log::warn!("Secret not found for key: {}", key);
        }
        Ok(result)
    }

    pub fn delete_secret(&self, key: &str) -> Result<(), String> {
        log::info!("Deleting secret for key: {}", key);
        let mut store = self.store.lock().map_err(|e| e.to_string())?;
        store.remove(key);
        Ok(())
    }
}
