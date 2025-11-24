# HIGH: is_process_running() Masks All Errors

## Severity
**HIGH** - Causes incorrect behavior and difficult debugging

## Location
`packages/kodegend/src/main.rs:219-223`

## Issue Description
The `is_process_running()` function returns `false` for all error conditions, making it impossible to distinguish between "process not found" and actual errors like "permission denied". This leads to incorrect daemon behavior and masks production issues.

## Current Code
```rust
fn is_process_running(pid: i32) -> bool {
    match signal::kill(Pid::from_raw(pid), None) {
        Ok(_) => true,
        Err(_) => false,  // ← ALL ERRORS RETURN FALSE
    }
}
```

## Problem
The `kill(pid, None)` signal (also called signal 0 or null signal) checks if a process exists, but can fail for multiple reasons:

| Error | Meaning | Current Behavior | Correct Behavior |
|-------|---------|------------------|------------------|
| ESRCH | Process not found | Returns false ✓ | Return false ✓ |
| EPERM | Permission denied | Returns false ✗ | Return error or true |
| EINVAL | Invalid signal (shouldn't happen with None) | Returns false ✗ | Return error |

## Production Scenarios

### Scenario 1: Permission Denied Treated as "Not Running"
```bash
# Daemon running as root (PID 1234)
$ cat /var/run/kodegend.pid
1234

# User tries to check status
$ kodegend --status
Daemon is not running  # ← WRONG! Permission denied, but we say "not running"

# User starts new daemon
$ kodegend
Daemon started with PID 5678

# Now two daemons running!
```

### Scenario 2: Stale PID File with Permission Issue
```rust
// In main.rs:83-91
if let Some(pid) = read_pid_file(&config.daemon.pid_file)? {
    if is_process_running(pid) {  // Returns false due to EPERM
        // Not taken
    } else {
        warn!("Stale PID file found, removing it");  // ← WRONG MESSAGE
        let _ = fs::remove_file(&config.daemon.pid_file);
        // Removes valid PID file!
    }
}
```

### Scenario 3: SELinux/AppArmor Restrictions
```
System with SELinux enforcing mode:
- Process exists but security policy denies signal
- kill() returns EPERM
- Code thinks process not running
- Attempts to start duplicate daemon
```

### Scenario 4: PID Namespace Issues
```
In containerized environments:
- PID might exist in different namespace
- kill() returns ESRCH or EPERM
- Incorrect interpretation of process state
```

## Impact on Daemon Operations

### check_status Command
```rust
fn check_status(daemon_config: &DaemonConfig) -> Result<()> {
    match read_pid_file(&daemon_config.pid_file)? {
        Some(pid) => {
            if is_process_running(pid) {
                println!("Daemon is running with PID {}", pid);
            } else {
                println!("Daemon is not running (stale PID file found)");  // ← MISLEADING
            }
            Ok(())
        }
        // ...
    }
}
```

User sees "stale PID file" when actually they lack permissions.

### stop_daemon Command
```rust
fn stop_daemon(daemon_config: &DaemonConfig) -> Result<()> {
    match read_pid_file(&daemon_config.pid_file)? {
        Some(pid) => {
            if is_process_running(pid) {
                // Send SIGTERM
            } else {
                warn!("Daemon is not running (stale PID file)");  // ← WRONG
                let _ = fs::remove_file(&daemon_config.pid_file);
                Ok(())
            }
        }
        // ...
    }
}
```

Permission denied treated as "daemon not running", PID file incorrectly removed.

## Debugging Nightmare
When troubleshooting production issues:
```
Support: "Is the daemon running?"
User: "I ran 'kodegend --status', it says not running"
Support: "OK, try starting it"
User: "Getting 'address already in use' error"
Support: "But you said it's not running?"
User: "That's what kodegend --status told me!"

Root cause: Permission denied, but reported as "not running"
```

## Recommended Fix

### Option 1: Return Result Instead of Bool
```rust
fn is_process_running(pid: i32) -> Result<bool> {
    match signal::kill(Pid::from_raw(pid), None) {
        Ok(_) => Ok(true),
        Err(nix::errno::Errno::ESRCH) => Ok(false),  // Process not found
        Err(e) => Err(e).context(format!("Cannot determine if process {} is running", pid))?,
    }
}

// Update callers:
if is_process_running(pid)? {
    // Process is running
} else {
    // Process definitely not running
}
// If error, bubbles up to caller for proper handling
```

### Option 2: Return Enum for Clarity
```rust
enum ProcessStatus {
    Running,
    NotRunning,
    PermissionDenied,
    Unknown(nix::errno::Errno),
}

fn check_process_status(pid: i32) -> ProcessStatus {
    match signal::kill(Pid::from_raw(pid), None) {
        Ok(_) => ProcessStatus::Running,
        Err(nix::errno::Errno::ESRCH) => ProcessStatus::NotRunning,
        Err(nix::errno::Errno::EPERM) => ProcessStatus::PermissionDenied,
        Err(e) => ProcessStatus::Unknown(e),
    }
}

// Usage:
match check_process_status(pid) {
    ProcessStatus::Running => {
        anyhow::bail!("Daemon is already running with PID {}", pid);
    }
    ProcessStatus::NotRunning => {
        warn!("Stale PID file found, removing it");
        fs::remove_file(&config.daemon.pid_file)?;
    }
    ProcessStatus::PermissionDenied => {
        warn!("Cannot check if daemon is running (permission denied). PID: {}", pid);
        warn!("Daemon may be running with different permissions. Use 'ps' to verify.");
        // Don't remove PID file!
    }
    ProcessStatus::Unknown(e) => {
        anyhow::bail!("Error checking process status: {}", e);
    }
}
```

### Option 3: Enhanced Bool with Logging
```rust
fn is_process_running(pid: i32) -> bool {
    match signal::kill(Pid::from_raw(pid), None) {
        Ok(_) => {
            debug!("Process {} is running", pid);
            true
        }
        Err(nix::errno::Errno::ESRCH) => {
            debug!("Process {} not found", pid);
            false
        }
        Err(nix::errno::Errno::EPERM) => {
            warn!("Permission denied checking process {} - assuming running", pid);
            true  // ← Conservative: assume running if we can't check
        }
        Err(e) => {
            warn!("Error checking process {}: {} - assuming running", pid, e);
            true  // ← Conservative: assume running on unknown errors
        }
    }
}
```

**Option 2 (Enum) is recommended** for maximum clarity and correct handling.

## Conservative vs Aggressive Interpretation

When unsure if process is running:

**Conservative (recommended for daemon):**
- If can't determine (EPERM), assume running
- Prevents starting duplicate daemons
- User must manually resolve permission issues

**Aggressive (current behavior):**
- If can't determine (EPERM), assume not running
- Causes duplicate daemon starts
- Silently removes valid PID files

Production daemons should be **conservative** to avoid resource conflicts.

## Testing Requirements
1. Test with process running, normal permissions → should return Running
2. Test with process not running → should return NotRunning
3. Test with process running as different user → should handle PermissionDenied
4. Test with invalid PID → should handle appropriately
5. Test in container with PID namespace → verify behavior
6. Test with SELinux enforcing → handle security denials

## Error Messages
Update error messages for clarity:

```rust
// Instead of:
println!("Daemon is not running (stale PID file found)");

// Use:
match check_process_status(pid) {
    ProcessStatus::NotRunning => 
        println!("Daemon is not running (PID {} not found)", pid),
    ProcessStatus::PermissionDenied =>
        println!("Cannot verify daemon status (permission denied for PID {})", pid),
    // ...
}
```

## Related Issues
- PID validation (task file 01) should be done before calling this function
- Restart verification (task file 04) needs accurate process status
- TOCTOU race (task file 02) is partly caused by unreliable status check

## Documentation
Add to troubleshooting docs:

```markdown
### "Permission Denied" when checking daemon status

If you see permission errors, the daemon may be running as a different user:

1. Check with ps: `ps aux | grep kodegend`
2. Check PID file: `cat /var/run/kodegend.pid`
3. Verify process: `sudo kodegend --status`
4. Stop safely: `sudo kodegend --stop`
```

## Performance Considerations
- `kill(pid, 0)` is very fast (syscall, no actual signal sent)
- Adding error handling adds negligible overhead
- Logging should use debug! or trace! to avoid log spam
