# Task: Clean Up Unused Imports

## Priority: P3 (Style/Quality)

## Related Errors

| File | Line | Import |
|------|------|--------|
| `install/installer/config/mod.rs` | 13 | `add_kodegen_host_entries` |
| `install/installer/windows/mod.rs` | 31 | `hosts_file` |
| `install/installer/windows/mod.rs` | 31 | `install_dir` |
| `install/installer/windows/mod.rs` | 31 | `installer_data_dir` |
| `install/installer/windows/mod.rs` | 31 | `temp_cert_file` |
| `install/privilege.rs` | 177 | `std::process::Command` |
| `install/privilege.rs` | 606 | `InstallerBuilder` |
| `manager.rs` | 596 | `NamedPipeStream` |
| `platform/windows/mod.rs` | 22 | `create_named_pipe_server` |
| `platform/windows/mod.rs` | 99 | `std::ptr` |

## Problem Statement

Various files have unused imports that were left over from refactoring or are no longer needed.

## Analysis Per Import

### 1. `add_kodegen_host_entries` (config/mod.rs:13)
**Status**: Function not called - see WINDOWS_04 task
**Action**: Will be fixed when WINDOWS_04 is implemented. If keeping structured command approach, remove import.

### 2. Path functions (windows/mod.rs:31)
**Status**: Functions exist but not used - see WINDOWS_06 task
**Action**: Will be fixed when WINDOWS_06 is implemented. Import should be used then.

### 3. `temp_cert_file` (windows/mod.rs:31)
**Status**: VERIFY - may be false positive
**Action**: Check if actually used at line 315. If yes, this is a false positive in clippy.

### 4. `std::process::Command` (privilege.rs:177)
**Status**: Leftover from refactor - was using Rust Command, now uses ShellExecuteExW
**Action**: Remove import

### 5. `InstallerBuilder` (privilege.rs:606)
**Status**: Leftover from refactor
**Action**: Remove import

### 6. `NamedPipeStream` (manager.rs:596)
**Status**: Type imported but not used directly - only `create_named_pipe_server` return type
**Action**: Check if type annotation needed. If not, remove import.

### 7. `create_named_pipe_server` (platform/windows/mod.rs:22)
**Status**: Re-exported but may not be used at this level
**Action**: Check usage, remove if not needed at module level

### 8. `std::ptr` (platform/windows/mod.rs:99)
**Status**: Leftover from removed code
**Action**: Remove import

## Required Implementation

### Step 1: Verify Each Import

For each import, search for usage:
```bash
# Example for Command
grep -n "Command" src/install/privilege.rs
```

### Step 2: Remove Confirmed Unused

```rust
// Before
use std::process::Command;

// After
// (remove line entirely)
```

### Step 3: Handle False Positives

If an import is used but flagged:
- Check for conditional compilation (`#[cfg(...)]`)
- May need to move import inside cfg block

## Special Case: Glob Import Visibility

`logging/mod.rs:26`:
```rust
#[cfg(windows)]
pub(crate) use windows::*;
```

Error: `glob import doesn't reexport anything with visibility pub(crate)`

**Root Cause**: The `windows` submodule exports `platform_init_logging()` as `pub(super)`, not `pub(crate)`.

**Fix Options**:
1. Change function visibility to `pub(crate)`
2. Use specific import instead of glob: `use windows::platform_init_logging;`

## Files to Modify

- `src/install/installer/config/mod.rs`
- `src/install/installer/windows/mod.rs`
- `src/install/privilege.rs`
- `src/manager.rs`
- `src/platform/windows/mod.rs`
- `src/logging/mod.rs`

## Dependency Note

Some of these imports will be USED after other tasks are completed:
- `add_kodegen_host_entries` - after WINDOWS_04
- Path functions - after WINDOWS_06

Consider implementing those tasks first, then cleaning up any remaining unused imports.

## Testing

After changes, run:
```bash
cargo check --target x86_64-pc-windows-msvc 2>&1 | grep "unused import"
```

Should return no results.

## Acceptance Criteria

- [ ] All genuinely unused imports removed
- [ ] False positives documented/fixed
- [ ] No "unused import" warnings on Windows build
- [ ] No breakage from removed imports
