# MEDIUM: Windows Service Detection Using GetConsoleWindow Is Unreliable

## Severity
**MEDIUM - CORRECTNESS**

## Location
`packages/kodegend/src/platform/windows.rs:79-86`

## Issue Description
The `platform_running_under_service_manager()` function uses `GetConsoleWindow()` to detect if running as a Windows Service. This approach is unreliable because console-less GUI applications also return NULL, and some interactive services can have consoles.

### Current Code
```rust
pub(super) fn platform_running_under_service_manager() -> bool {
    unsafe {
        // If GetConsoleWindow returns NULL, we're likely a service
        // (Services don't have console windows)
        use windows::Win32::System::Console::GetConsoleWindow;
        GetConsoleWindow().0 == 0
    }
}
```

### Problems

1. **False positives**: GUI applications without console return NULL
2. **False negatives**: Interactive services (deprecated but possible) have consoles
3. **Semantic mismatch**: Unix checks for systemd/launchd parent, Windows just checks console
4. **Use case confusion**: "Running under service manager" vs "is a service"

### Platform Inconsistency

**Unix implementation** (unix.rs:23-38):
```rust
pub(super) fn platform_running_under_service_manager() -> bool {
    // systemd sets INVOCATION_ID
    if std::env::var_os("INVOCATION_ID").is_some() {
        return true;
    }

    // macOS launchd detection
    if cfg!(target_os = "macos") {
        if std::env::var_os("LAUNCHED_BY_LAUNCHD").is_some()
            || std::env::var_os("XPC_SERVICE_NAME").is_some() {
            return true;
        }
    }

    false
}
```

Unix checks parent process manager, Windows checks console presence.

## Recommended Fix

### Option 1: Check parent process is services.exe
```rust
use windows::Win32::System::Threading::{
    GetCurrentProcess,
    OpenProcess,
    PROCESS_QUERY_LIMITED_INFORMATION,
};
use windows::Win32::System::ProcessStatus::K32GetModuleFileNameExW;
use windows::core::PWSTR;

pub(super) fn platform_running_under_service_manager() -> bool {
    unsafe {
        // Get parent process ID
        let ntdll = windows::Win32::System::LibraryLoader::GetModuleHandleW(
            windows::core::w!("ntdll.dll")
        ).ok()?;
        
        // Use NtQueryInformationProcess to get parent PID
        // (Requires dynamic loading of ntdll function)
        
        // Open parent process
        let parent_handle = OpenProcess(
            PROCESS_QUERY_LIMITED_INFORMATION,
            false,
            parent_pid
        ).ok()?;
        
        // Get parent executable path
        let mut path = vec![0u16; 260];
        if K32GetModuleFileNameExW(
            parent_handle,
            None,
            &mut path
        ) == 0 {
            return false;
        }
        
        // Check if parent is services.exe
        let path_str = String::from_utf16_lossy(&path);
        path_str.to_lowercase().contains("services.exe")
    }
}
```

### Option 2: Use environment variable (similar to Unix)
```rust
pub(super) fn platform_running_under_service_manager() -> bool {
    // Windows services can set environment variable during startup
    std::env::var("KODEGEND_SERVICE_MODE").is_ok()
}

// In windows_service.rs service_main():
fn service_main(_arguments: Vec<OsString>) {
    // Set environment variable to indicate service mode
    std::env::set_var("KODEGEND_SERVICE_MODE", "1");
    
    if let Err(e) = run_service() {
        error!("kodegend service error: {}", e);
    }
}
```

### Option 3: Check if stdin/stdout are NULL
```rust
use windows::Win32::System::Console::{GetStdHandle, STD_INPUT_HANDLE, STD_OUTPUT_HANDLE};
use windows::Win32::Foundation::INVALID_HANDLE_VALUE;

pub(super) fn platform_running_under_service_manager() -> bool {
    unsafe {
        // Services have NULL stdin/stdout
        let stdin = GetStdHandle(STD_INPUT_HANDLE);
        let stdout = GetStdHandle(STD_OUTPUT_HANDLE);
        
        stdin.is_err() && stdout.is_err()
    }
}
```

### Option 4: Combine multiple heuristics
```rust
pub(super) fn platform_running_under_service_manager() -> bool {
    // Check multiple indicators
    let no_console = unsafe {
        use windows::Win32::System::Console::GetConsoleWindow;
        GetConsoleWindow().0 == 0
    };
    
    let no_stdin = unsafe {
        use windows::Win32::System::Console::{GetStdHandle, STD_INPUT_HANDLE};
        GetStdHandle(STD_INPUT_HANDLE).is_err()
    };
    
    // Environment variable set by service dispatcher
    let env_flag = std::env::var("KODEGEND_SERVICE_MODE").is_ok();
    
    // Require at least 2 of 3 indicators
    (no_console as u8 + no_stdin as u8 + env_flag as u8) >= 2
}
```

## Recommended Approach

**Option 2 (Environment variable)** is recommended:
- Simple and reliable
- Explicit intent (not heuristic)
- Matches Unix pattern (checking environment)
- Easy to debug (can check env vars)

Set environment variable in `windows_service.rs::service_main()` before calling `run_service()`.

## Testing

### Test Case 1: Console Application
```bash
# Run as console app (not service)
kodegend.exe --console

# Should return false
```

### Test Case 2: Windows Service
```bash
# Install and start as service
sc create kodegend binPath= "C:\path\to\kodegend.exe"
sc start kodegend

# Should return true
# Check with: sc query kodegend
```

### Test Case 3: GUI Application
```bash
# Run with no console (GUI mode)
kodegend.exe --gui

# Should return false (not a service)
```

## Impact
- **Severity**: MEDIUM - Detection can be wrong
- **Probability**: LOW - Usually works in practice
- **User Impact**: Wrong code path taken (daemon vs service)
- **Consequences**: Logging, file paths, behavior differences

## Files to Modify
- `packages/kodegend/src/platform/windows.rs`
- `packages/kodegend/src/platform/windows_service.rs` (if using env var approach)

## Documentation Updates
- Update doc comment in `mod.rs:64-71` to clarify detection method
- Note platform differences between Unix and Windows detection

## References
- Windows Services and Interactive Services: https://learn.microsoft.com/en-us/windows/win32/services/interactive-services
- Service process isolation: https://learn.microsoft.com/en-us/windows/win32/services/service-security-and-access-rights
- GetConsoleWindow: https://learn.microsoft.com/en-us/windows/console/getconsolewindow
