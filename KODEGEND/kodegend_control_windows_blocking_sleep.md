# Windows: Blocking Sleep in restart_daemon()

## Location
`packages/kodegend/src/control/windows_control.rs:166`

## Issue Type
Performance Bottleneck

## Severity
Medium

## Description
The `restart_daemon()` function uses `std::thread::sleep(Duration::from_secs(1))` as a workaround for the fact that `stop_daemon()` doesn't wait for the service to actually stop. This arbitrary 1-second delay is inefficient and doesn't guarantee correctness.

## Current Code
```rust
pub fn restart_daemon() -> Result<()> {
    // Stop the service
    stop_daemon()?;

    // Wait for service to fully stop
    std::thread::sleep(Duration::from_secs(1));  // ← Arbitrary 1-second block

    // Start the service
    start_daemon()?;

    Ok(())
}
```

## Problems

### 1. Arbitrary Timeout
- **1 second has no basis** in actual service behavior
- Some services stop in 50ms → waste 950ms
- Some services need 5s to stop → 1s is insufficient
- No relationship to the service's actual shutdown time

### 2. Not Guaranteed to Work
```
T+0ms:   stop_daemon() sends STOP signal
T+0ms:   stop_daemon() returns (service still stopping)
T+0ms:   sleep(1s) starts
T+1000ms: sleep ends
T+1000ms: start_daemon() called
T+1000ms: Service STILL stopping (needs 2s)
T+1000ms: start_daemon() fails - service hasn't stopped yet
```

### 3. Inefficient
```
T+0ms:   stop_daemon() sends STOP signal
T+50ms:  Service finishes stopping
T+50ms:  Still sleeping...
T+1000ms: Finally continue
Result:  Wasted 950ms doing nothing
```

### 4. Blocks Calling Thread
- Can't do any other work during the sleep
- If called from async context, blocks executor thread
- Makes CLI tools feel slow and unresponsive

## Impact
- **Performance**: restart_daemon() takes at minimum 1 second, even if service stops in 50ms
- **Reliability**: May still fail if service takes >1s to stop
- **User Experience**: CLI tools feel sluggish
- **Scalability**: Blocks thread that could be doing other work

## Recommendation

### Fix stop_daemon() to wait properly
The root cause is that `stop_daemon()` doesn't wait for the service to stop. Once that's fixed, the sleep can be removed entirely:

```rust
pub fn restart_daemon() -> Result<()> {
    // Stop the service (waits until fully stopped)
    stop_daemon()?;

    // Start the service (waits until fully started)
    start_daemon()?;

    Ok(())
}
```

### If stop_daemon() can't be changed, poll instead
```rust
pub fn restart_daemon() -> Result<()> {
    // Stop the service
    stop_daemon()?;

    // Poll until service is actually stopped
    let timeout = Duration::from_secs(10);
    let start = std::time::Instant::now();
    
    loop {
        match check_status() {
            Ok(false) => break,  // Service stopped
            Ok(true) => {
                if start.elapsed() > timeout {
                    anyhow::bail!("Timeout waiting for service to stop during restart");
                }
                std::thread::sleep(Duration::from_millis(100));
            }
            Err(e) => anyhow::bail!("Error checking service status: {}", e),
        }
    }

    // Start the service
    start_daemon()?;

    Ok(())
}
```

## Performance Comparison

### Current Implementation
- **Best case** (service stops in 50ms): 1000ms total (950ms wasted)
- **Typical case** (service stops in 200ms): 1000ms total (800ms wasted)
- **Worst case** (service takes 2s): FAILS ❌

### With Polling (100ms interval)
- **Best case** (service stops in 50ms): 100ms total
- **Typical case** (service stops in 200ms): 200ms total
- **Worst case** (service takes 2s): 2000ms total, but WORKS ✓

### With Fixed stop_daemon()
- **Best case** (service stops in 50ms): 50ms total
- **Typical case** (service stops in 200ms): 200ms total
- **Worst case** (service takes 2s): 2000ms total, but WORKS ✓

## Related Issues
- Similar blocking sleep in macOS implementation
- Root cause is stop_daemon() not waiting for service to stop
- Part of broader pattern of not verifying async operations complete
