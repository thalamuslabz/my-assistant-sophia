# Desktop App v1 — Detailed Sprint Implementation Plan

## Purpose
Build the desktop-first, local-only v1 app with explicit trust guarantees, contract enforcement, and release readiness. Scope strictly excludes cloud execution.

## Inputs (Authoritative Docs)
- `docs/Deep_Operational_Assistant_Vision_FINAL/10_Requirements_v1.md`
- `docs/Desktop_App_Release_Readiness_Package/*`
- `docs/Internal_Commitments_and_Operating_Contracts/*`

## Architecture Decisions (v1)
- Desktop shell: Tauri
- Runtime: local-only service process, decoupled from UI
- Data: local-first, versioned, exportable
- Security: BYOK encrypted at rest, never logged
- Behavioral guardrails: pause always respected, no silent behavior change

## Sprint 0 — Repo + Process Foundations
**Goal:** establish structure, governance artifacts, and testing evidence pipeline.

### Deliverables
- Repository scaffold and `.gitignore`
- Docs organized under `docs/`
- Sprint plan + artifacts registry
- Test evidence templates + hashing script

### Acceptance Criteria
- Documentation lives under `docs/` and is indexable
- Planning artifacts exist under `planning/`
- Test evidence artifacts can be hashed and recorded

---

## Sprint 1 — Core Runtime Skeleton
**Goal:** runtime scaffolding with explicit state machine + logging.

### Scope
- Local runtime lifecycle (Stopped/Starting/Running/Paused/Error)
- Health monitoring + restart policy
- Runtime loop orchestration scaffold (Observe→Interpret→Engage→Act→Verify→Log)
- Audit log pipeline

### Deliverables
- Runtime service skeleton
- State machine + lifecycle transitions
- Log schema and file sink

### Acceptance Criteria
- State transitions are explicit and tested
- Logs show all actions and state changes
- Pause state halts all actions

---

## Sprint 2 — Data & Memory Foundations
**Goal:** establish versioned, auditable local storage.

### Scope
- Understanding Snapshot store
- Decision Memory store
- Preferences store
- Exportable data schema

### Deliverables
- Storage layer with versioning and audit trail
- Export API

### Acceptance Criteria
- Data writes are versioned and auditable
- Export includes snapshot + preferences

---

## Sprint 3 — Onboarding & Assistant Contract
**Goal:** implement onboarding wizard and contract enforcement hooks.

### Scope
- Onboarding wizard (initial + rerun diff/confirm)
- Capture run mode (local) + model preferences
- Assistant Contract definition + enforcement hooks

### Deliverables
- Onboarding UX flow
- Contract schema and enforcement points

### Acceptance Criteria
- Wizard can be re-run with diff confirmation
- Contract is persisted and enforced

---

## Sprint 4 — Model Routing v1
**Goal:** explicit task-based routing with local-only models.

### Scope
- Task class taxonomy and classifier
- Routing policy engine
- Configurable model mapping

### Deliverables
- Routing engine with explicit config
- Task classification tests

### Acceptance Criteria
- Classification uses fast model
- Execution uses stable model
- All routing choices logged

---

## Sprint 5 — UI Release Checklist
**Goal:** implement required UI surfaces for release.

### Scope
- Transparency panel
- Action approval queue
- Pause/resume control
- Run-state indicator

### Deliverables
- UI surfaces wired to runtime

### Acceptance Criteria
- UI checklist fully satisfied
- No action occurs without approval

---

## Sprint 6 — Release Readiness
**Goal:** packaging, QA, and release gate compliance.

### Scope
- Signed binaries
- Optional auto-update
- Kill switch + feature flags
- Test matrix execution

### Deliverables
- Release candidate build
- Completed QA results + hashed evidence

### Acceptance Criteria
- All v1 features shipped
- No critical bugs
- Test evidence hashed and recorded

---

## QA/Test Evidence Policy
- All required tests recorded in `/test-evidence/`
- Each test run generates a results file and a SHA256 hash
- Hashes recorded in `planning/TEST_EVIDENCE_INDEX.md`

## Risks & Dependencies
- Local model availability across OS targets
- OS-specific keychain APIs for BYOK storage
- Packaging and signing per platform
