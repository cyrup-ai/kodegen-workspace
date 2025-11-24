# Security: PID File Symlink Attack Vulnerability

## Severity: HIGH (Security)

## Location
`packages/kodegend/src/daemon.rs:86-145`

## Issue Description
The PID file operations don't validate against symlink attacks, allowing potential security vulnerabilities where an attacker could manipulate PID file operations to affect arbitrary files.

## Attack Vectors

### Attack 1: Symlink to Sensitive File
```bash
# Attacker creates symlink before daemon starts
$ rm /var/run/kodegend.pid
$ ln -s /etc/passwd /var/run/kodegend.pid

# Daemon tries to create PID file
$ kodegend start
# If running as root, could corrupt /etc/passwd!
```

### Attack 2: TOCTOU Race
```bash
# Daemon checks PID file
if path.exists() {
    # ... reads/checks ...
    remove_pid_file(path)?;  # <-- Race window here
}
# Attacker replaces with symlink HERE
File::create(path)?;  # Writes to symlink target!
```

### Attack 3: Directory Traversal
```bash
# If PID file path is user-controlled or improperly validated
$ kodegend --pid-file ../../../etc/cron.d/malicious
# Creates file in sensitive location
```

## Current Vulnerable Code

```rust
pub fn create_pid_file(path: &Path) -> Result<()> {
    // NO SYMLINK CHECK!
    if path.exists() {
        let existing_pid = read_pid_file(path)?;
        if is_process_running(existing_pid)? {
            return Err(anyhow!("Service already running"));
        }
        remove_pid_file(path)?;  // Could remove arbitrary file via symlink!
    }
    
    let mut file = File::create(path)?;  // Could write to arbitrary file!
    // ...
}

pub fn remove_pid_file(path: &Path) -> Result<()> {
    // NO SYMLINK CHECK!
    fs::remove_file(path)?;  // Could remove arbitrary file!
    Ok(())
}
```

## Real-World Impact

### If running as root:
- Could corrupt system files
- Could write to arbitrary locations
- Could escalate privileges

### If running as daemon user:
- Could corrupt daemon's own files
- Could cause denial of service
- Limited but still dangerous

## Recommended Solutions

### Solution 1: Validate Against Symlinks (Recommended)
```rust
use std::os::unix::fs::MetadataExt;

pub fn validate_pid_file_path(path: &Path) -> Result<()> {
    // Check if path exists
    if !path.exists() {
        return Ok(());  // Safe to create
    }
    
    // Get metadata WITHOUT following symlinks
    let metadata = std::fs::symlink_metadata(path)
        .context("Failed to get file metadata")?;
    
    // Reject symlinks
    if metadata.file_type().is_symlink() {
        return Err(anyhow!(
            "PID file {} is a symlink - potential security issue",
            path.display()
        ));
    }
    
    // Validate file is a regular file
    if !metadata.file_type().is_file() {
        return Err(anyhow!(
            "PID file {} is not a regular file",
            path.display()
        ));
    }
    
    // Validate ownership and permissions
    #[cfg(unix)]
    {
        let current_uid = unsafe { libc::geteuid() };
        
        // If running as root, require root ownership
        if current_uid == 0 && metadata.uid() != 0 {
            return Err(anyhow!(
                "PID file {} not owned by root (owned by UID {})",
                path.display(),
                metadata.uid()
            ));
        }
        
        // If running as user, require user ownership
        if current_uid != 0 && metadata.uid() != current_uid {
            return Err(anyhow!(
                "PID file {} not owned by current user (owned by UID {})",
                path.display(),
                metadata.uid()
            ));
        }
        
        // Check permissions - should not be world-writable
        let mode = metadata.mode();
        if mode & 0o002 != 0 {
            return Err(anyhow!(
                "PID file {} is world-writable (mode: {:o})",
                path.display(),
                mode
            ));
        }
    }
    
    Ok(())
}

pub fn create_pid_file(path: &Path) -> Result<()> {
    // Validate parent directory
    if let Some(parent) = path.parent() {
        let parent_meta = std::fs::symlink_metadata(parent)
            .context("PID file directory does not exist")?;
        
        if parent_meta.file_type().is_symlink() {
            return Err(anyhow!(
                "PID file directory {} is a symlink",
                parent.display()
            ));
        }
    }
    
    // Validate existing PID file
    validate_pid_file_path(path)?;
    
    if path.exists() {
        let existing_pid = read_pid_file(path)?;
        if is_process_running(existing_pid)? {
            return Err(anyhow!("Service already running"));
        }
        
        // Safe to remove - we validated it's not a symlink
        remove_pid_file(path)?;
    }
    
    // Create with atomic guarantees
    use std::os::unix::fs::OpenOptionsExt;
    
    let mut options = OpenOptions::new();
    options.write(true)
           .create_new(true)
           .mode(0o644);  // rw-r--r--
    
    let mut file = options.open(path)
        .context("Failed to create PID file")?;
    
    let pid = std::process::id();
    write!(file, "{}", pid)?;
    file.flush()?;
    
    Ok(())
}
```

### Solution 2: Use O_NOFOLLOW (Unix)
```rust
#[cfg(unix)]
pub fn create_pid_file_safe(path: &Path) -> Result<()> {
    use std::os::unix::fs::OpenOptionsExt;
    use std::os::unix::io::AsRawFd;
    
    let mut options = OpenOptions::new();
    options.write(true)
           .create(true)
           .truncate(true)
           .custom_flags(libc::O_NOFOLLOW);  // Fail if path is symlink
    
    let mut file = options.open(path)
        .context("Failed to create PID file (may be a symlink)")?;
    
    write!(file, "{}", std::process::id())?;
    Ok(())
}
```

### Solution 3: Use Secure Temporary File + Rename
```rust
pub fn create_pid_file_atomic(path: &Path) -> Result<()> {
    use tempfile::NamedTempFile;
    
    // Validate directory is safe
    let parent = path.parent()
        .ok_or_else(|| anyhow!("Invalid PID file path"))?;
    
    // Create temp file in same directory
    let mut temp = NamedTempFile::new_in(parent)?;
    write!(temp, "{}", std::process::id())?;
    temp.flush()?;
    
    // Atomic rename - will fail if target is symlink (on some systems)
    temp.persist(path)
        .map_err(|e| anyhow!("Failed to create PID file: {}", e))?;
    
    Ok(())
}
```

## Additional Security Hardening

### Restrict PID File Directory
```rust
pub fn validate_pid_file_directory(path: &Path) -> Result<()> {
    let parent = path.parent()
        .ok_or_else(|| anyhow!("Invalid PID file path"))?;
    
    // Whitelist of acceptable directories
    let allowed_dirs = [
        Path::new("/var/run"),
        Path::new("/run"),
        Path::new("/tmp"),
    ];
    
    // Check if path is within allowed directories
    let is_allowed = allowed_dirs.iter().any(|dir| {
        path.starts_with(dir)
    });
    
    if !is_allowed {
        return Err(anyhow!(
            "PID file must be in /var/run, /run, or /tmp, got: {}",
            path.display()
        ));
    }
    
    Ok(())
}
```

### Set Restrictive Permissions
```rust
#[cfg(unix)]
pub fn secure_pid_file_permissions(path: &Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    
    // Set to 0600 (rw-------)
    let permissions = std::fs::Permissions::from_mode(0o600);
    std::fs::set_permissions(path, permissions)?;
    
    Ok(())
}
```

## Testing Strategy

### Security Tests
```rust
#[cfg(test)]
mod security_tests {
    #[test]
    #[cfg(unix)]
    fn test_rejects_symlink() {
        let temp_dir = tempfile::tempdir().unwrap();
        let pid_file = temp_dir.path().join("test.pid");
        let target = temp_dir.path().join("target");
        
        // Create symlink
        std::os::unix::fs::symlink(&target, &pid_file).unwrap();
        
        // Should reject symlink
        assert!(create_pid_file(&pid_file).is_err());
    }
    
    #[test]
    fn test_rejects_world_writable() {
        let temp_dir = tempfile::tempdir().unwrap();
        let pid_file = temp_dir.path().join("test.pid");
        
        // Create world-writable file
        std::fs::write(&pid_file, "1234").unwrap();
        std::fs::set_permissions(&pid_file, 
            std::fs::Permissions::from_mode(0o666)).unwrap();
        
        // Should reject
        assert!(validate_pid_file_path(&pid_file).is_err());
    }
}
```

### Manual Security Audit
```bash
# Test symlink attack
$ ln -s /etc/passwd /tmp/kodegend.pid
$ kodegend start
# Should fail with error about symlink

# Test directory traversal
$ kodegend --pid-file ../../../etc/malicious.pid
# Should fail with validation error

# Test permissions
$ touch /tmp/kodegend.pid
$ chmod 777 /tmp/kodegend.pid
$ kodegend start
# Should fail with permissions error
```

## Recommended Configuration

Add to config:
```toml
[daemon]
# Strict mode: additional security checks
strict_pid_file_checks = true

# Allowed PID file directories (whitelist)
allowed_pid_directories = [
    "/var/run/kodegend",
    "/run/kodegend",
]
```

## References
- CWE-362: Concurrent Execution using Shared Resource with Improper Synchronization (TOCTOU)
- CWE-59: Improper Link Resolution Before File Access ('Link Following')
- OpenBSD's pledge/unveil for symlink protection
- "Secure Programming Cookbook" - Chapter 1.5: Race Conditions
- OWASP: Path Traversal vulnerabilities
