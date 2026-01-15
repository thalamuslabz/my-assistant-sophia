# Release Notes - Sophia Assistant v1.1.0

**Release Date:** January 15, 2026  
**Codename:** BYOK (Bring Your Own Key)

## 🎉 What's New

### Gemini API Integration
- **Cloud AI Access:** Connect to Google's Gemini API using your own API key
- **Model Support:** gemini-2.5-flash-lite (fast, cost-effective)
- **Direct Connection:** All API calls go directly to Google - no intermediaries

### Multi-Provider Architecture
- **Provider Registry:** Gemini-first priority with automatic fallback
- **Supported Providers:** Gemini (active), OpenAI, Anthropic, DeepSeek, OpenRouter, Ollama (code-complete)
- **Easy Switching:** Change providers and models through Settings panel

### Enhanced Onboarding
- **Gemini Key Collection:** Guided setup during first launch
- **Network Consent:** Explicit permission for cloud API calls
- **Privacy First:** Clear explanation of data flow

### Settings Panel
- **Provider Management:** Select and configure AI providers
- **API Key Storage:** Secure key management
- **Model Override:** Customize which model to use
- **Reset Configuration:** Quick reset to defaults

## 🔧 Technical Improvements

### Architecture
- **SecretStore:** Abstraction layer for secure key storage
- **Provider Registry:** Centralized provider management with fallback logic
- **Model Router:** Intelligent routing based on provider availability

### API Clients
- **HTTP Client:** Switched to `ureq` for better async compatibility
- **Timeout Protection:** 30-second timeout on all API calls
- **Error Handling:** Detailed error messages with status codes

### Developer Experience
- **Tauri 2.x Compatibility:** Fixed parameter naming conventions
- **Logging:** Comprehensive debug logging for troubleshooting
- **Hot Reload:** Faster development iteration

## ⚠️ Known Issues

### API Key Persistence
**Issue:** API keys are stored in memory and don't persist across app restarts.

**Impact:** Users must re-enter their API keys each time they launch the app.

**Workaround:**
1. Launch the app
2. Go to Settings
3. Select your provider (e.g., Gemini)
4. Enter your API key
5. Click "Save Key"

**Status:** Fix planned for v1.2 (encrypted file storage or system keychain integration)

### Provider Testing
**Issue:** Only Gemini has been fully tested. Other providers (OpenAI, Anthropic, DeepSeek, OpenRouter) are code-complete but untested.

**Impact:** Other providers may have bugs or API compatibility issues.

**Recommendation:** Stick with Gemini for v1.1, or test other providers at your own risk.

## 📋 Requirements

### System Requirements
- **OS:** macOS 10.15+, Windows 10+, or Linux
- **Memory:** 4GB RAM minimum
- **Disk:** 500MB free space
- **Network:** Internet connection for cloud API calls

### API Keys
To use Sophia v1.1, you need:
- **Gemini API Key** (Required): Get one at https://makersuite.google.com/app/apikey
- **Other Provider Keys** (Optional): OpenAI, Anthropic, DeepSeek, OpenRouter

## 🚀 Getting Started

### First Launch
1. Start the application
2. Complete the onboarding wizard:
   - Read the welcome message
   - Review privacy information
   - Enter your Gemini API key
   - Accept network egress consent
   - Sign the operating contract
3. Start chatting!

### Using the Chat Interface
1. Type your message in the input field
2. Click "Send" or press Enter
3. Wait for Gemini's response
4. View conversation history above

### Changing Providers (Advanced)
1. Open the Settings panel (right side)
2. Select a different provider from the dropdown
3. Enter the API key for that provider
4. Click "Save Key"
5. Your next message will use the new provider

## 🔒 Privacy & Security

### Data Flow
- **User Input** → **Local App** → **Direct HTTPS** → **Provider API** (e.g., Gemini)
- **No Intermediaries:** Your data goes directly to the AI provider
- **No Telemetry:** We don't collect usage data or analytics

### API Key Security
- Keys are stored in memory (not written to disk in v1.1)
- Keys are never logged in plaintext
- Keys are transmitted over HTTPS only

### Network Calls
All network calls audited and verified to go directly to provider APIs:
- ✅ Gemini: `generativelanguage.googleapis.com`
- ✅ OpenAI: `api.openai.com`
- ✅ Anthropic: `api.anthropic.com`
- ✅ DeepSeek: `api.deepseek.com`
- ✅ OpenRouter: `openrouter.ai`
- ✅ Ollama: `localhost:11434` (local only)

## 📚 Documentation

- **User Guide:** `docs/USER_GUIDE.md`
- **Architecture:** `planning/v1.1/ARCHITECTURE_BYOK.md`
- **Sprint Plan:** `planning/v1.1/SPRINT_PLAN_BYOK.md`
- **Test Evidence:** `test-evidence/gate5_approval_20260115.md`
- **Network Audit:** `test-evidence/network_audit_20260115.md`

## 🐛 Reporting Issues

If you encounter bugs or have feature requests:
1. Check the Known Issues section above
2. Review existing issues in the repository
3. Create a new issue with:
   - Steps to reproduce
   - Expected vs actual behavior
   - System information
   - Relevant log output

## 🙏 Acknowledgments

- **Tauri Team:** For the excellent desktop framework
- **Google:** For the Gemini API
- **Community:** For testing and feedback

## 📅 Roadmap

### v1.2 (Planned)
- Persistent API key storage (encrypted file or system keychain)
- Full testing of all provider clients
- Usage statistics and cost tracking
- Provider health monitoring

### v2.0 (Future)
- Multi-model conversations
- Context management and memory
- Plugin system
- Advanced routing strategies

---

**Thank you for using Sophia Assistant!**

For support, visit: [Repository URL]
