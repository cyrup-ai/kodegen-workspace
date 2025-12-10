# Task: Fix Must-Use Warnings

## Priority: P3 (Style/Quality)

## Related Errors

4 warnings about unused return values from Windows API functions.

## Issues and Fixes

### 1. CloseHandle Return Value
**File**: `install/installer/core/context.rs:582`
**Code**: `CloseHandle(sei.hProcess);`
**Error**: `unused return value of CloseHandle that must be used`

**Analysis**: `CloseHandle` returns `Result<(), windows::core::Error>`. In a Drop context, we typically ignore errors because:
- We're already cleaning up
- Nothing useful can be done with the error
- Panicking in cleanup is worse

**Fix**:
```rust
// Before
CloseHandle(sei.hProcess);

// After
let _ = CloseHandle(sei.hProcess);
```

### 2-3. CloseServiceHandle Return Values
**File**: `install/installer/windows/handles.rs:24,45`
**Code**: `CloseServiceHandle(self.0);`
**Error**: `unused return value of CloseServiceHandle that must be used`

**Analysis**: Same situation - these are in Drop implementations for RAII handles.

**Current Code**:
```rust
impl Drop for ScManagerHandle {
    fn drop(&mut self) {
        if !self.0.is_invalid() {
            unsafe {
                CloseServiceHandle(self.0);  // Line 24
            }
        }
    }
}

impl Drop for ServiceHandle {
    fn drop(&mut self) {
        if !self.0.is_invalid() {
            unsafe {
                CloseServiceHandle(self.0);  // Line 45
            }
        }
    }
}
```

**Fix**:
```rust
impl Drop for ScManagerHandle {
    fn drop(&mut self) {
        if !self.0.is_invalid() {
            unsafe {
                let _ = CloseServiceHandle(self.0);
            }
        }
    }
}
```

### 4. RegCloseKey Return Value
**File**: `install/installer/windows/handles.rs:66`
**Code**: `RegCloseKey(self.0);`
**Error**: `unused return value of RegCloseKey that must be used`

**Analysis**: `RegCloseKey` returns `WIN32_ERROR`. Same cleanup context.

**Current Code**:
```rust
impl Drop for RegistryHandle {
    fn drop(&mut self) {
        if !self.0.is_invalid() {
            unsafe {
                RegCloseKey(self.0);  // Line 66
            }
        }
    }
}
```

**Fix**:
```rust
impl Drop for RegistryHandle {
    fn drop(&mut self) {
        if !self.0.is_invalid() {
            unsafe {
                let _ = RegCloseKey(self.0);
            }
        }
    }
}
```

## Why `let _ =` is Appropriate

In Rust, `let _ = expr;` explicitly acknowledges "I know this returns something, and I'm intentionally discarding it."

For cleanup operations like closing handles:
- Errors are rare (double-close, invalid handle)
- Nothing useful can be done with the error in Drop
- Logging in Drop can cause issues
- Panicking in Drop causes abort

The `_` binding tells both the compiler and future readers: "This is intentional."

## Alternative: Debug Logging

For debug builds, could log failures:
```rust
impl Drop for ServiceHandle {
    fn drop(&mut self) {
        if !self.0.is_invalid() {
            unsafe {
                if let Err(e) = CloseServiceHandle(self.0) {
                    #[cfg(debug_assertions)]
                    eprintln!("Warning: CloseServiceHandle failed: {:?}", e);
                }
            }
        }
    }
}
```

But this adds complexity and the `let _ =` pattern is idiomatic Rust.

## Files to Modify

- `src/install/installer/core/context.rs:582`
- `src/install/installer/windows/handles.rs:24`
- `src/install/installer/windows/handles.rs:45`
- `src/install/installer/windows/handles.rs:66`

## Testing

After fixes:
```bash
cargo check --target x86_64-pc-windows-msvc 2>&1 | grep "must be used"
```

Should return no results.

## Acceptance Criteria

- [ ] All 4 must-use warnings fixed
- [ ] Using `let _ =` pattern consistently
- [ ] No functional changes to handle cleanup
- [ ] Code compiles without warnings
