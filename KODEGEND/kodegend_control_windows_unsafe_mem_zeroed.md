# Windows: Unsafe mem::zeroed() Usage

## Location
- `packages/kodegend/src/control/windows_control.rs:99` (check_status)
- `packages/kodegend/src/control/windows_control.rs:147` (stop_daemon)

## Issue Type
Unsafe Code / Potential Undefined Behavior

## Severity
Medium

## Description
The code uses `unsafe { mem::zeroed() }` to initialize Windows API structures. While this currently works, it relies on zero-initialized memory being valid for these structures, which is not guaranteed by the Rust memory model and could break with Windows API changes.

## Current Code

### Example 1: check_status (Line 99)
```rust
let mut status: SERVICE_STATUS_PROCESS = unsafe { mem::zeroed() };
let mut bytes_needed: u32 = 0;

let result = unsafe {
    QueryServiceStatusEx(
        service.handle(),
        SC_STATUS_PROCESS_INFO,
        Some(&mut status as *mut _ as *mut u8),
        mem::size_of::<SERVICE_STATUS_PROCESS>() as u32,
        &mut bytes_needed,
    )
};
```

### Example 2: stop_daemon (Line 147)
```rust
let mut status: SERVICE_STATUS = unsafe { mem::zeroed() };

let result = unsafe {
    ControlService(service.handle(), SERVICE_CONTROL_STOP, &mut status)
};
```

## Problems

### 1. Not Guaranteed Safe
`mem::zeroed()` creates a value with all bytes set to 0. While this works for most C-compatible structs, it's not guaranteed to be valid for all types:
- If Windows adds fields with non-zero defaults in future API versions
- If the structure has padding that must be initialized
- If any field has an invalid zero representation

### 2. Violates Rust's Safety Principles
From Rust docs:
> "Note that zeroed is not guaranteed to be a valid value for all types. Types like references and function pointers must be non-null."

While `SERVICE_STATUS_PROCESS` and `SERVICE_STATUS` currently have no such fields, this could change.

### 3. Better Alternatives Exist
`MaybeUninit` is the modern, safer approach for uninitialized memory.

## Impact
- **Current**: Works correctly but fragile
- **Future**: Could break if Windows changes structure definitions
- **Code Quality**: Uses discouraged unsafe pattern
- **Audit**: Makes security audits harder (more unsafe blocks to review)

## Recommendation

### Option 1: Use MaybeUninit (Rust 1.36+)
```rust
use std::mem::MaybeUninit;

pub fn check_status() -> Result<bool> {
    let sc_manager = ScManagerHandle::new()
        .context("Failed to open Service Control Manager for status check")?;

    let service = open_service(&sc_manager, SERVICE_QUERY_STATUS.0)
        .context("Failed to open service for status check")?;

    let mut status = MaybeUninit::<SERVICE_STATUS_PROCESS>::uninit();
    let mut bytes_needed: u32 = 0;

    let result = unsafe {
        QueryServiceStatusEx(
            service.handle(),
            SC_STATUS_PROCESS_INFO,
            Some(status.as_mut_ptr() as *mut u8),
            mem::size_of::<SERVICE_STATUS_PROCESS>() as u32,
            &mut bytes_needed,
        )
    };

    if result.is_err() {
        anyhow::bail!("Failed to query service status");
    }

    // Safe because QueryServiceStatusEx succeeded and initialized the structure
    let status = unsafe { status.assume_init() };

    Ok(status.dwCurrentState == SERVICE_RUNNING.0)
}
```

### Option 2: Use Default trait if available
Check if the `windows` crate provides Default implementations:
```rust
let mut status = SERVICE_STATUS_PROCESS::default();
```

### Option 3: Explicit initialization
If the structures are simple enough:
```rust
let mut status = SERVICE_STATUS {
    dwServiceType: 0,
    dwCurrentState: 0,
    dwControlsAccepted: 0,
    dwWin32ExitCode: 0,
    dwServiceSpecificExitCode: 0,
    dwCheckPoint: 0,
    dwWaitHint: 0,
};
```

## Benefits of MaybeUninit
1. **Explicit about uninitialized state**: Code clearly shows the value is uninitialized until the API call
2. **Safer**: Can't accidentally use uninitialized value without `assume_init()`
3. **More correct**: Doesn't rely on zero being valid
4. **Better for audits**: Clear that initialization happens via Windows API
5. **Modern Rust**: Aligns with current best practices

## Related Documentation
- [Rust std::mem::zeroed documentation](https://doc.rust-lang.org/std/mem/fn.zeroed.html)
- [MaybeUninit documentation](https://doc.rust-lang.org/std/mem/union.MaybeUninit.html)
- [Rustonomicon on uninitialized memory](https://doc.rust-lang.org/nomicon/uninitialized.html)
