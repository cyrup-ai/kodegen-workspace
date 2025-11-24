# Windows: Missing Access Rights Validation

## Location
`packages/kodegend/src/control/windows_control.rs:71-87` (open_service function)

## Issue Type
Hidden Errors / Poor User Experience

## Severity
Medium

## Description
The `open_service()` function returns a generic "Failed to open service" error regardless of whether the failure was due to insufficient permissions or the service not existing. Windows distinguishes between these cases via error codes, but this distinction is lost.

## Current Code
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
        anyhow::bail!("Failed to open service: {}", SERVICE_NAME);
        //            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        //            No distinction between permission denied vs not found
    }

    Ok(ServiceHandle(handle))
}
```

## The Problem

### Windows Returns Specific Error Codes

When `OpenServiceW` fails, Windows sets `GetLastError()` to specific codes:
- **ERROR_ACCESS_DENIED (5)**: User lacks required permissions
- **ERROR_SERVICE_DOES_NOT_EXIST (1060)**: Service is not installed
- **ERROR_INVALID_NAME (123)**: Service name is invalid
- **ERROR_INVALID_HANDLE (6)**: SCM handle is invalid

But the current code treats all failures identically.

## User Impact

### Scenario 1: Service Not Installed
```bash
$ kodegend status
Error: Failed to open service: kodegend
```
**User doesn't know**: Is it a permission issue or is the service not installed?
**What they should see**: "Service 'kodegend' is not installed. Run 'kodegend install' first."

### Scenario 2: Insufficient Permissions
```bash
$ kodegend start
Error: Failed to open service: kodegend
```
**User doesn't know**: Why did it fail?
**What they should see**: "Access denied. Run as administrator to start the service."

### Scenario 3: Different Access Rights
```rust
// check_status() opens with SERVICE_QUERY_STATUS
check_status()?;  // Might fail with "access denied"

// start_daemon() opens with SERVICE_START
start_daemon()?;  // Also fails with "access denied"

// But the required permissions are different!
// SERVICE_QUERY_STATUS needs less privileges than SERVICE_START
// Error message should reflect which permission was lacking
```

## Real-World Example

User interaction:
```
User: "kodegend status" fails
Support: What's the error?
User: "Failed to open service: kodegend"
Support: Is the service installed?
User: I don't know, how do I check?
Support: Try running "sc query kodegend"
User: "The specified service does not exist"
Support: OK, so you need to install it first
```

**This back-and-forth could be avoided with a clear error message.**

## Recommendation

### Distinguish common error cases:

```rust
use windows::core::Error as WindowsError;

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
        let error_code = win_error.code().0 as u32;
        
        return Err(match error_code {
            1060 => {
                // ERROR_SERVICE_DOES_NOT_EXIST
                anyhow::anyhow!(
                    "Service '{}' is not installed. \n\
                     Install it with: kodegend install",
                    SERVICE_NAME
                )
            }
            5 => {
                // ERROR_ACCESS_DENIED
                let access_type = match access {
                    x if x == SERVICE_QUERY_STATUS.0 => "query status",
                    x if x == SERVICE_START.0 => "start",
                    x if x == SERVICE_STOP.0 => "stop",
                    _ => "access",
                };
                
                anyhow::anyhow!(
                    "Access denied - insufficient permissions to {} service '{}'. \n\
                     Run as administrator or ensure your user has the required privileges.",
                    access_type,
                    SERVICE_NAME
                )
            }
            123 => {
                // ERROR_INVALID_NAME
                anyhow::anyhow!(
                    "Invalid service name '{}' (ERROR_INVALID_NAME). \n\
                     This is likely a bug in kodegend.",
                    SERVICE_NAME
                )
            }
            6 => {
                // ERROR_INVALID_HANDLE
                anyhow::anyhow!(
                    "Invalid Service Control Manager handle (ERROR_INVALID_HANDLE). \n\
                     This is likely a bug in kodegend."
                )
            }
            _ => {
                // Unknown error - include error code for debugging
                anyhow::anyhow!(
                    "Failed to open service '{}': {} (error code: {})",
                    SERVICE_NAME,
                    win_error.message(),
                    error_code
                )
            }
        });
    }

    Ok(ServiceHandle(handle))
}
```

### Add helper to get access type name:
```rust
fn access_type_name(access: u32) -> &'static str {
    match access {
        x if x == SERVICE_QUERY_STATUS.0 => "query status of",
        x if x == SERVICE_START.0 => "start",
        x if x == SERVICE_STOP.0 => "stop",
        x if x == SERVICE_CHANGE_CONFIG.0 => "configure",
        x if x == SERVICE_ALL_ACCESS.0 => "fully control",
        _ => "access",
    }
}
```

## Benefits

1. **Clearer Errors**: Users immediately know what went wrong
2. **Actionable**: Error messages include what to do (run as admin, install service, etc.)
3. **Better Support**: Support team can help users faster with clear error messages
4. **Debugging**: Error codes help developers diagnose issues
5. **Professional**: Shows attention to detail and user experience

## Error Message Comparison

### Before
```
Error: Failed to open service: kodegend
```

### After
```
Error: Service 'kodegend' is not installed.
Install it with: kodegend install
```

Or:
```
Error: Access denied - insufficient permissions to start service 'kodegend'.
Run as administrator or ensure your user has the required privileges.
```

Much better! 

## Related Issues
- Windows missing error codes (general issue across all functions)
- Inconsistent error handling across platforms
