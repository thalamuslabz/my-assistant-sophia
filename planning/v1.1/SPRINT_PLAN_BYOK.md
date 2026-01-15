# Sprint Plan v1.1 — BYOK & Cloud Models

## Context
v1.0.0 established the local runtime and Ollama support.
v1.1 expands "Local-Only" to include **Bring Your Own Key (BYOK)** support with Gemini-First approach, maintaining the "Direct Connection" architecture (no intermediate SaaS).

**Gemini-First Strategy:** During onboarding, users will be guided to obtain a Gemini API key first, followed by optional setup for other providers (OpenAI, Anthropic, DeepSeek).

## Gates

### Gate 4 — BYOK Architecture
- [ ] Provider Abstraction Trait defined
- [ ] Secure Storage Strategy (Keyring/Encryption) approved
- [ ] Gemini-first onboarding flow wireframes
- [ ] API Provider selection strategy

### Gate 5 — Implementation Complete
- [ ] Gemini API Client implemented (Gemini 1.5 Flash/Pro)
- [ ] OpenAI, Anthropic, DeepSeek Clients implemented
- [ ] Keys stored securely (not plain text in logs/DB)
- [ ] User can switch between providers with fallback strategy
- [ ] Gemini-first onboarding tested

## Sprints

### Sprint 7 — Secure Storage & Provider Registry
**Goal:** Foundational architecture for multiple provider support.
- **Task:** Refactor `LLMClient` into a registry pattern (`ProviderRegistry`).
- **Task:** Implement `SecretStore` using system keychain.
- **Task:** Define provider capabilities (model lists, token limits).

### Sprint 8 — Gemini-First Implementation
**Goal:** Gemini API integration and prioritized onboarding.
- **Task:** `GeminiClient` implementation with Gemini 1.5 Flash/Pro.
- **Task:** Modify onboarding wizard to prioritize Gemini key collection.
- **Task:** Network permission logic with explicit warnings.

### Sprint 9 — Additional Provider Integration
**Goal:** Expand provider support with fallback strategy.
- **Task:** `OpenAIClient` implementation (GPT-4o, GPT-3.5).
- **Task:** `AnthropicClient` implementation (Claude 3.5 Sonnet/Haiku).
- **Task:** `DeepSeekClient` implementation (V2/V3 models).
- **Task:** Provider fallback logic based on cost/performance.

### Sprint 10 — Configuration UI
**Goal:** Advanced settings and provider management.
- **Task:** Provider Settings Page with prioritized Gemini placement.
- **Task:** API Key input with validation/masking.
- **Task:** Connection testing for each provider.
- **Task:** Usage statistics and cost tracking.
