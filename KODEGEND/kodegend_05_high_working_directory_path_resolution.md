# HIGH: Working Directory Path Resolution in Configuration Files

## Severity
**HIGH** - Relative paths in user-edited config files will resolve incorrectly after daemonization

## Status Summary

### FIXED (Already Implemented)
- **Main ordering issue**: Daemonization now happens BEFORE config loading ([main.rs:84-86](../packages/kodegend/src/main.rs))
- **PID file creation**: Now happens AFTER config loading with RAII guard ([main.rs:132-133](../packages/kodegend/src/main.rs))
- **Platform-specific defaults**: All default paths are absolute ([config.rs:61-66, 308-339](../packages/kodegend/src/config.rs))

### REMAINING WORK
- **Path canonicalization**: Relative paths in user-edited config files need to be resolved when loading config
- **Config validation**: Need to detect and handle relative paths with appropriate warnings
- **Service definition paths**: Need to canonicalize paths in nested ServiceDefinition structs

## Current Architecture

### Code Flow (as of current implementation)
```rust
// 1. Daemonization happens FIRST (main.rs:84-86)
if !should_stay_foreground {
    daemon::daemonise()?;  // Changes cwd to "/" on Unix
}

// 2. Config path determined (main.rs:89-101) - uses absolute paths

// 3. Config loaded from disk (main.rs:122-126)
let cfg: config::ServiceConfig = toml::from_str(&cfg_str)?;
// Problem: If config contains relative paths, they're not canonicalized

// 4. PID file created (main.rs:132-133)
let pid_file = daemon::PidFile::create(cfg.pid_file.clone())?;
// If cfg.pid_file is relative, it resolves against "/" on Unix
```

### Daemonization Working Directory Change
```rust
// packages/kodegend/src/daemon.rs:203
chdir("/").context("chdir")?;  // Unix daemons change to root directory
```

This is correct daemon behavior, but means any relative paths in config must be resolved BEFORE use.

## Root Cause (Updated)

The issue occurs when users manually edit config files and use relative paths:

1. **Default configs work fine**: All default paths from `default_pid_file()`, `default_services_dir()`, etc. are absolute
2. **User-edited configs may fail**: If a user sets `pid_file = "./run/daemon.pid"` in their config:
   - Config is loaded after daemonization
   - Current directory is now `/` (on Unix)
   - Relative path `./run/daemon.pid` resolves to `/run/daemon.pid`
   - This is wrong - user expected it relative to config file or original cwd

## Affected Configuration Fields

### ServiceConfig (packages/kodegend/src/config.rs:8-54)
- `pid_file: PathBuf` (line 49) - PID file location
- `services_dir: Option<String>` (line 11) - Service definitions directory  
- `log_dir: Option<String>` (line 14) - Global log directory

### ServiceDefinition (packages/kodegend/src/config.rs:359-399)
- `working_dir: Option<String>` (line 364) - Service working directory
- `log_stdout: Option<String>` (line 367) - Stdout log file
- `log_stderr: Option<String>` (line 369) - Stderr log file
- `watch_dirs: Vec<String>` (line 394) - Directories to watch for changes
- `ephemeral_dir: Option<String>` (line 395) - Temporary directory

## Implementation Plan

### Step 1: Add Path Canonicalization Helper

Add to `packages/kodegend/src/config.rs`:

```rust
use std::path::{Path, PathBuf};

/// Canonicalize a path that may be relative or absolute
/// 
/// - If path is absolute: return as-is
/// - If path is relative: resolve relative to `base_dir`
/// 
/// This allows relative paths in config files to be resolved relative to
/// the config file's directory, making configs portable and predictable.
/// 
/// Unlike std::fs::canonicalize(), this does NOT require the path to exist,
/// and does NOT resolve symlinks (which is desirable for daemon configs).
fn canonicalize_config_path<P: AsRef<Path>>(path: P, base_dir: &Path) -> PathBuf {
    let path = path.as_ref();
    
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        // Resolve relative to base_dir (typically config file's directory)
        base_dir.join(path)
    }
}

/// Canonicalize an optional path
fn canonicalize_optional_string(
    path: &Option<String>, 
    base_dir: &Path
) -> Option<String> {
    path.as_ref().map(|p| {
        canonicalize_config_path(p, base_dir)
            .display()
            .to_string()
    })
}
```

### Step 2: Modify ServiceConfig::load_from_file()

Update `packages/kodegend/src/config.rs:295-305`:

```rust
pub fn load_from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self, anyhow::Error> {
    let path = path.as_ref();
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read config file: {}", path.display()))?;
    
    let mut cfg: ServiceConfig = toml::from_str(&content)
        .with_context(|| format!("Failed to parse config file: {}", path.display()))?;
    
    // Determine base directory for resolving relative paths
    // Use config file's directory, or current directory if config is from stdin
    let base_dir = path.parent().unwrap_or_else(|| Path::new("."));
    
    // Canonicalize top-level paths
    cfg.pid_file = canonicalize_config_path(&cfg.pid_file, base_dir);
    cfg.services_dir = canonicalize_optional_string(&cfg.services_dir, base_dir);
    cfg.log_dir = canonicalize_optional_string(&cfg.log_dir, base_dir);
    
    // Canonicalize paths in each service definition
    for service in &mut cfg.services {
        service.working_dir = canonicalize_optional_string(&service.working_dir, base_dir);
        service.log_stdout = canonicalize_optional_string(&service.log_stdout, base_dir);
        service.log_stderr = canonicalize_optional_string(&service.log_stderr, base_dir);
        service.ephemeral_dir = canonicalize_optional_string(&service.ephemeral_dir, base_dir);
        
        // Canonicalize watch directories
        service.watch_dirs = service.watch_dirs
            .iter()
            .map(|dir| canonicalize_config_path(dir, base_dir).display().to_string())
            .collect();
    }
    
    // Log warnings for any paths that were relative
    if !path.is_absolute() {
        log::warn!(
            "Config file path is relative: {}. Paths inside config will be \
             resolved relative to config file's directory: {}",
            path.display(),
            base_dir.display()
        );
    }
    
    cfg.config_file_path = Some(path.to_path_buf());
    Ok(cfg)
}
```

### Step 3: Update Default Config Generation

The default config generator in `main.rs:114` already creates proper defaults via `ServiceConfig::default()`, which uses platform-specific absolute paths. No changes needed.

## Edge Cases to Handle

### 1. Symlinks in Config Path
**Behavior**: Preserve symlinks (don't resolve)
```rust
// Current approach: path.join(relative) preserves symlinks
// Alternative: path.canonicalize().join(relative) would resolve symlinks
// Decision: Preserve symlinks for flexibility
```

### 2. Non-Existent Parent Directories
**Behavior**: Allow - validation happens at runtime when creating files
```rust
// Don't validate existence during config load
// Let PidFile::create() handle directory creation (daemon.rs:53-56)
```

### 3. Windows UNC Paths
**Behavior**: Windows paths starting with `\\?\` are already absolute
```rust
// PathBuf::is_absolute() correctly handles UNC paths on Windows
// No special handling needed
```

### 4. Config File in Current Directory
```rust
// Example: ./kodegend.toml
// base_dir = path.parent() = Some(".")
// Relative paths resolve correctly against current directory
```

## Example Scenarios

### Scenario 1: Absolute Paths (Current Defaults)
```toml
# /etc/kodegend/kodegend.toml
[daemon]
pid_file = "/var/run/kodegend/kodegend.pid"
log_dir = "/var/log/kodegend"

[[services]]
working_dir = "/opt/myapp"
```
**Result**: All paths used as-is (already absolute)

### Scenario 2: Relative Paths (User-Edited)
```toml
# /home/user/myapp/kodegend.toml
[daemon]
pid_file = "./run/daemon.pid"
log_dir = "./logs"

[[services]]
working_dir = "./workspace"
```
**Before Fix**: Paths resolve to `/run/daemon.pid`, `/logs`, `/workspace` (wrong!)
**After Fix**: Paths resolve to:
- `/home/user/myapp/run/daemon.pid`
- `/home/user/myapp/logs`
- `/home/user/myapp/workspace`

### Scenario 3: Mixed Paths
```toml
# /etc/kodegend/kodegend.toml
[daemon]
pid_file = "/var/run/kodegend/kodegend.pid"  # Absolute - use as-is
log_dir = "./logs"                            # Relative - resolve to /etc/kodegend/logs

[[services]]
working_dir = "/opt/service"                  # Absolute - use as-is
log_stdout = "../logs/service.log"            # Relative - resolve to /etc/logs/service.log
```

## Files to Modify

### Primary Changes
1. **packages/kodegend/src/config.rs**
   - Add `canonicalize_config_path()` helper function
   - Add `canonicalize_optional_string()` helper function  
   - Modify `ServiceConfig::load_from_file()` to canonicalize all path fields

### No Changes Needed
1. **packages/kodegend/src/main.rs** - Already correct (daemon → config → PID)
2. **packages/kodegend/src/daemon.rs** - Working as designed
3. **packages/kodegend/src/service.rs** - Uses paths from config as-is (correct)

## Definition of Done

### Implementation Complete When:
1. Path canonicalization helpers are added to config.rs
2. ServiceConfig::load_from_file() canonicalizes all path fields
3. Relative paths in config are resolved relative to config file's directory
4. Absolute paths in config are preserved unchanged
5. Warning logged when relative paths are detected (optional but recommended)

### Validation Approach:
1. Create test config with relative paths
2. Start daemon from different directory than config location
3. Verify paths resolve correctly relative to config file
4. Check daemon log for PID file location - should match expected absolute path

## Research References

### Rust Path Canonicalization
- **std::fs::canonicalize()** - Requires path to exist, resolves symlinks, has Windows UNC quirks ([Rust docs](https://doc.rust-lang.org/std/fs/fn.canonicalize.html))
- **soft-canonicalize crate** - Can normalize non-existent paths ([docs.rs](https://docs.rs/soft-canonicalize)) - NOT NEEDED for this implementation
- **PathBuf::is_absolute()** - Cross-platform absolute path detection - USE THIS

### Daemon Best Practices
- Unix daemons typically `chdir("/")` to avoid holding directory locks ([daemon.rs:203](../packages/kodegend/src/daemon.rs))
- Config files should use absolute paths or paths relative to config location
- systemd recommends absolute paths in unit files

### Codebase References
- [Platform path utilities](../packages/kodegend/src/platform/mod.rs) - Cross-platform directory functions
- [Current ServiceConfig structure](../packages/kodegend/src/config.rs) - Shows all affected fields
- [Service working_dir usage](../packages/kodegend/src/service.rs#L163-165) - How paths are currently used
- [Default path generation](../packages/kodegend/src/config.rs#L61-66) - Platform-specific defaults (already absolute)

## Impact Assessment

### Breaking Changes: NONE
- Default configs already use absolute paths
- Existing configs with absolute paths unchanged
- Only affects user-edited configs with relative paths (which currently don't work correctly)

### Backward Compatibility
- Configs that work now will continue to work
- Configs that fail now may start working (if paths become correct after canonicalization)
- No migration required

### Performance Impact: NEGLIGIBLE
- Path canonicalization happens once during config load
- No runtime overhead during daemon operation
