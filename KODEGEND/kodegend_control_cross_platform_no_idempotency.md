# Cross-Platform: No Idempotency Guarantees

## Location
All platform implementations - `start_daemon()` and `stop_daemon()` functions

## Issue Type
Logical Error / API Design Flaw

## Severity
Medium

## Description
The behavior of `start_daemon()` when the service is already running, and `stop_daemon()` when the service is already stopped, is **undefined** and **inconsistent across platforms**. The public API doesn't document expected behavior, and implementations vary.

## Current Behavior by Platform

### Linux: systemctl start (already running)
```bash
$ systemctl start kodegend.service  # Already running
# Exit code: 0 (success)
# Behavior: Idempotent, no error
```

### Linux: systemctl stop (already stopped)
```bash
$ systemctl stop kodegend.service  # Already stopped
# Exit code: 0 (success)  
# Behavior: Idempotent, no error
```

### macOS: launchctl kickstart (already running)
```bash
$ launchctl kickstart ai.kodegen.kodegend  # Already running
# Behavior: Depends on flags (-k flag kills first)
# Without -k: May return error OR succeed depending on version
```

### macOS: launchctl bootout (not running)
```bash
$ launchctl bootout system /Library/LaunchDaemons/kodegend.plist  # Not loaded
# Exit code: Non-zero
# Behavior: Returns error
```

### Windows: StartServiceW (already running)
```cpp
StartServiceW(hService, 0, NULL);  // Service already running
// Returns: FALSE
// GetLastError(): ERROR_SERVICE_ALREADY_RUNNING (1056)
```
**Current code**: Returns error `"Failed to start service"`

### Windows: ControlService STOP (not running)
```cpp
ControlService(hService, SERVICE_CONTROL_STOP, &status);  // Not running
// Returns: FALSE
// GetLastError(): ERROR_SERVICE_NOT_ACTIVE (1062)
```
**Current code**: Returns error `"Failed to stop service"`

## The Problem

### Inconsistent Behavior
```rust
// On Linux:
start_daemon()?;  // Service already running
// Result: Ok(()) ✓

// On Windows:
start_daemon()?;  // Service already running
// Result: Err("Failed to start service") ❌

// User code breaks on Windows but works on Linux!
```

### No Documentation
The public API has no documentation about expected behavior:
```rust
/// Start the daemon service
pub fn start_daemon() -> Result<()> {
    // What happens if already running? Not documented!
}
```

### Forces Error Handling in Caller
```rust
// Every caller must handle idempotency manually:
match start_daemon() {
    Ok(()) => println!("Started"),
    Err(e) => {
        // Is this "already running" or a real error?
        // Have to parse error message - fragile!
        if e.to_string().contains("already running") {
            println!("Already running, OK");
        } else {
            return Err(e);
        }
    }
}
```

## Real-World Impact

### Issue 1: init scripts break
```bash
#!/bin/bash
kodegend start  # Idempotent on Linux
kodegend start  # Fails on Windows
```

### Issue 2: Retry logic complicated
```rust
// Want to ensure service is running:
for attempt in 0..3 {
    match start_daemon() {
        Ok(()) => break,
        Err(e) => {
            // Is this a transient error (retry) or already running (done)?
            // Can't tell!
        }
    }
}
```

### Issue 3: Healthcheck endpoints inconsistent
```rust
// HTTP endpoint: POST /daemon/start
async fn start_handler() -> Result<Response> {
    start_daemon()?;  // Fails if already running on Windows!
    Ok(Response::new("Started"))
}
```

## Recommendation

### Make operations idempotent by default:

```rust
/// Start the daemon service
///
/// This operation is idempotent - if the service is already running,
/// this function returns Ok(()) without error.
pub fn start_daemon() -> Result<()> {
    // Platform-specific implementation should:
    // 1. Check if already running
    // 2. If running, return Ok(())
    // 3. If not running, start it
    // 4. Wait for it to reach running state
    // 5. Return Ok(()) or detailed error
    
    platform::start_daemon()
}

/// Stop the daemon service
///
/// This operation is idempotent - if the service is already stopped,
/// this function returns Ok(()) without error.
pub fn stop_daemon() -> Result<()> {
    // Platform-specific implementation should:
    // 1. Check if already stopped
    // 2. If stopped, return Ok(())
    // 3. If running, stop it
    // 4. Wait for it to reach stopped state
    // 5. Return Ok(()) or detailed error
    
    platform::stop_daemon()
}
```

### Update Linux implementation:
```rust
pub fn start_daemon() -> Result<()> {
    // Check if already running
    if check_status()? {
        return Ok(());  // Already running, idempotent
    }
    
    // Not running, start it
    let output = Command::new("systemctl")
        .args(&args)
        .output()
        .context("Failed to execute systemctl start")?;
    
    if !output.status.success() {
        anyhow::bail!(
            "Failed to start daemon: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}
```

### Update Windows implementation:
```rust
pub fn start_daemon() -> Result<()> {
    let result = unsafe {
        StartServiceW(service.handle(), None)
    };

    if result.is_err() {
        let win_error = WindowsError::from_win32();
        let error_code = win_error.code().0 as u32;
        
        // ERROR_SERVICE_ALREADY_RUNNING is OK - idempotent
        if error_code == 1056 {
            return Ok(());
        }
        
        // Other errors are real problems
        anyhow::bail!(
            "Failed to start service: {} (code: 0x{:08X})",
            win_error.message(),
            error_code
        );
    }

    Ok(())
}

pub fn stop_daemon() -> Result<()> {
    let result = unsafe {
        ControlService(service.handle(), SERVICE_CONTROL_STOP, &mut status)
    };

    if result.is_err() {
        let win_error = WindowsError::from_win32();
        let error_code = win_error.code().0 as u32;
        
        // ERROR_SERVICE_NOT_ACTIVE is OK - idempotent
        if error_code == 1062 {
            return Ok(());
        }
        
        // Other errors are real problems
        anyhow::bail!(
            "Failed to stop service: {} (code: 0x{:08X})",
            win_error.message(),
            error_code
        );
    }

    Ok(())
}
```

### Update macOS implementation:
```rust
pub fn start_daemon() -> Result<()> {
    // Check if already running
    if check_status()? {
        return Ok(());  // Already running, idempotent
    }
    
    // Not running, start it
    // ... rest of implementation
}
```

## Benefits

1. **Predictable**: Same behavior across all platforms
2. **Simpler API**: Callers don't need error handling for idempotency
3. **Script-friendly**: Can call `start` multiple times safely
4. **Matches systemd**: Follows systemd's idempotent design
5. **Easier testing**: Can call start/stop in any order without errors

## Alternative: Explicit Force Parameter

If idempotency is not desired, make it explicit:
```rust
pub fn start_daemon(force: bool) -> Result<()> {
    if !force && check_status()? {
        anyhow::bail!("Service is already running (use force=true to restart)");
    }
    // ...
}
```

But this is more complex and doesn't match platform conventions.
