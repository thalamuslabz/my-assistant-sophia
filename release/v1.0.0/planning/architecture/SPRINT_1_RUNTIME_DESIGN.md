# Sprint 1 Architecture: Core Runtime Skeleton

## 1. Module Structure (Rust)
The core runtime will be isolated from the Tauri UI layer to ensure strict separation of concerns and testability.

```
src-tauri/
  src/
    main.rs            # Entry point, Tauri setup
    runtime/           # Core domain logic (Headless)
      mod.rs           # Public Interface
      state.rs         # State Machine definition
      manager.rs       # Lifecycle orchestration
      audit.rs         # Structured logging sink
```

## 2. State Machine Design
We will use a strictly typed Rust Enum for states, avoiding invalid intermediate states.

### States (`RuntimeState`)
1.  **Stopped**: Initial state. No processing.
2.  **Starting**: Bootstrapping, health checks.
3.  **Running**: Active loop (Observe -> Act).
4.  **Paused**: Explicit user override. Safe state.
5.  **Error**: Critical failure requiring intervention.

### Allowed Transitions
- `Stopped` -> `Starting`
- `Starting` -> `Running`
- `Starting` -> `Error`
- `Running` -> `Paused` (User Action)
- `Running` -> `Error` (System Fault)
- `Paused` -> `Running` (User Action)
- `Error` -> `Stopped` (Reset)
- `*` -> `Stopped` (Shutdown)

## 3. Audit Log Schema
All state changes and core actions must be logged to a local JSONL file.

**Schema:**
```json
{
  "timestamp": "ISO-8601",
  "level": "INFO|WARN|ERROR",
  "component": "runtime",
  "event": "state_transition",
  "context": {
    "from": "Running",
    "to": "Paused",
    "reason": "User requested pause"
  },
  "signature": "SHA256(previous_log + this_log)" // Chain for integrity (Sprint 2+)
}
```

## 4. Scaffolding Strategy
- **Frontend:** React + TypeScript (Vite)
- **Backend:** Rust (Tauri)
- **IPC:** Commands for `get_state`, `set_paused`, `get_logs`.
