# Task: Consolidate Windows Path Function Usage

## Priority: P2 (Installation Polish)

## Related Errors
- `install/installer/windows/paths.rs:83` - function `kodegen_exe` never used
- `install/installer/windows/paths.rs:107` - function `installer_log_dir` never used
- `install/installer/windows/paths.rs:112` - function `installer_config_dir` never used
- `install/installer/windows/paths.rs:126` - function `temp_dir` never used
- `install/installer/windows/paths.rs:145` - function `create_installer_directories` never used

## Problem Statement

The `windows/paths.rs` module provides well-designed path utility functions:
```rust
pub fn kodegen_exe(scope: InstallScope) -> PathBuf
pub fn installer_log_dir() -> PathBuf
pub fn installer_config_dir() -> PathBuf
pub fn temp_dir() -> PathBuf
pub fn create_installer_directories(scope: InstallScope) -> Result<()>
```

However, the Windows installer code doesn't use these functions. Instead, it:
- Hardcodes paths inline
- Uses different path construction logic
- Duplicates path logic in multiple places

## Current State Analysis

### Functions That ARE Used
- `program_files_dir()` - Used
- `program_data_dir()` - Used
- `install_dir(scope)` - Used in some places
- `kodegend_exe(scope)` - Used
- `cert_dir()` - Used
- `services_dir()` - Used
- `hosts_file()` - Unused (see WINDOWS_04 task)
- `temp_cert_file()` - Possibly used (need to verify)

### Functions That Are NOT Used
- `kodegen_exe(scope)` - CLI binary path
- `installer_log_dir()` - Log directory
- `installer_config_dir()` - Config directory
- `temp_dir()` - Temp directory wrapper
- `create_installer_directories(scope)` - Directory creation helper

## Required Implementation

### 1. Audit Current Path Usage

Search the codebase for hardcoded Windows paths:
```bash
grep -r "Program Files" src/
grep -r "ProgramData" src/
grep -r "AppData" src/
grep -r "C:\\\\" src/
```

### 2. Replace Hardcoded Paths

For each hardcoded path, use the appropriate function:

Before:
```rust
let log_dir = PathBuf::from(r"C:\ProgramData\Kodegen\logs");
```

After:
```rust
use crate::install::installer::windows::paths::installer_log_dir;
let log_dir = installer_log_dir();
```

### 3. Use Directory Creation Helper

Before:
```rust
std::fs::create_dir_all(&install_dir)?;
std::fs::create_dir_all(&data_dir)?;
std::fs::create_dir_all(&cert_dir)?;
// ... repeated for each directory
```

After:
```rust
create_installer_directories(scope)?;
```

### 4. Add Missing Path Functions If Needed

If the audit reveals paths not covered by existing functions, add them to `paths.rs`:
```rust
/// Get the uninstall log path
pub fn uninstall_log_path() -> PathBuf {
    installer_log_dir().join("uninstall.log")
}
```

## Files to Modify

- `src/install/installer/core/context.rs` - Main installation context
- `src/install/privilege.rs` - Elevated operations
- `src/install/installer/windows/mod.rs` - Windows installer module
- `src/install/installer/windows/service_creation.rs` - Service setup
- Any other files using hardcoded Windows paths

## Benefits

1. **Consistency**: All paths come from one source
2. **Maintainability**: Change path logic in one place
3. **Testability**: Path functions can be unit tested
4. **Configuration**: Easy to add environment variable overrides
5. **Documentation**: Functions are self-documenting

## Testing

1. Verify all installation paths are correct
2. Test with non-default Program Files location
3. Test with non-English Windows (different path names)
4. Verify uninstall cleans up all directories

## Acceptance Criteria

- [ ] All hardcoded Windows paths replaced with function calls
- [ ] `create_installer_directories()` used for directory setup
- [ ] All path functions in `paths.rs` are used
- [ ] No dead code warnings for path functions
- [ ] Installation works correctly with consolidated paths
