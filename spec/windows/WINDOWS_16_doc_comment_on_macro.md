# Task: Fix Doc Comment on Macro Invocation

## Priority: P3 (Style/Quality)

## Related Error
- `platform/windows_service.rs:32` - unused doc comment

## Problem Statement

Doc comments (`///`) on macro invocations are not captured by rustdoc:

```rust
/// Define the Windows service entry point
/// This macro generates the FFI wrapper required by SCM
define_windows_service!(ffi_service_main, service_main);
```

The compiler warns that these doc comments serve no purpose.

## Analysis

The `define_windows_service!` macro from the `windows-service` crate generates FFI code. Doc comments before macro invocations:
- Are not attached to any item
- Are not visible in generated documentation
- Trigger compiler warnings

## Required Implementation

### Option A: Convert to Regular Comment (Recommended)

```rust
// Define the Windows service entry point.
// This macro generates the FFI wrapper required by SCM.
define_windows_service!(ffi_service_main, service_main);
```

Regular comments (`//`) don't trigger the warning and still provide context for developers reading the code.

### Option B: Move Comment Above Function

If the comment should be documentation, attach it to a documented item:

```rust
/// Windows Service entry point.
///
/// This module uses `define_windows_service!` to generate the FFI wrapper
/// required by the Service Control Manager (SCM). The generated `ffi_service_main`
/// function is the actual entry point called by Windows when the service starts.
///
/// # Generated Functions
///
/// - `ffi_service_main`: FFI-compatible entry point for SCM
/// - Calls `service_main` with converted arguments
mod service_impl {
    define_windows_service!(ffi_service_main, service_main);

    fn service_main(arguments: Vec<OsString>) {
        // ...
    }
}
```

### Option C: Use Inner Doc Comment

Some macros support inner doc comments:
```rust
define_windows_service!(
    //! Define the Windows service entry point
    ffi_service_main,
    service_main
);
```

But this depends on the macro supporting it (unlikely for `windows-service`).

## Recommendation

**Option A** is simplest and maintains the helpful context for developers.

## Files to Modify

- `src/platform/windows_service.rs:32-33`

## Change

```rust
// Before
/// Define the Windows service entry point
/// This macro generates the FFI wrapper required by SCM
define_windows_service!(ffi_service_main, service_main);

// After
// Define the Windows service entry point.
// This macro generates the FFI wrapper required by SCM.
define_windows_service!(ffi_service_main, service_main);
```

## Testing

After fix:
```bash
cargo check --target x86_64-pc-windows-msvc 2>&1 | grep "unused doc comment"
```

Should return no results.

## Acceptance Criteria

- [ ] No "unused doc comment" warning
- [ ] Comment content preserved for developer reference
- [ ] Code compiles cleanly
