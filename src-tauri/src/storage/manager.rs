use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;
use tauri::AppHandle;
use tauri::Manager;
use uuid::Uuid;
use chrono::Utc;

#[derive(Clone)]
pub struct StorageManager {
    db_path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Snapshot {
    pub id: i64,
    pub version: i64,
    pub content: Value,
    pub created_at: String,
    pub active: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Decision {
    pub id: String,
    pub task_id: Option<String>,
    pub input_context: Value,
    pub decision_output: Value,
    pub rationale: Option<String>,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExportData {
    pub metadata: Value,
    pub snapshots: Vec<Snapshot>,
    pub decisions: Vec<Decision>,
    pub preferences: Value,
}

impl StorageManager {
    pub fn new(app_handle: &AppHandle) -> Self {
        let app_dir = app_handle.path().app_data_dir().expect("Failed to get app data dir");
        let data_dir = app_dir.join("data");
        std::fs::create_dir_all(&data_dir).expect("Failed to create data dir");
        let db_path = data_dir.join("sophia.db");

        let manager = StorageManager { db_path };
        manager.init().expect("Failed to initialize database");
        manager
    }

    // Constructor for testing
    pub fn new_with_path(path: PathBuf) -> Self {
        let manager = StorageManager { db_path: path };
        manager.init().expect("Failed to initialize database");
        manager
    }

    fn get_connection(&self) -> Result<Connection> {
        Connection::open(&self.db_path)
    }

    fn init(&self) -> Result<()> {
        let conn = self.get_connection()?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS snapshots (
                id INTEGER PRIMARY KEY,
                version INTEGER NOT NULL,
                content TEXT NOT NULL,
                created_at TEXT NOT NULL,
                active BOOLEAN DEFAULT 0
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS decisions (
                id TEXT PRIMARY KEY,
                task_id TEXT,
                input_context TEXT,
                decision_output TEXT,
                rationale TEXT,
                timestamp TEXT NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS preferences (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;

        Ok(())
    }

    pub fn save_snapshot(&self, content: Value) -> Result<i64> {
        let mut conn = self.get_connection()?;
        let tx = conn.transaction()?;

        // Get current max version
        let current_version: i64 = tx.query_row(
            "SELECT COALESCE(MAX(version), 0) FROM snapshots",
            [],
            |row| row.get(0),
        )?;

        let new_version = current_version + 1;
        let created_at = Utc::now().to_rfc3339();

        // Deactivate old active snapshots
        tx.execute("UPDATE snapshots SET active = 0 WHERE active = 1", [])?;

        // Insert new snapshot
        tx.execute(
            "INSERT INTO snapshots (version, content, created_at, active) VALUES (?1, ?2, ?3, 1)",
            params![new_version, content.to_string(), created_at],
        )?;

        let id = tx.last_insert_rowid();

        tx.commit()?;
        Ok(id)
    }

    pub fn get_active_snapshot(&self) -> Result<Option<Snapshot>> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare(
            "SELECT id, version, content, created_at, active FROM snapshots WHERE active = 1"
        )?;

        let mut rows = stmt.query_map([], |row| {
            let content_str: String = row.get(2)?;
            Ok(Snapshot {
                id: row.get(0)?,
                version: row.get(1)?,
                content: serde_json::from_str(&content_str).unwrap_or(Value::Null),
                created_at: row.get(3)?,
                active: row.get(4)?,
            })
        })?;

        if let Some(row) = rows.next() {
            Ok(Some(row?))
        } else {
            Ok(None)
        }
    }

    pub fn record_decision(&self, task_id: Option<String>, input: Value, output: Value, rationale: Option<String>) -> Result<String> {
        let conn = self.get_connection()?;
        let id = Uuid::new_v4().to_string();
        let timestamp = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO decisions (id, task_id, input_context, decision_output, rationale, timestamp) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                id, 
                task_id, 
                input.to_string(), 
                output.to_string(), 
                rationale, 
                timestamp
            ],
        )?;

        Ok(id)
    }

    pub fn set_preference(&self, key: &str, value: Value) -> Result<()> {
        let conn = self.get_connection()?;
        let updated_at = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT OR REPLACE INTO preferences (key, value, updated_at) VALUES (?1, ?2, ?3)",
            params![key, value.to_string(), updated_at],
        )?;

        Ok(())
    }

    pub fn get_preference(&self, key: &str) -> Result<Option<Value>> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare("SELECT value FROM preferences WHERE key = ?1")?;
        
        let mut rows = stmt.query_map(params![key], |row| {
            let val_str: String = row.get(0)?;
            Ok(serde_json::from_str(&val_str).unwrap_or(Value::Null))
        })?;

        if let Some(row) = rows.next() {
            Ok(Some(row?))
        } else {
            Ok(None)
        }
    }

    pub fn export_all(&self) -> Result<ExportData> {
        let conn = self.get_connection()?;

        // Snapshots
        let mut stmt = conn.prepare("SELECT id, version, content, created_at, active FROM snapshots")?;
        let snapshots = stmt.query_map([], |row| {
            let content_str: String = row.get(2)?;
            Ok(Snapshot {
                id: row.get(0)?,
                version: row.get(1)?,
                content: serde_json::from_str(&content_str).unwrap_or(Value::Null),
                created_at: row.get(3)?,
                active: row.get(4)?,
            })
        })?.collect::<Result<Vec<_>>>()?;

        // Decisions
        let mut stmt = conn.prepare("SELECT id, task_id, input_context, decision_output, rationale, timestamp FROM decisions")?;
        let decisions = stmt.query_map([], |row| {
            let input_str: String = row.get(2)?;
            let output_str: String = row.get(3)?;
            Ok(Decision {
                id: row.get(0)?,
                task_id: row.get(1)?,
                input_context: serde_json::from_str(&input_str).unwrap_or(Value::Null),
                decision_output: serde_json::from_str(&output_str).unwrap_or(Value::Null),
                rationale: row.get(4)?,
                timestamp: row.get(5)?,
            })
        })?.collect::<Result<Vec<_>>>()?;

        // Preferences
        let mut stmt = conn.prepare("SELECT key, value FROM preferences")?;
        let pref_map = stmt.query_map([], |row| {
            let key: String = row.get(0)?;
            let val_str: String = row.get(1)?;
            let val: Value = serde_json::from_str(&val_str).unwrap_or(Value::Null);
            Ok((key, val))
        })?;

        let mut preferences = serde_json::Map::new();
        for p in pref_map {
            let (k, v) = p?;
            preferences.insert(k, v);
        }

        Ok(ExportData {
            metadata: serde_json::json!({
                "exported_at": Utc::now().to_rfc3339(),
                "version": "1.0"
            }),
            snapshots,
            decisions,
            preferences: Value::Object(preferences),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_snapshot_versioning() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let storage = StorageManager::new_with_path(db_path);

        // Save v1
        let content1 = serde_json::json!({"state": "initial"});
        storage.save_snapshot(content1).unwrap();

        let active = storage.get_active_snapshot().unwrap().unwrap();
        assert_eq!(active.version, 1);
        assert_eq!(active.content["state"], "initial");

        // Save v2
        let content2 = serde_json::json!({"state": "updated"});
        storage.save_snapshot(content2).unwrap();

        let active_v2 = storage.get_active_snapshot().unwrap().unwrap();
        assert_eq!(active_v2.version, 2);
        assert_eq!(active_v2.content["state"], "updated");
    }

    #[test]
    fn test_preferences() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let storage = StorageManager::new_with_path(db_path);

        storage.set_preference("theme", serde_json::json!("dark")).unwrap();
        
        let val = storage.get_preference("theme").unwrap().unwrap();
        assert_eq!(val, "dark");
    }

    #[test]
    fn test_export() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let storage = StorageManager::new_with_path(db_path);

        // Seed data
        storage.save_snapshot(serde_json::json!({"v": 1})).unwrap();
        storage.record_decision(None, serde_json::json!("in"), serde_json::json!("out"), None).unwrap();
        storage.set_preference("k", serde_json::json!("v")).unwrap();

        let export = storage.export_all().unwrap();
        
        assert_eq!(export.snapshots.len(), 1);
        assert_eq!(export.decisions.len(), 1);
        assert_eq!(export.preferences["k"], "v");
    }
}
