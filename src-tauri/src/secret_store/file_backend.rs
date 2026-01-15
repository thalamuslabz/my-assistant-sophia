use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use super::crypto;

#[derive(Debug, Serialize, Deserialize)]
struct SecretsFile {
    version: String,
    secrets: HashMap<String, String>,
    metadata: Metadata,
}

#[derive(Debug, Serialize, Deserialize)]
struct Metadata {
    created_at: String,
    updated_at: String,
}

impl Default for SecretsFile {
    fn default() -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        SecretsFile {
            version: "1".to_string(),
            secrets: HashMap::new(),
            metadata: Metadata {
                created_at: now.clone(),
                updated_at: now,
            },
        }
    }
}

pub struct FileBackend {
    file_path: PathBuf,
    backup_path: PathBuf,
    master_key: [u8; 32],
}

impl FileBackend {
    /// Creates a new FileBackend with the secrets file at ~/.sophia/secrets.enc
    pub fn new() -> Result<Self, String> {
        // Get home directory
        let home = dirs::home_dir()
            .ok_or("Could not determine home directory")?;
        
        // Create .sophia directory if it doesn't exist
        let sophia_dir = home.join(".sophia");
        if !sophia_dir.exists() {
            fs::create_dir_all(&sophia_dir)
                .map_err(|e| format!("Failed to create .sophia directory: {}", e))?;
            
            // Set restrictive permissions on directory
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = fs::metadata(&sophia_dir)
                    .map_err(|e| e.to_string())?
                    .permissions();
                perms.set_mode(0o700); // Owner only
                fs::set_permissions(&sophia_dir, perms)
                    .map_err(|e| e.to_string())?;
            }
            
            log::info!("Created .sophia directory at: {:?}", sophia_dir);
        }
        
        let file_path = sophia_dir.join("secrets.enc");
        let backup_path = sophia_dir.join("secrets.enc.bak");
        
        // Derive master key
        let master_key = crypto::derive_master_key()
            .map_err(|e| format!("Failed to derive master key: {}", e))?;
        
        log::info!("FileBackend initialized with path: {:?}", file_path);
        
        Ok(FileBackend {
            file_path,
            backup_path,
            master_key,
        })
    }
    
    /// Loads secrets from encrypted file
    pub fn load(&self) -> Result<HashMap<String, String>, String> {
        if !self.file_path.exists() {
            log::info!("Secrets file does not exist, returning empty map");
            return Ok(HashMap::new());
        }
        
        // Read encrypted file
        let encrypted_data = fs::read(&self.file_path)
            .map_err(|e| format!("Failed to read secrets file: {}", e))?;
        
        // Decrypt
        let plaintext = crypto::decrypt(&encrypted_data, &self.master_key)
            .map_err(|e| {
                log::error!("Decryption failed, attempting backup...");
                format!("Failed to decrypt secrets: {}", e)
            })?;
        
        // Deserialize JSON
        let secrets_file: SecretsFile = serde_json::from_slice(&plaintext)
            .map_err(|e| format!("Failed to parse secrets file: {}", e))?;
        
        log::info!("Loaded {} secrets from file", secrets_file.secrets.len());
        Ok(secrets_file.secrets)
    }
    
    /// Loads secrets from backup file (if main file is corrupted)
    pub fn load_from_backup(&self) -> Result<HashMap<String, String>, String> {
        if !self.backup_path.exists() {
            return Err("Backup file does not exist".to_string());
        }
        
        log::warn!("Loading from backup file: {:?}", self.backup_path);
        
        let encrypted_data = fs::read(&self.backup_path)
            .map_err(|e| format!("Failed to read backup file: {}", e))?;
        
        let plaintext = crypto::decrypt(&encrypted_data, &self.master_key)
            .map_err(|e| format!("Failed to decrypt backup: {}", e))?;
        
        let secrets_file: SecretsFile = serde_json::from_slice(&plaintext)
            .map_err(|e| format!("Failed to parse backup file: {}", e))?;
        
        log::info!("Loaded {} secrets from backup", secrets_file.secrets.len());
        Ok(secrets_file.secrets)
    }
    
    /// Saves secrets to encrypted file (atomic write)
    pub fn save(&self, secrets: &HashMap<String, String>) -> Result<(), String> {
        // Create backup of existing file
        if self.file_path.exists() {
            fs::copy(&self.file_path, &self.backup_path)
                .map_err(|e| format!("Failed to create backup: {}", e))?;
            log::debug!("Created backup at: {:?}", self.backup_path);
        }
        
        // Create secrets file structure
        let mut secrets_file = SecretsFile::default();
        secrets_file.secrets = secrets.clone();
        secrets_file.metadata.updated_at = chrono::Utc::now().to_rfc3339();
        
        // Serialize to JSON
        let plaintext = serde_json::to_vec_pretty(&secrets_file)
            .map_err(|e| format!("Failed to serialize secrets: {}", e))?;
        
        // Encrypt
        let encrypted_data = crypto::encrypt(&plaintext, &self.master_key)
            .map_err(|e| format!("Failed to encrypt secrets: {}", e))?;
        
        // Atomic write: write to temp file, then rename
        let temp_path = self.file_path.with_extension("tmp");
        
        {
            let mut file = fs::File::create(&temp_path)
                .map_err(|e| format!("Failed to create temp file: {}", e))?;
            
            file.write_all(&encrypted_data)
                .map_err(|e| format!("Failed to write temp file: {}", e))?;
            
            file.sync_all()
                .map_err(|e| format!("Failed to sync temp file: {}", e))?;
        }
        
        // Set restrictive permissions before renaming
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&temp_path)
                .map_err(|e| e.to_string())?
                .permissions();
            perms.set_mode(0o600); // Owner read/write only
            fs::set_permissions(&temp_path, perms)
                .map_err(|e| e.to_string())?;
        }
        
        // Atomic rename
        fs::rename(&temp_path, &self.file_path)
            .map_err(|e| format!("Failed to rename temp file: {}", e))?;
        
        log::info!("Saved {} secrets to encrypted file", secrets.len());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_save_and_load() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("secrets.enc");
        let backup_path = dir.path().join("secrets.enc.bak");
        let master_key = crypto::derive_master_key().unwrap();
        
        let backend = FileBackend {
            file_path,
            backup_path,
            master_key,
        };
        
        // Create test secrets
        let mut secrets = HashMap::new();
        secrets.insert("key1".to_string(), "value1".to_string());
        secrets.insert("key2".to_string(), "value2".to_string());
        
        // Save
        backend.save(&secrets).unwrap();
        
        // Load
        let loaded = backend.load().unwrap();
        
        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded.get("key1").unwrap(), "value1");
        assert_eq!(loaded.get("key2").unwrap(), "value2");
    }

    #[test]
    fn test_load_nonexistent_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("nonexistent.enc");
        let backup_path = dir.path().join("nonexistent.enc.bak");
        let master_key = crypto::derive_master_key().unwrap();
        
        let backend = FileBackend {
            file_path,
            backup_path,
            master_key,
        };
        
        // Should return empty map, not error
        let loaded = backend.load().unwrap();
        assert_eq!(loaded.len(), 0);
    }

    #[test]
    fn test_backup_creation() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("secrets.enc");
        let backup_path = dir.path().join("secrets.enc.bak");
        let master_key = crypto::derive_master_key().unwrap();
        
        let backend = FileBackend {
            file_path: file_path.clone(),
            backup_path: backup_path.clone(),
            master_key,
        };
        
        // Save first version
        let mut secrets1 = HashMap::new();
        secrets1.insert("key1".to_string(), "value1".to_string());
        backend.save(&secrets1).unwrap();
        
        // Save second version (should create backup)
        let mut secrets2 = HashMap::new();
        secrets2.insert("key2".to_string(), "value2".to_string());
        backend.save(&secrets2).unwrap();
        
        // Backup should exist
        assert!(backup_path.exists());
        
        // Load from backup should give first version
        let backup_secrets = backend.load_from_backup().unwrap();
        assert_eq!(backup_secrets.get("key1").unwrap(), "value1");
    }

    #[test]
    fn test_file_permissions() {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            
            let dir = tempdir().unwrap();
            let file_path = dir.path().join("secrets.enc");
            let backup_path = dir.path().join("secrets.enc.bak");
            let master_key = crypto::derive_master_key().unwrap();
            
            let backend = FileBackend {
                file_path: file_path.clone(),
                backup_path,
                master_key,
            };
            
            let mut secrets = HashMap::new();
            secrets.insert("key1".to_string(), "value1".to_string());
            backend.save(&secrets).unwrap();
            
            // Check permissions
            let metadata = fs::metadata(&file_path).unwrap();
            let permissions = metadata.permissions();
            let mode = permissions.mode();
            
            // Should be 0o600 (owner read/write only)
            assert_eq!(mode & 0o777, 0o600);
        }
    }
}
