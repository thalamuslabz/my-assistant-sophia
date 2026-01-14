# Test Plan Matrix

## Required Tests (v1)
| Test | Feature/Sprint | Evidence Artifact | Notes |
|---|---|---|---|
| Runtime State Machine Transitions | Sprint 1 — Runtime | `test-evidence/runtime-state-machine/` | Unit tests for all valid/invalid transitions |
| Audit Log Writing | Sprint 1 — Runtime | `test-evidence/audit-log-writing/` | Verify JSONL format and persistence |
| Pause / resume behavior | Sprint 1 — Runtime | `test-evidence/pause-resume/` | Pause enforcement |
| Snapshot Versioning | Sprint 2 — Data | `test-evidence/snapshot-versioning/` | Verify new versions created on save |
| Data Persistence | Sprint 2 — Data | `test-evidence/data-persistence/` | Verify DB survives restart |
| Full Export | Sprint 2 — Data | `test-evidence/full-export/` | Verify JSON export contains all data |
| Onboarding happy path | Sprint 3 — Onboarding | `test-evidence/onboarding-happy-path/` | Wizard flow, snapshot creation |
| Action approval flow | Sprint 5 — UI | `test-evidence/action-approval/` | No action without approval |
| Model routing correctness | Sprint 4 — Routing | `test-evidence/model-routing/` | Task class → model mapping |

## Non-Functional
| Test | Feature/Sprint | Evidence Artifact | Notes |
|---|---|---|---|
| Startup reliability | Sprint 1 — Runtime | `test-evidence/startup-reliability/` | Repeated cold starts |
| Memory persistence | Sprint 2 — Data | `test-evidence/memory-persistence/` | Restart preserves state |
| Failure recovery | Sprint 1 — Runtime | `test-evidence/failure-recovery/` | Crash + auto-restart |
