# Task: Use ProcessHandle RAII Wrapper

## Priority: P3 (Code Quality)

## Related Error
- `platform/windows/handles.rs:31` - associated function `open_query` is never used

## Problem Statement

The `ProcessHandle` RAII wrapper provides safe handle management:
```rust
pub struct ProcessHandle(HANDLE);

impl ProcessHandle {
    /// Open process with PROCESS_QUERY_LIMITED_INFORMATION access
    pub fn open_query(pid: u32) -> Result<Self> { ... }

    /// Open process with PROCESS_TERMINATE access
    pub fn open_terminate(pid: u32) -> Result<Self> { ... }

    pub fn as_raw(&self) -> HANDLE { ... }
}

impl Drop for ProcessHandle {
    fn drop(&mut self) {
        unsafe { let _ = CloseHandle(self.0); }
    }
}
```

However, `platform_is_process_running()` uses raw Windows API calls instead:
```rust
pub(super) fn platform_is_process_running(pid: u32) -> Result<bool, std::io::Error> {
    unsafe {
        match OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) {
            Ok(handle) => {
                let _ = CloseHandle(handle);  // Manual cleanup
                Ok(true)
            }
            Err(e) => { ... }
        }
    }
}
```

## Why Use RAII Wrapper

1. **Safety**: Handle is always closed, even on early return or panic
2. **Clarity**: Intent is clear from type name
3. **Consistency**: All handle operations use same pattern
4. **Debugging**: Debug trait shows handle value

## Required Implementation

### Refactor platform_is_process_running()

Before:
```rust
pub(super) fn platform_is_process_running(pid: u32) -> Result<bool, std::io::Error> {
    unsafe {
        match OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) {
            Ok(handle) => {
                let _ = CloseHandle(handle);
                Ok(true)
            }
            Err(e) => {
                let error_code = e.code().0 as u32;
                if error_code == ERROR_INVALID_PARAMETER.0 {
                    return Ok(false);
                }
                if error_code == ERROR_ACCESS_DENIED.0 {
                    return Ok(true);
                }
                Err(std::io::Error::from_raw_os_error(e.code().0))
            }
        }
    }
}
```

After:
```rust
pub(super) fn platform_is_process_running(pid: u32) -> Result<bool, std::io::Error> {
    match ProcessHandle::open_query(pid) {
        Ok(_handle) => {
            // Handle automatically closed when _handle drops
            Ok(true)
        }
        Err(e) => {
            // Parse error from anyhow::Error
            // Need to extract Windows error code
            // This requires updating ProcessHandle::open_query error handling
            ...
        }
    }
}
```

### Update ProcessHandle Error Handling

Current `open_query` returns `anyhow::Result` which loses the raw error code. For `platform_is_process_running`, we need access to the Windows error code.

Option A: Add method that returns io::Error:
```rust
impl ProcessHandle {
    pub fn try_open_query(pid: u32) -> Result<Self, std::io::Error> {
        unsafe {
            OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid)
                .map(ProcessHandle)
                .map_err(|e| std::io::Error::from_raw_os_error(e.code().0))
        }
    }
}
```

Option B: Return custom error enum with code:
```rust
pub enum ProcessHandleError {
    InvalidParameter,  // Process doesn't exist
    AccessDenied,      // Process exists but no access
    Other(std::io::Error),
}
```

## Files to Modify

- `src/platform/windows/handles.rs` - Add error-preserving method
- `src/platform/windows/mod.rs` - Use RAII wrapper in `platform_is_process_running`

## Testing

1. Verify process existence check still works
2. Verify handles are properly closed (use Process Explorer to check handle count)
3. Test error cases (invalid PID, access denied)

## Acceptance Criteria

- [ ] `ProcessHandle::open_query()` is used
- [ ] No manual CloseHandle calls for process handles
- [ ] Error handling preserves Windows error codes
- [ ] No dead code warning for `open_query`
