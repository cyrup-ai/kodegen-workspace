# Missing File Permission Validation and Safety Checks

## Severity: MEDIUM

## Location
`packages/kodegend/src/daemon.rs` - all file operations

## Issue Description
File operations don't validate permissions, ownership, or disk space, which can lead to cryptic errors or security issues.

## Missing Validations

### 1. No Permission Checks Before Operations
```rust
pub fn create_pid_file(path: &Path) -> Result<()> {
    // No check if directory is writable!
    let mut file = File::create(path)?;
    // Might fail here with "Permission denied" with no context
}
```

Better to check upfront:
```rust
pub fn create_pid_file(path: &Path) -> Result<()> {
    // Check parent directory exists and is writable
    if let Some(parent) = path.parent() {
        validate_directory_writable(parent)?;
    }
    
    // Now create file
    let mut file = File::create(path)
        .with_context(|| format!("Failed to create PID file at {}", path.display()))?;
    // ...
}

fn validate_directory_writable(dir: &Path) -> Result<()> {
    if !dir.exists() {
        return Err(anyhow!(
            "Directory {} does not exist",
            dir.display()
        ));
    }
    
    if !dir.is_dir() {
        return Err(anyhow!(
            "{} is not a directory",
            dir.display()
        ));
    }
    
    // Try to create a temp file to verify write access
    tempfile::NamedTempFile::new_in(dir)
        .with_context(|| format!(
            "Directory {} is not writable",
            dir.display()
        ))?;
    
    Ok(())
}
```

### 2. No Disk Space Checks
```rust
pub fn create_pid_file(path: &Path) -> Result<()> {
    let mut file = File::create(path)?;
    write!(file, "{}", std::process::id())?;  // Could fail with ENOSPC
    // No specific handling for "disk full"
}
```

Better error handling:
```rust
pub fn create_pid_file(path: &Path) -> Result<()> {
    let mut file = File::create(path)?;
    
    if let Err(e) = write!(file, "{}", std::process::id()) {
        if e.kind() == std::io::ErrorKind::Other {
            // Check if it's a disk full error
            if let Some(os_error) = e.raw_os_error() {
                #[cfg(unix)]
                if os_error == libc::ENOSPC {
                    return Err(anyhow!(
                        "Cannot create PID file: disk full (ENOSPC)"
                    ));
                }
            }
        }
        return Err(e.into());
    }
    
    file.flush()?;
    Ok(())
}
```

### 3. No Ownership Validation
```rust
pub fn create_pid_file(path: &Path) -> Result<()> {
    // If running as root, should verify directory is owned by root
    // If file exists, should verify it's owned by current user
    // Currently no checks!
}
```

Security enhancement:
```rust
#[cfg(unix)]
fn validate_file_ownership(path: &Path) -> Result<()> {
    use std::os::unix::fs::MetadataExt;
    
    if !path.exists() {
        return Ok(());  // File doesn't exist yet
    }
    
    let metadata = std::fs::metadata(path)?;
    let file_uid = metadata.uid();
    let current_uid = unsafe { libc::geteuid() };
    
    // If running as root (UID 0)
    if current_uid == 0 {
        // Verify file is owned by root
        if file_uid != 0 {
            return Err(anyhow!(
                "Security: PID file {} is owned by UID {} but daemon is running as root. \
                 This could be a privilege escalation attempt.",
                path.display(),
                file_uid
            ));
        }
    } else {
        // Running as regular user, verify we own the file
        if file_uid != current_uid {
            return Err(anyhow!(
                "PID file {} is owned by UID {} but current user is UID {}",
                path.display(),
                file_uid,
                current_uid
            ));
        }
    }
    
    Ok(())
}
```

### 4. No Permission Mode Validation
```rust
pub fn create_pid_file(path: &Path) -> Result<()> {
    let mut file = File::create(path)?;
    // File created with default umask permissions
    // Might be world-writable depending on umask!
}
```

Set explicit permissions:
```rust
#[cfg(unix)]
pub fn create_pid_file(path: &Path) -> Result<()> {
    use std::os::unix::fs::OpenOptionsExt;
    
    let mut options = OpenOptions::new();
    options.write(true)
           .create_new(true)
           .mode(0o644);  // rw-r--r--
    
    let mut file = options.open(path)
        .with_context(|| format!("Failed to create PID file at {}", path.display()))?;
    
    write!(file, "{}", std::process::id())?;
    file.flush()?;
    
    Ok(())
}
```

## Recommended Comprehensive Validation

```rust
/// Validate PID file path and permissions before operations
pub fn validate_pid_file_path(path: &Path) -> Result<()> {
    // 1. Check parent directory
    let parent = path.parent()
        .ok_or_else(|| anyhow!("Invalid PID file path: no parent directory"))?;
    
    if !parent.exists() {
        return Err(anyhow!(
            "PID file directory {} does not exist",
            parent.display()
        ));
    }
    
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        
        // 2. Check directory permissions
        let dir_meta = std::fs::metadata(parent)?;
        
        // Directory should be writable
        let mode = dir_meta.mode();
        let user_write = mode & 0o200;
        if user_write == 0 {
            return Err(anyhow!(
                "PID file directory {} is not writable (mode: {:o})",
                parent.display(),
                mode
            ));
        }
        
        // 3. Check directory ownership
        let dir_uid = dir_meta.uid();
        let current_uid = unsafe { libc::geteuid() };
        
        // If not root and don't own directory, might not be able to write
        if current_uid != 0 && dir_uid != current_uid {
            // Try to actually write a temp file to verify
            if tempfile::NamedTempFile::new_in(parent).is_err() {
                return Err(anyhow!(
                    "PID file directory {} is not writable by current user (owned by UID {})",
                    parent.display(),
                    dir_uid
                ));
            }
        }
    }
    
    // 4. If file exists, validate it
    if path.exists() {
        validate_existing_pid_file(path)?;
    }
    
    Ok(())
}

#[cfg(unix)]
fn validate_existing_pid_file(path: &Path) -> Result<()> {
    use std::os::unix::fs::MetadataExt;
    
    let metadata = std::fs::symlink_metadata(path)?;
    
    // Reject symlinks (security)
    if metadata.file_type().is_symlink() {
        return Err(anyhow!(
            "PID file {} is a symlink (security risk)",
            path.display()
        ));
    }
    
    // Validate ownership
    let file_uid = metadata.uid();
    let current_uid = unsafe { libc::geteuid() };
    
    if current_uid == 0 && file_uid != 0 {
        return Err(anyhow!(
            "Security: PID file {} owned by UID {}, expected root (UID 0)",
            path.display(),
            file_uid
        ));
    } else if current_uid != 0 && file_uid != current_uid {
        return Err(anyhow!(
            "PID file {} owned by UID {}, expected UID {}",
            path.display(),
            file_uid,
            current_uid
        ));
    }
    
    // Validate permissions
    let mode = metadata.mode() & 0o777;
    
    // Should not be world-writable
    if mode & 0o002 != 0 {
        return Err(anyhow!(
            "PID file {} is world-writable (mode: {:o}), security risk",
            path.display(),
            mode
        ));
    }
    
    // Should not be group-writable unless intentional
    if mode & 0o020 != 0 {
        log::warn!(
            "PID file {} is group-writable (mode: {:o})",
            path.display(),
            mode
        );
    }
    
    Ok(())
}
```

## Usage Pattern

```rust
pub fn create_pid_file(path: &Path) -> Result<()> {
    // Validate before attempting to create
    validate_pid_file_path(path)?;
    
    // Validate ownership if file exists
    #[cfg(unix)]
    validate_file_ownership(path)?;
    
    // If existing file is stale, remove it
    if path.exists() {
        let existing_pid = read_pid_file(path)?;
        if !is_process_running(existing_pid)? {
            remove_pid_file(path)?;
        } else {
            return Err(anyhow!(
                "Service already running with PID {}",
                existing_pid
            ));
        }
    }
    
    // Create with explicit permissions
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        let mut options = OpenOptions::new();
        options.write(true)
               .create_new(true)
               .mode(0o644);
        let mut file = options.open(path)?;
        write!(file, "{}", std::process::id())?;
        file.flush()?;
    }
    
    #[cfg(not(unix))]
    {
        let mut file = File::create(path)?;
        write!(file, "{}", std::process::id())?;
        file.flush()?;
    }
    
    Ok(())
}
```

## Testing Strategy

### Permission Tests
```rust
#[cfg(test)]
#[cfg(unix)]
mod permission_tests {
    use std::os::unix::fs::PermissionsExt;
    
    #[test]
    fn test_rejects_world_writable_pid_file() {
        let temp = tempfile::NamedTempFile::new().unwrap();
        let path = temp.path();
        
        // Make world-writable
        std::fs::set_permissions(path, 
            std::fs::Permissions::from_mode(0o666)).unwrap();
        
        // Should reject
        assert!(validate_existing_pid_file(path).is_err());
    }
    
    #[test]
    fn test_rejects_wrong_owner() {
        // Would need to actually change ownership
        // Requires root or separate test user
        // Skip in normal test runs
    }
    
    #[test]
    fn test_rejects_unwritable_directory() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("test.pid");
        
        // Make directory read-only
        std::fs::set_permissions(temp_dir.path(),
            std::fs::Permissions::from_mode(0o555)).unwrap();
        
        // Should detect not writable
        assert!(validate_pid_file_path(&path).is_err());
        
        // Restore permissions for cleanup
        std::fs::set_permissions(temp_dir.path(),
            std::fs::Permissions::from_mode(0o755)).unwrap();
    }
}
```

## Error Message Improvements

### Before (cryptic)
```
Error: Permission denied (os error 13)
```

### After (actionable)
```
Error: Failed to create PID file at /var/run/kodegend.pid
  Caused by: Permission denied

  Directory /var/run is owned by root (UID 0)
  Current user: kodegend (UID 1000)
  
  Suggestion: Either:
  - Run daemon as root
  - Use a different PID file location (e.g., /tmp/kodegend.pid)
  - Create directory /var/run/kodegend owned by user kodegend
```

## Implementation Priority
**Medium** - Improves error messages and catches issues early, but not critical for correctness.

## References
- UNIX file permissions: chmod(2), chown(2)
- Secure file operations: "Secure Programming Cookbook"
- tmpfiles.d for proper runtime directory setup
