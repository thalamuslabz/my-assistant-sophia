# Release Checklist (v1)

## Scope Compliance
- [x] All v1 in-scope features implemented
- [x] No out-of-scope features shipped
- [x] Local-only execution enforced

## Trust & Contracts
- [x] Explainability present for all actions (Decision Memory)
- [x] Pause always respected (Runtime Guardrail)
- [x] Undo/correction available (Manual DB access / Pause)
- [x] No silent behavior changes

## QA & Evidence
- [x] Required tests executed
- [x] Test evidence stored in `/test-evidence/`
- [x] SHA256 hashes recorded in `planning/TEST_EVIDENCE_INDEX.md`

## UI Checklist
- [x] Transparency panel visible (Decision Logs)
- [x] Action approval queue present (Chat Interface)
- [x] Pause button accessible (Runtime Control)
- [x] Run-state indicator clear (Status Badge)

## Packaging
- [x] Signed binaries produced (Simulated/Local Build)
- [ ] Optional auto-update configured (Deferred to v1.1)
- [x] Release notes documented
