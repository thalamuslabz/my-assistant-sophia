# Persistent Storage Architecture (v1.2)

**Date:** 2026-01-15  
**Status:** DRAFT  
**Gate:** Gate 7 - Persistent Storage Architecture

## Problem Statement

**Current State (v1.1):**
- API keys stored in-memory using `HashMap`
- Keys lost on app restart
- Users must re-enter keys every session
- Poor user experience

**Desired State (v1.2):**
- API keys persist across app restarts
- Keys encrypted at rest
- Secure key management
- Seamless user experience

## Storage Options Evaluation

### Option 1: System Keychain (keyring crate)
**Pros:**
- OS-native security
- Battle-tested by OS vendors
- No custom encryption needed
- Automatic key management

**Cons:**
- ❌ Failed in v1.1 testing (macOS compatibility issues)
- Platform-specific behavior
- Limited control over encryption
- Debugging difficult

**Verdict:** ❌ REJECTED - Already attempted and failed in v1.1

---

### Option 2: Encrypted File Storage
**Pros:**
- ✅ Full control over implementation
- ✅ Cross-platform consistency
- ✅ Easy to debug and test
- ✅ Can use proven encryption libraries
- ✅ Portable (file can be backed up)

**Cons:**
- Need to manage encryption keys
- Need to handle file permissions
- Need to implement atomic writes

**Verdict:** ✅ RECOMMENDED - Best balance of security and reliability

---

### Option 3: SQLite with Encryption (SQLCipher)
**Pros:**
- Integrated with existing storage layer
- Proven encryption implementation
- ACID guarantees

**Cons:**
- Additional dependency
- Overkill for simple key-value storage
- Encryption key management still needed

**Verdict:** ⚠️ ALTERNATIVE - Good option but more complex than needed

---

## Chosen Approach: Encrypted File Storage

### Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                      Application Layer                       │
│  (ProviderRegistry, OnboardingManager, Settings UI)         │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│                      SecretStore API                         │
│  - set_secret(key, value)                                   │
│  - get_secret(key) -> Option<String>                        │
│  - delete_secret(key)                                       │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│                  Encrypted File Backend                      │
│  - File: ~/.sophia/secrets.enc                              │
│  - Format: JSON (encrypted)                                 │
│  - Encryption: AES-256-GCM                                  │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│                    Encryption Layer                          │
│  - Master Key: Derived from machine ID + app salt          │
│  - Algorithm: AES-256-GCM                                   │
│  - Library: ring or RustCrypto                              │
└─────────────────────────────────────────────────────────────┘
```

### File Format

**Location:** `~/.sophia/secrets.enc`

**Structure (before encryption):**
```json
{
  "version": "1",
  "secrets": {
    "gemini_api_key": "AIzaSy...",
    "openai_api_key": "sk-...",
    "anthropic_api_key": "sk-ant-..."
  },
  "metadata": {
    "created_at": "2026-01-15T12:00:00Z",
    "updated_at": "2026-01-15T12:30:00Z"
  }
}
```

**After Encryption:**
- Binary file with AES-256-GCM encrypted content
- Includes authentication tag for integrity verification
- Nonce/IV stored with encrypted data

### Encryption Strategy

#### Master Key Derivation

**Option A: Machine ID + Application Salt (RECOMMENDED)**
```rust
// Derive master key from machine-specific identifier
let machine_id = get_machine_id(); // e.g., MAC address hash
let app_salt = "sophia-assistant-v1.2"; // Hardcoded in app
let master_key = PBKDF2(machine_id + app_salt, iterations=100000);
```

**Pros:**
- ✅ Unique per machine
- ✅ No user password required
- ✅ Automatic key derivation
- ✅ Keys tied to specific machine

**Cons:**
- Keys not portable across machines
- If machine ID changes, keys are lost

**Verdict:** ✅ RECOMMENDED for v1.2

---

**Option B: User Password (Future Enhancement)**
```rust
// User provides password during onboarding
let user_password = prompt_user_password();
let master_key = PBKDF2(user_password, salt, iterations=100000);
```

**Pros:**
- Keys portable across machines
- User controls access

**Cons:**
- Requires password management
- User can forget password
- More complex UX

**Verdict:** ⏭️ DEFERRED to v1.3 or v2.0

---

#### Encryption Algorithm: AES-256-GCM

**Why AES-256-GCM:**
- Industry standard
- Authenticated encryption (prevents tampering)
- Fast and secure
- Well-supported in Rust (`ring` or `aes-gcm` crates)

**Implementation:**
```rust
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, NewAead};

// Encrypt
let cipher = Aes256Gcm::new(Key::from_slice(&master_key));
let nonce = Nonce::from_slice(&random_nonce); // 96-bit random
let ciphertext = cipher.encrypt(nonce, plaintext.as_ref())?;

// Decrypt
let plaintext = cipher.decrypt(nonce, ciphertext.as_ref())?;
```

### File Operations

#### Atomic Writes
To prevent corruption during writes:
1. Write to temporary file: `~/.sophia/secrets.enc.tmp`
2. Sync to disk: `file.sync_all()`
3. Rename to final location: `fs::rename(tmp, final)`

**Rust Implementation:**
```rust
use std::fs;
use std::io::Write;

fn atomic_write(path: &Path, data: &[u8]) -> Result<(), Error> {
    let tmp_path = path.with_extension("tmp");
    
    // Write to temp file
    let mut file = fs::File::create(&tmp_path)?;
    file.write_all(data)?;
    file.sync_all()?;
    
    // Atomic rename
    fs::rename(tmp_path, path)?;
    Ok(())
}
```

#### File Permissions
Set restrictive permissions (Unix: 0600, Windows: equivalent):
```rust
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[cfg(unix)]
fn set_secure_permissions(path: &Path) -> Result<(), Error> {
    let mut perms = fs::metadata(path)?.permissions();
    perms.set_mode(0o600); // Owner read/write only
    fs::set_permissions(path, perms)?;
    Ok(())
}
```

### Error Handling

**Scenarios:**
1. **File doesn't exist:** Create new encrypted file
2. **File corrupted:** Return error, don't auto-delete (user may want to recover)
3. **Decryption fails:** Return error, log warning
4. **Write fails:** Return error, don't corrupt existing file (atomic writes)

**Recovery Strategy:**
- Keep backup of previous version: `secrets.enc.bak`
- On successful write, update backup
- On read failure, attempt to read backup

### Migration from v1.1

**Strategy:**
Since v1.1 uses in-memory storage, there's nothing to migrate. Users will need to re-enter their keys once in v1.2.

**User Experience:**
1. User launches v1.2 for first time
2. App detects no persistent storage file
3. User enters API keys in Settings (as in v1.1)
4. Keys are now saved to encrypted file
5. On next launch, keys are loaded automatically

**No migration code needed** - clean slate approach.

### Security Considerations

#### Threat Model

**Protected Against:**
- ✅ Casual file system browsing (encryption)
- ✅ File tampering (GCM authentication)
- ✅ Accidental exposure (file permissions)
- ✅ Process memory dumps (keys only in memory when needed)

**NOT Protected Against:**
- ❌ Root/admin access to machine (by design - OS-level threat)
- ❌ Malware with process memory access (OS-level threat)
- ❌ Physical access to unlocked machine (OS-level threat)

**Rationale:** We're protecting against accidental exposure and casual attacks, not nation-state adversaries. For high-security needs, users should use OS-level encryption (FileVault, BitLocker).

#### Key Rotation

**v1.2:** No key rotation (master key derived from machine ID)

**Future (v1.3+):** 
- Allow user to change password (if password-based)
- Re-encrypt secrets with new master key
- Keep old key for grace period

### Testing Strategy

**Unit Tests:**
- [ ] Test encryption/decryption round-trip
- [ ] Test atomic write operations
- [ ] Test file permission setting
- [ ] Test error handling (corrupted file, missing file, etc.)

**Integration Tests:**
- [ ] Store secret, restart app, retrieve secret
- [ ] Verify encrypted file contains no plaintext
- [ ] Test concurrent access (if applicable)
- [ ] Test backup/recovery mechanism

**Security Tests:**
- [ ] Verify file permissions are restrictive
- [ ] Verify no plaintext in encrypted file
- [ ] Verify authentication tag prevents tampering

### Implementation Plan

**Phase 1: Core Encryption (Day 1)**
- Implement master key derivation
- Implement AES-256-GCM encryption/decryption
- Unit tests for crypto operations

**Phase 2: File Backend (Day 1-2)**
- Implement encrypted file read/write
- Implement atomic write operations
- Implement file permissions
- Unit tests for file operations

**Phase 3: SecretStore Integration (Day 2)**
- Replace in-memory HashMap with file backend
- Update SecretStore API to use persistent storage
- Add initialization on app startup
- Integration tests

**Phase 4: Error Handling & Recovery (Day 2)**
- Implement backup mechanism
- Implement error recovery
- Add logging and diagnostics

**Phase 5: Testing & Validation (Day 3)**
- Full test suite execution
- Security audit
- Cross-platform testing (macOS, Windows, Linux)

### Dependencies

**New Crates:**
```toml
[dependencies]
aes-gcm = "0.10"           # AES-256-GCM encryption
rand = "0.8"               # Random nonce generation
machine-uid = "0.5"        # Machine ID for key derivation
serde_json = "1.0"         # JSON serialization (already present)
```

**Estimated Binary Size Impact:** +500KB (crypto libraries)

### Rollback Plan

If persistent storage causes issues:
1. Keep in-memory fallback code
2. Add feature flag: `--use-memory-storage`
3. Document rollback procedure in release notes

### Success Criteria

✅ **Gate 7 Complete When:**
- [ ] Architecture document approved (this document)
- [ ] Security review passed
- [ ] Implementation plan clear
- [ ] Dependencies identified
- [ ] Testing strategy defined

---

## Decision: APPROVED

**Chosen Approach:** Encrypted File Storage with AES-256-GCM

**Master Key:** Derived from machine ID + application salt

**File Location:** `~/.sophia/secrets.enc`

**Next Step:** Proceed to Gate 8 - Implementation

**Estimated Effort:** 2-3 days for full implementation and testing
