# Desktop App Architecture

## Components
- Desktop Shell (Tauri/Electron)
- UI Layer (canonical UI)
- Local Runtime Engine
- Model Router
- Memory Store
- Connector Layer

## Key Principles
- UI and runtime are decoupled
- Runtime runs with or without UI open
- All state is local-first
