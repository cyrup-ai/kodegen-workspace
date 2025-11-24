# HIGH: Incomplete Error Handling in Windows platform_is_process_running()

## Severity
**HIGH - CORRECTNESS**

## Location
`packages/kodegend/src/platform/windows.rs:105-126`

## Issue Description
The `platform_is_process_running()` function only handles `ERROR_INVALID_PARAMETER` specially, but `OpenProcess()` can return several error codes that should be treated differently. Most critically, `ERROR_ACCESS_DENIED` should return `Ok(true)` (process exists but no permission), similar to Unix `EPERM` handling.

### Current Code
```rust
pub(super) fn platform_is_process_running(pid: u32) -> Result<bool, std::io::Error> {
    unsafe {
        match OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) {
            Ok(handle) => {
                let _ = CloseHandle(handle);
                Ok(true)
            }
            Err(e) => {
                if e.code().0 as u32 == ERROR_INVALID_PARAMETER.0 {
                    Ok(false)  // Process doesn't exist
                } else {
                    // All other errors returned as Err
                    Err(std::io::Error::from_raw_os_error(e.code().0))
                }
            }
        }
    }
}
```

### Missing Error Code Handling

**ERROR_ACCESS_DENIED (0x5)**
- Returned when process exists but caller lacks permission
- Protected processes (System, csrss.exe, services.exe)
- Processes running at higher integrity level
- Should return `Ok(true)` like Unix EPERM case

**ERROR_INVALID_PARAMETER (0x57)**
- Invalid PID (out of range or never existed)
- Currently handled correctly - returns `Ok(false)`

**Other possible errors:**
- `ERROR_NOT_FOUND` - Process terminated between enumeration and open
- `ERROR_PROC_NOT_FOUND` - Process doesn't exist (should be Ok(false))

### Platform Inconsistency
Unix implementation (unix.rs:55-62):
```rust
pub(super) fn platform_is_process_running(pid: i32) -> Result<bool, std::io::Error> {
    match kill(Pid::from_raw(pid), None) {
        Ok(_) => Ok(true),
        Err(nix::errno::Errno::ESRCH) => Ok(false),  // No such process
        Err(nix::errno::Errno::EPERM) => Ok(true),   // Process exists, no permission
        Err(e) => Err(std::io::Error::from_raw_os_error(e as i32)),
    }
}
```

Windows should match this behavior for consistency.

## Recommended Fix

```rust
use windows::Win32::Foundation::ERROR_ACCESS_DENIED;

pub(super) fn platform_is_process_running(pid: u32) -> Result<bool, std::io::Error> {
    unsafe {
        match OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) {
            Ok(handle) => {
                let _ = CloseHandle(handle);
                Ok(true)  // Process exists and we can query it
            }
            Err(e) => {
                let error_code = e.code().0 as u32;
                
                // Process doesn't exist
                if error_code == ERROR_INVALID_PARAMETER.0 {
                    return Ok(false);
                }
                
                // Process exists but access denied (like Unix EPERM)
                if error_code == ERROR_ACCESS_DENIED.0 {
                    return Ok(true);
                }
                
                // Any other error - return as system error
                Err(std::io::Error::from_raw_os_error(e.code().0))
            }
        }
    }
}
```

### Alternative: More comprehensive error handling
```rust
pub(super) fn platform_is_process_running(pid: u32) -> Result<bool, std::io::Error> {
    use windows::Win32::Foundation::{
        ERROR_ACCESS_DENIED,
        ERROR_INVALID_PARAMETER,
        ERROR_NOT_FOUND,
        WIN32_ERROR,
    };
    
    unsafe {
        match OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) {
            Ok(handle) => {
                let _ = CloseHandle(handle);
                Ok(true)
            }
            Err(e) => {
                let error_code = WIN32_ERROR(e.code().0 as u32);
                
                match error_code {
                    // Process doesn't exist
                    ERROR_INVALID_PARAMETER | ERROR_NOT_FOUND => Ok(false),
                    
                    // Process exists but no permission (matches Unix EPERM)
                    ERROR_ACCESS_DENIED => Ok(true),
                    
                    // Unknown error
                    _ => Err(std::io::Error::from_raw_os_error(e.code().0)),
                }
            }
        }
    }
}
```

## Testing

### Test Case 1: Access Denied
```rust
#[test]
#[cfg(target_os = "windows")]
fn test_protected_process() {
    // Try to check if System process (PID 4) exists
    let result = platform_is_process_running(4);
    
    // Should return Ok(true) even though we can't open it
    assert_eq!(result.unwrap(), true);
}
```

### Test Case 2: Non-existent Process
```rust
#[test]
#[cfg(target_os = "windows")]
fn test_nonexistent_process() {
    // PID 99999999 should not exist
    let result = platform_is_process_running(99999999);
    
    assert_eq!(result.unwrap(), false);
}
```

### Test Case 3: Current Process
```rust
#[test]
#[cfg(target_os = "windows")]
fn test_current_process() {
    let pid = platform_current_process_id();
    let result = platform_is_process_running(pid);
    
    assert_eq!(result.unwrap(), true);
}
```

## Impact
- **Severity**: HIGH - Incorrect behavior for protected processes
- **Frequency**: MEDIUM - Protected processes common on Windows
- **User Impact**: False errors when checking system processes
- **Compatibility**: Inconsistent with Unix behavior

## Files to Modify
- `packages/kodegend/src/platform/windows.rs`

## References
- OpenProcess error codes: https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-openprocess
- System error codes: https://learn.microsoft.com/en-us/windows/win32/debug/system-error-codes--0-499-
- Protected processes: https://learn.microsoft.com/en-us/windows/win32/services/protecting-anti-malware-services-
