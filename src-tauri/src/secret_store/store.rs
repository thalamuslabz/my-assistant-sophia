use std::collections::HashMap;
use std::sync::Mutex;
use super::file_backend::FileBackend;

pub struct SecretStore {
    _service: String,
    // In-memory cache for fast access
    cache: Mutex<HashMap<String, String>>,
    // Persistent file backend
    backend: FileBackend,
}

impl SecretStore {
    pub fn new(service: &str) -> Self {
        log::info!("Initializing SecretStore with persistent file storage");
        
        // Initialize file backend
        let backend = match FileBackend::new() {
            Ok(b) => b,
            Err(e) => {
                log::error!("Failed to initialize file backend: {}", e);
                panic!("Cannot initialize SecretStore: {}", e);
            }
        };
        
        // Load existing secrets from file
        let cache = match backend.load() {
            Ok(secrets) => {
                log::info!("Loaded {} secrets from persistent storage", secrets.len());
                Mutex::new(secrets)
            },
            Err(e) => {
                log::warn!("Failed to load secrets from file: {}", e);
                log::info!("Attempting to load from backup...");
                
                // Try backup
                match backend.load_from_backup() {
                    Ok(secrets) => {
                        log::info!("Loaded {} secrets from backup", secrets.len());
                        Mutex::new(secrets)
                    },
                    Err(backup_err) => {
                        log::warn!("Failed to load from backup: {}", backup_err);
                        log::info!("Starting with empty secret store");
                        Mutex::new(HashMap::new())
                    }
                }
            }
        };
        
        SecretStore {
            _service: service.to_string(),
            cache,
            backend,
        }
    }

    pub fn set_secret(&self, key: &str, value: &str) -> Result<(), String> {
        log::info!("Storing secret for key: {}", key);
        
        // Update cache
        let mut cache = self.cache.lock().map_err(|e| e.to_string())?;
        cache.insert(key.to_string(), value.to_string());
        
        // Persist to file
        self.backend.save(&cache)?;
        
        log::info!("Secret stored and persisted for key: {}", key);
        Ok(())
    }

    pub fn get_secret(&self, key: &str) -> Result<Option<String>, String> {
        log::debug!("Retrieving secret for key: {}", key);
        let cache = self.cache.lock().map_err(|e| e.to_string())?;
        let result = cache.get(key).cloned();
        
        if result.is_some() {
            log::debug!("Secret found for key: {}", key);
        } else {
            log::debug!("Secret not found for key: {}", key);
        }
        
        Ok(result)
    }

    pub fn delete_secret(&self, key: &str) -> Result<(), String> {
        log::info!("Deleting secret for key: {}", key);
        
        // Update cache
        let mut cache = self.cache.lock().map_err(|e| e.to_string())?;
        cache.remove(key);
        
        // Persist to file
        self.backend.save(&cache)?;
        
        log::info!("Secret deleted and persisted for key: {}", key);
        Ok(())
    }
}
