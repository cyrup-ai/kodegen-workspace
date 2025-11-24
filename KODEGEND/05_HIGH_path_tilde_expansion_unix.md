# HIGH: Tilde (~) Not Expanded in Unix Path Fallbacks

## Severity
**HIGH - CORRECTNESS**

## Location
- `packages/kodegend/src/platform/unix.rs:74`
- `packages/kodegend/src/platform/unix.rs:107`

## Issue Description
The `platform_user_config_dir()` and `platform_log_dir()` functions use hardcoded tilde paths as fallbacks when the `dirs` crate returns `None`. However, `PathBuf::from()` does not expand shell tilde syntax, resulting in literal `~` directories being created.

### Problem Code
```rust
// Line 72-76
pub(super) fn platform_user_config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("~/.config"))  // ← Creates literal "~" directory
        .join("kodegend")
}

// Line 102-109
pub(super) fn platform_log_dir(is_elevated: bool) -> PathBuf {
    if is_elevated {
        PathBuf::from("/var/log/kodegend")
    } else {
        dirs::state_dir()
            .unwrap_or_else(|| PathBuf::from("~/.local/state"))  // ← Creates literal "~"
            .join("kodegend/logs")
    }
}
```

### Real-World Impact
1. If `dirs::config_dir()` returns `None` (rare but possible)
2. Creates directory `./~/.config/kodegend/` in current working directory
3. Config files written to wrong location
4. Daemon fails to find config files
5. User confusion - files in unexpected location

### When Does dirs Return None?
- HOME environment variable not set
- Running in restricted environment (chroot, container)
- macOS with SIP restrictions
- Unusual system configurations

## Recommended Fix

### Option 1: Use dirs::home_dir() with manual path construction
```rust
pub(super) fn platform_user_config_dir() -> PathBuf {
    dirs::config_dir()
        .or_else(|| {
            dirs::home_dir().map(|home| home.join(".config"))
        })
        .unwrap_or_else(|| {
            // Last resort: use current directory
            log::warn!("Could not determine config directory, using ./config");
            PathBuf::from("./config")
        })
        .join("kodegend")
}

pub(super) fn platform_log_dir(is_elevated: bool) -> PathBuf {
    if is_elevated {
        PathBuf::from("/var/log/kodegend")
    } else {
        dirs::state_dir()
            .or_else(|| {
                dirs::home_dir().map(|home| home.join(".local/state"))
            })
            .unwrap_or_else(|| {
                log::warn!("Could not determine log directory, using ./logs");
                PathBuf::from("./logs")
            })
            .join("kodegend/logs")
    }
}
```

### Option 2: Use std::env::var("HOME") with explicit expansion
```rust
pub(super) fn platform_user_config_dir() -> PathBuf {
    dirs::config_dir()
        .or_else(|| {
            std::env::var("HOME")
                .ok()
                .map(|home| PathBuf::from(home).join(".config"))
        })
        .unwrap_or_else(|| PathBuf::from("/tmp/kodegend-config"))
        .join("kodegend")
}
```

### Option 3: Use shellexpand crate (add dependency)
```rust
use shellexpand;

pub(super) fn platform_user_config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| {
            let expanded = shellexpand::tilde("~/.config");
            PathBuf::from(expanded.as_ref())
        })
        .join("kodegend")
}
```

## Testing
1. Unset HOME environment variable: `env -u HOME ./kodegend`
2. Check created directories with `find . -name "~"`
3. Verify no literal `~` directories created
4. Verify fallback creates valid paths

## Files to Modify
- `packages/kodegend/src/platform/unix.rs`

## Impact
- **Severity**: HIGH - Wrong file locations
- **Probability**: LOW - Requires dirs crate to fail
- **User Impact**: Config/logs in wrong location
- **Data Loss**: Possible if old files not found

## Additional Notes
The same issue exists in `platform_runtime_dir()` line 86-94, but that's addressed separately in task #01 due to security implications.
