# Cross-Platform: No Verification After Operations Complete

## Location
All platform implementations - all functions

## Issue Type
Logical Error / Correctness Issue

## Severity
High

## Description
After `start_daemon()` returns `Ok(())`, there is no guarantee that the service is actually running. Similarly, after `stop_daemon()` returns `Ok(())`, there is no guarantee the service is actually stopped. The functions only verify that the **command was accepted**, not that the **operation completed successfully**.

## The Gap Between Command Success and Operation Success

### start_daemon() Returns Too Early

**What the code verifies:**
- ✓ systemctl/launchctl/StartServiceW accepted the start command
- ✓ No immediate errors (permissions, service not found, etc.)

**What the code DOESN'T verify:**
- ✗ Service actually started running
- ✗ Service didn't crash immediately after starting
- ✗ Service passed health checks
- ✗ Service is ready to accept connections

### stop_daemon() Returns Too Early

**What the code verifies:**
- ✓ systemctl/launchctl/ControlService accepted the stop command

**What the code DOESN'T verify:**
- ✗ Service actually stopped
- ✗ Service finished cleanup (flushed logs, closed connections)
- ✗ Service released resources (ports, file locks)

## Real-World Failure Scenarios

### Scenario 1: Service crashes on startup
```rust
start_daemon()?;  // Returns Ok(())
println!("Service started!");

// Meanwhile:
// T+0ms:   systemctl starts service
// T+10ms:  Service binary loads
// T+50ms:  Service reads config file
// T+100ms: Config has invalid JSON
// T+100ms: Service crashes with exit code 1
// T+100ms: systemd sets service to "failed" state

// User thinks service is running, but it crashed 100ms ago!
```

### Scenario 2: Port conflict
```rust
start_daemon()?;  // Returns Ok(())

// Meanwhile:
// T+0ms:   Service starts
// T+50ms:  Service tries to bind to port 8080
// T+50ms:  Port already in use by another process
// T+50ms:  Service exits with error

// Function returned success, but service failed immediately
```

### Scenario 3: Dependency missing
```rust
start_daemon()?;  // Returns Ok(())

// Meanwhile:
// T+0ms:   Service starts
// T+100ms: Service tries to connect to database
// T+100ms: Database connection fails
// T+100ms: Service crashes

// Caller thinks everything is fine
```

### Scenario 4: Stop doesn't finish
```rust
stop_daemon()?;   // Returns Ok(())
// Service still shutting down...

start_daemon()?;  // Tries to start
// FAILS: Port still held by old instance
```

## Impact on Callers

### Issue 1: False Success Reports
```rust
match start_daemon() {
    Ok(()) => println!("✓ Daemon started successfully"),
    Err(e) => println!("✗ Failed to start: {}", e),
}
// Prints success even if daemon crashes 1 second later
```

### Issue 2: Integration Tests Flake
```rust
#[test]
fn test_daemon_api() {
    start_daemon().unwrap();
    
    // Try to connect to daemon API
    let response = http::get("http://localhost:8080/health").unwrap();
    //             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ FLAKES!
    // Sometimes passes (daemon started fast)
    // Sometimes fails (daemon still starting)
    
    assert_eq!(response.status(), 200);
}
```

### Issue 3: Race Conditions in Scripts
```bash
#!/bin/bash
kodegend start
curl http://localhost:8080/health  # May fail - daemon not ready yet
```

### Issue 4: Incorrect Assumptions
```rust
// Deployment code
stop_daemon()?;
// Assume service stopped, delete old binary
fs::remove_file("/usr/local/bin/kodegend")?;
// RACE: Service still running, may crash when binary deleted!

// Copy new binary
fs::copy("kodegend-new", "/usr/local/bin/kodegend")?;
start_daemon()?;
```

## What Other Tools Do

### systemd (Linux)
```bash
# systemctl start waits until service reports ready (if Type=notify)
systemctl start myservice
echo $?  # 0 only if service reached active state
```

### Docker
```bash
docker start mycontainer
docker wait mycontainer  # Waits for container to be ready
```

### Kubernetes
```yaml
readinessProbe:
  httpGet:
    path: /health
    port: 8080
# Pod not marked "Ready" until health check passes
```

## Recommendation

### Verify service state before returning:

```rust
pub fn start_daemon() -> Result<()> {
    // Send start command
    platform::send_start_command()?;
    
    // Wait for service to reach running state
    let timeout = Duration::from_secs(30);
    let start_time = Instant::now();
    
    loop {
        match check_status()? {
            true => {
                // Service is running
                return Ok(());
            }
            false => {
                // Service not running yet, check timeout
                if start_time.elapsed() > timeout {
                    anyhow::bail!(
                        "Timeout waiting for service to start ({}s). \
                         Service may have crashed immediately after starting. \
                         Check logs for details.",
                        timeout.as_secs()
                    );
                }
                
                // Wait before next check
                std::thread::sleep(Duration::from_millis(100));
            }
        }
    }
}

pub fn stop_daemon() -> Result<()> {
    // Send stop command
    platform::send_stop_command()?;
    
    // Wait for service to reach stopped state
    let timeout = Duration::from_secs(30);
    let start_time = Instant::now();
    
    loop {
        match check_status()? {
            false => {
                // Service stopped
                return Ok(());
            }
            true => {
                // Service still running, check timeout
                if start_time.elapsed() > timeout {
                    anyhow::bail!(
                        "Timeout waiting for service to stop ({}s). \
                         Service may be hung. Consider manual intervention.",
                        timeout.as_secs()
                    );
                }
                
                // Wait before next check
                std::thread::sleep(Duration::from_millis(100));
            }
        }
    }
}
```

### Optional: Add health check support
```rust
pub fn start_daemon_with_health_check<F>(health_check: F) -> Result<()>
where
    F: Fn() -> Result<bool>,
{
    // Start the daemon
    platform::send_start_command()?;
    
    // Wait for service to be running
    wait_for_running_state()?;
    
    // Wait for service to pass health check
    let timeout = Duration::from_secs(30);
    let start_time = Instant::now();
    
    loop {
        match health_check() {
            Ok(true) => return Ok(()),
            Ok(false) | Err(_) => {
                if start_time.elapsed() > timeout {
                    anyhow::bail!("Service started but failed health check");
                }
                std::thread::sleep(Duration::from_secs(1));
            }
        }
    }
}

// Usage:
start_daemon_with_health_check(|| {
    reqwest::get("http://localhost:8080/health")
        .map(|r| r.status().is_success())
})?;
```

## Benefits

1. **Correctness**: Guarantees service is in expected state
2. **Reliability**: Catches immediate crashes and failures
3. **Better Errors**: Can report specific failure reasons (timeout, crashed, etc.)
4. **Easier Testing**: Tests don't need to add their own polling loops
5. **Better UX**: CLI tools can report "Starting..." with spinner, then "Started ✓"

## Performance Impact
- **Additional latency**: 100-500ms to verify state (acceptable for daemon operations)
- **Better correctness**: Worth the latency to ensure operations actually complete
- **Timeout protection**: Prevents hanging forever if service is broken

## Related Issues
- Windows start_daemon() doesn't wait for SERVICE_RUNNING
- Windows stop_daemon() doesn't wait for SERVICE_STOPPED
- macOS has blocking sleeps instead of proper polling
- No timeout protection on command execution
