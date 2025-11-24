# macOS: Blocking Sleep in Critical Path

## Location
- `packages/kodegend/src/control/macos_control.rs:88` (stop_daemon)
- `packages/kodegend/src/control/macos_control.rs:129` (restart_daemon)

## Issue Type
Performance Bottleneck

## Severity
Medium

## Description
The macOS control implementation uses blocking `std::thread::sleep()` calls in the critical path, causing unnecessary delays and inefficiency.

## Occurrences

### 1. stop_daemon() - Line 88
```rust
pub fn stop_daemon() -> Result<()> {
    // Try to kill the service first (graceful shutdown)
    let _ = Command::new("launchctl")
        .args(["kill", "SIGTERM", SERVICE_LABEL])
        .output();

    // Give it a moment to shutdown gracefully
    std::thread::sleep(Duration::from_millis(500));  // ← Blocks for 500ms

    // Then bootout
    let output = Command::new("launchctl")
        .args(["bootout", "system", PLIST_PATH])
        .output()
        .context("Failed to execute launchctl bootout")?;
    // ...
}
```

### 2. restart_daemon() - Line 129
```rust
pub fn restart_daemon() -> Result<()> {
    // ...
    if !output.status.success() {
        // Fallback: manual stop + start
        stop_daemon()?;
        std::thread::sleep(Duration::from_secs(1));  // ← Blocks for 1 second
        start_daemon()?;
    }
    // ...
}
```

## Problems

### 1. Arbitrary Timeouts
- 500ms and 1s are arbitrary values with no basis in actual service shutdown time
- Some services stop in 50ms, others need 5+ seconds
- No relationship to actual daemon behavior

### 2. Inefficiency
- **Too long**: If service stops in 100ms, we waste 400ms (or 900ms) waiting
- **Too short**: If service needs 2s to stop, the 500ms/1s is insufficient
- No adaptive behavior

### 3. Blocks Calling Thread
- Can't do any other work during the sleep
- If this is called from async context, blocks the entire executor thread
- Hurts responsiveness of CLI tools

### 4. No Verification
- After sleeping, code doesn't verify the service actually stopped
- Could sleep for 500ms and service is still running
- Could sleep for 500ms and service stopped in 50ms

## Impact
- Daemon control operations take 500ms - 1.5s longer than necessary
- Wasted CPU time if calling thread could be doing other work
- Can cause issues in automated scripts with tight timing
- Goes against "blazing-fast performance" goal from CLAUDE.md

## Recommendation

### Option 1: Poll with timeout
```rust
pub fn stop_daemon() -> Result<()> {
    // Try to kill the service first (graceful shutdown)
    let _ = Command::new("launchctl")
        .args(["kill", "SIGTERM", SERVICE_LABEL])
        .output();

    // Poll for service to stop (max 5 seconds)
    let start = std::time::Instant::now();
    let timeout = Duration::from_secs(5);
    
    loop {
        // Check if service stopped
        if !check_status()? {
            break;  // Service stopped successfully
        }
        
        // Check timeout
        if start.elapsed() > timeout {
            log::warn!("Service didn't stop gracefully within timeout, forcing bootout");
            break;
        }
        
        // Small sleep between polls
        std::thread::sleep(Duration::from_millis(50));
    }

    // Then bootout to clean up
    let output = Command::new("launchctl")
        .args(["bootout", "system", PLIST_PATH])
        .output()
        .context("Failed to execute launchctl bootout")?;
    // ...
}
```

### Option 2: Exponential backoff
```rust
// Start with 50ms, double up to 500ms
let mut delay = Duration::from_millis(50);
let max_delay = Duration::from_millis(500);
let timeout = Duration::from_secs(5);
let start = Instant::now();

while check_status()? && start.elapsed() < timeout {
    std::thread::sleep(delay);
    delay = std::cmp::min(delay * 2, max_delay);
}
```

### Option 3: Signal-based wait (if possible)
```rust
// If we can get the PID, wait for it to exit
if let Some(pid) = get_service_pid()? {
    // Use waitpid or kill(pid, 0) to check if process exists
    wait_for_process_exit(pid, Duration::from_secs(5))?;
}
```
