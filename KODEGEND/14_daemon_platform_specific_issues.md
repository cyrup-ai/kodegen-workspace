# Platform-Specific Issues and Missing Features

## Severity: MEDIUM

## Location
`packages/kodegend/src/daemon.rs` - multiple locations

## Issue Description
The daemon module has several platform-specific implementation gaps and inconsistencies that limit cross-platform functionality.

## Issue 1: Windows Lacks Proper Daemon Support

### Current State
```rust
#[cfg(not(unix))]
{
    anyhow::bail!("Daemonization is only supported on Unix systems");
}
```

### Problem
Windows users can't run kodegend as a background service using native Windows Service API.

### Impact
- Windows users must keep terminal open
- No integration with Windows Service Manager
- Can't auto-start on boot
- No service lifecycle management

### Recommended Solution

Implement Windows Service support:

```rust
#[cfg(windows)]
pub fn run_as_windows_service(service_main: impl FnOnce() -> Result<()>) -> Result<()> {
    use windows_service::service::{
        ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState,
        ServiceStatus, ServiceType,
    };
    use windows_service::service_control_handler::{self, ServiceControlHandlerResult};
    
    const SERVICE_NAME: &str = "kodegend";
    const SERVICE_TYPE: ServiceType = ServiceType::OWN_PROCESS;
    
    let event_handler = move |control_event| -> ServiceControlHandlerResult {
        match control_event {
            ServiceControl::Stop => {
                // Handle stop request
                ServiceControlHandlerResult::NoError
            }
            ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,
            _ => ServiceControlHandlerResult::NotImplemented,
        }
    };
    
    let status_handle = service_control_handler::register(SERVICE_NAME, event_handler)?;
    
    // Tell Windows we're starting
    status_handle.set_service_status(ServiceStatus {
        service_type: SERVICE_TYPE,
        current_state: ServiceState::Running,
        controls_accepted: ServiceControlAccept::STOP,
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    })?;
    
    // Run the service
    service_main()?;
    
    // Tell Windows we're stopping
    status_handle.set_service_status(ServiceStatus {
        service_type: SERVICE_TYPE,
        current_state: ServiceState::Stopped,
        controls_accepted: ServiceControlAccept::empty(),
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    })?;
    
    Ok(())
}
```

**Dependencies needed**:
```toml
[target.'cfg(windows)'.dependencies]
windows-service = "0.6"
```

## Issue 2: Working Directory Hardcoded to "/" (Unix-specific)

### Current Code (line 40)
```rust
std::env::set_current_dir("/")?;
```

### Problem
- Hardcoded "/" is Unix-specific
- Would fail on Windows (no "/" root)
- Not configurable

### Recommended Solution
```rust
#[cfg(unix)]
const DAEMON_WORKING_DIR: &str = "/";

#[cfg(windows)]
const DAEMON_WORKING_DIR: &str = "C:\\";

pub fn set_daemon_working_directory() -> Result<()> {
    std::env::set_current_dir(DAEMON_WORKING_DIR)
        .with_context(|| format!(
            "Failed to change working directory to {}",
            DAEMON_WORKING_DIR
        ))
}
```

Or make it configurable:
```rust
pub struct DaemonConfig {
    pub working_directory: PathBuf,
    // ... other config
}

impl Default for DaemonConfig {
    fn default() -> Self {
        Self {
            #[cfg(unix)]
            working_directory: PathBuf::from("/"),
            #[cfg(windows)]
            working_directory: PathBuf::from("C:\\"),
        }
    }
}
```

## Issue 3: PID Type Inconsistency

### Problem
- Unix uses `i32` for PIDs
- Windows uses `u32` (DWORD) for process IDs
- Current code uses `i32` everywhere
- Could overflow on Windows for PIDs > i32::MAX

### Current Code
```rust
pub fn is_process_running(pid: i32) -> Result<bool> {
    #[cfg(windows)]
    {
        let handle = OpenProcess(PROCESS_QUERY_INFORMATION, 0, pid as u32);
        // Cast required because function uses i32 but Windows expects u32
    }
}
```

### Recommended Solution

Create platform-specific PID type:

```rust
#[cfg(unix)]
pub type Pid = i32;

#[cfg(windows)]
pub type Pid = u32;

pub fn is_process_running(pid: Pid) -> Result<bool> {
    #[cfg(unix)]
    {
        use nix::sys::signal::{kill, Signal};
        match kill(Pid::from_raw(pid), None) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
    
    #[cfg(windows)]
    {
        unsafe {
            let handle = OpenProcess(PROCESS_QUERY_INFORMATION, 0, pid);
            // No cast needed!
            if handle.is_null() {
                return Ok(false);
            }
            CloseHandle(handle);
            Ok(true)
        }
    }
}
```

For cross-platform PID storage, use largest type:
```rust
// For serialization/storage
pub type PidStorage = i64;

pub fn pid_to_storage(pid: Pid) -> PidStorage {
    pid as PidStorage
}

pub fn pid_from_storage(pid: PidStorage) -> Option<Pid> {
    #[cfg(unix)]
    {
        i32::try_from(pid).ok()
    }
    
    #[cfg(windows)]
    {
        u32::try_from(pid).ok()
    }
}
```

## Issue 4: Missing macOS launchd Support

### Current State
CLAUDE.md mentions "systemd/launchd integration" but only systemd is detected.

### Problem
macOS systems use launchd, not systemd. The code should detect and integrate with launchd.

### Recommended Solution

```rust
/// Check if running under launchd on macOS
#[cfg(target_os = "macos")]
pub fn is_launchd_available() -> bool {
    // Check if LAUNCHED_BY_LAUNCHD environment variable is set
    std::env::var("XPC_SERVICE_NAME").is_ok()
        || std::env::var("__LAUNCHD_FD").is_ok()
}

#[cfg(not(target_os = "macos"))]
pub fn is_launchd_available() -> bool {
    false
}

/// Check if running under a service manager (systemd or launchd)
pub fn is_service_manager_available() -> bool {
    #[cfg(target_os = "linux")]
    {
        is_systemd_available()
    }
    
    #[cfg(target_os = "macos")]
    {
        is_launchd_available()
    }
    
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    {
        false
    }
}

/// Decide whether daemonization is needed
pub fn should_daemonize() -> bool {
    // Don't daemonize if running under service manager
    !is_service_manager_available()
}
```

### Usage
```rust
fn main() -> Result<()> {
    if should_daemonize() {
        let pid = daemonize()?;
        create_pid_file(&get_pid_file_path())?;
    } else {
        // Running under systemd/launchd, they handle daemonization
        log::info!("Running under service manager, skipping daemonization");
    }
    
    run_service()
}
```

## Issue 5: FreeBSD/OpenBSD Support Gaps

### Problem
Code is mostly Linux-specific, may not work on BSD variants:
- `/proc` filesystem may not exist or have different format
- Different signal handling
- Different process APIs

### Testing Needed
- Test on FreeBSD
- Test on OpenBSD
- Test on NetBSD

### Potential Issues
```rust
// This assumes Linux /proc format
let stat = std::fs::read_to_string(format!("/proc/{}/stat", pid))?;
```

### BSD Alternative
```rust
#[cfg(target_os = "freebsd")]
fn get_process_info(pid: i32) -> Result<ProcessInfo> {
    // Use sysctl kern.proc.pid.{pid}
    use libc::{sysctl, CTL_KERN, KERN_PROC, KERN_PROC_PID};
    // ... implementation
}
```

## Summary of Platform Support

| Feature | Linux | macOS | Windows | BSD |
|---------|-------|-------|---------|-----|
| Daemonization | ✅ | ✅ | ❌ | ✅ |
| PID file | ✅ | ✅ | ✅ | ✅ |
| Process check | ✅ | ⚠️ | ⚠️ | ⚠️ |
| Service manager detect | ✅ systemd | ❌ no launchd | ❌ | ❌ |
| Process identity check | ✅ /proc | ❌ | ❌ | ❌ |
| Zombie detection | ✅ /proc | ❌ | N/A | ❌ |

Legend:
- ✅ Fully implemented
- ⚠️ Partial/basic implementation
- ❌ Not implemented
- N/A Not applicable

## Testing Strategy

### Cross-Platform CI
```yaml
# .github/workflows/ci.yml
name: CI
on: [push, pull_request]
jobs:
  test:
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v2
      - name: Run tests
        run: cargo test --package kodegend
```

### Platform-Specific Tests
```rust
#[cfg(target_os = "linux")]
#[test]
fn test_systemd_detection() {
    // Linux-specific test
}

#[cfg(target_os = "macos")]
#[test]
fn test_launchd_detection() {
    // macOS-specific test
}

#[cfg(windows)]
#[test]
fn test_windows_service() {
    // Windows-specific test
}
```

## Implementation Priority

1. **High**: Fix PID type inconsistency
2. **High**: Add macOS launchd detection
3. **Medium**: Add Windows Service support
4. **Medium**: Make working directory configurable
5. **Low**: Add BSD support

## References
- Windows Service API: https://docs.microsoft.com/en-us/windows/win32/services/services
- macOS launchd: https://developer.apple.com/library/archive/documentation/MacOSX/Conceptual/BPSystemStartup/
- FreeBSD rc.d: https://www.freebsd.org/doc/en/articles/rc-scripting/
