# Cross-Platform: Inconsistent Error Handling in Daemon Control

## Location
All platform implementations:
- `packages/kodegend/src/control/linux_control.rs`
- `packages/kodegend/src/control/macos_control.rs`
- `packages/kodegend/src/control/windows_control.rs`
- **NEW**: `packages/kodegend/src/control/error.rs` (to be created)

## Issue Type
Code Quality / User Experience

## Severity
Medium

## Core Objective

Standardize error handling across all three platform-specific daemon control implementations (Linux/systemd, macOS/launchd, Windows/SCM) to provide consistent, actionable error messages with proper error codes. This improves debugging, reduces support burden, and creates a unified user experience across platforms.

---

## Current State Analysis

### Existing Error Pattern in Codebase

The kodegend codebase already uses **`thiserror::Error`** for structured error handling. See existing examples:

**Reference 1**: [`src/service.rs:21`](../packages/kodegend/src/service.rs#L21)
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Failed to spawn thread for service '{service}': {source}")]
    SpawnFailed {
        service: String,
        #[source]
        source: std::io::Error,
    },
    #[error("Channel send failed: {0}")]
    ChannelSend(#[from] crossbeam_channel::SendError<crate::ipc::Evt>),
}
```

**Reference 2**: [`src/install/installer/error.rs:6`](../packages/kodegend/src/install/installer/error.rs#L6)
```rust
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum InstallerError {
    #[error("User cancelled authorization")]
    Cancelled,
    #[error("Permission denied")]
    PermissionDenied,
    #[error("Executable not found: {0}")]
    MissingExecutable(String),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("System error: {0}")]
    System(String),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
```

**Key Pattern**: Use `#[derive(Error, Debug)]` with `#[error(...)]` attributes for Display implementation.

### Windows Error Handling Pattern

**Reference 3**: [`src/install/installer/core/context.rs:559`](../packages/kodegend/src/install/installer/core/context.rs#L559)
```rust
use windows::Win32::Foundation::GetLastError;

let error_code = unsafe { GetLastError() };
if error_code.0 == 1223 {
    return Err(anyhow::anyhow!("UAC elevation cancelled by user..."));
}
```

**Pattern**: Use `GetLastError()` and check `.0` field for error code as `u32`.

---

## Error Handling Problems

### Linux: Detailed stderr (Good, but not structured)
```rust
// From linux_control.rs:45
if !output.status.success() {
    anyhow::bail!(
        "Failed to start daemon: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
```
**Good**: Includes detailed systemctl stderr  
**Bad**: Not machine-parseable, just a string

### macOS: Inconsistent error handling
```rust
// From macos_control.rs:83 - SILENTLY IGNORES ERRORS
let _ = Command::new("launchctl")
    .args(["kill", "SIGTERM", SERVICE_LABEL])
    .output();
// ↑ No error checking whatsoever

// From macos_control.rs:70 - Sometimes detailed
if !load_output.status.success() {
    anyhow::bail!(
        "Failed to start daemon: {}",
        String::from_utf8_lossy(&load_output.stderr)
    );
}
```
**Bad**: Inconsistent - some operations ignore errors entirely

### Windows: Generic errors with NO details
```rust
// From windows_control.rs:31
if handle.is_invalid() {
    anyhow::bail!("Failed to open Service Control Manager");
}

// From windows_control.rs:92
if handle.is_invalid() {
    anyhow::bail!("Failed to open service: {}", SERVICE_NAME);
}

// From windows_control.rs:133
if result.is_err() {
    anyhow::bail!("Failed to start service");
}
```
**Bad**: No error codes, no details about WHY it failed  
**Missing**: Windows API provides error codes via `GetLastError()` but they're completely unused

---

## Windows Error Code Reference

### Critical Service Control Manager Error Codes

From Microsoft documentation research:

| Error Code | Constant | Meaning | User Action |
|------------|----------|---------|-------------|
| **5** | `ERROR_ACCESS_DENIED` | Permission denied | Run with elevated privileges (Administrator) |
| **1056** | `ERROR_SERVICE_ALREADY_RUNNING` | Service already running | Service is already started |
| **1060** | `ERROR_SERVICE_DOES_NOT_EXIST` | Service not found | Install the service first |
| **1062** | `ERROR_SERVICE_NOT_ACTIVE` | Service not running | Service is already stopped |
| **1063** | `ERROR_SERVICE_REQUEST_TIMEOUT` | Operation timed out | Wait and retry |
| **1223** | `ERROR_CANCELLED` | User cancelled | User denied UAC prompt |

**Source**: [StartServiceW function (Microsoft Learn)](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-startservicew)

---

## Implementation Plan

### Step 1: Create Common Error Type

**File**: `packages/kodegend/src/control/error.rs` (NEW FILE)

```rust
//! Common error types for daemon control operations across all platforms

use thiserror::Error;
use std::time::Duration;

/// Errors that can occur during daemon control operations
#[derive(Error, Debug)]
pub enum DaemonControlError {
    /// Service/daemon is not installed on the system
    #[error("Service '{service}' is not installed. Run 'kodegen install' first.")]
    ServiceNotFound { service: String },

    /// Insufficient permissions to perform the operation
    #[error("Permission denied for operation: {operation}. Try running with elevated privileges (sudo/Administrator).")]
    PermissionDenied { operation: String },

    /// Service is already running
    #[error("Service is already running")]
    ServiceAlreadyRunning,

    /// Service is not currently running
    #[error("Service is not running")]
    ServiceNotRunning,

    /// Operation timed out
    #[error("Operation '{operation}' timed out after {duration:?}")]
    Timeout { operation: String, duration: Duration },

    /// Platform-specific system error with details
    #[error("{message} (error code: {code})")]
    SystemError { message: String, code: i32 },

    /// System error without a code
    #[error("{0}")]
    SystemErrorNoCode(String),

    /// I/O error executing platform command
    #[error("Failed to execute {command}: {source}")]
    CommandExecution {
        command: String,
        #[source]
        source: std::io::Error,
    },
}
```

**Why this structure**:
- Follows existing `thiserror::Error` pattern used in [`service.rs`](../packages/kodegend/src/service.rs#L21) and [`error.rs`](../packages/kodegend/src/install/installer/error.rs#L6)
- Each variant has actionable error message with user guidance
- SystemError includes error codes for debugging
- Machine-parseable via pattern matching

### Step 2: Export Error from control.rs

**File**: `packages/kodegend/src/control.rs`

**Add** after the module declarations (line ~13):
```rust
mod error;
pub use error::DaemonControlError;
```

### Step 3: Update Linux Implementation

**File**: `packages/kodegend/src/control/linux_control.rs`

**Change 1**: Update imports (line 3)
```rust
use anyhow::{Context, Result};
```
**TO**:
```rust
use super::DaemonControlError;
type Result<T> = std::result::Result<T, DaemonControlError>;
```

**Change 2**: Update `start_daemon()` function (lines 33-50)
```rust
pub fn start_daemon() -> Result<()> {
    let service_name = format!("{}.service", SERVICE_NAME);
    let args = if is_root() {
        vec!["start", &service_name]
    } else {
        vec!["--user", "start", &service_name]
    };

    let output = Command::new("systemctl")
        .args(&args)
        .output()
        .map_err(|e| DaemonControlError::CommandExecution {
            command: "systemctl".to_string(),
            source: e,
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        // Parse common systemd error patterns
        if stderr.contains("not found") || stderr.contains("could not be found") {
            return Err(DaemonControlError::ServiceNotFound {
                service: SERVICE_NAME.to_string(),
            });
        } else if stderr.contains("Permission denied") || stderr.contains("Access denied") {
            return Err(DaemonControlError::PermissionDenied {
                operation: "start".to_string(),
            });
        } else if stderr.contains("already running") || stderr.contains("already active") {
            return Err(DaemonControlError::ServiceAlreadyRunning);
        }
        
        // Fallback to detailed system error
        return Err(DaemonControlError::SystemError {
            message: stderr.to_string(),
            code: output.status.code().unwrap_or(-1),
        });
    }

    Ok(())
}
```

**Apply similar changes to**: `stop_daemon()`, `restart_daemon()`, `check_status()`

### Step 4: Update macOS Implementation

**File**: `packages/kodegend/src/control/macos_control.rs`

**Change 1**: Update imports and result type (same as Linux)

**Change 2**: Update `start_daemon()` (lines 47-76)
```rust
pub fn start_daemon() -> Result<()> {
    // Try modern bootstrap first (may fail if already loaded - that's OK)
    let _ = Command::new("launchctl")
        .args(["bootstrap", "system", PLIST_PATH])
        .output();

    // Then kickstart to ensure it starts
    let output = Command::new("launchctl")
        .args(["kickstart", SERVICE_LABEL])
        .output()
        .map_err(|e| DaemonControlError::CommandExecution {
            command: "launchctl kickstart".to_string(),
            source: e,
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        // Try fallback to legacy load command
        let load_output = Command::new("launchctl")
            .args(["load", "-w", PLIST_PATH])
            .output()
            .map_err(|e| DaemonControlError::CommandExecution {
                command: "launchctl load".to_string(),
                source: e,
            })?;

        if !load_output.status.success() {
            let load_stderr = String::from_utf8_lossy(&load_output.stderr);
            
            // Parse launchd error patterns
            if load_stderr.contains("Could not find") || load_stderr.contains("not found") {
                return Err(DaemonControlError::ServiceNotFound {
                    service: SERVICE_LABEL.to_string(),
                });
            } else if load_stderr.contains("Operation not permitted") 
                || load_stderr.contains("Permission denied") {
                return Err(DaemonControlError::PermissionDenied {
                    operation: "start".to_string(),
                });
            } else if load_stderr.contains("Already loaded") {
                return Err(DaemonControlError::ServiceAlreadyRunning);
            }
            
            return Err(DaemonControlError::SystemError {
                message: load_stderr.to_string(),
                code: load_output.status.code().unwrap_or(-1),
            });
        }
    }

    Ok(())
}
```

**Change 3**: **FIX** `stop_daemon()` silent error (lines 81-84)
```rust
// OLD: Silent error ignore
let _ = Command::new("launchctl")
    .args(["kill", "SIGTERM", SERVICE_LABEL])
    .output();

// NEW: Proper error handling
let kill_result = Command::new("launchctl")
    .args(["kill", "SIGTERM", SERVICE_LABEL])
    .output()
    .map_err(|e| DaemonControlError::CommandExecution {
        command: "launchctl kill".to_string(),
        source: e,
    })?;

// Check if kill failed because service is not running (acceptable)
if !kill_result.status.success() {
    let stderr = String::from_utf8_lossy(&kill_result.stderr);
    if !stderr.contains("Could not find service") {
        // Only error if it's not "service not found"
        eprintln!("Warning: Failed to send SIGTERM: {}", stderr);
    }
}
```

**Apply similar changes to**: `stop_daemon()`, `restart_daemon()`, `check_status()`

### Step 5: Update Windows Implementation

**File**: `packages/kodegend/src/control/windows_control.rs`

**Change 1**: Update imports (line 3)
```rust
use anyhow::{Context, Result};
use std::mem;
use std::time::Duration;
use windows::core::PCWSTR;
use windows::Win32::System::Services::{...};
```
**TO**:
```rust
use super::DaemonControlError;
use std::mem;
use std::time::Duration;
use windows::core::PCWSTR;
use windows::Win32::Foundation::GetLastError;
use windows::Win32::System::Services::{...};

type Result<T> = std::result::Result<T, DaemonControlError>;
```

**Change 2**: Update `ScManagerHandle::new()` (lines 23-35)
```rust
fn new() -> Result<Self> {
    let handle = unsafe {
        OpenSCManagerW(
            PCWSTR::null(),
            PCWSTR::null(),
            SC_MANAGER_CONNECT.0,
        )
    };

    if handle.is_invalid() {
        let error_code = unsafe { GetLastError() };
        return Err(map_windows_error(error_code.0, "open Service Control Manager"));
    }

    Ok(ScManagerHandle(handle))
}
```

**Change 3**: Update `open_service()` (lines 64-82)
```rust
fn open_service(sc_manager: &ScManagerHandle, access: u32) -> Result<ServiceHandle> {
    let service_name: Vec<u16> = SERVICE_NAME.encode_utf16().chain(Some(0)).collect();

    let handle = unsafe {
        OpenServiceW(
            sc_manager.handle(),
            PCWSTR(service_name.as_ptr()),
            access,
        )
    };

    if handle.is_invalid() {
        let error_code = unsafe { GetLastError() };
        return Err(map_windows_error(error_code.0, "open service"));
    }

    Ok(ServiceHandle(handle))
}
```

**Change 4**: Update `check_status()` (lines 84-120)
```rust
pub fn check_status() -> Result<bool> {
    let sc_manager = ScManagerHandle::new()?;
    let service = open_service(&sc_manager, SERVICE_QUERY_STATUS.0)?;

    let mut status: SERVICE_STATUS_PROCESS = unsafe { mem::zeroed() };
    let mut bytes_needed: u32 = 0;

    let result = unsafe {
        QueryServiceStatusEx(
            service.handle(),
            SC_STATUS_PROCESS_INFO,
            Some(&mut status as *mut _ as *mut u8),
            mem::size_of::<SERVICE_STATUS_PROCESS>() as u32,
            &mut bytes_needed,
        )
    };

    if result.is_err() {
        let error_code = unsafe { GetLastError() };
        return Err(map_windows_error(error_code.0, "query service status"));
    }

    Ok(status.dwCurrentState == SERVICE_RUNNING.0)
}
```

**Change 5**: Update `start_daemon()` (lines 122-139)
```rust
pub fn start_daemon() -> Result<()> {
    let sc_manager = ScManagerHandle::new()?;
    let service = open_service(&sc_manager, SERVICE_START.0)?;

    let result = unsafe {
        StartServiceW(service.handle(), None)
    };

    if result.is_err() {
        let error_code = unsafe { GetLastError() };
        return Err(map_windows_error(error_code.0, "start service"));
    }

    Ok(())
}
```

**Change 6**: Update `stop_daemon()` (lines 141-160)
```rust
pub fn stop_daemon() -> Result<()> {
    let sc_manager = ScManagerHandle::new()?;
    let service = open_service(&sc_manager, SERVICE_STOP.0)?;

    let mut status: SERVICE_STATUS = unsafe { mem::zeroed() };

    let result = unsafe {
        ControlService(service.handle(), SERVICE_CONTROL_STOP, &mut status)
    };

    if result.is_err() {
        let error_code = unsafe { GetLastError() };
        return Err(map_windows_error(error_code.0, "stop service"));
    }

    Ok(())
}
```

**Change 7**: Add Windows error mapping helper at end of file
```rust
/// Map Windows error codes to DaemonControlError
fn map_windows_error(error_code: u32, operation: &str) -> DaemonControlError {
    match error_code {
        5 => DaemonControlError::PermissionDenied {
            operation: operation.to_string(),
        },
        1056 => DaemonControlError::ServiceAlreadyRunning,
        1060 => DaemonControlError::ServiceNotFound {
            service: SERVICE_NAME.to_string(),
        },
        1062 => DaemonControlError::ServiceNotRunning,
        1063 => DaemonControlError::Timeout {
            operation: operation.to_string(),
            duration: Duration::from_secs(30),
        },
        _ => DaemonControlError::SystemError {
            message: format!("Windows error during {}", operation),
            code: error_code as i32,
        },
    }
}
```

**Apply similar changes to**: `restart_daemon()`

---

## Cross-Platform Error Mapping

### Service Not Found

| Platform | Detection Method |
|----------|------------------|
| **Linux** | stderr contains "not found" or "could not be found" |
| **macOS** | stderr contains "Could not find" or "not found" |
| **Windows** | Error code 1060 (`ERROR_SERVICE_DOES_NOT_EXIST`) |

### Permission Denied

| Platform | Detection Method |
|----------|------------------|
| **Linux** | stderr contains "Permission denied" or "Access denied" |
| **macOS** | stderr contains "Operation not permitted" or "Permission denied" |
| **Windows** | Error code 5 (`ERROR_ACCESS_DENIED`) |

### Service Already Running

| Platform | Detection Method |
|----------|------------------|
| **Linux** | stderr contains "already running" or "already active" |
| **macOS** | stderr contains "Already loaded" |
| **Windows** | Error code 1056 (`ERROR_SERVICE_ALREADY_RUNNING`) |

### Service Not Running

| Platform | Detection Method |
|----------|------------------|
| **Linux** | Exit code 3 from `systemctl is-active` |
| **macOS** | PID is "-" in `launchctl list` output |
| **Windows** | Error code 1062 (`ERROR_SERVICE_NOT_ACTIVE`) or `dwCurrentState != SERVICE_RUNNING` |

---

## Files to Modify

1. **CREATE**: `packages/kodegend/src/control/error.rs` - Common error type definition
2. **MODIFY**: `packages/kodegend/src/control.rs` - Export DaemonControlError
3. **MODIFY**: `packages/kodegend/src/control/linux_control.rs` - Use DaemonControlError, parse systemd errors
4. **MODIFY**: `packages/kodegend/src/control/macos_control.rs` - Use DaemonControlError, parse launchd errors, **FIX silent error ignoring**
5. **MODIFY**: `packages/kodegend/src/control/windows_control.rs` - Use DaemonControlError, add GetLastError() calls, map error codes

---

## Definition of Done

- [ ] `control/error.rs` created with `DaemonControlError` enum using `thiserror::Error`
- [ ] All three platform control modules (`linux_control.rs`, `macos_control.rs`, `windows_control.rs`) return `Result<T, DaemonControlError>` instead of `anyhow::Result<T>`
- [ ] **Linux**: Parse systemd stderr for common error patterns (not found, permission denied, already running)
- [ ] **macOS**: Parse launchd stderr for common error patterns, **remove all silent error ignoring** (`let _ =`)
- [ ] **Windows**: Use `GetLastError()` after failed API calls, map error codes 5/1056/1060/1062/1063 to appropriate error variants
- [ ] `control.rs` exports `DaemonControlError` publicly
- [ ] All error messages include actionable guidance (e.g., "Try running with elevated privileges")
- [ ] Error codes are included in error messages where available

---

## Benefits

### 1. Consistent User Experience
Users on Linux, macOS, and Windows will see similar error messages for the same underlying problems.

### 2. Actionable Errors
Every error includes what the user should do next:
- "Run 'kodegen install' first" for service not found
- "Try running with elevated privileges" for permission denied
- Error codes for debugging system-specific issues

### 3. Better Debugging
Support team can quickly identify issues from error messages without needing platform-specific knowledge.

### 4. Machine-Parseable
Calling code can match on error types instead of parsing strings:
```rust
match control::start_daemon() {
    Ok(()) => println!("Started successfully"),
    Err(DaemonControlError::ServiceNotFound { .. }) => {
        println!("Installing service...");
        // Attempt installation
    }
    Err(DaemonControlError::PermissionDenied { .. }) => {
        eprintln!("Rerun with sudo/Administrator");
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

### 5. Testable
Can test error handling logic without mocking platform APIs by creating error instances directly.

---

## Related Code References

- Error pattern: [`src/service.rs:21`](../packages/kodegend/src/service.rs#L21)
- Error pattern: [`src/install/installer/error.rs:6`](../packages/kodegend/src/install/installer/error.rs#L6)
- Windows GetLastError usage: [`src/install/installer/core/context.rs:559`](../packages/kodegend/src/install/installer/core/context.rs#L559)
- Current Linux implementation: [`src/control/linux_control.rs`](../packages/kodegend/src/control/linux_control.rs)
- Current macOS implementation: [`src/control/macos_control.rs`](../packages/kodegend/src/control/macos_control.rs)
- Current Windows implementation: [`src/control/windows_control.rs`](../packages/kodegend/src/control/windows_control.rs)

---

## Dependencies Already Available

From `packages/kodegend/Cargo.toml`:
- ✅ `thiserror = "2"` - Already in dependencies
- ✅ `windows = { version = "0.62", features = [..., "Win32_Foundation", "Win32_System_Services", ...] }` - Already configured for Windows

No new dependencies required.
