# Sprint 4 Architecture: Model Routing & Task Taxonomy

## 1. Routing Strategy (Local-Only)
We will route tasks based on their complexity to the appropriate "model profile". For v1, we assume a single local model provider (e.g., Ollama/Llama.cpp) but structure the code to support switching.

**Profiles:**
- **Fast:** Low latency, for classification/simple formatting (e.g., `llama3.2:3b`).
- **Complex:** High reasoning, for planning/analysis (e.g., `llama3.1:8b` or `mistral`).

## 2. Taxonomy (Task Classification)
Every user request is classified into a `TaskType` before execution.

```rust
pub enum TaskType {
    GeneralChat,    // Simple conversation
    CodeAnalysis,   // Reading/reasoning about code
    Planning,       // Generating steps/todos
    DataProcessing, // JSON/Text manipulation
}
```

## 3. Router Module (`src-tauri/src/router/`)
- `Classifier`: Analyzes input text to determine `TaskType`.
- `ModelClient`: Abstract trait for model interaction (HTTP to localhost:11434 for Ollama).
- `Router`: Selects the model based on `TaskType` and config.

## 4. Configuration
stored in `preferences`:
```json
{
  "models": {
    "fast": "llama3.2:3b",
    "complex": "llama3.1:8b",
    "endpoint": "http://localhost:11434"
  }
}
```

## 5. Privacy Guardrail
The router **MUST** check the `RuntimeState`. If `Paused`, no request is sent to the model interface, effectively severing the "brain" from the body.
