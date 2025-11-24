# HIGH: ServiceManager Shutdown Has No Timeout

## Severity
**HIGH - SERVICE HANG**

## Location
`packages/kodegend/src/platform/windows_service.rs:119-143`

## Issue Description
The `run_service()` function calls `service_manager.shutdown()` without any timeout mechanism. If the ServiceManager or its managed MCP servers hang during shutdown, the Windows service will hang indefinitely during stop.

### Problems
1. **No timeout**: Line 128 calls `shutdown()` with no time limit
2. **SCM wait_hint ignored**: Line 123 sets 5-second wait_hint, but no enforcement
3. **Hung service**: If MCP servers don't respond, service never stops
4. **Forced termination**: SCM eventually kills process, not graceful
5. **Resource leaks**: Hung servers may leave resources locked

### Code Analysis
```rust
// Report service is stopping
report_service_status(
    &status_handle,
    ServiceState::StopPending,
    Duration::from_secs(5),  // ← 5 second wait_hint
    0,
)?;

// Perform graceful shutdown of all MCP servers
if let Err(e) = service_manager.shutdown() {  // ← No timeout!
    error!("Error during ServiceManager shutdown: {}", e);
}
```

### Real-World Scenario
1. User stops kodegend service via SCM
2. ServiceManager tries to stop MCP servers
3. One MCP server is hung/deadlocked
4. `shutdown()` blocks waiting for server to exit
5. Service remains in StopPending state indefinitely
6. After ~30 seconds, SCM force-kills entire process
7. Resources not cleaned up properly

## Recommended Fix

### Option 1: Timeout with tokio
```rust
use tokio::time::{timeout, Duration};

// Create async context for timeout
let shutdown_result = tokio::task::block_in_place(|| {
    tokio::runtime::Handle::current().block_on(async {
        timeout(
            Duration::from_secs(5),
            async { service_manager.shutdown() }
        ).await
    })
});

match shutdown_result {
    Ok(Ok(())) => {
        info!("ServiceManager shutdown completed successfully");
    }
    Ok(Err(e)) => {
        error!("ServiceManager shutdown error: {}", e);
    }
    Err(_timeout) => {
        error!("ServiceManager shutdown timed out after 5 seconds");
        // Force termination of hung servers
        service_manager.force_kill_all()?;
    }
}
```

### Option 2: Thread-based timeout
```rust
use std::sync::mpsc::channel;
use std::time::Duration;

let (tx, rx) = channel();
let mgr = Arc::new(service_manager);
let mgr_clone = mgr.clone();

// Spawn shutdown in separate thread
thread::spawn(move || {
    let result = mgr_clone.shutdown();
    let _ = tx.send(result);
});

// Wait with timeout
match rx.recv_timeout(Duration::from_secs(5)) {
    Ok(Ok(())) => {
        info!("ServiceManager shutdown completed");
    }
    Ok(Err(e)) => {
        error!("ServiceManager shutdown failed: {}", e);
    }
    Err(_timeout) => {
        error!("ServiceManager shutdown timed out - force killing servers");
        mgr.force_kill_all()?;
    }
}
```

### Option 3: Incremental checkpoints
```rust
// Report progress during shutdown
let shutdown_timeout = Duration::from_secs(5);
let start = std::time::Instant::now();
let mut checkpoint = 1;

loop {
    if start.elapsed() >= shutdown_timeout {
        error!("Shutdown timeout - force killing");
        service_manager.force_kill_all()?;
        break;
    }
    
    // Try non-blocking shutdown step
    match service_manager.try_shutdown_step() {
        Ok(ShutdownProgress::Complete) => {
            info!("Shutdown complete");
            break;
        }
        Ok(ShutdownProgress::InProgress) => {
            // Update checkpoint
            checkpoint += 1;
            report_service_status(
                &status_handle,
                ServiceState::StopPending,
                Duration::from_secs(5),
                checkpoint,
            )?;
            thread::sleep(Duration::from_millis(100));
        }
        Err(e) => {
            error!("Shutdown error: {}", e);
            break;
        }
    }
}
```

## Additional Improvements
1. **Incremental checkpoints**: Update SCM checkpoint during shutdown
2. **Progress reporting**: Log which servers are still shutting down
3. **Force kill fallback**: Kill hung processes after timeout
4. **Configurable timeout**: Read from config file or registry

## Impact
- **Severity**: HIGH - Service hangs on stop
- **Frequency**: Depends on MCP server reliability
- **User Impact**: Service appears unresponsive, requires task kill
- **Data Risk**: Potential data loss from forced termination

## Testing
1. Create test MCP server that hangs on shutdown
2. Stop kodegend service via SCM
3. Verify timeout triggers after 5 seconds
4. Verify hung server is force-killed
5. Verify service stops cleanly

## Files to Modify
- `packages/kodegend/src/platform/windows_service.rs` - Add timeout logic
- `packages/kodegend/src/manager.rs` - May need to add force_kill_all() method

## References
- Windows SCM wait_hint: https://learn.microsoft.com/en-us/windows/win32/api/winsvc/ns-winsvc-service_status
- Service shutdown best practices: https://learn.microsoft.com/en-us/windows/win32/services/service-control-requests
