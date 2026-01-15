# BYOK Architecture (v1.1)

## 1. Provider Registry Pattern

The system will manage multiple LLM providers through a centralized registry that handles:

- **Provider Registration:** Each provider implements the `LLMProvider` trait
- **Key Management:** Secure storage using system keychain APIs
- **Fallback Routing:** Automatic provider switching based on availability/cost
- **Capability Mapping:** Each provider exposes its available models and limits

## 2. Provider Priority Strategy

### Primary Strategy: Gemini-First
1. **Gemini (Google):** Priority 1 (Free tier available, Gemini 1.5 Flash)
2. **DeepSeek:** Priority 2 (Economic, high performance)
3. **OpenAI:** Priority 3 (Premium, GPT-4o)
4. **Anthropic:** Priority 4 (Premium, Claude 3.5)
5. **Ollama (Local):** Fallback when network unavailable

### Fallback Logic
If primary provider fails or reaches rate limit, system automatically tries next provider.

## 3. Secure Storage Implementation

**Key Storage:**
- macOS: `Security Framework` (Keychain Services)
- Windows: `DPAPI` via appropriate crate
- Linux: `Secret Service` or encrypted file

**Audit Trail:** All key storage operations logged with `REDACTED` placeholder.

## 4. Onboarding Flow Modification

The v1 onboarding wizard will be extended:

1. Welcome
2. Privacy Education
3. **Gemini Key Collection** (Required for "Cloud" mode)
4. Contract Agreement
5. Optional: Additional Provider Setup

## 5. API Endpoints (Direct Connection)

| Provider | API Endpoint | Default Models |
|---|---|---|
| Google Gemini | `https://generativelanguage.googleapis.com/v1` | `gemini-1.5-flash`, `gemini-1.5-pro` |
| OpenAI | `https://api.openai.com/v1` | `gpt-4o`, `gpt-3.5-turbo` |
| Anthropic | `https://api.anthropic.com/v1` | `claude-3-5-sonnet`, `claude-3-haiku` |
| DeepSeek | `https://api.deepseek.com/v1` | `deepseek-chat`, `deepseek-coder` |

## 6. Cost Control

- Token counting per provider
- Usage quotas per billing cycle
- Automatic fallback if budget exceeded
- Cost estimation display in UI