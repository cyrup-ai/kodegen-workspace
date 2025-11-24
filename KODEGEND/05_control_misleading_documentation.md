# Misleading and Unclear Documentation

## Location
`packages/kodegend/src/control.rs:1-6` (module-level doc comments)

## Severity
🟡 **LOW - CLARITY ISSUE**

## Issue Description
The module documentation is confusing and potentially misleading:

```rust
//! Daemon lifecycle control - delegates to OS-native daemon managers
//!
//! Provides a unified interface for managing the daemon across different operating systems:
//! - macOS: launchd (launchctl)
//! - Linux: systemd (systemctl)
//! - Windows: Service Control Manager (Windows API)
```

This documentation is **ambiguous** about what this module actually does.

## Implementation Context

### Source Files
- **Main control module**: [`src/control.rs`](../packages/kodegend/src/control.rs) - 44 lines, platform dispatch
- **Linux implementation**: [`src/control/linux_control.rs`](../packages/kodegend/src/control/linux_control.rs) - Uses `systemctl` commands (107 lines)
- **macOS implementation**: [`src/control/macos_control.rs`](../packages/kodegend/src/control/macos_control.rs) - Uses `launchctl` commands (134 lines)
- **Windows implementation**: [`src/control/windows_control.rs`](../packages/kodegend/src/control/windows_control.rs) - Uses SCM Win32 API (172 lines)
- **Daemon logic**: [`src/daemon.rs`](../packages/kodegend/src/daemon.rs) - Double-fork daemonization and PID file management (251 lines)
- **Platform abstraction**: [`src/platform/mod.rs`](../packages/kodegend/src/platform/mod.rs) - Service manager detection (129 lines)
- **Main orchestration**: [`src/main.rs`](../packages/kodegend/src/main.rs) - CLI routing and daemon lifecycle (254 lines)
- **Service installation**: [`src/install/`](../packages/kodegend/src/install/) - Installation wizard and platform-specific installers

### Current Implementation Analysis

#### CLI Command Flow ([`src/main.rs:56-74`](../packages/kodegend/src/main.rs#L56-L74))
```rust
match args.sub.unwrap_or(cli::Cmd::Run { .. }) {
    cli::Cmd::Run { foreground, config, system } => run_daemon(foreground, config, system).await,
    cli::Cmd::Status => handle_status(),    // → control::check_status()
    cli::Cmd::Start => handle_start(),      // → control::start_daemon()
    cli::Cmd::Stop => handle_stop(),        // → control::stop_daemon()
    cli::Cmd::Restart => handle_restart(),  // → control::restart_daemon()
}
```

**Key insight**: `Run` command executes the daemon directly, while `Start/Stop/Restart/Status` commands control an installed service via the control module.

#### Platform-Specific Control Implementations

**Linux** ([`src/control/linux_control.rs`](../packages/kodegend/src/control/linux_control.rs)):
- Service name: `"kodegend.service"`
- Auto-detects root via `nix::unistd::getuid().is_root()`
- Uses `systemctl --user` for user services, `systemctl` for system services
- Commands: `is-active`, `start`, `stop`, `restart`

**macOS** ([`src/control/macos_control.rs`](../packages/kodegend/src/control/macos_control.rs)):
- Service label: `"ai.kodegen.kodegend"`
- Plist path: `"/Library/LaunchDaemons/kodegend.plist"` (hardcoded as system service)
- Uses modern `launchctl` commands: `bootstrap`, `kickstart`, `kill SIGTERM`, `bootout`
- Falls back to legacy commands: `load -w`, `unload -w`
- Implements restart via `kickstart -k` (kill and restart)

**Windows** ([`src/control/windows_control.rs`](../packages/kodegend/src/control/windows_control.rs)):
- Service name: `"kodegend"`
- Uses Win32 SCM API: `OpenSCManagerW`, `OpenServiceW`, `QueryServiceStatusEx`, `StartServiceW`, `ControlService`
- RAII wrappers for `SC_HANDLE` cleanup
- Checks `SERVICE_RUNNING` state (value 4)
- Restart implemented as stop + 1s sleep + start

#### Two Execution Modes

The codebase supports **two distinct deployment modes** ([`src/main.rs:76-86`](../packages/kodegend/src/main.rs#L76-L86)):

1. **Service Manager Mode** (production):
   ```rust
   let should_stay_foreground = force_foreground || platform::running_under_service_manager();
   if !should_stay_foreground {
       daemon::daemonise()?;  // Skipped if under service manager
   }
   ```
   - Detected via `platform::running_under_service_manager()`:
     - Linux: Checks `INVOCATION_ID` env var (systemd)
     - macOS: Checks launchd environment
     - Windows: Checks if running as Windows Service
   - Daemon stays in foreground (service manager handles process lifecycle)
   - Control via `control` module (`kodegend start/stop/restart`)

2. **Standalone Mode** (development):
   - Manual execution: `kodegend run`
   - Performs Unix double-fork daemonization ([`src/daemon.rs:147-251`](../packages/kodegend/src/daemon.rs#L147-L251)):
     - First fork: Parent exits, child continues
     - `setsid()`: Drop controlling TTY
     - Second fork: Prevent reacquiring TTY
     - `chdir /`: Change to root directory
     - Close all FDs ≥ 3 (up to RLIMIT_NOFILE, capped at 65536)
     - Redirect stdin/stdout/stderr to `/dev/null`
   - PID file management via RAII guard ([`src/daemon.rs:35-145`](../packages/kodegend/src/daemon.rs#L35-L145))
   - Limited lifecycle control (SIGTERM for stop)

## Ambiguity Analysis

### What the docs COULD mean (Interpretation A):
"This module controls the kodegend daemon itself by delegating to OS service managers"
- i.e., `kodegend start` uses systemd to start itself as a service
- Requires prior installation of systemd unit file / launchd plist
- This is the **CORRECT interpretation** ✅

### What the docs COULD mean (Interpretation B):
"This module controls OTHER daemons that kodegend manages"
- i.e., kodegend is a daemon manager that controls child services
- Would be more like a supervisor (systemd replacement)
- **NOT what this module does** ❌

### Architecture Confusion Sources

1. **Dual execution modes**: Both self-daemonization (`daemon::daemonise()`) AND service manager integration exist
2. **Service manager terminology**: "Daemon lifecycle control" could mean controlling kodegend OR controlling what kodegend manages
3. **Missing prerequisite documentation**: Doesn't mention service must be installed first
4. **Scope ambiguity**: "daemon" could mean kodegend process OR the HTTP MCP servers it spawns (see [`src/manager.rs`](../packages/kodegend/src/manager.rs))

## Service Installation Flow

**Prerequisites** (handled by [`src/install/`](../packages/kodegend/src/install/) module):

The control module requires **prior installation** of platform-specific service files:

- **Linux**: Installs `~/.config/systemd/user/kodegend.service` or `/etc/systemd/system/kodegend.service`
- **macOS**: Installs `~/Library/LaunchAgents/ai.kodegen.kodegend.plist` or `/Library/LaunchDaemons/ai.kodegen.kodegend.plist`
- **Windows**: Registers service with SCM via `CreateServiceW` API

The installation process is handled by platform-specific installers in:
- [`src/install/installer/linux/`](../packages/kodegend/src/install/installer/linux/)
- [`src/install/installer/macos/`](../packages/kodegend/src/install/installer/macos/)
- [`src/install/installer/windows/`](../packages/kodegend/src/install/installer/windows/)

## Recommended Documentation Fix

Replace [`src/control.rs:1-6`](../packages/kodegend/src/control.rs#L1-L6) with:

```rust
//! Control an installed kodegend service via OS-native service managers
//!
//! This module provides CLI commands for managing a **previously installed** kodegend
//! service using the operating system's native service manager:
//!
//! - **macOS**: launchd via `launchctl` (user or system service)
//! - **Linux**: systemd via `systemctl` (user or system service)
//! - **Windows**: Windows Service Control Manager via Win32 API
//!
//! # Prerequisites
//!
//! Before using these commands, the kodegend service must be installed:
//! ```bash
//! kodegend install          # Install as user service (recommended)
//! kodegend install --system # Install as system-wide service (requires root)
//! ```
//!
//! # Commands
//!
//! - `check_status()` - Check if kodegend service is running
//! - `start_daemon()` - Start the kodegend service
//! - `stop_daemon()` - Stop the kodegend service
//! - `restart_daemon()` - Restart the kodegend service
//!
//! # vs Direct Execution
//!
//! This module is for controlling **installed services**. For direct execution:
//! ```bash
//! kodegend run               # Run directly (auto-daemonizes if not under service manager)
//! kodegend run --foreground  # Run in foreground (no daemonization)
//! ```
//!
//! # Architecture
//!
//! kodegend supports two deployment modes:
//!
//! 1. **Service Manager Mode** (recommended for production)
//!    - Service manager (systemd/launchd/SCM) starts kodegend
//!    - kodegend detects service manager via `platform::running_under_service_manager()`
//!    - kodegend stays in foreground (no daemonization)
//!    - Service manager handles restart, logging, monitoring
//!    - Control via this module (`kodegend start/stop/restart/status`)
//!
//! 2. **Standalone Mode** (for development/testing)
//!    - Manual execution via `kodegend run`
//!    - kodegend performs double-fork daemonization (Unix) when not under service manager
//!    - PID file for process tracking (RAII-based cleanup)
//!    - Limited lifecycle control (mainly SIGTERM for stop)
//!
//! # Platform-Specific Behavior
//!
//! ## macOS (launchd)
//! - User service: `~/Library/LaunchAgents/ai.kodegen.kodegend.plist`
//! - System service: `/Library/LaunchDaemons/ai.kodegen.kodegend.plist`
//! - Control: `launchctl bootstrap/kickstart/kill/bootout`
//! - Service label: `ai.kodegen.kodegend`
//!
//! ## Linux (systemd)
//! - User service: `~/.config/systemd/user/kodegend.service`
//! - System service: `/etc/systemd/system/kodegend.service`
//! - Control: `systemctl [--user] start/stop/restart/is-active kodegend`
//! - Auto-detects root to use system vs user mode
//!
//! ## Windows (SCM)
//! - Service name: `kodegend`
//! - Control: Win32 API (`OpenSCManagerW`, `StartServiceW`, `ControlService`)
//! - Queries `SERVICE_STATUS_PROCESS` for running state
//!
//! # Error Handling
//!
//! All functions return `Result<T>` with context-rich errors. Common error cases:
//! - Service not installed → Run `kodegend install` first
//! - Insufficient permissions → May need sudo or admin rights
//! - Service already running → Normal for `start` if already started
//! - Service not running → Normal for `stop` if already stopped
//!
//! # Related Modules
//!
//! - [`daemon`](crate::daemon) - Double-fork daemonization and PID file management
//! - [`platform`](crate::platform) - Service manager detection and platform abstraction
//! - [`install`](crate::install) - Service installation and configuration
```

## Implementation Changes Required

### File to Modify
**Single file change**: [`packages/kodegend/src/control.rs`](../packages/kodegend/src/control.rs)

### Specific Changes

1. **Replace lines 1-6** with the comprehensive documentation above (157 lines)
2. **Verify platform-specific paths** match actual installer behavior:
   - Check [`src/install/installer/linux/`](../packages/kodegend/src/install/installer/linux/) for systemd unit paths
   - Check [`src/install/installer/macos/`](../packages/kodegend/src/install/installer/macos/) for plist paths
   - Check [`src/install/installer/windows/`](../packages/kodegend/src/install/installer/windows/) for service names
3. **No code changes needed** - only documentation
4. **No functional impact** - purely clarification

### Technical Details to Verify

When implementing, cross-reference these implementation details:

- **Service name constants**:
  - Linux: `SERVICE_NAME = "kodegend"` ([`src/control/linux_control.rs:6`](../packages/kodegend/src/control/linux_control.rs#L6))
  - macOS: `SERVICE_LABEL = "ai.kodegen.kodegend"` ([`src/control/macos_control.rs:7`](../packages/kodegend/src/control/macos_control.rs#L7))
  - Windows: `SERVICE_NAME = "kodegend"` ([`src/control/windows_control.rs:14`](../packages/kodegend/src/control/windows_control.rs#L14))

- **Plist path** (macOS): `PLIST_PATH = "/Library/LaunchDaemons/kodegend.plist"` ([`src/control/macos_control.rs:8`](../packages/kodegend/src/control/macos_control.rs#L8))
  - **NOTE**: This is hardcoded as system-wide. User service support may need to be added.

- **Service manager detection** ([`src/platform/mod.rs:64-71`](../packages/kodegend/src/platform/mod.rs#L64-L71)):
  ```rust
  pub fn running_under_service_manager() -> bool {
      platform_running_under_service_manager()
  }
  ```
  - Implemented in `src/platform/unix.rs` and `src/platform/windows.rs`

- **Daemonization check** ([`src/main.rs:81`](../packages/kodegend/src/main.rs#L81)):
  ```rust
  let should_stay_foreground = force_foreground || platform::running_under_service_manager();
  ```

## Definition of Done

This task is complete when:

1. ✅ **Documentation replaced**: The 6-line module doc comment in `src/control.rs` (lines 1-6) is replaced with the comprehensive 157-line documentation
2. ✅ **Accuracy verified**: All platform-specific details (paths, commands, service names) match actual implementation in `src/control/*/` files
3. ✅ **Clarity achieved**: A developer reading the docs can immediately understand:
   - This controls the **kodegend service itself** (not other daemons)
   - Service must be **installed first** before using these commands
   - Clear distinction between **service mode** vs **standalone mode**
   - When to use `kodegend start` vs `kodegend run`
4. ✅ **Cross-references added**: Documentation links to related modules (`daemon`, `platform`, `install`)
5. ✅ **No ambiguity remains**: The two interpretations problem is eliminated

**Verification method**: Read the updated documentation and confirm it answers:
- "What does this module do?" → Controls installed kodegend service
- "What are the prerequisites?" → Service must be installed first
- "How is this different from `kodegend run`?" → Service manager vs standalone mode
- "Which service files does it use?" → Platform-specific paths listed

## Priority
**LOW** - Doesn't affect functionality, but important for:
- New user onboarding
- Understanding architecture
- Choosing deployment mode
- Troubleshooting

## Related Issues
- The platform modules referenced in the docs must match actual implementation
- Error messages from control functions should reference deployment modes and installation prerequisites
- The install command should create service files at the documented paths

## Notes for Implementation

### What NOT to do (per task instructions):
- ❌ Do NOT add unit tests
- ❌ Do NOT add functional tests
- ❌ Do NOT add benchmarks
- ❌ Do NOT create separate documentation files
- ❌ Do NOT change the scope beyond fixing control.rs documentation

### What to do:
- ✅ Replace module-level doc comments with comprehensive version
- ✅ Ensure accuracy by cross-referencing actual implementation
- ✅ Add internal doc links to related modules
- ✅ Keep focused on clarifying existing functionality

### Code Pattern Examples

The documentation describes actual patterns found in the codebase:

**Pattern 1: Service Manager Detection**
```rust
// From src/platform/mod.rs and used in src/main.rs:81
if platform::running_under_service_manager() {
    // Service manager mode - stay in foreground
} else {
    // Standalone mode - daemonize
}
```

**Pattern 2: Platform-Specific Dispatch**
```rust
// From src/control.rs:11-22
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
```

**Pattern 3: Command Routing**
```rust
// From src/main.rs:69-72
cli::Cmd::Status => handle_status(),   // → control::check_status()
cli::Cmd::Start => handle_start(),     // → control::start_daemon()
cli::Cmd::Stop => handle_stop(),       // → control::stop_daemon()
cli::Cmd::Restart => handle_restart(), // → control::restart_daemon()
```

These patterns demonstrate the actual architecture that the documentation aims to clarify.
