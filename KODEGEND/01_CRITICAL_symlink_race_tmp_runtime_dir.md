# CRITICAL: /tmp Symlink Race Condition in runtime_dir()

## Severity
**CRITICAL - SECURITY VULNERABILITY (CWE-59, CWE-362)**

## Location
`packages/kodegend/src/platform/unix.rs:86-95` - `platform_runtime_dir()` function

## Vulnerability Summary

The `platform_runtime_dir()` function creates a runtime directory at `/tmp/kodegend-{uid}/kodegend` for non-elevated users without performing atomic, secure directory creation. This creates a **TOCTOU (Time-Of-Check-Time-Of-Use)** race condition vulnerable to symlink attacks, enabling:

- **PID file poisoning** - attacker controls daemon process tracking
- **Socket hijacking** - attacker intercepts daemon IPC
- **Privilege escalation** - via manipulated runtime state

### Attack Vector Analysis

An attacker (even same UID) can pre-create `/tmp/kodegend-{uid}` as a symlink pointing to an attacker-controlled directory. When kodegend starts:

1. `platform_runtime_dir()` returns `/tmp/kodegend-{uid}/kodegend`
2. `daemon.rs:60` calls `fs::create_dir_all(parent)` on `/tmp/kodegend-{uid}`
3. This follows the symlink and creates directories in attacker's location
4. PID file written to `{attacker-controlled-path}/kodegend/kodegend.pid`
5. Attacker can now manipulate PID files, inject fake process IDs, or hijack daemon operations

**Race Condition Extension**: Attackers can artificially extend the TOCTOU window by creating deep symlink chains (symlink → symlink → ...), increasing attack success probability to near 100%.

### Current Vulnerable Code

```rust
// packages/kodegend/src/platform/unix.rs:86-95
pub fn platform_runtime_dir(is_elevated: bool) -> PathBuf {
    if is_elevated {
        PathBuf::from("/var/run/kodegend")
    } else {
        std::env::var("XDG_RUNTIME_DIR")
            .ok()
            .map(PathBuf::from)
            .or_else(dirs::runtime_dir)
            .unwrap_or_else(|| {
                // ⚠️ VULNERABLE: No symlink check, no atomic creation
                PathBuf::from(format!("/tmp/kodegend-{}", geteuid()))
            })
            .join("kodegend")  // Results in /tmp/kodegend-{uid}/kodegend
    }
}
```

**Note**: The `.join("kodegend")` happens AFTER `unwrap_or_else`, creating TWO directories that must be secured:
1. Base: `/tmp/kodegend-{uid}`
2. Subdir: `/tmp/kodegend-{uid}/kodegend`

Both must be validated to prevent symlink attacks at any level.

### Impact Chain

The vulnerable directory path flows through:

1. **config.rs:65** - `default_pid_file()` calls `platform::runtime_dir(is_elevated).join("kodegend.pid")`
2. **daemon.rs:60** - PID file creation uses `fs::create_dir_all(parent)` on the runtime directory
3. **Result**: PID file written to potentially attacker-controlled location

Related files discovered via search:
- `src/platform/unix.rs` (lines 82, 83, 87, 90, 92, 93)
- `src/platform/mod.rs` (lines 114, 117, 118)
- `src/config.rs` (lines 40, 65)
- `src/daemon.rs` (line 60, 169)

## Security Standards Violated

- **CWE-59**: Improper Link Resolution Before File Access ('Link Following')
- **CWE-362**: Concurrent Execution using Shared Resource with Improper Synchronization ('Race Condition')
- **CAPEC-27**: Leveraging Race Conditions via Symbolic Links
- **OWASP**: Insecure Temporary File Creation

### References
- [CWE-59 Official](https://cwe.mitre.org/data/definitions/59.html) - Link following vulnerabilities
- [CAPEC-27](https://capec.mitre.org/data/definitions/27.html) - Symlink race attack patterns
- [Nix crate documentation](https://docs.rs/nix/latest/nix/sys/stat/index.html) - Unix file metadata APIs
- [Rust std::fs](https://github.com/rust-lang/rust/blob/master/library/std/src/fs.rs) - Symlink security notes

## Required Fix Implementation

### Implementation Location
**File**: `packages/kodegend/src/platform/unix.rs`

### Step 1: Add Security Helper Function

Add a new helper function `ensure_secure_directory()` that creates and validates directories atomically:

```rust
use std::fs::{self, DirBuilder};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use nix::unistd::Uid;
use anyhow::{Context, Result, bail};

/// Securely ensure a directory exists with correct ownership and permissions
/// 
/// This function prevents symlink attacks by:
/// 1. Attempting atomic creation with restrictive permissions (0o700)
/// 2. If directory exists, validating it is NOT a symlink
/// 3. Verifying ownership matches expected UID
/// 4. Verifying permissions are 0o700 (owner-only access)
/// 
/// # Security Properties
/// - Fails-secure: Returns error rather than using unsafe directory
/// - TOCTOU-resistant: Validates after creation attempt, not before
/// - Defense-in-depth: Multiple validation layers
/// 
/// # Arguments
/// - `path`: Directory path to create/validate
/// - `expected_uid`: Expected owner UID (typically current user)
/// 
/// # Errors
/// Returns security error if:
/// - Path is a symlink (CWE-59 prevention)
/// - Ownership mismatch (privilege escalation prevention)
/// - Permissions too permissive (data exposure prevention)
fn ensure_secure_directory(path: &Path, expected_uid: Uid) -> Result<()> {
    // Step 1: Attempt atomic directory creation with restrictive permissions
    let mut builder = DirBuilder::new();
    builder.mode(0o700); // Owner-only: rwx------
    
    match builder.create(path) {
        Ok(_) => {
            // Successfully created - we own it with correct permissions
            log::debug!("Created secure directory: {}", path.display());
            return Ok(());
        }
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            // Directory exists - must validate it's safe to use
            log::debug!("Directory exists, validating security: {}", path.display());
        }
        Err(e) => {
            // Other error (permission denied, disk full, etc.)
            return Err(e).context(format!("Failed to create directory: {}", path.display()));
        }
    }
    
    // Step 2: Validate existing directory is NOT a symlink (CRITICAL)
    // Use symlink_metadata to avoid following symlinks
    let metadata = fs::symlink_metadata(path)
        .context(format!("Failed to read metadata for: {}", path.display()))?;
    
    if metadata.file_type().is_symlink() {
        bail!(
            "SECURITY: Directory is a symlink - potential attack detected: {}\n\
             This could be a CWE-59 symlink attack. Directory will not be used.",
            path.display()
        );
    }
    
    if !metadata.is_dir() {
        bail!(
            "SECURITY: Path exists but is not a directory: {}",
            path.display()
        );
    }
    
    // Step 3: Verify ownership matches expected UID
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        let file_uid = Uid::from_raw(metadata.uid());
        
        if file_uid != expected_uid {
            bail!(
                "SECURITY: Directory ownership mismatch: {}\n\
                 Expected UID: {}, Found UID: {}\n\
                 This could indicate a privilege escalation attack.",
                path.display(),
                expected_uid,
                file_uid
            );
        }
    }
    
    // Step 4: Verify permissions are sufficiently restrictive (0o700)
    let perms = metadata.permissions();
    let mode = perms.mode() & 0o777; // Extract permission bits
    
    if mode != 0o700 {
        bail!(
            "SECURITY: Directory has unsafe permissions: {}\n\
             Expected: 0o700 (rwx------), Found: 0o{:o}\n\
             Permissions must be owner-only to prevent privilege escalation.",
            path.display(),
            mode
        );
    }
    
    log::debug!("Validated secure directory: {} (uid={}, mode=0o{:o})", 
                path.display(), expected_uid, mode);
    Ok(())
}
```

### Step 2: Modify platform_runtime_dir()

Replace the current vulnerable implementation with a secure version:

```rust
/// Runtime directory for PID files and sockets
///
/// Elevated: /var/run/kodegend
/// User: $XDG_RUNTIME_DIR/kodegend or /tmp/kodegend-{uid}/kodegend (securely created)
///
/// # Security
/// 
/// When falling back to /tmp, this function ensures both the base directory
/// and subdirectory are created securely to prevent CWE-59 symlink attacks.
pub fn platform_runtime_dir(is_elevated: bool) -> PathBuf {
    if is_elevated {
        PathBuf::from("/var/run/kodegend")
    } else {
        // Try XDG_RUNTIME_DIR first (preferred, systemd provides this securely)
        if let Ok(xdg_runtime) = std::env::var("XDG_RUNTIME_DIR") {
            return PathBuf::from(xdg_runtime).join("kodegend");
        }
        
        // Try dirs::runtime_dir() (platform-specific secure runtime directory)
        if let Some(runtime) = dirs::runtime_dir() {
            return runtime.join("kodegend");
        }
        
        // Fallback: /tmp/kodegend-{uid}/kodegend with security validation
        let current_uid = geteuid();
        let base_dir = PathBuf::from(format!("/tmp/kodegend-{}", current_uid));
        let runtime_dir = base_dir.join("kodegend");
        
        // Securely create both base and subdirectory
        // This prevents symlink attacks at ANY level of the path
        if let Err(e) = ensure_secure_directory(&base_dir, current_uid) {
            log::error!(
                "Failed to create secure base runtime directory: {}\n\
                 Error: {}\n\
                 Daemon cannot start safely. Please check directory permissions.",
                base_dir.display(),
                e
            );
            panic!(
                "SECURITY: Cannot create secure runtime directory at {}: {}",
                base_dir.display(),
                e
            );
        }
        
        if let Err(e) = ensure_secure_directory(&runtime_dir, current_uid) {
            log::error!(
                "Failed to create secure runtime subdirectory: {}\n\
                 Error: {}\n\
                 Daemon cannot start safely.",
                runtime_dir.display(),
                e
            );
            panic!(
                "SECURITY: Cannot create secure runtime directory at {}: {}",
                runtime_dir.display(),
                e
            );
        }
        
        log::info!("Using secure fallback runtime directory: {}", runtime_dir.display());
        runtime_dir
    }
}
```

### Step 3: Add Required Imports

Ensure these imports are present at the top of `unix.rs`:

```rust
use std::path::PathBuf;
use std::fs::{self, DirBuilder};
use std::os::unix::fs::PermissionsExt;
use nix::unistd::{Pid, getpid, geteuid, Uid};
use nix::sys::signal::kill;
use anyhow::{Context, Result, bail};
```

## Implementation Notes

### Why This Approach is Correct

1. **TOCTOU-Resistant**: We attempt creation first, then validate only if it exists. This minimizes the race window to near-zero.

2. **Fail-Secure**: If validation fails, daemon panics rather than using an unsafe directory. This prevents silent security compromises.

3. **Defense-in-Depth**: Multiple validation layers (symlink check, ownership, permissions) ensure security even if one layer is bypassed.

4. **No External Dependencies**: Uses only `std::fs` and existing `nix` crate features already in `Cargo.toml`.

5. **Atomic Permissions**: `DirBuilder::mode(0o700)` sets permissions atomically during creation on Unix systems.

### Why Previous "Fixes" Were Insufficient

The original task file's example fix had a **critical TOCTOU flaw**:

```rust
// ❌ VULNERABLE: Check-then-create pattern
if let Ok(metadata) = fs::symlink_metadata(&tmp_path) {
    if metadata.file_type().is_symlink() {
        panic!("Security: {} is a symlink", tmp_path.display());
    }
} else {
    // ⚠️ RACE WINDOW HERE: Attacker can create symlink between check and create
    let mut builder = fs::DirBuilder::new();
    builder.mode(0o700);
    let _ = builder.create(&tmp_path); // Too late!
}
```

Our fix uses **create-then-validate** pattern:
- Attempt creation first (atomic with permissions)
- Only if AlreadyExists, then validate
- This reverses the race: attacker must create symlink AFTER we attempt creation, which fails because directory exists

### Testing Attack Scenario

To verify the fix prevents attacks:

```bash
# As target user, create symlink attack
ln -s /tmp/attacker-controlled /tmp/kodegend-$(id -u)

# Try to start kodegend
kodegend start

# Expected result: Daemon FAILS to start with security error:
# "SECURITY: Directory is a symlink - potential attack detected"
```

## Definition of Done

The task is complete when:

1. ✅ `ensure_secure_directory()` helper function is added to `unix.rs`
2. ✅ `platform_runtime_dir()` is rewritten to use `ensure_secure_directory()` for both base and subdirectory
3. ✅ Required imports are added to `unix.rs`
4. ✅ Code compiles without errors: `cargo check -p kodegend`
5. ✅ Clippy passes: `cargo clippy -p kodegend -- -D warnings`
6. ✅ Daemon starts successfully in normal conditions (no pre-existing symlinks)
7. ✅ Daemon fails-secure when symlink attack is detected (manual verification)

## Files to Modify

### Primary Changes
- `packages/kodegend/src/platform/unix.rs` - Add `ensure_secure_directory()`, rewrite `platform_runtime_dir()`

### No Changes Required
- `daemon.rs` - Already uses correct pattern, just receives secure path
- `config.rs` - No changes needed, consumes platform API
- `platform/mod.rs` - Re-exports remain unchanged

## Risk Assessment

- **Exploitability**: HIGH - Local access, predictable paths, world-writable /tmp
- **Impact**: CRITICAL - PID file control, daemon hijacking, privilege escalation
- **Likelihood**: MEDIUM - Requires local access and timing
- **Overall Risk**: **CRITICAL** - Must fix before production deployment

## Platform Scope

- **Affected**: Unix/Linux/macOS (all platforms using `platform/unix.rs`)
- **Not Affected**: Windows (uses `platform/windows.rs` with different runtime directory strategy)
- **Elevated Users**: Not affected (use `/var/run/kodegend` which requires root to manipulate)

---

## Summary for Developer

**OBJECTIVE**: Eliminate CWE-59 symlink race condition in `/tmp/kodegend-{uid}` directory creation.

**APPROACH**: Replace check-then-create pattern with create-then-validate using `ensure_secure_directory()` helper.

**SCOPE**: Single file modification (`unix.rs`), ~100 lines of new code, zero external dependencies.

**VALIDATION**: Code must compile, pass clippy, and fail-secure when symlink is detected.
