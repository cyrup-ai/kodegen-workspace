# Opaque Error Handling in Control Module

## Location
- **Primary Target**: [`packages/kodegend/src/control.rs:27-44`](../packages/kodegend/src/control.rs)
- **Related Files**: 
  - [`packages/kodegend/src/control/linux_control.rs`](../packages/kodegend/src/control/linux_control.rs)
  - [`packages/kodegend/src/control/macos_control.rs`](../packages/kodegend/src/control/macos_control.rs)
  - [`packages/kodegend/src/control/windows_control.rs`](../packages/kodegend/src/control/windows_control.rs)
  - [`packages/kodegend/src/main.rs:196-254`](../packages/kodegend/src/main.rs) (error display handlers)

## Severity
🟠 **MEDIUM - USER EXPERIENCE ISSUE**

## Core Objective

**Improve error messages when daemon control operations fail** by adding contextual information at the control abstraction layer. Currently, when users run commands like `kodegend start`, they receive raw platform errors with no indication of what operation failed or how to fix it.

---

## Current Implementation Analysis

### Control Module (Primary Target)

**File**: `packages/kodegend/src/control.rs`

```rust
//! Daemon lifecycle control - delegates to OS-native daemon managers
//!
//! Provides a unified interface for managing the daemon across different operating systems:
//! - macOS: launchd (launchctl)
//! - Linux: systemd (systemctl)
//! - Windows: Service Control Manager (Windows API)

use anyhow::Result;

// Platform-specific implementations
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
    }
}

/// Check if daemon is running
///
/// Returns: Ok(true) if running, Ok(false) if stopped
pub fn check_status() -> Result<bool> {
    platform::check_status()  // ⚠️ NO CONTEXT ADDED
}

/// Start the daemon service
pub fn start_daemon() -> Result<()> {
    platform::start_daemon()  // ⚠️ NO CONTEXT ADDED
}

/// Stop the daemon service
pub fn stop_daemon() -> Result<()> {
    platform::stop_daemon()  // ⚠️ NO CONTEXT ADDED
}

/// Restart the daemon service
pub fn restart_daemon() -> Result<()> {
    platform::restart_daemon()  // ⚠️ NO CONTEXT ADDED
}
```

**Problem**: All four public functions pass through errors without adding context about which daemon operation failed.

### How Errors Are Displayed to Users

**File**: `packages/kodegend/src/main.rs` (lines 196-254)

```rust
/// Handle status command - check if daemon is running
fn handle_status() -> Result<()> {
    match control::check_status() {
        Ok(true) => {
            println!("kodegend is running");
            std::process::exit(0);
        }
        Ok(false) => {
            println!("kodegend is stopped");
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("Error checking status: {e:#}");  // ← Uses {e:#} for full error chain
            std::process::exit(1);
        }
    }
}

/// Handle start command - start the daemon service
fn handle_start() -> Result<()> {
    match control::start_daemon() {
        Ok(()) => {
            println!("kodegend started successfully");
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("Failed to start: {e:#}");  // ← Uses {e:#} for full error chain
            std::process::exit(1);
        }
    }
}

// Similar patterns for handle_stop() and handle_restart()
```

**Key Insight**: Errors are displayed using `{e:#}` formatting, which shows the **full error chain** including all `.context()` messages. This means adding context in `control.rs` will **directly improve user-facing error messages**.

### Platform Module Analysis

All three platform modules already use `anyhow::Context` for some operations, but have room for improvement:

#### Linux (systemd)
**File**: [`packages/kodegend/src/control/linux_control.rs`](../packages/kodegend/src/control/linux_control.rs)

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
        .context("Failed to execute systemctl start")?;  // ✓ Has context for Command

    if !output.status.success() {
        anyhow::bail!(
            "Failed to start daemon: {}",  // ⚠️ Generic message
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}
```

**Status**: Uses `.context()` for Command execution but could provide more helpful messages in `bail!()`.

#### macOS (launchd)
**File**: [`packages/kodegend/src/control/macos_control.rs`](../packages/kodegend/src/control/macos_control.rs)

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
        .context("Failed to execute launchctl kickstart")?;  // ✓ Has context

    if !output.status.success() {
        // Fallback to legacy load command
        let load_output = Command::new("launchctl")
            .args(["load", "-w", PLIST_PATH])
            .output()
            .context("Failed to execute launchctl load")?;  // ✓ Has context

        if !load_output.status.success() {
            anyhow::bail!(
                "Failed to start daemon: {}",  // ⚠️ Generic message
                String::from_utf8_lossy(&load_output.stderr)
            );
        }
    }

    Ok(())
}
```

**Status**: Good use of `.context()` but could add actionable suggestions in error messages.

#### Windows (Service Control Manager)
**File**: [`packages/kodegend/src/control/windows_control.rs`](../packages/kodegend/src/control/windows_control.rs)

```rust
pub fn start_daemon() -> Result<()> {
    let sc_manager = ScManagerHandle::new()
        .context("Failed to open Service Control Manager for start")?;  // ✓ Has context

    let service = open_service(&sc_manager, SERVICE_START.0)
        .context("Failed to open service for start")?;  // ✓ Has context

    let result = unsafe {
        StartServiceW(service.handle(), None)
    };

    if result.is_err() {
        anyhow::bail!("Failed to start service");  // ⚠️ Generic message
    }

    Ok(())
}
```

**Status**: Good use of `.context()` but could be more specific about failures.

---

## Impact: Before vs After

### Current User Experience (Bad)

User runs: `kodegend start`

**Output**:
```
Failed to start: No such file or directory (os error 2)
```

**User confusion**:
- Which file is missing?
- Is it the plist file? The executable? The systemctl binary?
- What should I do to fix this?

### Improved User Experience (Good)

User runs: `kodegend start`

**Output**:
```
Failed to start: Failed to start daemon service: Failed to execute launchctl kickstart: No such file or directory (os error 2)
```

**Now the user knows**:
- The operation that failed: "start daemon service"
- The specific command: "launchctl kickstart"
- The root cause: "No such file or directory"

This is **much more actionable** and helps users self-diagnose issues.

---

## Implementation Guide

### Step 1: Add Context to Control Module (REQUIRED)

**File**: `packages/kodegend/src/control.rs` (lines 27-44)

**Change needed**: Import `Context` trait and add `.context()` to each function.

**Before**:
```rust
use anyhow::Result;

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

**After**:
```rust
use anyhow::{Context, Result};

/// Check if daemon is running
///
/// Returns: Ok(true) if running, Ok(false) if stopped
pub fn check_status() -> Result<bool> {
    platform::check_status()
        .context("Failed to check daemon status")
}

/// Start the daemon service
pub fn start_daemon() -> Result<()> {
    platform::start_daemon()
        .context("Failed to start daemon service")
}

/// Stop the daemon service
pub fn stop_daemon() -> Result<()> {
    platform::stop_daemon()
        .context("Failed to stop daemon service")
}

/// Restart the daemon service
pub fn restart_daemon() -> Result<()> {
    platform::restart_daemon()
        .context("Failed to restart daemon service")
}
```

**Exact changes**:
1. Line 8: Change `use anyhow::Result;` to `use anyhow::{Context, Result};`
2. Line 28: Add `.context("Failed to check daemon status")` after `platform::check_status()`
3. Line 33: Add `.context("Failed to start daemon service")` after `platform::start_daemon()`
4. Line 38: Add `.context("Failed to stop daemon service")` after `platform::stop_daemon()`
5. Line 43: Add `.context("Failed to restart daemon service")` after `platform::restart_daemon()`

---

### Step 2: Enhanced Platform Error Messages (OPTIONAL)

The following enhancements are **optional** and can improve error messages further, but are **not required** for this task.

#### Linux Enhancement Example

**File**: `packages/kodegend/src/control/linux_control.rs`

**Current** (lines 45-50):
```rust
if !output.status.success() {
    anyhow::bail!(
        "Failed to start daemon: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
```

**Enhanced**:
```rust
if !output.status.success() {
    let stderr = String::from_utf8_lossy(&output.stderr);
    anyhow::bail!(
        "systemctl start failed: {}\n\
         \n\
         Possible solutions:\n\
         - Check if service is installed: systemctl --user list-unit-files | grep kodegend\n\
         - View service logs: journalctl --user -u kodegend\n\
         - Reinstall service: kodegend install",
        stderr
    );
}
```

#### macOS Enhancement Example

**File**: `packages/kodegend/src/control/macos_control.rs`

**Current** (lines 68-71):
```rust
anyhow::bail!(
    "Failed to start daemon: {}",
    String::from_utf8_lossy(&load_output.stderr)
);
```

**Enhanced**:
```rust
let stderr = String::from_utf8_lossy(&load_output.stderr);
anyhow::bail!(
    "launchctl failed to start daemon: {}\n\
     \n\
     Possible causes:\n\
     - Launch agent plist not installed at {}\n\
     - Insufficient permissions to access /Library/LaunchDaemons/\n\
     - Service already loaded but not running\n\
     \n\
     Try: sudo kodegend install",
    stderr,
    PLIST_PATH
);
```

#### Windows Enhancement Example

**File**: `packages/kodegend/src/control/windows_control.rs`

**Current** (lines 132-134):
```rust
if result.is_err() {
    anyhow::bail!("Failed to start service");
}
```

**Enhanced**:
```rust
if result.is_err() {
    let win_error = unsafe { windows::Win32::Foundation::GetLastError() };
    anyhow::bail!(
        "Failed to start Windows service (error code: {:?})\n\
         \n\
         Possible causes:\n\
         - Service not installed (run 'kodegend install' as Administrator)\n\
         - Service already running\n\
         - Insufficient permissions (requires Administrator)\n\
         \n\
         Check Windows Event Viewer for details",
        win_error
    );
}
```

---

## Dependencies

**Already available**: `anyhow` is already a dependency in `Cargo.toml`:

```toml
anyhow = "1"
```

**No additional dependencies needed.**

---

## Error Handling Reference

### anyhow::Context Documentation

- **Official Docs**: https://docs.rs/anyhow/latest/anyhow/
- **Context trait**: https://docs.rs/anyhow/latest/anyhow/trait.Context.html

**Key points**:
1. `.context()` is zero-cost on the happy path (only allocates on error)
2. Context messages are displayed in order from most recent to root cause
3. Use descriptive, actionable messages
4. Format: `"Failed to <action>"` or `"Error during <operation>"`

### Error Chain Display Format

When using `{e:#}` formatting (as used in `main.rs`), errors display like:

```
Failed to start daemon service: Failed to execute launchctl kickstart: No such file or directory (os error 2)
│                                │                                      │
│                                │                                      └─ Root cause (from OS)
│                                └─ Platform layer context
└─ Control layer context (what we're adding)
```

---

## Definition of Done

### Minimum Requirements (MUST HAVE)

- [ ] **File modified**: `packages/kodegend/src/control.rs`
  - [ ] Import `Context` trait: `use anyhow::{Context, Result};`
  - [ ] Add `.context("Failed to check daemon status")` to `check_status()`
  - [ ] Add `.context("Failed to start daemon service")` to `start_daemon()`
  - [ ] Add `.context("Failed to stop daemon service")` to `stop_daemon()`
  - [ ] Add `.context("Failed to restart daemon service")` to `restart_daemon()`

### Success Criteria

1. **Code compiles** without warnings: `cargo check` passes in `packages/kodegend/`
2. **Error messages improved**: When a daemon control operation fails, the error message now includes:
   - Which operation failed (start/stop/restart/status)
   - The control layer context
   - The platform layer details
   - The root cause from the OS

### Verification

Run `cargo check` in the kodegend package:
```bash
cd packages/kodegend
cargo check
```

Should complete without errors or warnings.

---

## Related Files for Reference

- **Control abstraction**: [`packages/kodegend/src/control.rs`](../packages/kodegend/src/control.rs)
- **Linux implementation**: [`packages/kodegend/src/control/linux_control.rs`](../packages/kodegend/src/control/linux_control.rs)
- **macOS implementation**: [`packages/kodegend/src/control/macos_control.rs`](../packages/kodegend/src/control/macos_control.rs)
- **Windows implementation**: [`packages/kodegend/src/control/windows_control.rs`](../packages/kodegend/src/control/windows_control.rs)
- **Error display handlers**: [`packages/kodegend/src/main.rs:196-254`](../packages/kodegend/src/main.rs)
- **Dependencies**: [`packages/kodegend/Cargo.toml`](../packages/kodegend/Cargo.toml)

---

## Architecture Context

### Control Module Hierarchy

```
packages/kodegend/src/
├── control.rs                    # ← MODIFY THIS (add .context())
│   └── Platform abstraction layer (public API)
└── control/
    ├── linux_control.rs          # ← Optional enhancements
    ├── macos_control.rs          # ← Optional enhancements
    └── windows_control.rs        # ← Optional enhancements
        └── Platform implementation layer
```

### Error Flow

```
User runs: kodegend start
    ↓
main.rs::handle_start()
    ↓
control.rs::start_daemon()          ← Add .context() here
    ↓
platform::start_daemon()            ← Already has .context() on Command
    (linux_control.rs OR macos_control.rs OR windows_control.rs)
    ↓
OS error (if failure)
    ↓
Error chain bubbles up with all contexts
    ↓
main.rs displays with {e:#}
```

---

## Performance Considerations

**anyhow::Context is zero-cost on the happy path**:
- Adding `.context()` does **NOT** allocate memory when operations succeed
- Context strings are only materialized when an error occurs
- No performance impact on normal daemon operations
- Negligible overhead even on error path (string allocation is trivial compared to daemon startup/shutdown)

**Benchmark**: According to anyhow documentation, the overhead is ~1ns per context on error path (unmeasurable in practice).

---

## Priority

**MEDIUM** - This task:
- ✅ Doesn't break functionality (errors still propagate correctly)
- ✅ Significantly improves user experience (clearer error messages)
- ✅ Helps users troubleshoot issues independently
- ✅ Makes the daemon feel more polished and professional
- ⚠️ Not urgent (daemon works, just has poor error UX)

---

## Related Issues

- **Task 01**: Platform modules are already implemented ✓
- **Task 07**: Command visibility would complement error context
- **Future**: Could add telemetry to track common error patterns

---

## Additional Resources

### anyhow Crate Resources
- Official documentation: https://docs.rs/anyhow/latest/anyhow/
- Context trait: https://docs.rs/anyhow/latest/anyhow/trait.Context.html
- Error handling guide: https://blog.logrocket.com/error-handling-rust/

### Local Documentation
- Scraped anyhow docs: [`docs/docs.rs/`](../docs/docs.rs/) (if crawl completed)

### Rust Error Handling Patterns
- Rust book on error handling: https://doc.rust-lang.org/book/ch09-00-error-handling.html
- Error handling best practices: https://rust-for-c-programmers.com/ch15/15_6_best_practices_for_error_handling.html
