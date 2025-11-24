# CRITICAL: Missing PID Validation in PID File Handling

## Severity
**CRITICAL** - Could cause catastrophic system failure by signaling critical system processes (init/systemd, kernel)

## Location
Primary vulnerability: `packages/kodegend/src/daemon.rs:79`  
Related code: `packages/kodegend/src/platform/unix.rs:56`

## Issue Description
The `PidFile::handle_existing_pid_file()` function in daemon.rs parses PIDs from disk without validation. The parsed PID is then passed directly to `platform::is_process_running()` which calls `kill(pid, None)` (signal 0 check). This creates a critical security and stability vulnerability where corrupted or malicious PID files can cause the daemon to signal system-critical processes.

## Current Vulnerable Code

### daemon.rs:73-115 - PID File Parsing (NO VALIDATION)
```rust
fn handle_existing_pid_file(path: &Path) -> Result<()> {
    // Read existing PID
    let pid_str = fs::read_to_string(path)
        .with_context(|| format!("Reading existing PID file: {}", path.display()))?;
    
    // Use platform::ProcessId for cross-platform compatibility
    let existing_pid = pid_str.trim().parse::<platform::ProcessId>()  // ← LINE 79: NO VALIDATION
        .with_context(|| format!("Parsing PID from file {}: '{}'", path.display(), pid_str))?;
    
    // Use platform-agnostic process checking
    match platform::is_process_running(existing_pid) {  // ← Unvalidated PID passed to kill()
        Ok(true) => {
            Err(anyhow!(
                "Daemon already running with PID {} (PID file: {})\n\
                 Use 'kodegend stop' to stop the existing daemon first.",
                existing_pid,
                path.display()
            ))
        }
        Ok(false) => {
            warn!(
                "Removing stale PID file {} (PID {} not running)",
                path.display(),
                existing_pid
            );
            fs::remove_file(path)
                .with_context(|| format!("Removing stale PID file: {}", path.display()))?;
            Ok(())
        }
        Err(e) => {
            Err(anyhow!(
                "Error checking if daemon is running (PID {}): {}\n\
                 PID file: {}",
                existing_pid,
                e,
                path.display()
            ))
        }
    }
}
```

### platform/unix.rs:55-62 - Kill Call with Unvalidated PID
```rust
pub(super) fn platform_is_process_running(pid: i32) -> Result<bool, std::io::Error> {
    match kill(Pid::from_raw(pid), None) {  // ← LINE 56: Unvalidated PID used in kill()
        Ok(_) => Ok(true),
        Err(nix::errno::Errno::ESRCH) => Ok(false),
        Err(nix::errno::Errno::EPERM) => Ok(true),
        Err(e) => Err(std::io::Error::from_raw_os_error(e as i32)),
    }
}
```

**The vulnerability**: Any i32 value that parses successfully is used, including:
- **PID 0**: Kernel scheduler (Unix reserved)
- **PID 1**: init/systemd (system crashes if killed)
- **Negative PIDs**: Process group signals (e.g., -1 signals ALL processes in group)
- **Out-of-range values**: PIDs exceeding system maximum

## Attack Scenarios

### 1. Corrupted PID File - System Crash
```bash
# Disk corruption or filesystem bug writes "1" to PID file
echo "1" > /var/run/kodegend/kodegend.pid

# User attempts to check daemon status
kodegend status

# Result: kill(1, 0) signals init/systemd
# On some configurations, this could trigger protective system shutdown
```

### 2. Malicious Modification - Targeted Process Termination
```bash
# Attacker with file write access (compromised user account, shared directory)
echo "1234" > ~/.config/kodegend/kodegend.pid  # PID of target process

# Legitimate daemon restart
kodegend restart

# Result: Target process receives signals intended for daemon
```

### 3. Race Condition - Partial Write
```bash
# Process writes PID file but crashes mid-write
# File contains partial data: "1" instead of "12345"

# Next daemon start reads corrupted PID
# Result: Attempts to signal PID 1
```

### 4. Process Group Signal (Negative PID)
```bash
# Malicious modification
echo "-1" > /var/run/kodegend/kodegend.pid

# On 32-bit systems, -1 could parse successfully
# Result: kill(-1, 0) checks ALL processes in current group
```

## Platform-Specific PID Constraints

### Linux
- **PID 0**: Kernel idle/scheduler (reserved)
- **PID 1**: init or systemd (system-critical)
- **Default maximum**: 32768 (2^15) - compatible with old Unix systems
- **64-bit maximum**: 4194304 (2^22) - configurable via `/proc/sys/kernel/pid_max`
- **32-bit maximum**: 32768 (hard limit)
- **Source**: `/proc/sys/kernel/pid_max` - value is one greater than maximum assignable PID

Reference: [Linux kernel documentation](https://www.kernel.org/doc/html/latest/admin-guide/sysctl/kernel.html#pid-max), [StackOverflow: Maximum PID in Linux](https://stackoverflow.com/questions/6294133/maximum-pid-in-linux)

### macOS
- **PID 0**: Kernel idle (reserved)
- **PID 1**: launchd (system-critical)
- **Maximum PID**: 99998 (PID_MAX = 99999, but PIDs are assigned < PID_MAX)
- **Source**: `kern_fork.c` in XNU kernel, `sysctl kern.maxproc` for process limits
- **Process limit**: Typically 532-2048 (configurable via `kern.maxproc`)

Reference: [Apple StackExchange: Maximum PID for macOS](https://apple.stackexchange.com/questions/51119/whats-the-maximum-pid-for-mac-os-x)

### Windows
- **ProcessId type**: `u32` (unsigned 32-bit) - see [platform/mod.rs:46](../src/platform/mod.rs#L46)
- **No negative PIDs possible** on Windows (unsigned type prevents this)
- Windows validation not needed for this task (different type system)

## Implementation Strategy

### Step 1: Create PID Validation Function

Add to `packages/kodegend/src/daemon.rs` after the `PidFile` impl blocks (around line 145):

```rust
/// Validate a PID value before using it for process operations
///
/// Ensures PID is safe to use with kill() and other process APIs:
/// - Rejects PID 0 (kernel scheduler)
/// - Rejects PID 1 (init/systemd/launchd)
/// - Rejects negative PIDs (process groups)
/// - Validates against system-specific maximum
///
/// # Arguments
/// * `pid` - The PID value to validate
///
/// # Returns
/// * `Ok(())` if PID is valid and safe to use
/// * `Err(anyhow::Error)` with detailed error message if invalid
fn validate_pid(pid: platform::ProcessId) -> Result<()> {
    // Reject reserved system PIDs
    if pid <= 1 {
        anyhow::bail!(
            "Invalid PID {}: Cannot signal kernel (PID 0) or init/systemd (PID 1)",
            pid
        );
    }
    
    // Get platform-specific maximum PID value
    let pid_max = get_system_pid_max();
    
    if pid > pid_max {
        anyhow::bail!(
            "Invalid PID {}: Exceeds system maximum {} (likely corrupted PID file)",
            pid,
            pid_max
        );
    }
    
    Ok(())
}

/// Get the system's maximum PID value
///
/// Platform-specific implementations:
/// - Linux: Read /proc/sys/kernel/pid_max (default 32768, max 4194304)
/// - macOS: Use PID_MAX constant (99999)
/// - Fallback: Conservative default (32768) if detection fails
fn get_system_pid_max() -> platform::ProcessId {
    #[cfg(target_os = "linux")]
    {
        // Linux: /proc/sys/kernel/pid_max contains the value
        // Note: This is one greater than the maximum assignable PID
        std::fs::read_to_string("/proc/sys/kernel/pid_max")
            .ok()
            .and_then(|s| s.trim().parse::<i32>().ok())
            .map(|max| max - 1)  // Subtract 1 to get actual maximum assignable PID
            .unwrap_or(32768)     // Fallback to conservative default
    }
    
    #[cfg(target_os = "macos")]
    {
        // macOS: PID_MAX is 99999 in kern_fork.c
        // PIDs are assigned < PID_MAX, so maximum is 99998
        99998
    }
    
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    {
        // Conservative fallback for other Unix systems
        32768
    }
}
```

### Step 2: Apply Validation in PidFile::handle_existing_pid_file()

Modify `packages/kodegend/src/daemon.rs:73-115` to validate the parsed PID:

**FIND** (line 79):
```rust
    // Use platform::ProcessId for cross-platform compatibility
    let existing_pid = pid_str.trim().parse::<platform::ProcessId>()
        .with_context(|| format!("Parsing PID from file {}: '{}'", path.display(), pid_str))?;
    
    // Use platform-agnostic process checking
    match platform::is_process_running(existing_pid) {
```

**REPLACE WITH**:
```rust
    // Use platform::ProcessId for cross-platform compatibility
    let existing_pid = pid_str.trim().parse::<platform::ProcessId>()
        .with_context(|| format!("Parsing PID from file {}: '{}'", path.display(), pid_str))?;
    
    // CRITICAL SECURITY: Validate PID before using it
    // Prevents signaling kernel (PID 0), init (PID 1), or invalid PIDs
    validate_pid(existing_pid)
        .with_context(|| format!("Invalid PID in file {}", path.display()))?;
    
    // Use platform-agnostic process checking
    match platform::is_process_running(existing_pid) {
```

### Step 3: Optional Defense-in-Depth - Validate in platform::is_process_running()

For additional safety, add validation directly in the platform layer. This provides defense-in-depth if PIDs come from other code paths.

Modify `packages/kodegend/src/platform/unix.rs:55-62`:

**FIND** (line 55):
```rust
pub(super) fn platform_is_process_running(pid: i32) -> Result<bool, std::io::Error> {
    match kill(Pid::from_raw(pid), None) {
```

**REPLACE WITH**:
```rust
pub(super) fn platform_is_process_running(pid: i32) -> Result<bool, std::io::Error> {
    // Defense-in-depth: Validate PID before signaling
    // Prevents catastrophic errors from signaling PID 0 (kernel) or PID 1 (init)
    if pid <= 1 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Invalid PID {}: Cannot signal kernel (0) or init (1)", pid)
        ));
    }
    
    match kill(Pid::from_raw(pid), None) {
```

## Files to Modify

1. **Primary fix**: `packages/kodegend/src/daemon.rs`
   - Add `validate_pid()` function (after line 145)
   - Add `get_system_pid_max()` function (after `validate_pid()`)
   - Modify `PidFile::handle_existing_pid_file()` to call `validate_pid()` (line ~82, after parsing)

2. **Defense-in-depth** (optional but recommended): `packages/kodegend/src/platform/unix.rs`
   - Add PID validation at start of `platform_is_process_running()` (line ~57)

3. **No changes needed**:
   - `control/linux_control.rs` - Uses systemctl, no direct PID handling
   - `control/macos_control.rs` - Uses launchctl, no direct PID handling
   - `platform/windows.rs` - Windows PIDs are unsigned (u32), preventing negative values

## Existing Code Patterns to Follow

### Error Context Pattern
The codebase uses `anyhow::Context` extensively for error messages:
```rust
// See daemon.rs:75-76
.with_context(|| format!("Reading existing PID file: {}", path.display()))?;
```

### Platform Conditional Compilation
The codebase uses `#[cfg(target_os = "...")]` for platform-specific code:
```rust
// See platform/unix.rs:13-18, daemon.rs:15-29
#[cfg(target_os = "linux")]
{
    // Linux-specific implementation
}
```

### Logging Pattern
The codebase uses `log` crate macros:
```rust
// See daemon.rs:95-98
warn!(
    "Removing stale PID file {} (PID {} not running)",
    path.display(),
    existing_pid
);
```

## Definition of Done

1. **PID validation function exists** in `daemon.rs` with:
   - Rejection of PIDs <= 1
   - Platform-specific maximum PID checking
   - Clear error messages for each rejection case

2. **System PID max detection** works for:
   - Linux: Reads `/proc/sys/kernel/pid_max` with fallback
   - macOS: Uses hardcoded PID_MAX (99998)
   - Other Unix: Conservative fallback (32768)

3. **Validation is applied** in `PidFile::handle_existing_pid_file()`:
   - Called immediately after parsing PID (line ~82)
   - Provides context about which PID file failed validation
   - Returns clear error message to user

4. **Optional defense-in-depth** in `platform::is_process_running()`:
   - Validates PID before calling `kill()`
   - Returns `InvalidInput` error for invalid PIDs
   - Prevents any code path from signaling PID 0 or 1

5. **Code compiles successfully**:
   - `cargo check` passes in `packages/kodegend`
   - No new clippy warnings introduced
   - Follows existing error handling patterns

## Security Impact After Fix

- **Prevents system crashes**: Cannot signal PID 1 (init/systemd)
- **Prevents kernel errors**: Cannot signal PID 0 (kernel scheduler)
- **Validates data integrity**: Rejects corrupted PID files with out-of-range values
- **Defense in depth**: Validation at both parsing layer and platform layer
- **Clear error messages**: Users understand why PID file was rejected

## References

### Codebase References
- [daemon.rs:73-115](../packages/kodegend/src/daemon.rs#L73-L115) - Current vulnerable code
- [platform/unix.rs:55-62](../packages/kodegend/src/platform/unix.rs#L55-L62) - Kill syscall wrapper
- [platform/mod.rs:44](../packages/kodegend/src/platform/mod.rs#L44) - Unix ProcessId type alias

### External Documentation
- [POSIX kill() specification](https://pubs.opengroup.org/onlinepubs/9699919799/functions/kill.html)
- [Linux /proc/sys/kernel/pid_max documentation](https://www.kernel.org/doc/html/latest/admin-guide/sysctl/kernel.html#pid-max)
- [CWE-20: Improper Input Validation](https://cwe.mitre.org/data/definitions/20.html)
- [Linux PID limits - Stack Overflow](https://stackoverflow.com/questions/6294133/maximum-pid-in-linux)
- [macOS PID_MAX - Apple StackExchange](https://apple.stackexchange.com/questions/51119/whats-the-maximum-pid-for-mac-os-x)

### Security Considerations
- PIDs are untrusted input from filesystem (can be corrupted or maliciously modified)
- kill() syscall with PID 1 can destabilize or crash system
- Negative PIDs signal process groups (potential for mass process termination)
- Defense-in-depth: Validate at multiple layers (parsing + syscall wrapper)
