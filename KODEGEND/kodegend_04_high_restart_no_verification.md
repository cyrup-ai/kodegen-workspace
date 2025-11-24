# HIGH: Restart Command Lacks Stop Verification

## Severity
**HIGH** - Can cause multiple daemon instances or resource conflicts

## Location
- **macOS**: `packages/kodegend/src/control/macos_control.rs:117-134`
- **Windows**: `packages/kodegend/src/control/windows_control.rs:161-172`
- **Linux**: ✅ SAFE - delegates to systemd which handles verification internally

## Issue Description
The restart command sends stop signal and waits a fixed 1 second, but never verifies the daemon actually stopped. This can lead to starting a new daemon while the old one is still shutting down.

## Current Problematic Code

### macOS Implementation (`control/macos_control.rs`)
```rust
/// Restart daemon via launchctl
///
/// Uses kickstart -k (kill flag) with manual stop+start fallback
pub fn restart_daemon() -> Result<()> {
    // macOS launchctl doesn't have a direct restart command
    // Use kickstart with -k (kill) flag which restarts the service
    
    let output = Command::new("launchctl")
        .args(["kickstart", "-k", SERVICE_LABEL])
        .output()
        .context("Failed to execute launchctl kickstart -k")?;

    if !output.status.success() {
        // Fallback: manual stop + start
        stop_daemon()?;
        std::thread::sleep(Duration::from_secs(1));  // ← PROBLEM: No verification
        start_daemon()?;
    }

    Ok(())
}
```

### Windows Implementation (`control/windows_control.rs`)
```rust
/// Restart daemon (Windows doesn't have native restart - stop + start)
pub fn restart_daemon() -> Result<()> {
    // Stop the service
    stop_daemon()?;

    // Wait for service to fully stop
    std::thread::sleep(Duration::from_secs(1));  // ← PROBLEM: No verification

    // Start the service
    start_daemon()?;

    Ok(())
}
```

## Problem Analysis

The 1-second sleep is arbitrary and doesn't account for:
1. **Slow service shutdown**: If services take >1s to stop gracefully
2. **File descriptor cleanup**: Sockets/files may not be released yet
3. **Database connections**: Connection pools draining
4. **Long-running operations**: In-flight requests or batch jobs
5. **System load**: Under high load, shutdown may take longer

## Race Condition Timeline
```
Time  Old Daemon                    New Daemon
────────────────────────────────────────────────────────────────
0s    Receives SIGTERM
      Starts shutdown...
      
1s    Still shutting down           Starts!
      - Closing database conns       - Tries to bind port 30438
      - Flushing buffers             - Error: Address already in use
      - Stopping services            
      
2s    Still running...              Crashes or waits
      
3s    Finally exits                 Retry logic?
```

## Production Impact Scenarios

### Scenario 1: Port Binding Conflicts
```
Old daemon: Still bound to port 30438
New daemon: Tries to bind port 30438
Result: EADDRINUSE error, daemon fails to start
```

### Scenario 2: PID File Race
```
Old daemon: Still running, holds PID file lock
New daemon: Can't create PID file
Result: Startup fails with "daemon already running"
```

### Scenario 3: Database Connection Limits
```
Old daemon: 50 DB connections still draining
New daemon: Tries to open 50 DB connections
Result: "Too many connections" error
```

### Scenario 4: File Lock Conflicts
```
Old daemon: Log files still open with exclusive locks
New daemon: Can't open log files
Result: Permission denied or file locked errors
```

### Scenario 5: Service State Corruption
```
Old daemon: Still managing service processes
New daemon: Tries to manage same services
Result: Undefined behavior, possible data corruption
```

## Why 1 Second Is Insufficient
Typical daemon shutdown times:
- **Minimal daemon**: 100-500ms
- **With database**: 1-3s (connection pool drain)
- **With active requests**: 5-30s (graceful request drain)
- **With batch jobs**: Can be minutes

The current code assumes best-case (minimal daemon), which is unrealistic in production.

---

# SOLUTION IMPLEMENTATION

## Overview

Replace the arbitrary 1-second sleep with a **polling loop** that uses existing platform utilities to verify the process has actually stopped before starting a new instance.

## Existing Code We Can Leverage

The codebase already has all the utilities we need:

### 1. Process Checking Utility
**File**: [`packages/kodegend/src/platform/mod.rs`](../packages/kodegend/src/platform/mod.rs)

```rust
/// Check if process exists and is running
///
/// - Unix: kill(pid, None) - signal 0 doesn't send signal, just checks existence
/// - Windows: OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION) - succeeds if exists
///
/// Returns Ok(true) if process exists, Ok(false) if not, Err on permission/system error
pub fn is_process_running(pid: ProcessId) -> Result<bool, std::io::Error> {
    platform_is_process_running(pid)
}
```

This function is **already implemented** for both Unix ([`unix.rs:55-62`](../packages/kodegend/src/platform/unix.rs#L55-L62)) and Windows ([`windows.rs`](../packages/kodegend/src/platform/windows.rs)) and is currently used by the PID file validation logic.

### 2. PID File Reading Pattern
**File**: [`packages/kodegend/src/daemon.rs`](../packages/kodegend/src/daemon.rs)

The daemon already shows how to read and parse PID files (lines 74-80):

```rust
// Read existing PID
let pid_str = fs::read_to_string(path)
    .with_context(|| format!("Reading existing PID file: {}", path.display()))?;

// Use platform::ProcessId for cross-platform compatibility
let existing_pid = pid_str.trim().parse::<platform::ProcessId>()
    .with_context(|| format!("Parsing PID from file {}: '{}'", path.display(), pid_str))?;
```

### 3. PID File Path Configuration
**File**: [`packages/kodegend/src/config.rs`](../packages/kodegend/src/config.rs)

The PID file path is available via `ServiceConfig::pid_file` (line 48), which defaults to platform-specific locations:
- Unix (elevated): `/var/run/kodegend/kodegend.pid`
- Unix (user): `$XDG_RUNTIME_DIR/kodegend/kodegend.pid`
- Windows (elevated): `C:\ProgramData\kodegend\run\kodegend.pid`
- Windows (user): `%LOCALAPPDATA%\kodegend\run\kodegend.pid`

### 4. Signal Utilities (Unix)
**File**: [`packages/kodegend/src/platform/unix.rs`](../packages/kodegend/src/platform/unix.rs)

For force-killing hung processes, Unix platforms can use:
```rust
use nix::unistd::Pid;
use nix::sys::signal::{kill, Signal};

// Force kill
kill(Pid::from_raw(pid), Signal::SIGKILL)?;
```

---

## Implementation Changes

### File 1: `packages/kodegend/src/control/macos_control.rs`

**Change Location**: Lines 117-134 (the `restart_daemon()` function)

**Current Code**:
```rust
pub fn restart_daemon() -> Result<()> {
    let output = Command::new("launchctl")
        .args(["kickstart", "-k", SERVICE_LABEL])
        .output()
        .context("Failed to execute launchctl kickstart -k")?;

    if !output.status.success() {
        // Fallback: manual stop + start
        stop_daemon()?;
        std::thread::sleep(Duration::from_secs(1));  // ← REMOVE THIS
        start_daemon()?;
    }

    Ok(())
}
```

**New Implementation**:
```rust
pub fn restart_daemon() -> Result<()> {
    let output = Command::new("launchctl")
        .args(["kickstart", "-k", SERVICE_LABEL])
        .output()
        .context("Failed to execute launchctl kickstart -k")?;

    if !output.status.success() {
        // Fallback: manual stop + start with verification
        stop_daemon()?;
        
        // Wait for daemon to actually stop (with timeout and verification)
        wait_for_daemon_stop()?;
        
        start_daemon()?;
    }

    Ok(())
}

/// Wait for daemon to fully stop, with timeout and verification
///
/// Uses PID file and platform::is_process_running() to verify shutdown
fn wait_for_daemon_stop() -> Result<()> {
    use std::fs;
    use std::time::{Duration, Instant};
    use crate::platform::{self, ProcessId};
    
    // Determine PID file location (matches config.rs default_pid_file logic)
    let is_elevated = platform::is_elevated();
    let pid_file_path = platform::runtime_dir(is_elevated).join("kodegend.pid");
    
    // If PID file doesn't exist, daemon is already stopped
    if !pid_file_path.exists() {
        return Ok(());
    }
    
    // Read PID from file
    let pid_str = fs::read_to_string(&pid_file_path)
        .context("Failed to read PID file")?;
    let pid: ProcessId = pid_str.trim().parse()
        .context("Failed to parse PID from PID file")?;
    
    // Poll with timeout
    let timeout = Duration::from_secs(30);
    let poll_interval = Duration::from_millis(100);
    let start = Instant::now();
    
    log::info!("Waiting for daemon (PID {}) to stop...", pid);
    
    loop {
        // Check if process is still running
        match platform::is_process_running(pid) {
            Ok(false) => {
                log::info!("Daemon stopped successfully");
                break;
            }
            Ok(true) => {
                // Process still running, check timeout
                if start.elapsed() > timeout {
                    log::warn!("Daemon did not stop within {}s, sending SIGKILL", timeout.as_secs());
                    
                    // Force kill on macOS
                    use nix::unistd::Pid;
                    use nix::sys::signal::{kill, Signal};
                    
                    kill(Pid::from_raw(pid), Signal::SIGKILL)
                        .context("Failed to send SIGKILL")?;
                    
                    // Wait a bit after SIGKILL
                    std::thread::sleep(Duration::from_millis(500));
                    
                    // Verify it's actually dead
                    if platform::is_process_running(pid).unwrap_or(true) {
                        anyhow::bail!("Daemon still running after SIGKILL (PID {}), cannot restart", pid);
                    }
                    
                    break;
                }
                
                // Still running, sleep and retry
                std::thread::sleep(poll_interval);
            }
            Err(e) => {
                log::warn!("Error checking process status: {}, assuming stopped", e);
                break;
            }
        }
    }
    
    // Extra safety delay to ensure resources fully released
    std::thread::sleep(Duration::from_millis(500));
    
    Ok(())
}
```

**Additional Import Required** at top of file:
```rust
use std::time::{Duration, Instant};
use std::fs;
use log;  // For logging (already imported in other control modules)
```

---

### File 2: `packages/kodegend/src/control/windows_control.rs`

**Change Location**: Lines 161-172 (the `restart_daemon()` function)

**Current Code**:
```rust
pub fn restart_daemon() -> Result<()> {
    // Stop the service
    stop_daemon()?;

    // Wait for service to fully stop
    std::thread::sleep(Duration::from_secs(1));  // ← REMOVE THIS

    // Start the service
    start_daemon()?;

    Ok(())
}
```

**New Implementation**:
```rust
pub fn restart_daemon() -> Result<()> {
    // Stop the service
    stop_daemon()?;

    // Wait for service to fully stop (with verification)
    wait_for_service_stop()?;

    // Start the service
    start_daemon()?;

    Ok(())
}

/// Wait for Windows service to fully stop, with timeout and verification
///
/// Uses PID file and platform::is_process_running() to verify shutdown
fn wait_for_service_stop() -> Result<()> {
    use std::fs;
    use std::time::{Duration, Instant};
    use crate::platform::{self, ProcessId};
    
    // Determine PID file location (matches config.rs default_pid_file logic)
    let is_elevated = platform::is_elevated();
    let pid_file_path = platform::runtime_dir(is_elevated).join("kodegend.pid");
    
    // If PID file doesn't exist, service is already stopped
    if !pid_file_path.exists() {
        return Ok(());
    }
    
    // Read PID from file
    let pid_str = fs::read_to_string(&pid_file_path)
        .context("Failed to read PID file")?;
    let pid: ProcessId = pid_str.trim().parse()
        .context("Failed to parse PID from PID file")?;
    
    // Poll with timeout
    let timeout = Duration::from_secs(30);
    let poll_interval = Duration::from_millis(100);
    let start = Instant::now();
    
    // Note: Windows doesn't have log crate imported in this module yet,
    // so we'll skip logging or add the import
    
    loop {
        // Check if process is still running
        match platform::is_process_running(pid) {
            Ok(false) => {
                // Process stopped successfully
                break;
            }
            Ok(true) => {
                // Process still running, check timeout
                if start.elapsed() > timeout {
                    // Windows services don't respond well to force termination
                    // If SCM stop didn't work after 30s, bail out
                    anyhow::bail!(
                        "Service did not stop within {}s (PID {}), cannot restart safely. \
                         Check Windows Event Viewer for service errors.",
                        timeout.as_secs(),
                        pid
                    );
                }
                
                // Still running, sleep and retry
                std::thread::sleep(poll_interval);
            }
            Err(e) => {
                // Error checking process (permission denied, etc)
                // Assume service stopped
                break;
            }
        }
    }
    
    // Extra safety delay to ensure resources fully released
    std::thread::sleep(Duration::from_millis(500));
    
    Ok(())
}
```

**Additional Imports Required** at top of file:
```rust
use std::time::{Duration, Instant};
use std::fs;
```

**Note**: The existing `use std::time::Duration;` on line 5 can be updated to include `Instant`.

---

## Implementation Notes

### Why This Approach Works

1. **Uses Existing Infrastructure**: Leverages `platform::is_process_running()` which is already tested and used by PID file validation
2. **Cross-Platform**: Works identically on macOS and Windows with platform-specific process checking
3. **No New Dependencies**: Uses only existing crates and patterns from the codebase
4. **Graceful with Escalation**: 
   - Waits up to 30 seconds for graceful shutdown
   - On macOS: Escalates to SIGKILL if needed
   - On Windows: Fails with clear error if service won't stop
5. **Safety Margins**: 500ms delay after verification ensures OS has time to release resources

### Why Linux Doesn't Need Changes

The Linux implementation delegates to systemd's native restart command:
```rust
pub fn restart_daemon() -> Result<()> {
    let output = Command::new("systemctl")
        .args(["restart", "kodegend.service"])
        .output()?;
    // systemd handles stop verification internally
}
```

Systemd's restart command already implements proper verification and timeout handling according to the service's `TimeoutStopSec` directive (default 90 seconds).

### Timeout Choice: 30 Seconds

Industry standards for graceful shutdown:
- **systemd**: 90s (TimeoutStopSec default)
- **Docker**: 10s (stop timeout default)
- **Kubernetes**: 30s (terminationGracePeriodSeconds default)

We chose 30 seconds as a reasonable middle ground that:
- Gives services adequate time to drain connections
- Isn't so long that users get impatient during restart
- Matches Kubernetes conventions (familiar to DevOps engineers)

### Poll Interval: 100ms

- **Responsive**: Detects shutdown within 100ms of actual stop
- **Low overhead**: 10 checks per second is negligible CPU usage
- **Proven pattern**: Matches polling intervals in other process supervisors

---

## Files Modified Summary

1. **`packages/kodegend/src/control/macos_control.rs`**
   - Modify `restart_daemon()` function (lines 117-134)
   - Add `wait_for_daemon_stop()` helper function
   - Add imports: `std::time::Instant`, `std::fs`, `log`

2. **`packages/kodegend/src/control/windows_control.rs`**
   - Modify `restart_daemon()` function (lines 161-172)
   - Add `wait_for_service_stop()` helper function
   - Add imports: `std::time::Instant`, `std::fs`

3. **`packages/kodegend/src/control/linux_control.rs`**
   - ✅ **No changes needed** - systemd handles verification

---

## Definition of Done

### Functional Requirements

1. **macOS restart command**:
   - Stops daemon via launchctl
   - Polls PID file and verifies process stopped using `platform::is_process_running()`
   - Waits up to 30 seconds for graceful shutdown
   - Escalates to SIGKILL after timeout
   - Waits additional 500ms after verification before starting new daemon
   - Fails with clear error if process won't die after SIGKILL

2. **Windows restart command**:
   - Stops service via SCM
   - Polls PID file and verifies process stopped using `platform::is_process_running()`
   - Waits up to 30 seconds for service to stop
   - Fails with clear error message if service won't stop (directs user to Event Viewer)
   - Waits additional 500ms after verification before starting new service

3. **Linux restart command**:
   - No changes (existing systemd delegation is correct)

### Code Quality Requirements

1. Uses existing `platform::is_process_running()` utility (no reinventing the wheel)
2. Uses existing PID file reading pattern from `daemon.rs`
3. Uses existing platform path utilities from `platform` module
4. Follows existing error handling patterns with `anyhow::Context`
5. Includes informative log messages on macOS (Windows can skip logging if not already set up)
6. No code duplication - both platforms use similar polling pattern

### Error Handling Requirements

1. Handles missing PID file gracefully (assumes already stopped)
2. Handles unparseable PID file content with clear error
3. Handles timeout scenario with appropriate escalation (SIGKILL on Unix, error on Windows)
4. Handles process check errors gracefully (assumes stopped if can't verify)
5. Final verification after SIGKILL to ensure process is actually dead

---

## Related Code References

- **PID file management**: [`packages/kodegend/src/daemon.rs`](../packages/kodegend/src/daemon.rs) (lines 31-145)
- **Platform utilities**: [`packages/kodegend/src/platform/mod.rs`](../packages/kodegend/src/platform/mod.rs)
- **Unix process checking**: [`packages/kodegend/src/platform/unix.rs`](../packages/kodegend/src/platform/unix.rs) (lines 55-62)
- **Windows process checking**: [`packages/kodegend/src/platform/windows.rs`](../packages/kodegend/src/platform/windows.rs)
- **Configuration**: [`packages/kodegend/src/config.rs`](../packages/kodegend/src/config.rs) (lines 31-66)
- **Control delegation**: [`packages/kodegend/src/control.rs`](../packages/kodegend/src/control.rs)
- **Main entry point**: [`packages/kodegend/src/main.rs`](../packages/kodegend/src/main.rs)

---

## Implementation Checklist

- [ ] Modify `packages/kodegend/src/control/macos_control.rs`:
  - [ ] Add required imports (`Instant`, `fs`, `log`)
  - [ ] Add `wait_for_daemon_stop()` helper function
  - [ ] Update `restart_daemon()` to call helper in fallback path
  - [ ] Verify SIGKILL logic compiles on macOS

- [ ] Modify `packages/kodegend/src/control/windows_control.rs`:
  - [ ] Add required imports (`Instant`, `fs`)
  - [ ] Add `wait_for_service_stop()` helper function
  - [ ] Update `restart_daemon()` to call helper
  - [ ] Verify error messages are clear and actionable

- [ ] Verify compilation:
  - [ ] `cd packages/kodegend && cargo check`
  - [ ] `cd packages/kodegend && cargo clippy`

- [ ] Manual verification (if possible):
  - [ ] Test restart on macOS (normal case - quick shutdown)
  - [ ] Test restart on macOS (slow case - simulate with sleep in service)
  - [ ] Test restart on Windows (normal case)
  - [ ] Test restart on Linux (verify no regression)
