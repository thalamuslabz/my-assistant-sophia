# Sprint 2 Architecture: Data & Memory Layer

## 1. Storage Strategy
We will use **SQLite** (`rusqlite`) for structured, ACID-compliant local storage. This ensures data integrity, easy querying, and reliable versioning.

**Location:** `app_data_dir/data/sophia.db`

## 2. Schema Design

### A. Understanding Snapshot (Versioned)
Stores the assistant's understanding of the user and context.
```sql
CREATE TABLE snapshots (
    id INTEGER PRIMARY KEY,
    version INTEGER NOT NULL,
    content TEXT NOT NULL, -- JSON blob of the snapshot
    created_at TEXT NOT NULL,
    active BOOLEAN DEFAULT 0
);
```
*Strategy:* Append-only. New updates insert a new row with `version + 1` and set it to `active=1`, flipping previous to `0`.

### B. Decision Memory (Append-Only)
Records decisions made by the assistant for future reference/learning.
```sql
CREATE TABLE decisions (
    id TEXT PRIMARY KEY, -- UUID
    task_id TEXT,
    input_context TEXT, -- JSON
    decision_output TEXT, -- JSON
    rationale TEXT,
    timestamp TEXT NOT NULL
);
```

### C. Preferences (Key-Value)
User settings.
```sql
CREATE TABLE preferences (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL, -- JSON value
    updated_at TEXT NOT NULL
);
```

## 3. Data Access Layer (Rust)
We will create a `StorageManager` struct in a new `storage` module.

- **Dependencies:** `rusqlite`, `serde`, `serde_json`, `uuid`.
- **API:**
  - `save_snapshot(content: Value) -> Result<Version>`
  - `get_active_snapshot() -> Result<Value>`
  - `record_decision(decision: DecisionStruct)`
  - `set_preference(key, value)`
  - `get_preference(key)`
  - `export_all() -> Result<ExportData>`

## 4. Export Format
A single JSON structure containing all data, enabling portability and transparency.
```json
{
  "metadata": { "exported_at": "..." },
  "snapshots": [ ... ],
  "decisions": [ ... ],
  "preferences": { ... }
}
```
