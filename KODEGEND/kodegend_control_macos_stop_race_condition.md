# macOS: Race Condition in stop_daemon()

## Location
`packages/kodegend/src/control/macos_control.rs:81-112`

## Issue Type
Race Condition / Data Corruption Risk

## Severity
High

## Description
The `stop_daemon()` function has a dangerous race condition between sending SIGTERM and calling bootout. If the daemon doesn't stop within 500ms, bootout will forcefully remove the service while it may still be running, potentially causing data corruption or incomplete cleanup.

## Current Code
```rust
pub fn stop_daemon() -> Result<()> {
    // Try to kill the service first (graceful shutdown)
    let _ = Command::new("launchctl")
        .args(["kill", "SIGTERM", SERVICE_LABEL])
        .output();

    // Give it a moment to shutdown gracefully
    std::thread::sleep(Duration::from_millis(500));

    // Then bootout
    let output = Command::new("launchctl")
        .args(["bootout", "system", PLIST_PATH])
        .output()
        .context("Failed to execute launchctl bootout")?;
    // ...
}
```

## The Race Condition

### Sequence of Events:
1. **T+0ms**: Send SIGTERM to daemon
2. **T+500ms**: Sleep expires, proceed to bootout
3. **T+500ms**: Call `launchctl bootout` (forcefully removes service)
4. **T+700ms**: Daemon finishes cleanup and tries to exit gracefully
5. **Result**: Daemon's cleanup code may fail because service registration is already removed

### What Can Go Wrong:

1. **Data Corruption**
   - Daemon writing to database when bootout happens
   - Database connection forcefully closed mid-transaction
   - Partial writes, corrupted indexes

2. **Resource Leaks**
   - Daemon cleanup code tries to close file handles after bootout
   - May leave temp files, sockets, or lock files behind
   - MCP server connections not properly closed

3. **Inconsistent State**
   - Daemon may be in the middle of updating state files
   - Service registry removed before daemon can deregister properly
   - Child processes may be orphaned

4. **Signal Handling Issues**
   - If daemon has signal handlers for graceful shutdown, bootout prevents them from completing
   - May interrupt checkpoint writing, log flushing, etc.

## Impact
- **Critical**: Potential data corruption in daemon state files
- **High**: Resource leaks (sockets, file handles, temp files)
- **Medium**: Incomplete cleanup of child processes
- **Medium**: Logs may not be flushed, losing diagnostic info

## Recommendation

### Must verify daemon stopped before bootout:
```rust
pub fn stop_daemon() -> Result<()> {
    // Try to kill the service first (graceful shutdown)
    let _ = Command::new("launchctl")
        .args(["kill", "SIGTERM", SERVICE_LABEL])
        .output();

    // Wait for daemon to actually stop (poll with timeout)
    let start = std::time::Instant::now();
    let timeout = Duration::from_secs(10);  // Reasonable timeout for graceful shutdown
    
    loop {
        // Check if service is still running
        match check_status() {
            Ok(false) => break,  // Service stopped successfully
            Ok(true) => {
                // Still running, check timeout
                if start.elapsed() > timeout {
                    log::warn!(
                        "Daemon did not stop gracefully within {}s, sending SIGKILL",
                        timeout.as_secs()
                    );
                    
                    // Force kill before bootout
                    let _ = Command::new("launchctl")
                        .args(["kill", "SIGKILL", SERVICE_LABEL])
                        .output();
                    
                    // Wait a bit more for SIGKILL
                    std::thread::sleep(Duration::from_millis(100));
                    break;
                }
                
                // Small sleep between polls
                std::thread::sleep(Duration::from_millis(100));
            }
            Err(e) => {
                log::warn!("Error checking daemon status during stop: {}", e);
                break;
            }
        }
    }

    // Now safe to bootout - daemon is confirmed stopped
    let output = Command::new("launchctl")
        .args(["bootout", "system", PLIST_PATH])
        .output()
        .context("Failed to execute launchctl bootout")?;
    
    // ...
}
```

### Alternative: Use launchctl's built-in timeout
Some versions of launchctl may support timeout options - investigate if `launchctl kill -9` or similar has timeout semantics.
