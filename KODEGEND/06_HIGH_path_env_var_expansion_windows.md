# HIGH: Environment Variables Not Expanded in Windows Path Fallbacks

## Severity
**HIGH - CORRECTNESS**

## Location
- `packages/kodegend/src/platform/windows.rs:144`
- `packages/kodegend/src/platform/windows.rs:157`
- `packages/kodegend/src/platform/windows.rs:171`

## Issue Description
The Windows platform functions use hardcoded environment variable syntax (`%APPDATA%`, `%LOCALAPPDATA%`) as fallbacks when the `dirs` crate returns `None`. However, `PathBuf::from()` does not expand Windows environment variables, resulting in literal `%APPDATA%` directories being created.

### Problem Code
```rust
// Line 142-146
pub(super) fn platform_user_config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("%APPDATA%"))  // ← Creates literal "%APPDATA%"
        .join("kodegend")
}

// Line 152-160
pub(super) fn platform_runtime_dir(is_elevated: bool) -> PathBuf {
    if is_elevated {
        platform_system_config_dir().join("run")
    } else {
        dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("%LOCALAPPDATA%"))  // ← Literal variable
            .join("kodegend\\run")
    }
}

// Line 166-174
pub(super) fn platform_log_dir(is_elevated: bool) -> PathBuf {
    if is_elevated {
        platform_system_config_dir().join("logs")
    } else {
        dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("%LOCALAPPDATA%"))  // ← Literal variable
            .join("kodegend\\logs")
    }
}
```

### Real-World Impact
1. If `dirs::config_dir()` returns `None` (rare on Windows)
2. Creates directory `./%APPDATA%/kodegend/` in current directory
3. Config files written to wrong location
4. Daemon fails to find config files
5. Weird directory names visible in file explorer

### When Does dirs Return None on Windows?
- Registry corruption
- Roaming profile issues
- Running as SYSTEM account without proper setup
- Sandboxed environments
- Wine/compatibility layers

## Recommended Fix

### Option 1: Use std::env::var with manual expansion
```rust
pub(super) fn platform_user_config_dir() -> PathBuf {
    dirs::config_dir()
        .or_else(|| {
            std::env::var("APPDATA")
                .ok()
                .map(PathBuf::from)
        })
        .unwrap_or_else(|| {
            // Last resort: use ProgramData
            log::warn!("Could not determine user config directory, using ProgramData");
            PathBuf::from(r"C:\ProgramData\kodegend")
        })
        .join("kodegend")
}

pub(super) fn platform_runtime_dir(is_elevated: bool) -> PathBuf {
    if is_elevated {
        platform_system_config_dir().join("run")
    } else {
        dirs::data_local_dir()
            .or_else(|| {
                std::env::var("LOCALAPPDATA")
                    .ok()
                    .map(PathBuf::from)
            })
            .unwrap_or_else(|| {
                log::warn!("Could not determine runtime directory, using TEMP");
                std::env::temp_dir().join("kodegend")
            })
            .join("run")
    }
}

pub(super) fn platform_log_dir(is_elevated: bool) -> PathBuf {
    if is_elevated {
        platform_system_config_dir().join("logs")
    } else {
        dirs::data_local_dir()
            .or_else(|| {
                std::env::var("LOCALAPPDATA")
                    .ok()
                    .map(PathBuf::from)
            })
            .unwrap_or_else(|| {
                log::warn!("Could not determine log directory, using TEMP");
                std::env::temp_dir().join("kodegend")
            })
            .join("logs")
    }
}
```

### Option 2: Use known_folders crate (Windows-specific)
```toml
# Add to Cargo.toml
[dependencies]
known-folders = "1.1"
```

```rust
use known_folders::{get_known_folder_path, KnownFolder};

pub(super) fn platform_user_config_dir() -> PathBuf {
    dirs::config_dir()
        .or_else(|| get_known_folder_path(KnownFolder::RoamingAppData))
        .unwrap_or_else(|| PathBuf::from(r"C:\ProgramData"))
        .join("kodegend")
}
```

### Option 3: Use Windows API directly
```rust
use windows::Win32::UI::Shell::{SHGetKnownFolderPath, FOLDERID_RoamingAppData};

pub(super) fn platform_user_config_dir() -> PathBuf {
    dirs::config_dir()
        .or_else(|| {
            unsafe {
                SHGetKnownFolderPath(&FOLDERID_RoamingAppData, 0, None)
                    .ok()
                    .and_then(|path_pwstr| {
                        let path = path_pwstr.to_string().ok()?;
                        Some(PathBuf::from(path))
                    })
            }
        })
        .unwrap_or_else(|| PathBuf::from(r"C:\ProgramData"))
        .join("kodegend")
}
```

## Testing
1. Rename APPDATA registry keys temporarily
2. Run kodegend
3. Check for `%APPDATA%` directories: `dir /s %APPDATA%`
4. Verify fallback creates valid paths
5. Restore registry and verify normal operation

## Files to Modify
- `packages/kodegend/src/platform/windows.rs`

## Impact
- **Severity**: HIGH - Wrong file locations
- **Probability**: LOW - Requires dirs crate to fail
- **User Impact**: Config/logs in wrong location
- **Data Loss**: Possible if old files not found

## Additional Notes
Note that `platform_system_config_dir()` at line 132 has the same issue with `C:\ProgramData` fallback, but uses a valid absolute path so it's less critical.

Also consider standardizing path separators - currently mixing `\\` and using `.join()` (see related issue #29).
