# Windows: No Verification Service Stopped

## Location
`packages/kodegend/src/control/windows_control.rs:140-158`

## Issue Type
Logical Error / Race Condition

## Severity
High

## Description
The `stop_daemon()` function sends `SERVICE_CONTROL_STOP` via `ControlService()` and immediately returns Ok(()) if the API call succeeds. However, `ControlService()` only **signals** the service to stop - it doesn't wait for the service to actually stop. This is the mirror issue to the start_daemon problem.

## Current Code
```rust
pub fn stop_daemon() -> Result<()> {
    let sc_manager = ScManagerHandle::new()
        .context("Failed to open Service Control Manager for stop")?;

    let service = open_service(&sc_manager, SERVICE_STOP.0)
        .context("Failed to open service for stop")?;

    let mut status: SERVICE_STATUS = unsafe { mem::zeroed() };

    let result = unsafe {
        ControlService(service.handle(), SERVICE_CONTROL_STOP, &mut status)
    };

    if result.is_err() {
        anyhow::bail!("Failed to stop service");
    }

    Ok(())  // ← Returns immediately, service may still be running!
}
```

## The Problem

### ControlService Behavior
From Microsoft documentation:
> "When the service control manager sends a control code to a service, it waits for the handler function to return before returning. The SCM returns immediately after the service control handler returns, **not after the service finishes processing the control code**."

### Service Stop States
After `ControlService` with `SERVICE_CONTROL_STOP` succeeds:
1. **SERVICE_STOP_PENDING** - Service is stopping
2. **SERVICE_STOPPED** - Service stopped successfully
3. Could take seconds for cleanup

The current code returns at state 1, not waiting for state 2.

## Real-World Impact

### Issue 1: restart_daemon() Race
```rust
pub fn restart_daemon() -> Result<()> {
    stop_daemon()?;  // Returns immediately, service still running
    std::thread::sleep(Duration::from_secs(1));  // Arbitrary wait
    start_daemon()?;  // May fail if service still stopping
    Ok(())
}
```

The 1-second sleep is a band-aid that:
- May be too short (service needs 2s to stop)
- May be too long (service stopped in 100ms, waste 900ms)
- Doesn't guarantee correctness

### Issue 2: Port/File Conflicts
```rust
// Service holds port 8080
stop_daemon()?;
// Service still running, holding port 8080
start_daemon()?;
// New instance tries to bind port 8080 → fails
```

### Issue 3: Data Corruption
```rust
stop_daemon()?;
// Service is in the middle of:
// - Writing to database
// - Flushing logs
// - Closing file handles
// But function returned, so caller thinks it's safe to:
delete_service_files()?;  // Deletes files while service is writing!
```

## Recommendation

### Wait for SERVICE_STOPPED state with timeout:
```rust
pub fn stop_daemon() -> Result<()> {
    let sc_manager = ScManagerHandle::new()
        .context("Failed to open Service Control Manager for stop")?;

    let service = open_service(&sc_manager, SERVICE_STOP.0)
        .context("Failed to open service for stop")?;

    let mut status = MaybeUninit::<SERVICE_STATUS>::uninit();

    let result = unsafe {
        ControlService(
            service.handle(),
            SERVICE_CONTROL_STOP,
            status.as_mut_ptr()
        )
    };

    if result.is_err() {
        let win_error = WindowsError::from_win32();
        let error_code = win_error.code().0 as u32;
        
        // ERROR_SERVICE_NOT_ACTIVE is OK (already stopped)
        if error_code == 1062 {
            return Ok(());
        }
        
        anyhow::bail!("Failed to stop service: {}", win_error.message());
    }

    // Wait for service to reach SERVICE_STOPPED state
    let timeout = Duration::from_secs(30);
    let start_time = std::time::Instant::now();
    
    loop {
        // Re-open service for status query
        let service_query = open_service(&sc_manager, SERVICE_QUERY_STATUS.0)?;
        
        let mut status = MaybeUninit::<SERVICE_STATUS_PROCESS>::uninit();
        let mut bytes_needed: u32 = 0;

        let result = unsafe {
            QueryServiceStatusEx(
                service_query.handle(),
                SC_STATUS_PROCESS_INFO,
                Some(status.as_mut_ptr() as *mut u8),
                mem::size_of::<SERVICE_STATUS_PROCESS>() as u32,
                &mut bytes_needed,
            )
        };

        if result.is_ok() {
            let status = unsafe { status.assume_init() };
            
            if status.dwCurrentState == SERVICE_STOPPED.0 {
                // Success - service is stopped
                return Ok(());
            }
            
            // Still stopping (SERVICE_STOP_PENDING)
            // Check if service is hung
            if status.dwCurrentState != SERVICE_STOP_PENDING.0 {
                log::warn!(
                    "Unexpected service state during stop: {}",
                    status.dwCurrentState
                );
            }
        }

        // Check timeout
        if start_time.elapsed() > timeout {
            anyhow::bail!(
                "Timeout waiting for service to stop ({}s). Service may be hung.",
                timeout.as_secs()
            );
        }

        // Sleep before next poll (use dwWaitHint if available)
        std::thread::sleep(Duration::from_millis(250));
    }
}
```

### Use dwWaitHint for Optimal Polling
The SERVICE_STATUS structure includes `dwWaitHint` which tells how long to wait before polling again:
```rust
let wait_time = if status.dwWaitHint > 0 {
    Duration::from_millis(status.dwWaitHint as u64)
} else {
    Duration::from_millis(250)
};
std::thread::sleep(wait_time);
```

## Benefits of Waiting

1. **Correctness**: Guarantees service is actually stopped
2. **No Race Conditions**: restart_daemon() works reliably
3. **Better Errors**: Can detect and report hung services
4. **Resource Cleanup**: Ensures ports/files are released
5. **Data Safety**: Service finishes writing data before proceeding

## Performance Impact
- **Worst case**: 30s timeout if service is hung (better than hanging forever)
- **Typical case**: 100-500ms to verify service stopped
- **Best case**: Immediate if service already stopped (ERROR_SERVICE_NOT_ACTIVE)
- **Better than current**: Removes arbitrary 1s sleep in restart_daemon()

## Microsoft Documentation
- [ControlService function](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-controlservice)
- [SERVICE_STATUS structure](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/ns-winsvc-service_status)
- [dwWaitHint documentation](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/ns-winsvc-service_status#members)
