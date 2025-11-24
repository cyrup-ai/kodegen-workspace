# MEDIUM: Working Directory Not Validated Before Spawn

## Priority
**MEDIUM** - Poor error messages, edge case failures

## Location
`packages/kodegend/src/manager.rs` - `spawn_service()` method (line 383)

## Issue Description

The working directory is not validated before spawning a service. If the directory doesn't exist or is inaccessible, spawn fails with cryptic errors.

### Current Code

```rust
// Line 383
Command::new(&service.command)
    .args(&service.args)
    // ...
    .current_dir(&service.working_dir)  // No validation!
    .spawn()?;
```

### Problems

#### Problem 1: No Existence Check

If `working_dir` doesn't exist:

```bash
# Config:
working_dir: /nonexistent/path

# Error:
Error: No such file or directory (os error 2)
```

**Issues**:
- Error message doesn't say WHICH directory doesn't exist
- Could be the command or the working_dir
- Hard to debug

#### Problem 2: No Expansion

```bash
# Config:
working_dir: ~/my-app

# Spawn fails - tilde not expanded
# Looking for literal "~" directory
```

**Issues**:
- Tilde (`~`) not expanded to home directory
- Environment variables not expanded (`$HOME/app`)
- Relative paths not handled

#### Problem 3: No Canonicalization

```bash
# Config:
working_dir: /app/../data/./logs

# Works but suboptimal
# Symlinks not resolved
# Path not normalized
```

**Issues**:
- Symlinks not followed
- Relative components not resolved
- Hard to debug (logs show raw path)

#### Problem 4: No Permission Check

```bash
# Config:
working_dir: /root/app

# Running as non-root user
# Spawn fails:
Error: Permission denied (os error 13)
```

**Issues**:
- No early validation
- Fails during spawn, not during config load
- Service marked as failed without clear reason

#### Problem 5: Network Mount Edge Cases

```bash
# Config:
working_dir: /mnt/nfs/app

# NFS mount is down/slow
# current_dir() hangs
# Spawn blocks indefinitely
```

**Issues**:
- No timeout for directory access
- Can hang entire manager
- Hard to recover

## Impact

### User Experience

**Poor error messages**:
```
Failed to spawn service my-app: No such file or directory
```

User must guess:
- Is the command not found?
- Is working_dir wrong?
- Is it a permission issue?

**Better error**:
```
Failed to spawn service my-app: Working directory does not exist: /nonexistent/path
```

### Configuration Issues

Users expect common shell features:
- Tilde expansion: `~/app` → `/home/user/app`
- Variable expansion: `$HOME/app`
- Relative paths: `../data`

Current: None of these work.

### Debugging Difficulty

```
# Multiple services fail
# All show same error: "No such file or directory"
# Which one has the bad config?
# Is it command or working_dir?
```

## Root Cause

No validation or normalization of `working_dir` before use.

## Solution

### Validate and Normalize at Config Load

```rust
use std::path::{Path, PathBuf};
use std::env;

pub struct ServiceConfig {
    pub name: String,
    pub command: String,
    pub working_dir: PathBuf,  // Normalized
    // ...
}

impl ServiceConfig {
    pub fn validate(&mut self) -> Result<()> {
        // Expand and normalize working_dir
        self.working_dir = normalize_path(&self.working_dir)?;
        
        // Validate it exists
        if !self.working_dir.exists() {
            return Err(anyhow!(
                "Working directory does not exist: {}",
                self.working_dir.display()
            ));
        }
        
        // Validate it's a directory
        if !self.working_dir.is_dir() {
            return Err(anyhow!(
                "Working directory is not a directory: {}",
                self.working_dir.display()
            ));
        }
        
        // Validate we can access it
        if let Err(e) = std::fs::read_dir(&self.working_dir) {
            return Err(anyhow!(
                "Cannot access working directory {}: {}",
                self.working_dir.display(),
                e
            ));
        }
        
        Ok(())
    }
}

fn normalize_path(path: &Path) -> Result<PathBuf> {
    let path_str = path.to_string_lossy();
    
    // Expand tilde
    let expanded = if path_str.starts_with("~/") {
        let home = env::var("HOME")
            .or_else(|_| env::var("USERPROFILE"))  // Windows
            .context("HOME not set")?;
        PathBuf::from(path_str.replacen("~", &home, 1))
    } else {
        path.to_path_buf()
    };
    
    // Expand environment variables
    let expanded_str = shellexpand::full(&expanded.to_string_lossy())
        .context("Failed to expand variables")?;
    let expanded = PathBuf::from(expanded_str.as_ref());
    
    // Canonicalize (resolve symlinks, relative paths)
    let canonical = expanded.canonicalize()
        .context(format!("Failed to canonicalize {}", expanded.display()))?;
    
    Ok(canonical)
}
```

### Or: Validate at Spawn Time

If we don't want to fail fast at config load (allow NFS mounts to be down temporarily):

```rust
async fn spawn_service(&mut self, service_name: &str) -> Result<()> {
    let service = self.config.services.iter()
        .find(|s| s.name == service_name)
        .ok_or_else(|| anyhow!("Service {} not found", service_name))?;
    
    // Normalize working directory
    let working_dir = normalize_path(&service.working_dir)
        .context(format!(
            "Invalid working directory for service {}: {}",
            service_name,
            service.working_dir.display()
        ))?;
    
    // Validate with timeout (handle NFS)
    let validation = tokio::time::timeout(
        Duration::from_secs(5),
        tokio::task::spawn_blocking(move || {
            // Check exists
            if !working_dir.exists() {
                return Err(anyhow!(
                    "Working directory does not exist: {}",
                    working_dir.display()
                ));
            }
            
            // Check is directory
            if !working_dir.is_dir() {
                return Err(anyhow!(
                    "Working directory is not a directory: {}",
                    working_dir.display()
                ));
            }
            
            // Check accessible
            std::fs::read_dir(&working_dir)
                .context(format!(
                    "Cannot access working directory: {}",
                    working_dir.display()
                ))?;
            
            Ok(working_dir)
        })
    ).await??;
    
    let working_dir = validation?;
    
    // Now spawn with validated directory
    let mut child = Command::new(&service.command)
        .current_dir(&working_dir)
        .spawn()
        .context(format!(
            "Failed to spawn command {} in directory {}",
            service.command,
            working_dir.display()
        ))?;
    
    // ...
}
```

## Recommended Solution

**Hybrid**:

1. **Normalize at config load** (expand ~, resolve symlinks)
2. **Validate at spawn** (check exists, accessible) with timeout
3. **Better error messages** that distinguish directory vs command issues

This allows:
- Early detection of obviously wrong paths
- Tolerance for temporarily unavailable mounts
- Clear error messages

## Required Changes

1. Add `normalize_path()` helper function
2. Add validation to `ServiceConfig::validate()` or similar
3. Update `spawn_service()` to validate working_dir with timeout
4. Add shellexpand crate for variable expansion
5. Improve error messages to specify what failed
6. Add tests for edge cases

## Dependencies

```toml
shellexpand = "3.1"  # For env var expansion
```

Or implement manually without dependency.

## Testing

```rust
#[test]
fn test_tilde_expansion() {
    let path = PathBuf::from("~/myapp");
    let normalized = normalize_path(&path).unwrap();
    assert!(!normalized.to_string_lossy().contains('~'));
}

#[test]
fn test_nonexistent_directory() {
    let config = ServiceConfig {
        working_dir: PathBuf::from("/nonexistent/path"),
        // ...
    };
    assert!(config.validate().is_err());
}

#[test]
fn test_symlink_resolution() {
    // Create symlink: /tmp/link -> /tmp/target
    std::fs::create_dir("/tmp/target").ok();
    std::os::unix::fs::symlink("/tmp/target", "/tmp/link").ok();
    
    let path = PathBuf::from("/tmp/link");
    let normalized = normalize_path(&path).unwrap();
    assert_eq!(normalized, PathBuf::from("/tmp/target"));
}

#[tokio::test]
async fn test_working_dir_timeout() {
    // Mock slow NFS mount
    // Verify spawn fails fast with timeout, not hangs
}
```

## Error Message Improvements

**Before**:
```
Error: No such file or directory (os error 2)
```

**After**:
```
Error: Failed to spawn service my-app
Caused by:
  0: Working directory does not exist: /app/data
  1: Configured as: ~/app/data
  2: Expanded to: /home/user/app/data
```

## Related Issues

None directly, but improves overall robustness.

## References

- Path canonicalization: `std::fs::canonicalize()`
- Shell expansion: shellexpand crate
- Environment variables: `std::env::var()`
