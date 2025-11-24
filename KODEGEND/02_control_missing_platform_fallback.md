# Generic Unix Daemon Control for BSD and Other Unix-Like Systems

## Location
`packages/kodegend/src/control.rs` and `packages/kodegend/src/control/generic_control.rs` (new file)

## Severity
🟡 **MEDIUM - PORTABILITY ENHANCEMENT**

## Current Status

The control module currently has a `compile_error!` for unsupported platforms (lines 21-47 in [`control.rs`](../packages/kodegend/src/control.rs)):

```rust
else {
    compile_error!(
        "kodegend daemon control is not supported on this platform.\n\
         ...(helpful error message)..."
    );
}
```

This prevents compilation on BSD systems (FreeBSD, OpenBSD, NetBSD, DragonFly BSD) and other Unix-like platforms that could benefit from basic daemon control functionality.

## Objective

Implement generic PID-based daemon control for Unix-like systems without service manager integration. This provides partial functionality for BSD and other Unix systems:

- ✅ **`check_status()`** - Check if daemon is running via PID file
- ❌ **`start_daemon()`** - Return helpful error (no service manager)
- ✅ **`stop_daemon()`** - Send SIGTERM to daemon PID
- ❌ **`restart_daemon()`** - Return helpful error (no service manager)

## Architecture Analysis

### Existing Infrastructure

The codebase already provides all necessary utilities for generic daemon control:

#### 1. PID File Path Resolution
**Location:** [`src/config.rs:60-66`](../packages/kodegend/src/config.rs)

```rust
fn default_pid_file() -> PathBuf {
    use crate::platform;
    let is_elevated = platform::is_elevated();
    platform::runtime_dir(is_elevated).join("kodegend.pid")
}
```

**Runtime directory paths by platform:**
- **Unix (elevated/root):** `/var/run/kodegend/kodegend.pid`
- **Unix (user):** `$XDG_RUNTIME_DIR/kodegend/kodegend.pid` or `~/.local/state/kodegend/kodegend.pid`

**Implementation:** [`src/platform/unix.rs:86-95`](../packages/kodegend/src/platform/unix.rs)

#### 2. Process Status Checking
**Location:** [`src/platform/unix.rs:53-62`](../packages/kodegend/src/platform/unix.rs)

```rust
pub fn platform_is_process_running(pid: i32) -> Result<bool, std::io::Error> {
    match kill(Pid::from_raw(pid), None) {  // Signal 0 = check only
        Ok(_) => Ok(true),                    // Process exists
        Err(nix::errno::Errno::ESRCH) => Ok(false),  // No such process
        Err(nix::errno::Errno::EPERM) => Ok(true),   // Exists, no permission
        Err(e) => Err(std::io::Error::from_raw_os_error(e as i32)),
    }
}
```

Uses POSIX `kill(pid, 0)` to check process existence without sending a signal.

#### 3. Signal Sending for Daemon Shutdown
**Available via nix crate:** Already imported in [`platform/unix.rs:11`](../packages/kodegend/src/platform/unix.rs)

```rust
use nix::sys::signal::kill;
use nix::unistd::Pid;
```

For sending SIGTERM:
```rust
use nix::sys::signal::{kill, Signal};
kill(Pid::from_raw(pid), Signal::SIGTERM)?;
```

#### 4. Privilege Checking
**Location:** [`src/platform/unix.rs:13-16`](../packages/kodegend/src/platform/unix.rs)

```rust
pub fn platform_is_elevated() -> bool {
    geteuid().is_root()
}
```

### Platform-Specific Control Interface

All platform implementations expose the same 4-function interface:

**macOS:** [`src/control/macos_control.rs`](../packages/kodegend/src/control/macos_control.rs) - Uses `launchctl`
**Linux:** [`src/control/linux_control.rs`](../packages/kodegend/src/control/linux_control.rs) - Uses `systemctl`  
**Windows:** [`src/control/windows_control.rs`](../packages/kodegend/src/control/windows_control.rs) - Uses Service Control Manager API

```rust
pub fn check_status() -> Result<bool>
pub fn start_daemon() -> Result<()>
pub fn stop_daemon() -> Result<()>
pub fn restart_daemon() -> Result<()>
```

The generic implementation must match this interface.

## Implementation

### Step 1: Create Generic Control Module

**File:** `packages/kodegend/src/control/generic_control.rs` (new file)

**Complete implementation:**

```rust
//! Generic Unix daemon control using PID files and POSIX signals
//!
//! This is a fallback implementation for Unix-like systems without
//! service manager integration (BSD systems, Solaris, etc.).
//!
//! ## Capabilities
//!
//! - ✅ Check daemon status via PID file and process existence
//! - ✅ Stop daemon via SIGTERM signal
//! - ❌ Cannot start daemon (no service manager to spawn process)
//! - ❌ Cannot restart daemon (requires start capability)
//!
//! ## Limitations
//!
//! - No service manager integration (no systemd/launchd/rc.d)
//! - Cannot spawn daemon process (manual startup required)
//! - No automatic restart on crashes
//! - No auto-start on boot
//! - Manual PID file management only
//!
//! For full daemon lifecycle management, use platform-specific implementations:
//! - macOS: [`macos_control`] (launchd via launchctl)
//! - Linux: [`linux_control`] (systemd via systemctl)
//! - Windows: [`windows_control`] (Service Control Manager)

use anyhow::{Context, Result, bail};
use std::fs;
use std::path::PathBuf;

// Import existing platform utilities
use crate::platform;

/// Get PID file path using existing platform logic
///
/// Uses the same path resolution as the daemon itself:
/// - Root: /var/run/kodegend/kodegend.pid
/// - User: $XDG_RUNTIME_DIR/kodegend/kodegend.pid or ~/.local/state/kodegend/kodegend.pid
///
/// Reuses: config.rs::default_pid_file() logic
fn pid_file_path() -> PathBuf {
    let is_elevated = platform::is_elevated();
    platform::runtime_dir(is_elevated).join("kodegend.pid")
}

/// Read PID from PID file
///
/// Returns: PID as i32
/// Errors: File doesn't exist, can't read, or invalid PID format
fn read_pid() -> Result<i32> {
    let path = pid_file_path();
    
    if !path.exists() {
        bail!(
            "Daemon not running (PID file does not exist: {})\n\
             \n\
             To start the daemon manually:\n\
             \n\
             kodegend run --foreground\n\
             \n\
             Or with nohup for background:\n\
             \n\
             nohup kodegend run --foreground > /var/log/kodegend.log 2>&1 &",
            path.display()
        );
    }
    
    let pid_str = fs::read_to_string(&path)
        .with_context(|| format!("Reading PID file: {}", path.display()))?;
    
    pid_str.trim()
        .parse::<i32>()
        .with_context(|| {
            format!(
                "Parsing PID from file {}: '{}' is not a valid process ID",
                path.display(),
                pid_str.trim()
            )
        })
}

/// Check if daemon is running using PID file and process validation
///
/// Algorithm:
/// 1. Check if PID file exists
/// 2. Read PID from file
/// 3. Validate process exists using kill(pid, 0)
///
/// Returns:
/// - Ok(true): Daemon is running
/// - Ok(false): Daemon is not running (no PID file or stale PID)
/// - Err: System error checking process status
pub fn check_status() -> Result<bool> {
    let path = pid_file_path();
    
    // No PID file = not running
    if !path.exists() {
        return Ok(false);
    }
    
    // Read PID from file
    let pid = match read_pid() {
        Ok(pid) => pid,
        Err(_) => {
            // Stale or corrupted PID file - treat as not running
            return Ok(false);
        }
    };
    
    // Reuse existing process checking logic
    // Uses POSIX kill(pid, 0) to check existence
    platform::is_process_running(pid)
        .map_err(|e| anyhow::anyhow!("Error checking process status: {}", e))
}

/// Start daemon - NOT SUPPORTED
///
/// Generic implementation cannot start the daemon because there's no
/// service manager to spawn and supervise the process.
///
/// Returns: Error with instructions for manual startup
pub fn start_daemon() -> Result<()> {
    bail!(
        "Starting the daemon is not supported on this platform.\n\
         \n\
         This platform lacks service manager integration (systemd/launchd/rc.d).\n\
         Kodegend cannot automatically spawn the daemon process.\n\
         \n\
         MANUAL STARTUP OPTIONS:\n\
         \n\
         1. Run in foreground mode:\n\
         \n\
            kodegend run --foreground\n\
         \n\
         2. Run in background with nohup:\n\
         \n\
            nohup kodegend run --foreground > /var/log/kodegend.log 2>&1 &\n\
         \n\
         3. Create a custom rc.d script for your platform:\n\
         \n\
            - FreeBSD: /usr/local/etc/rc.d/kodegend\n\
            - OpenBSD: /etc/rc.d/kodegend (use rcctl)\n\
            - NetBSD: /etc/rc.d/kodegend\n\
         \n\
         For automatic service management, request platform support:\n\
         https://github.com/kodegen-ai/kodegen/issues"
    );
}

/// Stop daemon by sending SIGTERM to PID
///
/// Algorithm:
/// 1. Read PID from PID file
/// 2. Send SIGTERM (signal 15) for graceful shutdown
/// 3. Return immediately (does not wait for termination)
///
/// The daemon's signal handler will:
/// - Receive SIGTERM
/// - Initiate graceful shutdown
/// - Clean up PID file via Drop trait
///
/// Returns:
/// - Ok(()): SIGTERM sent successfully
/// - Err: Cannot read PID file, or failed to send signal
pub fn stop_daemon() -> Result<()> {
    let pid = read_pid()
        .context("Cannot stop daemon: failed to read PID file")?;
    
    // Import signal types
    use nix::sys::signal::{kill, Signal};
    use nix::unistd::Pid;
    
    // Send SIGTERM for graceful shutdown
    kill(Pid::from_raw(pid), Signal::SIGTERM)
        .map_err(|e| {
            anyhow::anyhow!(
                "Failed to send SIGTERM to process {}: {}\n\
                 \n\
                 Possible causes:\n\
                 - Process already exited\n\
                 - Insufficient permissions (try sudo)\n\
                 - PID belongs to different user\n\
                 \n\
                 Try checking status: kodegend status",
                pid,
                e
            )
        })?;
    
    log::info!("Sent SIGTERM to kodegend daemon (PID: {})", pid);
    log::info!("Daemon will perform graceful shutdown and clean up PID file");
    
    Ok(())
}

/// Restart daemon - NOT SUPPORTED
///
/// Generic implementation cannot restart because start is not supported.
///
/// Returns: Error with instructions for manual restart
pub fn restart_daemon() -> Result<()> {
    bail!(
        "Restarting the daemon is not supported on this platform.\n\
         \n\
         This platform lacks service manager integration (systemd/launchd/rc.d).\n\
         \n\
         MANUAL RESTART PROCEDURE:\n\
         \n\
         1. Stop the daemon:\n\
         \n\
            kodegend stop\n\
         \n\
         2. Wait for graceful shutdown (check status):\n\
         \n\
            kodegend status\n\
         \n\
         3. Start manually in foreground:\n\
         \n\
            kodegend run --foreground\n\
         \n\
         Or with nohup for background:\n\
         \n\
            nohup kodegend run --foreground > /var/log/kodegend.log 2>&1 &\n\
         \n\
         For automatic service management, request platform support:\n\
         https://github.com/kodegen-ai/kodegen/issues"
    );
}
```

### Step 2: Update Control Module cfg_if Block

**File:** `packages/kodegend/src/control.rs`

**Location:** Lines 11-48 (replace the existing `cfg_if!` block)

**Current implementation:**
```rust
cfg_if::cfg_if! {
    if #[cfg(target_os = "macos")] {
        mod macos_control;
        use macos_control as platform;
    } else if #[cfg(target_os = "linux")] {
        mod linux_control;
        use linux_control as platform;
    } else if #[cfg(target_os = "windows")] {
        mod windows_control;
        use windows_control as platform;
    } else {
        compile_error!("...");
    }
}
```

**New implementation:**
```rust
cfg_if::cfg_if! {
    if #[cfg(target_os = "macos")] {
        mod macos_control;
        use macos_control as platform;
    } else if #[cfg(target_os = "linux")] {
        mod linux_control;
        use linux_control as platform;
    } else if #[cfg(target_os = "windows")] {
        mod windows_control;
        use windows_control as platform;
    } else if #[cfg(unix)] {
        // Generic Unix fallback for BSD and other Unix-like systems
        // Provides basic PID-based control without service manager integration
        //
        // Supported platforms: FreeBSD, OpenBSD, NetBSD, DragonFly BSD, Solaris, etc.
        //
        // Capabilities:
        // - ✅ check_status() - via PID file and process existence
        // - ✅ stop_daemon() - via SIGTERM signal
        // - ❌ start_daemon() - returns helpful error
        // - ❌ restart_daemon() - returns helpful error
        mod generic_control;
        use generic_control as platform;
    } else {
        // Non-Unix, non-Windows platform (extremely rare)
        compile_error!(
            "kodegend is only supported on Unix-like systems and Windows.\n\
             \n\
             Your platform does not appear to be Unix or Windows.\n\
             \n\
             Supported platforms:\n\
             - Unix: macOS, Linux, FreeBSD, OpenBSD, NetBSD, DragonFly BSD, Solaris\n\
             - Windows: Windows 7+\n\
             \n\
             If you believe this is an error, please report an issue:\n\
             https://github.com/kodegen-ai/kodegen/issues"
        );
    }
}
```

### Step 3: No Changes to Public API

The public API in `control.rs` (lines 50-70) remains unchanged:

```rust
pub fn check_status() -> Result<bool> {
    platform::check_status()
}

pub fn start_daemon() -> Result<()> {
    platform::start_daemon()
}

pub fn stop_daemon() -> Result<()> {
    platform::stop_daemon()
}

pub fn restart_daemon() -> Result<()> {
    platform::restart_daemon()
}
```

The platform abstraction seamlessly routes calls to the appropriate implementation.

## Implementation Checklist

### File Creation

- [ ] **Create** `packages/kodegend/src/control/generic_control.rs` with the complete implementation provided above

### File Modification

- [ ] **Modify** `packages/kodegend/src/control.rs`:
  - Replace the `cfg_if!` block (lines 11-48)
  - Add `else if #[cfg(unix)]` clause before the final `else`
  - Update the compile_error message to reflect Unix support

### Verification Steps

1. **Verify compilation on supported platforms:**
   ```bash
   cd packages/kodegend
   cargo check
   # Should succeed on macOS, Linux, Windows
   ```

2. **Verify generic_control module loads on BSD (if available):**
   ```bash
   # On FreeBSD, OpenBSD, or NetBSD
   cargo check
   # Should succeed with generic_control
   ```

3. **Verify compile error on unsupported platforms:**
   ```bash
   # If testing on exotic platform (unlikely)
   cargo check
   # Should fail with clear compile_error message
   ```

## Platform-Specific Behavior

### macOS, Linux, Windows
**Behavior:** Uses full service manager integration  
**Capabilities:** All 4 functions work (check, start, stop, restart)  
**Implementation:** Platform-specific control modules

### FreeBSD, OpenBSD, NetBSD, DragonFly BSD, Solaris
**Behavior:** Uses generic PID-based control  
**Capabilities:**
- ✅ `check_status()` - Reads PID file, validates process
- ✅ `stop_daemon()` - Sends SIGTERM signal
- ❌ `start_daemon()` - Returns error with manual startup instructions
- ❌ `restart_daemon()` - Returns error with manual restart procedure

**Implementation:** `generic_control` module

### Other Platforms
**Behavior:** Compilation fails with helpful error  
**Message:** Directs user to supported platforms and issue tracker

## Definition of Done

The implementation is complete when:

1. ✅ File `packages/kodegend/src/control/generic_control.rs` exists with the implementation provided above
2. ✅ File `packages/kodegend/src/control.rs` has updated `cfg_if!` block with `else if #[cfg(unix)]` clause
3. ✅ Compilation succeeds on macOS, Linux, and Windows (uses platform-specific modules)
4. ✅ Compilation succeeds on BSD systems (uses generic_control module)
5. ✅ `kodegend check` command succeeds on all platforms
6. ✅ On BSD systems:
   - `kodegend status` works (reads PID file and checks process)
   - `kodegend stop` works (sends SIGTERM)
   - `kodegend start` returns helpful error message
   - `kodegend restart` returns helpful error message

## Usage Examples

### FreeBSD User Workflow

```bash
# Start daemon manually (generic_control cannot start)
$ nohup kodegend run --foreground > /var/log/kodegend.log 2>&1 &

# Check status (works via PID file)
$ kodegend status
✓ Daemon is running (PID: 12345)

# Stop daemon (works via SIGTERM)
$ kodegend stop
✓ Sent SIGTERM to kodegend daemon (PID: 12345)

# Try to start (gets helpful error)
$ kodegend start
Error: Starting the daemon is not supported on this platform.

This platform lacks service manager integration (systemd/launchd/rc.d).
Kodegend cannot automatically spawn the daemon process.

MANUAL STARTUP OPTIONS:
...
```

### Creating rc.d Script (Optional - Not Required)

BSD users who want automatic startup can create an rc.d script:

**FreeBSD example:** `/usr/local/etc/rc.d/kodegend`
```sh
#!/bin/sh

# PROVIDE: kodegend
# REQUIRE: DAEMON
# KEYWORD: shutdown

. /etc/rc.subr

name="kodegend"
rcvar="kodegend_enable"
command="/usr/local/bin/kodegend"
command_args="run --foreground"
pidfile="/var/run/kodegend/kodegend.pid"

load_rc_config $name
run_rc_command "$1"
```

Then enable in `/etc/rc.conf`:
```sh
kodegend_enable="YES"
```

**Note:** rc.d script creation is NOT part of this task - it's mentioned only as a future enhancement for BSD users.

## Dependencies

All required dependencies already exist in `kodegend/Cargo.toml`:

- `cfg-if = "1.0"` - Conditional compilation ✓
- `anyhow = "1.0"` - Error handling ✓
- `nix = "0.27"` - Unix system calls (kill, getuid, etc.) ✓
- `log = "0.4"` - Logging ✓

**No new dependencies required.**

## Code References

### Files Modified
- [`packages/kodegend/src/control.rs`](../packages/kodegend/src/control.rs) - Update `cfg_if!` block

### Files Created
- `packages/kodegend/src/control/generic_control.rs` - New generic Unix implementation

### Referenced Utilities
- [`packages/kodegend/src/platform/unix.rs`](../packages/kodegend/src/platform/unix.rs) - Process checking, runtime dirs
- [`packages/kodegend/src/config.rs`](../packages/kodegend/src/config.rs) - PID file path configuration
- [`packages/kodegend/src/daemon.rs`](../packages/kodegend/src/daemon.rs) - PidFile RAII guard (for reference)

### Platform Implementations (Reference)
- [`packages/kodegend/src/control/linux_control.rs`](../packages/kodegend/src/control/linux_control.rs) - systemctl
- [`packages/kodegend/src/control/macos_control.rs`](../packages/kodegend/src/control/macos_control.rs) - launchctl
- [`packages/kodegend/src/control/windows_control.rs`](../packages/kodegend/src/control/windows_control.rs) - Service Control Manager

## Security Considerations

### PID File Race Conditions

The generic implementation reads PID files without the flock locking used in [`daemon.rs::PidFile::create()`](../packages/kodegend/src/daemon.rs). This is acceptable because:

1. **Read-only operations:** We only read the PID, never write
2. **Validation via kill(0):** The actual process check uses `kill(pid, 0)` which is atomic
3. **Fail-safe behavior:** If PID is stale or invalid, we return "not running"

### Signal Sending Permissions

`stop_daemon()` uses `kill(pid, SIGTERM)` which requires:
- Same user as target process, OR
- Root privileges

The error message guides users to use `sudo` if permission is denied.

### No Privilege Escalation

The generic implementation uses the same privilege model as platform-specific implementations:
- User daemon: Manages user's own daemon only
- Root daemon: Manages system-wide daemon

## Future Enhancements (Not Part of This Task)

1. **Wait for termination:** Add optional `--wait` flag to `stop_daemon()` that polls until process exits
2. **Forceful shutdown:** Add `--force` flag to send SIGKILL after SIGTERM timeout
3. **rc.d script generation:** Auto-generate platform-specific rc.d scripts
4. **Service installation:** Implement `kodegend install` for BSD platforms

These are explicitly OUT OF SCOPE for this task.

---

**PRIORITY:** This enhancement provides immediate value for BSD users and improves the project's cross-platform compatibility without compromising existing functionality.
