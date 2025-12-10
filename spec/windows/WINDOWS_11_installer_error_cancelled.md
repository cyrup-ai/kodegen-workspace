# Task: Use InstallerError::Cancelled for UAC Cancellation

## Priority: P3 (Code Quality)

## Related Error
- `install/installer/error.rs:10` - variant `Cancelled` is never constructed

## Problem Statement

The `InstallerError` enum defines a `Cancelled` variant:
```rust
#[derive(Error, Debug)]
pub enum InstallerError {
    /// User cancelled the authorization prompt
    #[error("User cancelled authorization")]
    #[cfg_attr(target_os = "linux", allow(dead_code))]
    Cancelled,

    // ... other variants
}
```

However, when UAC is cancelled, the code returns an ad-hoc error:
```rust
// src/install/installer/core/context.rs around line 561
if error_code.0 == 1223 {
    return Err(anyhow::anyhow!(
        "UAC elevation cancelled by user. Administrator privileges are required to install kodegen."
    ));
}
```

## Why Use Structured Error

1. **Pattern Matching**: Callers can handle cancellation differently
2. **Consistency**: All error types in one place
3. **i18n**: Easier to localize error messages
4. **Testing**: Can assert on specific error types

## Required Implementation

### 1. Use InstallerError::Cancelled

Before:
```rust
if error_code.0 == 1223 {
    return Err(anyhow::anyhow!(
        "UAC elevation cancelled by user. Administrator privileges are required to install kodegen."
    ));
}
```

After:
```rust
if error_code.0 == 1223 {
    return Err(InstallerError::Cancelled.into());
}
```

### 2. Update Error Message if Needed

The current `Cancelled` variant has a generic message:
```rust
#[error("User cancelled authorization")]
Cancelled,
```

If more detail is needed, update to:
```rust
#[error("User cancelled authorization. Administrator privileges are required to install kodegen.")]
Cancelled,
```

Or add context:
```rust
#[error("User cancelled {context}")]
Cancelled { context: String },
```

### 3. Handle in Caller

Where the error is caught:
```rust
match install_result {
    Err(e) => {
        if let Some(InstallerError::Cancelled) = e.downcast_ref() {
            // User chose to cancel - exit cleanly, no error output
            std::process::exit(0);
        }
        // Other errors - print and exit with error code
        eprintln!("Installation failed: {}", e);
        std::process::exit(1);
    }
    Ok(_) => { ... }
}
```

## Files to Modify

- `src/install/installer/core/context.rs` - Use `InstallerError::Cancelled`
- `src/install/privilege.rs` - Any other places checking error code 1223
- Potentially callers that need to handle cancellation specially

## Search for Other Ad-Hoc Errors

Check if other `InstallerError` variants are also unused:
```bash
grep -r "InstallerError::" src/
grep -r "anyhow::anyhow" src/install/
```

May find other opportunities to use structured errors.

## Testing

1. Start install, cancel UAC prompt - verify clean exit
2. Start install, complete UAC - verify installation proceeds
3. Check exit codes: cancelled = 0, error = 1, success = 0

## Acceptance Criteria

- [ ] `InstallerError::Cancelled` is used for UAC cancellation
- [ ] No dead code warning for `Cancelled` variant
- [ ] Cancellation is handled appropriately (clean exit, not error)
- [ ] Error messages are clear and helpful
