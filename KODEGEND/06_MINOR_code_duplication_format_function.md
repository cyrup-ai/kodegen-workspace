# MINOR: Code Duplication of Log Format Function

## Location
- `packages/kodegend/src/logging/unix.rs:14-25`
- `packages/kodegend/src/logging/windows.rs:31-42`

## Issue Type
**MINOR** - Code quality / DRY violation

## Description
The exact same log format function is duplicated in both `unix.rs` and `windows.rs`. This violates the DRY (Don't Repeat Yourself) principle and creates a maintenance burden.

## Code Analysis

### unix.rs lines 14-25:
```rust
.format(|buf, record| {
    use std::io::Write;
    writeln!(
        buf,
        "[{} {} {}:{}] {}",
        buf.timestamp_millis(),
        record.level(),
        record.file().unwrap_or("unknown"),
        record.line().unwrap_or(0),
        record.args()
    )
})
```

### windows.rs lines 31-42:
```rust
.format(|buf, record| {
    use std::io::Write;
    writeln!(
        buf,
        "[{} {} {}:{}] {}",
        buf.timestamp_millis(),
        record.level(),
        record.file().unwrap_or("unknown"),
        record.line().unwrap_or(0),
        record.args()
    )
})
```

These are **identical** - character for character duplication.

## Impact

### Maintenance Burden
If the log format needs to change (e.g., add timestamp format, change field order, add new fields):
1. Developer must remember to update BOTH files
2. Easy to forget one, causing inconsistency
3. Increases review overhead
4. Increases testing surface area

### Consistency Risk
Example scenario:
1. Bug found in format string (e.g., add timezone to timestamp)
2. Fixed in unix.rs
3. Forgotten in windows.rs
4. **Windows and Unix now have different log formats** - debugging nightmare

### Not DRY
Violates fundamental code quality principle: Don't Repeat Yourself

## Fix Options

### Option 1: Shared Function in mod.rs

Create a shared format function:

```rust
// In mod.rs
use log::Record;
use std::io::Write;

pub(crate) fn kodegend_log_format(
    buf: &mut env_logger::fmt::Formatter,
    record: &Record
) -> std::io::Result<()> {
    writeln!(
        buf,
        "[{} {} {}:{}] {}",
        buf.timestamp_millis(),
        record.level(),
        record.file().unwrap_or("unknown"),
        record.line().unwrap_or(0),
        record.args()
    )
}
```

Then use in both files:

```rust
// In unix.rs and windows.rs
.format(crate::logging::kodegend_log_format)
```

### Option 2: Macro

Define a macro in mod.rs:

```rust
// In mod.rs
#[macro_export]
macro_rules! kodegend_format {
    () => {
        |buf: &mut env_logger::fmt::Formatter, record: &log::Record| {
            use std::io::Write;
            writeln!(
                buf,
                "[{} {} {}:{}] {}",
                buf.timestamp_millis(),
                record.level(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )
        }
    };
}
```

Then use:
```rust
// In unix.rs and windows.rs
.format(kodegend_format!())
```

### Option 3: Const Function (if possible)

If env_logger supports const formatting functions, use that.

## Recommendation

**Option 1** (shared function) is cleanest:
- Most explicit and readable
- Easy to find and modify
- Good for future enhancements (e.g., configuration-based formatting)
- Clear ownership of format logic

## Priority
**P3 - Minor** - Code quality issue, not a functional bug

## Benefits of Fixing

1. **Single source of truth** for log format
2. **Easier to maintain** - change once, applies everywhere
3. **Prevents divergence** between platforms
4. **Better code organization**

## Testing After Fix

1. Build both Unix and Windows versions
2. Compare log output format from both
3. Verify they're identical
4. Make a format change in the shared function
5. Rebuild both
6. Verify change applied to both platforms

## Related Code

The comment in unix.rs line 12 says:
```rust
// Preserve exact format from src/main.rs:36-50
```

This suggests the format was copied from main.rs. Should verify:
1. Is main.rs still using this format?
2. If yes, main.rs should ALSO use the shared function
3. If no, the comment is outdated

This might reveal additional duplication to eliminate.
