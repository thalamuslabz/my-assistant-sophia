# Release Notes v1.0.0

## Sophia Assistant - Local-Only Desktop App

### Features
- **Local-Only Runtime:** No cloud dependencies for execution.
- **Explicit Trust:** "Pause means Pause" contract enforced at the system level.
- **Data Sovereignty:** All data stored locally in SQLite (`sophia.db`).
- **Auditability:** Full JSONL logs for every state change and decision.
- **Model Routing:** Intelligent routing to local Ollama endpoints.

### Installation
1. Ensure `Ollama` is installed and running (`ollama serve`).
2. Pull required models: `ollama run llama3.2:3b`.
3. Run the application binary.

### Known Issues
- Network timeout handling for Ollama could be more robust.
- UI is v1 (Functional but minimal).
