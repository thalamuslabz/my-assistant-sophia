# Sophia Assistant v1.1 — BYOK (Bring Your Own Key)

Local-first desktop assistant with cloud AI integration and explicit trust guardrails.

## Current Release: v1.1.0 (January 15, 2026)

**New in v1.1:**
- ✅ Gemini API integration (gemini-2.5-flash-lite)
- ✅ Multi-provider architecture (Gemini, OpenAI, Anthropic, DeepSeek, OpenRouter, Ollama)
- ✅ Secure API key management
- ✅ Provider switching via Settings panel

**Quick Start:**
1. `npm install`
2. `npm run tauri dev`
3. Complete onboarding with your Gemini API key
4. Start chatting!

See `RELEASE_NOTES_v1.1.0.md` for full details.

## Repo Layout
- `docs/` — source product and release documentation
- `planning/` — sprint plan and evidence registry
- `scripts/` — helper scripts (e.g., test result hashing)
- `templates/` — test evidence templates
- `test-evidence/` — test run artifacts (hashed)

## Gates
- No coding before sprint plan approval
- Sprint acceptance checklist required
- All test results recorded and hashed
- See `planning/GATES.md`
- Sprint plan template at `planning/SPRINT_PLAN_TEMPLATE.md`
- Changelog at `planning/CHANGELOG.md`
- Decisions log at `planning/DECISIONS.md`
- Test plan matrix at `planning/TEST_PLAN_MATRIX.md`
- Risk register at `planning/RISK_REGISTER.md`
- Release checklist at `planning/RELEASE_CHECKLIST.md`
- Test evidence storage guide at `test-evidence/README.md`
