# macOS: Incorrect Error Recovery in restart_daemon()

## Location
`packages/kodegend/src/control/macos_control.rs:117-134`

## Issue Type
Logical Error / Error Masking

## Severity
Medium

## Description
The `restart_daemon()` function uses `launchctl kickstart -k` to restart the service, and if that fails, falls back to manual stop+start. However, this error recovery is dangerous because `kickstart -k` can fail **partway through**, leaving the service in an inconsistent state. The fallback can then mask the real error or make things worse.

## Current Code
```rust
pub fn restart_daemon() -> Result<()> {
    // macOS launchctl doesn't have a direct restart command
    // Use kickstart with -k (kill) flag which restarts the service
    
    let output = Command::new("launchctl")
        .args(["kickstart", "-k", SERVICE_LABEL])
        .output()
        .context("Failed to execute launchctl kickstart -k")?;

    if !output.status.success() {
        // Fallback: manual stop + start
        stop_daemon()?;
        std::thread::sleep(Duration::from_secs(1));
        start_daemon()?;
    }

    Ok(())
}
```

## The Problem

### kickstart -k Can Fail Partway Through
The `-k` flag tells launchctl to:
1. Kill the running service (sends signal)
2. Start the service again

If it fails between these steps, the service is killed but not restarted.

### Failure Scenarios

#### Scenario 1: Service killed but restart fails
```
T+0ms:   kickstart -k executes
T+10ms:  Service receives kill signal
T+50ms:  Service is dead
T+100ms: launchctl tries to restart
T+100ms: Restart fails (plist error, missing binary, etc.)
T+100ms: kickstart -k returns error
T+101ms: Code enters fallback: stop_daemon()
T+101ms: stop_daemon() tries to stop already-dead service
Result:  Confusing error messages, service is dead
```

#### Scenario 2: Permission changes mid-operation
```
T+0ms:   kickstart -k starts (running as root)
T+10ms:  Service killed successfully
T+20ms:  User permissions revoked (sudo timeout, security policy)
T+30ms:  Restart fails - permission denied
T+30ms:  Fallback tries stop_daemon()
T+30ms:  stop_daemon() also fails - permission denied
Result:  Original error masked by fallback error
```

#### Scenario 3: Service modified during operation
```
T+0ms:   kickstart -k starts
T+10ms:  Service killed
T+15ms:  Another process updates plist file
T+20ms:  Restart picks up invalid plist
T+20ms:  Restart fails
T+21ms:  Fallback tries to stop (service already dead)
T+22ms:  Fallback tries to start with invalid plist
Result:  Same error, but now we've lost the original error context
```

## Impact

### 1. Error Masking
The original error from `kickstart -k` is lost when the fallback runs. The user sees the fallback error, not the root cause.

```
Real error:   "Service binary missing at /usr/local/bin/kodegend"
Fallback error: "Failed to stop daemon: service not found"
User sees:    "Failed to stop daemon" ← Not helpful!
```

### 2. Inconsistent State
If `kickstart -k` kills the service but fails to restart, and the fallback also fails, the service ends up stopped when the user expected it to restart.

### 3. Double Operations
If `kickstart -k` succeeds but returns non-zero for some other reason (e.g., warning message), the fallback unnecessarily stops and starts the service again:
```
T+0ms:   kickstart -k succeeds (service restarted)
T+100ms: kickstart returns exit 1 (warning: service was already running)
T+101ms: Fallback runs: stop + start
T+2000ms: Service restarted TWICE
```

## Recommendation

### Option 1: Don't use fallback - check error first
```rust
pub fn restart_daemon() -> Result<()> {
    let output = Command::new("launchctl")
        .args(["kickstart", "-k", SERVICE_LABEL])
        .output()
        .context("Failed to execute launchctl kickstart -k")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        // Only fallback for specific errors
        if stderr.contains("Could not find service") || stderr.contains("not loaded") {
            // Service not loaded - use manual start
            log::warn!("Service not loaded, using manual start instead of restart");
            return start_daemon();
        }
        
        // For other errors, fail with the original error
        anyhow::bail!(
            "Failed to restart daemon with kickstart -k: {}",
            stderr
        );
    }

    Ok(())
}
```

### Option 2: Check service state before fallback
```rust
pub fn restart_daemon() -> Result<()> {
    let output = Command::new("launchctl")
        .args(["kickstart", "-k", SERVICE_LABEL])
        .output()
        .context("Failed to execute launchctl kickstart -k")?;

    if !output.status.success() {
        // Check current state to see what happened
        match check_status() {
            Ok(true) => {
                // Service is still running - kickstart failed before killing
                // Try manual restart
                log::warn!("kickstart -k failed but service is running, trying manual restart");
                stop_daemon()?;
                std::thread::sleep(Duration::from_secs(1));
                start_daemon()?;
            }
            Ok(false) => {
                // Service is stopped - kickstart killed it but didn't restart
                // Just start it
                log::warn!("kickstart -k killed service but didn't restart, starting now");
                start_daemon()?;
            }
            Err(e) => {
                // Can't determine state - fail with both errors
                anyhow::bail!(
                    "Failed to restart daemon: kickstart -k failed: {}; status check also failed: {}",
                    String::from_utf8_lossy(&output.stderr),
                    e
                );
            }
        }
    }

    Ok(())
}
```

### Option 3: Don't use kickstart -k at all
```rust
pub fn restart_daemon() -> Result<()> {
    // Always use manual stop + start for predictability
    stop_daemon()?;
    // No sleep needed if stop_daemon() waits for service to stop
    start_daemon()?;
    Ok(())
}
```
This is simpler and more reliable, though potentially slower.

## Benefits of Fix

1. **Clearer Errors**: Users see the actual failure reason
2. **Correct State**: Service state matches user expectation
3. **No Double-Restart**: Service only restarted once
4. **Better Debugging**: Error messages reflect actual problem

## Related Issues
- macOS stop_daemon() doesn't wait for service to stop (compounds the problem)
- No verification after operations complete
