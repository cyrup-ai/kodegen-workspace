# MINOR: CloseHandle Error Result Ignored

## Severity
**MINOR - ERROR HANDLING**

## Location
`packages/kodegend/src/platform/windows.rs:111`

## Issue Description
The `platform_is_process_running()` function ignores the result of `CloseHandle()` using `let _ = ...`. While this is unlikely to cause issues in practice, it could hide bugs like double-close or invalid handle usage.

### Current Code
```rust
pub(super) fn platform_is_process_running(pid: u32) -> Result<bool, std::io::Error> {
    unsafe {
        match OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) {
            Ok(handle) => {
                // Process exists - close handle and return true
                let _ = CloseHandle(handle);  // ← Error ignored
                Ok(true)
            }
            Err(e) => {
                // ... error handling ...
            }
        }
    }
}
```

### Why CloseHandle Can Fail

**CloseHandle returns FALSE (error) if:**
1. Handle is invalid (already closed)
2. Handle is a special value (INVALID_HANDLE_VALUE)
3. Handle is NULL
4. Double-close bug

**Common causes:**
- Bug in OpenProcess return value handling
- Memory corruption
- Double-close if called twice
- Closing wrong handle

### Real-World Impact

**Very low because:**
- OpenProcess always returns valid handle on success
- Handle used immediately and once only
- No concurrent access to handle
- Function is short and simple

**Could matter if:**
- Refactoring introduces bug
- Future code reuses handle
- Debugging handle leaks

## Recommended Fix

### Option 1: Log error in debug builds
```rust
Ok(handle) => {
    // Process exists - close handle
    #[cfg(debug_assertions)]
    if let Err(e) = CloseHandle(handle) {
        log::error!("BUG: CloseHandle failed: {:?}", e);
        debug_assert!(false, "CloseHandle should never fail with valid handle");
    }
    
    #[cfg(not(debug_assertions))]
    let _ = CloseHandle(handle);
    
    Ok(true)
}
```

### Option 2: Always log errors
```rust
Ok(handle) => {
    // Process exists - close handle
    if let Err(e) = CloseHandle(handle) {
        log::warn!("CloseHandle failed (handle leak): {:?}", e);
    }
    Ok(true)
}
```

### Option 3: Panic on error (debug only)
```rust
Ok(handle) => {
    // Process exists - close handle
    CloseHandle(handle).expect("CloseHandle failed - handle leak or bug");
    Ok(true)
}
```

### Option 4: Use RAII wrapper
```rust
struct HandleGuard(HANDLE);

impl Drop for HandleGuard {
    fn drop(&mut self) {
        unsafe {
            if let Err(e) = CloseHandle(self.0) {
                log::error!("Failed to close handle in Drop: {:?}", e);
            }
        }
    }
}

pub(super) fn platform_is_process_running(pid: u32) -> Result<bool, std::io::Error> {
    unsafe {
        match OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) {
            Ok(handle) => {
                let _guard = HandleGuard(handle);  // Auto-closed on scope exit
                Ok(true)
            }
            Err(e) => {
                // ... error handling ...
            }
        }
    }
}
```

### Option 5: Use windows-rs HandleWrapper
```rust
use windows::Win32::Foundation::CloseHandle;

pub(super) fn platform_is_process_running(pid: u32) -> Result<bool, std::io::Error> {
    unsafe {
        match OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) {
            Ok(handle) => {
                // windows crate handles may auto-close in some contexts
                // Check windows crate docs for RAII wrappers
                let _ = CloseHandle(handle);
                Ok(true)
            }
            Err(e) => {
                // ... error handling ...
            }
        }
    }
}
```

## Recommended Approach

**Option 2 (Always log)** is best:
- Minimal code change
- Helps debugging if issues occur
- No performance impact
- Warns about unexpected errors

```rust
if let Err(e) = CloseHandle(handle) {
    log::warn!("CloseHandle failed: {:?} (PID: {})", e, pid);
}
```

## When This Actually Matters

Would detect:
- Bugs introduced during refactoring
- Memory corruption
- Windows API misuse
- Double-close errors

## Similar Code Patterns

Check for other ignored CloseHandle results:
```bash
grep -n "let _ = CloseHandle" packages/kodegend/src/
```

Also check `platform_is_elevated()` at line 62:
```rust
CloseHandle(token_handle);  // ← Also ignores result
```

Should apply same fix to all CloseHandle calls.

## Impact
- **Severity**: MINOR - Unlikely to cause issues
- **Debugging**: Makes issues easier to detect
- **Best Practice**: Should check API call results
- **Priority**: LOW - Defensive programming improvement

## Files to Modify
- `packages/kodegend/src/platform/windows.rs`

## Testing
No specific testing needed. Just ensure:
1. CloseHandle errors are logged
2. Function still returns correct result

Could add test with invalid handle:
```rust
#[test]
#[cfg(target_os = "windows")]
fn test_close_invalid_handle() {
    use windows::Win32::Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE};
    
    // Closing invalid handle should fail
    let result = unsafe { CloseHandle(INVALID_HANDLE_VALUE) };
    assert!(result.is_err());
}
```

## References
- CloseHandle documentation: https://learn.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle
- Handle best practices: https://learn.microsoft.com/en-us/windows/win32/sysinfo/handling-handles

## Conclusion
This is defensive programming. Current code is probably fine, but logging errors costs nothing and could help debug future issues.

**Priority: LOW - Nice to have**
