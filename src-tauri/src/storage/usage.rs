use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageRecord {
    pub id: Option<i64>,
    pub timestamp: String,
    pub provider: String,
    pub model: String,
    pub prompt_tokens: i64,
    pub completion_tokens: i64,
    pub total_tokens: i64,
    pub estimated_cost_usd: f64,
    pub request_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageStats {
    pub provider: String,
    pub total_requests: i64,
    pub total_tokens: i64,
    pub total_cost_usd: f64,
    pub period_start: String,
    pub period_end: String,
}

pub struct UsageTracker {
    conn: Connection,
}

impl UsageTracker {
    pub fn new(conn: Connection) -> Result<Self> {
        let tracker = UsageTracker { conn };
        tracker.init_schema()?;
        Ok(tracker)
    }

    fn init_schema(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS usage_records (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp TEXT NOT NULL,
                provider TEXT NOT NULL,
                model TEXT NOT NULL,
                prompt_tokens INTEGER NOT NULL,
                completion_tokens INTEGER NOT NULL,
                total_tokens INTEGER NOT NULL,
                estimated_cost_usd REAL NOT NULL,
                request_id TEXT
            )",
            [],
        )?;

        // Create index for faster queries
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_usage_timestamp ON usage_records(timestamp)",
            [],
        )?;

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_usage_provider ON usage_records(provider)",
            [],
        )?;

        log::info!("Usage tracking schema initialized");
        Ok(())
    }

    pub fn record_usage(&self, record: &UsageRecord) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO usage_records 
             (timestamp, provider, model, prompt_tokens, completion_tokens, total_tokens, estimated_cost_usd, request_id)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                record.timestamp,
                record.provider,
                record.model,
                record.prompt_tokens,
                record.completion_tokens,
                record.total_tokens,
                record.estimated_cost_usd,
                record.request_id,
            ],
        )?;

        let id = self.conn.last_insert_rowid();
        log::info!("Recorded usage: {} tokens for {} (${:.4})", 
            record.total_tokens, record.provider, record.estimated_cost_usd);
        Ok(id)
    }

    pub fn get_stats_by_provider(&self, provider: &str, days: i64) -> Result<UsageStats> {
        let period_start = Utc::now() - chrono::Duration::days(days);
        let period_start_str = period_start.to_rfc3339();
        let period_end_str = Utc::now().to_rfc3339();

        let mut stmt = self.conn.prepare(
            "SELECT 
                COUNT(*) as total_requests,
                SUM(total_tokens) as total_tokens,
                SUM(estimated_cost_usd) as total_cost
             FROM usage_records
             WHERE provider = ?1 AND timestamp >= ?2"
        )?;

        let (total_requests, total_tokens, total_cost) = stmt.query_row(
            params![provider, period_start_str],
            |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, Option<i64>>(1)?.unwrap_or(0),
                    row.get::<_, Option<f64>>(2)?.unwrap_or(0.0),
                ))
            }
        )?;

        Ok(UsageStats {
            provider: provider.to_string(),
            total_requests,
            total_tokens,
            total_cost_usd: total_cost,
            period_start: period_start_str,
            period_end: period_end_str,
        })
    }

    pub fn get_all_stats(&self, days: i64) -> Result<Vec<UsageStats>> {
        let period_start = Utc::now() - chrono::Duration::days(days);
        let period_start_str = period_start.to_rfc3339();
        let period_end_str = Utc::now().to_rfc3339();

        let mut stmt = self.conn.prepare(
            "SELECT 
                provider,
                COUNT(*) as total_requests,
                SUM(total_tokens) as total_tokens,
                SUM(estimated_cost_usd) as total_cost
             FROM usage_records
             WHERE timestamp >= ?1
             GROUP BY provider"
        )?;

        let stats = stmt.query_map(params![period_start_str], |row| {
            Ok(UsageStats {
                provider: row.get(0)?,
                total_requests: row.get(1)?,
                total_tokens: row.get::<_, Option<i64>>(2)?.unwrap_or(0),
                total_cost_usd: row.get::<_, Option<f64>>(3)?.unwrap_or(0.0),
                period_start: period_start_str.clone(),
                period_end: period_end_str.clone(),
            })
        })?
        .collect::<Result<Vec<_>>>()?;

        Ok(stats)
    }

    pub fn get_recent_records(&self, limit: i64) -> Result<Vec<UsageRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, timestamp, provider, model, prompt_tokens, completion_tokens, 
                    total_tokens, estimated_cost_usd, request_id
             FROM usage_records
             ORDER BY timestamp DESC
             LIMIT ?1"
        )?;

        let records = stmt.query_map(params![limit], |row| {
            Ok(UsageRecord {
                id: Some(row.get(0)?),
                timestamp: row.get(1)?,
                provider: row.get(2)?,
                model: row.get(3)?,
                prompt_tokens: row.get(4)?,
                completion_tokens: row.get(5)?,
                total_tokens: row.get(6)?,
                estimated_cost_usd: row.get(7)?,
                request_id: row.get(8)?,
            })
        })?
        .collect::<Result<Vec<_>>>()?;

        Ok(records)
    }

    pub fn get_total_cost(&self, days: i64) -> Result<f64> {
        let period_start = Utc::now() - chrono::Duration::days(days);
        let period_start_str = period_start.to_rfc3339();

        let total: Option<f64> = self.conn.query_row(
            "SELECT SUM(estimated_cost_usd) FROM usage_records WHERE timestamp >= ?1",
            params![period_start_str],
            |row| row.get(0)
        )?;

        Ok(total.unwrap_or(0.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_usage_tracking() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let conn = Connection::open(db_path).unwrap();
        let tracker = UsageTracker::new(conn).unwrap();

        // Record usage
        let record = UsageRecord {
            id: None,
            timestamp: Utc::now().to_rfc3339(),
            provider: "Gemini".to_string(),
            model: "gemini-2.0-flash-exp".to_string(),
            prompt_tokens: 100,
            completion_tokens: 50,
            total_tokens: 150,
            estimated_cost_usd: 0.0001,
            request_id: Some("test-123".to_string()),
        };

        let id = tracker.record_usage(&record).unwrap();
        assert!(id > 0);

        // Get stats
        let stats = tracker.get_stats_by_provider("Gemini", 7).unwrap();
        assert_eq!(stats.total_requests, 1);
        assert_eq!(stats.total_tokens, 150);
        assert_eq!(stats.total_cost_usd, 0.0001);

        // Get recent records
        let records = tracker.get_recent_records(10).unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].provider, "Gemini");
    }

    #[test]
    fn test_multiple_providers() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let conn = Connection::open(db_path).unwrap();
        let tracker = UsageTracker::new(conn).unwrap();

        // Record for Gemini
        tracker.record_usage(&UsageRecord {
            id: None,
            timestamp: Utc::now().to_rfc3339(),
            provider: "Gemini".to_string(),
            model: "gemini-2.0-flash-exp".to_string(),
            prompt_tokens: 100,
            completion_tokens: 50,
            total_tokens: 150,
            estimated_cost_usd: 0.0001,
            request_id: None,
        }).unwrap();

        // Record for OpenAI
        tracker.record_usage(&UsageRecord {
            id: None,
            timestamp: Utc::now().to_rfc3339(),
            provider: "OpenAI".to_string(),
            model: "gpt-4o-mini".to_string(),
            prompt_tokens: 200,
            completion_tokens: 100,
            total_tokens: 300,
            estimated_cost_usd: 0.0002,
            request_id: None,
        }).unwrap();

        // Get all stats
        let all_stats = tracker.get_all_stats(7).unwrap();
        assert_eq!(all_stats.len(), 2);

        // Get total cost
        let total_cost = tracker.get_total_cost(7).unwrap();
        assert!((total_cost - 0.0003).abs() < 0.00001); // Floating point tolerance
    }
}
