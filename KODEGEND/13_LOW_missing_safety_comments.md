# LOW: Missing SAFETY Comments in Unsafe Blocks

## Severity
**LOW - CODE QUALITY**

## Location
- `packages/kodegend/src/platform/windows.rs:42-70` (platform_is_elevated)
- `packages/kodegend/src/platform/windows.rs:80-85` (platform_running_under_service_manager)
- `packages/kodegend/src/platform/windows.rs:95` (platform_current_process_id)
- `packages/kodegend/src/platform/windows.rs:106-125` (platform_is_process_running)

## Issue Description
The Windows platform implementation contains 4 unsafe blocks without SAFETY comments explaining why the unsafe operations are sound. This is a Rust best practice violation that makes code harder to audit.

### Rust Guidelines
From the Rust API Guidelines:
> "All unsafe blocks should have a comment explaining why the unsafe code is sound and what invariants it relies on."

### Current Code Examples

**Example 1: platform_is_elevated (lines 42-70)**
```rust
pub(super) fn platform_is_elevated() -> bool {
    unsafe {  // ← No SAFETY comment
        let mut token_handle: HANDLE = HANDLE::default();
        
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token_handle).is_err() {
            return false;
        }
        
        let mut elevation: TOKEN_ELEVATION = mem::zeroed();  // ← Unsafe operation
        let mut return_length: u32 = 0;
        
        let result = GetTokenInformation(
            token_handle,
            TokenElevation,
            Some(&mut elevation as *mut _ as *mut std::ffi::c_void),  // ← Raw pointer
            mem::size_of::<TOKEN_ELEVATION>() as u32,
            &mut return_length,
        );
        
        CloseHandle(token_handle);
        // ...
    }
}
```

**Example 2: platform_is_process_running (lines 106-125)**
```rust
pub(super) fn platform_is_process_running(pid: u32) -> Result<bool, std::io::Error> {
    unsafe {  // ← No SAFETY comment
        match OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) {
            Ok(handle) => {
                let _ = CloseHandle(handle);
                Ok(true)
            }
            Err(e) => {
                // ...
            }
        }
    }
}
```

## Recommended Fix

Add SAFETY comments explaining why each unsafe block is sound:

### Fixed Example 1
```rust
pub(super) fn platform_is_elevated() -> bool {
    // SAFETY: All Windows API calls are unsafe but these are safe to call because:
    // 1. GetCurrentProcess() returns pseudo-handle (always valid, never needs closing)
    // 2. OpenProcessToken is called with valid process handle and out-parameter
    // 3. mem::zeroed() is safe for TOKEN_ELEVATION (POD type with no invalid bit patterns)
    // 4. GetTokenInformation is called with valid token handle and properly sized buffer
    // 5. CloseHandle is called on token_handle obtained from OpenProcessToken
    // 6. All handles are checked for validity and errors are handled
    unsafe {
        let mut token_handle: HANDLE = HANDLE::default();
        
        // Open process token with query access
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token_handle).is_err() {
            return false;
        }
        
        // Zero-initialize TOKEN_ELEVATION struct
        // SAFETY: TOKEN_ELEVATION is POD (Plain Old Data) with no invalid states
        let mut elevation: TOKEN_ELEVATION = mem::zeroed();
        let mut return_length: u32 = 0;
        
        // Query token elevation info
        // SAFETY: Passing valid token handle, properly typed buffer, and correct size
        let result = GetTokenInformation(
            token_handle,
            TokenElevation,
            Some(&mut elevation as *mut _ as *mut std::ffi::c_void),
            mem::size_of::<TOKEN_ELEVATION>() as u32,
            &mut return_length,
        );
        
        // Always close token handle
        CloseHandle(token_handle);
        
        if result.is_err() {
            return false;
        }
        
        elevation.TokenIsElevated != 0
    }
}
```

### Fixed Example 2
```rust
pub(super) fn platform_running_under_service_manager() -> bool {
    // SAFETY: GetConsoleWindow() is always safe to call
    // Returns NULL if process has no console (services don't have consoles)
    unsafe {
        use windows::Win32::System::Console::GetConsoleWindow;
        GetConsoleWindow().0 == 0
    }
}
```

### Fixed Example 3
```rust
pub(super) fn platform_current_process_id() -> u32 {
    // SAFETY: GetCurrentProcessId() is always safe to call
    // Returns current process's PID (never fails)
    unsafe { GetCurrentProcessId() }
}
```

### Fixed Example 4
```rust
pub(super) fn platform_is_process_running(pid: u32) -> Result<bool, std::io::Error> {
    // SAFETY: OpenProcess and CloseHandle are unsafe but used safely here:
    // 1. OpenProcess called with valid access rights and PID parameter
    // 2. On success, handle is immediately closed with CloseHandle
    // 3. On failure, no handle to clean up
    // 4. Error codes are properly translated to Ok(bool) or Err
    unsafe {
        match OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) {
            Ok(handle) => {
                // Process exists - close handle immediately
                let _ = CloseHandle(handle);
                Ok(true)
            }
            Err(e) => {
                // Check error code to determine if process exists
                if e.code().0 as u32 == ERROR_INVALID_PARAMETER.0 {
                    Ok(false)  // Invalid PID - process doesn't exist
                } else {
                    Err(std::io::Error::from_raw_os_error(e.code().0))
                }
            }
        }
    }
}
```

## Why This Matters

1. **Code review**: Reviewers can verify safety claims
2. **Maintenance**: Future developers understand assumptions
3. **Auditing**: Security audits can check unsafe code
4. **Best practice**: Follows Rust API guidelines
5. **Self-documentation**: Explains why code is correct

## Clippy Lint

Enable clippy lint to catch missing SAFETY comments:
```toml
# In .cargo/config.toml or clippy.toml
[lints.clippy]
undocumented_unsafe_blocks = "warn"
```

## Impact
- **Severity**: LOW - Documentation issue, not functional bug
- **Risk**: None - code is actually safe
- **Maintainability**: Makes code harder to audit
- **Priority**: LOW - Nice to have, not urgent

## Files to Modify
- `packages/kodegend/src/platform/windows.rs`

## Related Best Practices
- Document all unsafe code
- Minimize unsafe block scope
- Consider using safe abstractions (e.g., wrap unsafe Windows APIs)
- Use `#![deny(unsafe_op_in_unsafe_fn)]` lint

## Template for SAFETY Comments

```rust
// SAFETY: <Explain why this unsafe code is sound>
// - Invariant 1: <Description>
// - Invariant 2: <Description>
// - Checked: <What error handling is in place>
unsafe {
    // unsafe code
}
```
