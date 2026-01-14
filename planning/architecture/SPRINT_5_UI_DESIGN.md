# Sprint 5 Architecture: UI & Release Readiness

## 1. UI Components (React)
We need to visualize the backend state and controls implemented in previous sprints.

- **StatusIndicator:** Real-time display of `RuntimeState` (Running/Paused/Stopped).
- **PauseControl:** Big red/green button to toggle the global pause state.
- **TransparencyPanel:** Live feed of the `Audit Log` and `Decision Memory`.
- **ChatInterface:** Simple input/output connected to `submit_prompt`.

## 2. State Management
- Use `useQuery` or `useEffect` polling (v1) to sync `RuntimeState` every 500ms.
- Use `Tauri Events` (if available) or polling for log updates.

## 3. Action Approval Queue (Visual Only for v1)
Since v1 is "Passive" (Assistant doesn't autonomously execute tools yet), the approval queue is a placeholder for the "Chat" response. The user implicitly "approves" by reading it.
*Future:* When tools are added, this component will intercept `tool_call` events.

## 4. Release Polish
- Error Boundary for React.
- Loading states.
- Clean CSS (basic styling).
