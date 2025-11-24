# MEDIUM: PID File Permissions Not Explicitly Set

## Severity
**MEDIUM** - Security concern in multi-user environments

## Location
`packages/kodegend/src/main.rs:78-82` and likely in `daemon.rs`

## Issue Description
The daemon does not explicitly set permissions when creating the PID file. This leaves security to the default umask, which may allow unauthorized users to read or modify the PID file in multi-user environments.

## Current Code
```rust
// Ensure PID directory exists
if let Some(parent) = config.daemon.pid_file.parent() {
    fs::create_dir_all(parent)  // ← No explicit permissions
        .with_context(|| format!("Failed to create PID directory: {:?}", parent))?;
}

// Later, in Daemonize::execute() (daemon.rs):
// PID file created, but permissions not specified
```

## Problem
Without explicit permission settings:
- Permissions determined by process umask (typically 022 or 002)
- May allow world-readable PID files (644)
- May allow group-writable PID files (664)
- Varies between systems and configurations

## Security Risks

### Risk 1: Information Disclosure
```bash
# PID file created with default permissions (644)
$ ls -l /var/run/kodegend.pid
-rw-r--r-- 1 kodegen kodegen 5 Jan 15 10:30 /var/run/kodegend.pid

# Any user can read PID
$ cat /var/run/kodegend.pid
1234

# Now anyone knows daemon PID
$ ps aux | grep 1234
kodegen  1234  ... /usr/bin/kodegend
```

While PID is somewhat public information (`ps aux`), explicit PID file disclosure:
- Makes targeted attacks easier
- Reveals daemon is running
- Shows exact process ID for timing attacks

### Risk 2: Unauthorized Modification (Worse)
```bash
# PID file created with group-writable permissions (664)
$ ls -l /var/run/kodegend.pid  
-rw-rw-r-- 1 kodegen kodegen 5 Jan 15 10:30 /var/run/kodegend.pid

# Attacker in same group modifies PID file
$ echo "9999" > /var/run/kodegend.pid  # PID of attacker's process

# Admin tries to stop daemon
$ kodegend --stop
Stopping daemon with PID 9999
SIGTERM sent to daemon

# Attacker's process receives SIGTERM instead of real daemon!
```

### Risk 3: Symlink Attack
```bash
# Attacker creates symlink before daemon starts
$ ln -s /etc/passwd /var/run/kodegend.pid

# Daemon starts, overwrites /etc/passwd with PID
# System becomes unusable
```

### Risk 4: Race Condition Exploitation
```bash
# Attacker watches for daemon restart
$ while true; do
    if [ ! -f /var/run/kodegend.pid ]; then
        echo "MALICIOUS" > /var/run/kodegend.pid
    fi
    sleep 0.01
done

# During daemon restart:
# 1. Old daemon removes PID file
# 2. Attacker writes malicious content
# 3. New daemon reads malicious content
```

## Best Practices

### POSIX/LSB Standards
- PID files should be readable only by owner: **600** or **644**
- PID files should never be writable by group/world
- PID directory should be **755** (owner writable only)

### Industry Standards
```
systemd:     600 (owner read/write only)
nginx:       644 (world-readable, owner-writable)
apache:      644 (world-readable, owner-writable)
postgresql:  600 (owner-only)
redis:       644 (world-readable)
```

**Most secure**: 600 (owner read/write only)
**Common practice**: 644 (world-readable, owner-writable)

## Recommended Fix

### Option 1: Explicit Permissions (Recommended)
```rust
use std::fs::{File, OpenOptions};
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};

// When creating PID directory
if let Some(parent) = config.daemon.pid_file.parent() {
    fs::create_dir_all(parent)?;
    
    // Set directory permissions to 755 (rwxr-xr-x)
    let mut perms = fs::metadata(parent)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(parent, perms)?;
    
    info!("Created PID directory with permissions 755: {:?}", parent);
}

// When creating PID file (in Daemonize::execute or similar)
let pid_file = OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .mode(0o644)  // rw-r--r-- (owner write, all read)
    .open(&pid_file_path)?;

writeln!(&pid_file, "{}", process::id())?;
pid_file.sync_all()?;

info!("Created PID file with permissions 644: {:?}", pid_file_path);
```

### Option 2: Configurable Permissions
```rust
// In DaemonConfig:
pub struct DaemonConfig {
    pub pid_file: PathBuf,
    pub working_directory: PathBuf,
    
    #[serde(default = "default_pid_file_mode")]
    pub pid_file_mode: u32,  // Octal mode (e.g., 0o644)
    
    #[serde(default = "default_pid_dir_mode")]
    pub pid_dir_mode: u32,   // Octal mode (e.g., 0o755)
    
    // ... other fields ...
}

fn default_pid_file_mode() -> u32 { 0o644 }
fn default_pid_dir_mode() -> u32 { 0o755 }

// Usage:
let mut perms = fs::metadata(parent)?.permissions();
perms.set_mode(config.daemon.pid_dir_mode);
fs::set_permissions(parent, perms)?;
```

### Option 3: Strict Security (600)
```rust
// Most secure: only owner can read/write
let pid_file = OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .mode(0o600)  // rw------- (owner only)
    .open(&pid_file_path)?;
```

**Trade-off**: Non-root users can't check daemon status.

### Option 4: Use Symbolic Constants
```rust
use libc::{S_IRUSR, S_IWUSR, S_IRGRP, S_IROTH};

const PID_FILE_MODE: u32 = (S_IRUSR | S_IWUSR | S_IRGRP | S_IROTH) as u32; // 0644
const PID_DIR_MODE: u32 = 0o755;

let pid_file = OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .mode(PID_FILE_MODE)
    .open(&pid_file_path)?;
```

## Symlink Attack Prevention
In addition to permissions, prevent symlink attacks:

```rust
use std::os::unix::fs::MetadataExt;

// Before creating PID file, check if path exists and is regular file
if pid_file_path.exists() {
    let metadata = fs::symlink_metadata(&pid_file_path)?;
    
    if metadata.file_type().is_symlink() {
        anyhow::bail!(
            "PID file path {:?} is a symlink (possible attack), refusing to use",
            pid_file_path
        );
    }
    
    if !metadata.file_type().is_file() {
        anyhow::bail!(
            "PID file path {:?} exists but is not a regular file",
            pid_file_path
        );
    }
}

// Now safe to create/open
let pid_file = OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .mode(0o644)
    .open(&pid_file_path)?;
```

## Directory Permissions
The PID directory also needs correct permissions:

```bash
# Bad:
drwxrwxrwx  2 kodegen kodegen  4096 Jan 15 10:30 /var/run/kodegen/
# Anyone can create/delete files in this directory!

# Good:
drwxr-xr-x  2 kodegen kodegen  4096 Jan 15 10:30 /var/run/kodegen/
# Only owner can modify directory contents
```

## Configuration Example
```toml
[daemon]
pid_file = "/var/run/kodegend/kodegend.pid"
pid_file_mode = 0o644     # World-readable, owner-writable
pid_dir_mode = 0o755      # Standard directory permissions

# For maximum security:
# pid_file_mode = 0o600   # Owner-only (status command requires root)
```

## Platform Considerations

### Linux
- Default umask often 022 → files created as 644
- systemd sets umask 022 for services
- PID files in /var/run typically world-readable

### macOS
- Default umask often 022
- LaunchDaemons run with specific umask
- PID files in /var/run follow same pattern

### BSDs
- Similar to Linux
- Some use /var/run, others /var/db

## Security Checklist
- [ ] PID file created with explicit mode (644 or 600)
- [ ] PID directory created with explicit mode (755)
- [ ] Symlink attack prevention (check file type)
- [ ] Ownership verified (file owned by daemon user)
- [ ] Parent directory permissions secure (no world-write)
- [ ] File descriptor closed properly after writing
- [ ] Permissions logged for audit trail

## Testing Requirements
1. Verify PID file created with correct permissions
2. Verify PID directory created with correct permissions
3. Test symlink attack prevention
4. Test different umask values
5. Test multi-user scenarios
6. Verify permissions persist across restarts
7. Test permission denied handling

## Audit and Compliance
For security audits, log permission settings:

```rust
info!("PID file created: {:?}", pid_file_path);
info!("PID file permissions: {:o}", 0o644);
info!("PID file owner: {} (UID {})", user, uid);
```

This creates audit trail showing security was considered.

## Impact Assessment
- **High security environments**: CRITICAL (need 600 permissions)
- **Shared servers**: HIGH (prevent unauthorized modification)
- **Single-user systems**: LOW (default permissions acceptable)
- **Containerized deployments**: LOW (isolated environment)

## Related Issues
- TOCTOU race (task file 02) - combined with weak permissions increases risk
- Symlink attacks require both weak permissions and race conditions

## References
- OWASP: Insecure Temporary File (CWE-377)
- FHS: /var/run directory security
- systemd: File system permissions for services
- POSIX: File permission modes
- Security: Symlink attack prevention (CWE-59)

## Migration Path
If changing from default to explicit permissions:

```rust
// First deployment: warn if permissions wrong
let metadata = fs::metadata(&pid_file)?;
let mode = metadata.permissions().mode() & 0o777;
if mode != 0o644 {
    warn!("PID file has unexpected permissions: {:o}, expected 0644", mode);
}

// Later: enforce correct permissions
if mode != 0o644 {
    let mut perms = metadata.permissions();
    perms.set_mode(0o644);
    fs::set_permissions(&pid_file, perms)?;
    info!("Corrected PID file permissions to 0644");
}
```
