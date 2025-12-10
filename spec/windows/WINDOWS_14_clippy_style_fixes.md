# Task: Fix Clippy Style Issues

## Priority: P3 (Style/Quality)

## Related Errors

14 clippy warnings for code style improvements.

## Issues and Fixes

### 1. Transmute Without Annotations
**File**: `install/installer/config/certificates.rs:100`
**Error**: `transmute used without annotations`

**Fix**: Add explicit type annotations:
```rust
// Before
let ptr = std::mem::transmute(some_value);

// After
let ptr: *const SomeType = std::mem::transmute::<SourceType, *const SomeType>(some_value);
```

### 2-3. Unnecessary Casts
**Files**:
- `install/installer/core/context.rs:540`
- `install/privilege.rs:517`

**Error**: `casting to the same type is unnecessary (i32 -> i32)`

**Fix**: Remove redundant cast:
```rust
// Before
nShow: SW_SHOWNORMAL.0 as i32,

// After
nShow: SW_SHOWNORMAL.0,
```

Note: `SW_SHOWNORMAL.0` is already `i32`, no cast needed.

### 4-5. Redundant Closures
**Files**:
- `install/privilege.rs:487`
- `platform/windows/mod.rs:225`
- `platform/windows/mod.rs:263`

**Error**: `redundant closure`

**Fix**: Replace closure with function reference:
```rust
// Before
.unwrap_or_else(|| some_function())

// After
.unwrap_or_else(some_function)
```

Or if the function takes no arguments:
```rust
// Before
.or_else(|| dirs::data_local_dir())

// After
.or_else(dirs::data_local_dir)
```

### 6. Collapsible If
**File**: `install/privilege.rs:975`
**Error**: `this if statement can be collapsed`

**Fix**: Use if-let chain:
```rust
// Before
if let Some(parent) = path.parent() {
    if !parent.exists() {
        std::fs::create_dir_all(parent)?;
    }
}

// After
if let Some(parent) = path.parent() && !parent.exists() {
    std::fs::create_dir_all(parent)?;
}
```

### 7-9. io_other_error
**File**: `platform/windows/named_pipe.rs:45,63,179`
**Error**: `this can be std::io::Error::other(_)`

**Fix**: Use `io::Error::other()`:
```rust
// Before
io::Error::new(io::ErrorKind::Other, format!("ReadFile failed: {}", e))

// After
io::Error::other(format!("ReadFile failed: {}", e))
```

### 10. Useless Conversion
**File**: `platform/windows_service.rs:128`
**Error**: `useless conversion to the same type: anyhow::Error`

**Fix**: Remove `.into()`:
```rust
// Before
return Err(e.into());

// After
return Err(e);
```

### 11-12. Collapsible If in GUI Detection
**File**: `platform/gui_detection.rs:207,208`
**Error**: `this if statement can be collapsed`

**Fix**: Use if-let chains:
```rust
// Before
if let Some(current) = system.process(Pid::from_u32(current_pid)) {
    if let Some(parent_pid) = current.parent() {
        if let Some(parent) = system.process(parent_pid) {
            let parent_name = parent.name().to_string_lossy().to_lowercase();
            return parent_name == "services.exe";
        }
    }
}

// After
if let Some(current) = system.process(Pid::from_u32(current_pid))
    && let Some(parent_pid) = current.parent()
    && let Some(parent) = system.process(parent_pid)
{
    let parent_name = parent.name().to_string_lossy().to_lowercase();
    return parent_name == "services.exe";
}
```

## Files to Modify

- `src/install/installer/config/certificates.rs`
- `src/install/installer/core/context.rs`
- `src/install/privilege.rs`
- `src/platform/windows/mod.rs`
- `src/platform/windows/named_pipe.rs`
- `src/platform/windows_service.rs`
- `src/platform/gui_detection.rs`

## Testing

After fixes, run:
```bash
cargo clippy --target x86_64-pc-windows-msvc -- -D warnings
```

Should pass with no warnings.

## Notes on If-Let Chains

If-let chains (`if let ... && let ...`) require Rust edition 2024 or the `let_chains` feature. The codebase already uses edition 2024 per CLAUDE.md.

## Acceptance Criteria

- [ ] All 14 clippy style warnings fixed
- [ ] Code compiles without warnings
- [ ] No functional changes to code behavior
- [ ] Tests still pass
