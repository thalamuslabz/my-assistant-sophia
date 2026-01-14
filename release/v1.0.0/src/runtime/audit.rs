use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;
use std::path::PathBuf;
use tauri::AppHandle;
use tauri::Manager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    pub timestamp: DateTime<Utc>,
    pub level: String,
    pub component: String,
    pub event: String,
    pub context: Value,
}

pub struct AuditLogger {
    log_path: PathBuf,
}

impl AuditLogger {
    pub fn new(log_path: PathBuf) -> Self {
        // Ensure directory exists
        if let Some(parent) = log_path.parent() {
            let _ = create_dir_all(parent);
        }
        AuditLogger { log_path }
    }

    pub fn from_app(app_handle: &AppHandle) -> Self {
        let app_dir = app_handle.path().app_data_dir().expect("Failed to get app data dir");
        let log_dir = app_dir.join("logs");
        let log_path = log_dir.join("audit.jsonl");
        
        Self::new(log_path)
    }

    pub fn log(&self, level: &str, component: &str, event: &str, context: Value) {
        let entry = AuditLogEntry {
            timestamp: Utc::now(),
            level: level.to_string(),
            component: component.to_string(),
            event: event.to_string(),
            context,
        };

        if let Ok(json) = serde_json::to_string(&entry) {
            if let Ok(mut file) = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&self.log_path) 
            {
                let _ = writeln!(file, "{}", json);
            } else {
                eprintln!("Failed to open audit log file: {:?}", self.log_path);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_audit_log_format() {
        let entry = AuditLogEntry {
            timestamp: Utc::now(),
            level: "INFO".to_string(),
            component: "test".to_string(),
            event: "test_event".to_string(),
            context: serde_json::json!({"key": "value"}),
        };

        let json = serde_json::to_string(&entry).unwrap();
        let parsed: Value = serde_json::from_str(&json).unwrap();
        
        assert_eq!(parsed["level"], "INFO");
        assert_eq!(parsed["component"], "test");
        assert!(parsed["timestamp"].is_string());
    }

    #[test]
    fn test_audit_log_write() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("audit_test.jsonl");
        let logger = AuditLogger::new(file_path.clone());

        logger.log("INFO", "test", "write_event", serde_json::json!({}));

        let content = fs::read_to_string(file_path).unwrap();
        assert!(content.contains("write_event"));
        assert!(content.contains("INFO"));
    }
}
