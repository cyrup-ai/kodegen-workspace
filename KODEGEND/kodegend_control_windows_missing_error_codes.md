# Windows: Missing Error Code Handling

## Location
Multiple locations in `packages/kodegend/src/control/windows_control.rs`:
- Line 29-31 (ScManagerHandle::new)
- Line 82-84 (open_service)
- Line 112-114 (check_status)
- Line 132-134 (start_daemon)
- Line 153-155 (stop_daemon)

## Issue Type
Hidden Errors / Diagnostic Information Loss

## Severity
High

## Description
The Windows implementation uses generic error messages without capturing the underlying Windows error codes from `GetLastError()`. This makes debugging production issues nearly impossible since the actual failure reason is lost.

## Current Code Examples

### Example 1: ScManagerHandle::new (Line 29-31)
```rust
if handle.is_invalid() {
    anyhow::bail!("Failed to open Service Control Manager");
}
```
**Problem**: Doesn't capture WHY it failed. Could be:
- ERROR_ACCESS_DENIED (5) - insufficient permissions
- ERROR_DATABASE_DOES_NOT_EXIST (1065) - SCM database corrupted
- ERROR_INVALID_PARAMETER (87) - invalid flags
- Many other possible errors

### Example 2: open_service (Line 82-84)
```rust
if handle.is_invalid() {
    anyhow::bail!("Failed to open service: {}", SERVICE_NAME);
}
```
**Problem**: Could be:
- ERROR_ACCESS_DENIED (5) - user lacks permissions
- ERROR_SERVICE_DOES_NOT_EXIST (1060) - service not installed
- ERROR_INVALID_NAME (123) - invalid service name
- Different errors require different user actions!

### Example 3: check_status (Line 112-114)
```rust
if result.is_err() {
    anyhow::bail!("Failed to query service status");
}
```
**Problem**: Result type from Windows API already contains error info, but it's discarded.

### Example 4: start_daemon (Line 132-134)
```rust
if result.is_err() {
    anyhow::bail!("Failed to start service");
}
```
**Problem**: Could be:
- ERROR_SERVICE_ALREADY_RUNNING (1056) - harmless, should be idempotent
- ERROR_SERVICE_DISABLED (1058) - service disabled, need to enable first
- ERROR_PATH_NOT_FOUND (3) - service binary missing
- ERROR_ACCESS_DENIED (5) - insufficient permissions

## Impact
- **Production Debugging**: Impossible to diagnose failures from error logs
- **User Experience**: Users get unhelpful "failed" messages with no actionable info
- **Support Burden**: Support team can't help users without error codes
- **Development**: Hard to write proper error handling without specific error info

## Real-World Example
User reports: "Failed to start service"

Without error codes, we don't know if:
- Service binary is missing (reinstall needed)
- User needs admin rights (elevate privileges)
- Service is disabled (need to enable it)
- Service dependencies are missing (install dependencies)

All very different solutions!

## Recommendation

### Use windows::core::Error for detailed error info:
```rust
use windows::core::Error as WindowsError;

impl ScManagerHandle {
    fn new() -> Result<Self> {
        let handle = unsafe {
            OpenSCManagerW(
                PCWSTR::null(),
                PCWSTR::null(),
                SC_MANAGER_CONNECT.0,
            )
        };

        if handle.is_invalid() {
            // Capture the Windows error
            let win_error = WindowsError::from_win32();
            anyhow::bail!(
                "Failed to open Service Control Manager: {} (code: 0x{:08X})",
                win_error.message(),
                win_error.code().0
            );
        }

        Ok(ScManagerHandle(handle))
    }
}
```

### For open_service, distinguish common errors:
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
        let win_error = WindowsError::from_win32();
        let error_code = win_error.code().0;
        
        match error_code as u32 {
            1060 => anyhow::bail!(
                "Service '{}' is not installed (ERROR_SERVICE_DOES_NOT_EXIST)",
                SERVICE_NAME
            ),
            5 => anyhow::bail!(
                "Access denied - insufficient permissions to access service '{}' (ERROR_ACCESS_DENIED)",
                SERVICE_NAME
            ),
            123 => anyhow::bail!(
                "Invalid service name '{}' (ERROR_INVALID_NAME)",
                SERVICE_NAME
            ),
            _ => anyhow::bail!(
                "Failed to open service '{}': {} (code: 0x{:08X})",
                SERVICE_NAME,
                win_error.message(),
                error_code
            ),
        }
    }

    Ok(ServiceHandle(handle))
}
```

### For start_daemon, make it idempotent:
```rust
pub fn start_daemon() -> Result<()> {
    // ... open service ...
    
    let result = unsafe {
        StartServiceW(service.handle(), None)
    };

    if result.is_err() {
        let win_error = WindowsError::from_win32();
        let error_code = win_error.code().0 as u32;
        
        match error_code {
            1056 => {
                // ERROR_SERVICE_ALREADY_RUNNING - this is OK, idempotent operation
                log::debug!("Service is already running");
                return Ok(());
            }
            1058 => anyhow::bail!(
                "Service is disabled - enable it first (ERROR_SERVICE_DISABLED)"
            ),
            3 => anyhow::bail!(
                "Service binary not found (ERROR_PATH_NOT_FOUND)"
            ),
            5 => anyhow::bail!(
                "Access denied - run as administrator (ERROR_ACCESS_DENIED)"
            ),
            _ => anyhow::bail!(
                "Failed to start service: {} (code: 0x{:08X})",
                win_error.message(),
                error_code
            ),
        }
    }

    Ok(())
}
```

## Windows Error Code Reference
Common SCM error codes:
- 5: ERROR_ACCESS_DENIED
- 3: ERROR_PATH_NOT_FOUND
- 87: ERROR_INVALID_PARAMETER
- 123: ERROR_INVALID_NAME
- 1056: ERROR_SERVICE_ALREADY_RUNNING
- 1058: ERROR_SERVICE_DISABLED
- 1060: ERROR_SERVICE_DOES_NOT_EXIST
- 1061: ERROR_SERVICE_CANNOT_ACCEPT_CTRL
- 1062: ERROR_SERVICE_NOT_ACTIVE
- 1065: ERROR_DATABASE_DOES_NOT_EXIST
- 1072: ERROR_SERVICE_MARKED_FOR_DELETE

Full list: https://learn.microsoft.com/en-us/windows/win32/debug/system-error-codes
