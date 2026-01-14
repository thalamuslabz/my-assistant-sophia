# Sprint 3 Architecture: Onboarding & Assistant Contract

## 1. Onboarding State Machine (Frontend)
The onboarding wizard guides the user through the initial setup and contract agreement.

**States:**
1.  **Welcome:** Introduction to Local-First philosophy.
2.  **Privacy_Education:** Explains "Explicit Trust" and BYOK (Bring Your Own Key).
3.  **Model_Selection:** User selects local model path or API provider (if/when allowed).
4.  **Contract_Review:** Displays the Operating Contract (Pause is absolute, No training on user data, etc.). User must explicitly "Sign" (Checkbox/Button).
5.  **Complete:** Generates initial Snapshot v1 and transitions Runtime to `Running`.

## 2. Assistant Contract (Backend)
The Contract is not just UI text; it is a cryptographically verifiable commitment.

**Storage:**
- The accepted contract text/hash is stored in `preferences` table under `contract_signature`.
- **Enforcement:** On every startup, the runtime checks if a valid contract signature exists. If not, it forces the Onboarding state.

## 3. Data Flow
1.  **UI:** User accepts terms.
2.  **IPC:** `accept_contract(terms_hash: String)` -> Rust.
3.  **Rust:** 
    - Verifies hash matches known current contract.
    - Stores `contract_accepted_at`, `contract_version`, and `contract_hash` in `preferences`.
    - Creates `Snapshot v1` (Empty/Initial).
    - Sets flag `onboarding_completed = true`.

## 4. Module Updates
- **`src-tauri/src/onboarding/`**: New module.
- **`src/components/onboarding/`**: New React components.

## 5. Security
- **Explicit Consent:** The "Sign" action is a critical user intent event. It must be logged to the Audit Log.
