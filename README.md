# Sophia Assistant v1.2 — Persistent & Tracked

Local-first desktop assistant with cloud AI integration, persistent storage, and cost tracking.

## Current Release: v1.2.0 (January 15, 2026)

**New in v1.2:**
- 🔐 **Persistent API Keys** - Keys encrypted and saved across restarts (AES-256-GCM)
- 📊 **Usage Dashboard** - Real-time cost tracking and usage statistics
- 💰 **Cost Management** - See exactly how much you're spending
- 🚀 **All Providers Ready** - 6 providers fully implemented (Gemini, OpenAI, Anthropic, DeepSeek, OpenRouter, Ollama)
- ✅ **24 Unit Tests** - Comprehensive test coverage

**Quick Start:**
1. `npm install`
2. `npm run tauri dev`
3. Complete onboarding with your Gemini API key
4. Keys automatically persist - no more re-entering!
5. Monitor your costs in the Usage Dashboard

See `RELEASE_NOTES_v1.2.0.md` for full details.

**Upgrading from v1.1?** Just re-enter your API keys once - they'll persist forever!

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
