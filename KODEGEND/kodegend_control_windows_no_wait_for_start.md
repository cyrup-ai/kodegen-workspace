# Windows: No Verification Service Started

## Location
`packages/kodegend/src/control/windows_control.rs:121-136`

## Issue Type
Logical Error / Race Condition

## Severity
High

## Description
The `start_daemon()` function calls `StartServiceW()` and immediately returns Ok(()) if the API call succeeds. However, `StartServiceW()` only **initiates** the start process - it doesn't wait for the service to actually be running. The service could fail to start after the API returns success.

## Current Code
```rust
pub fn start_daemon() -> Result<()> {
    let sc_manager = ScManagerHandle::new()
        .context("Failed to open Service Control Manager for start")?;

    let service = open_service(&sc_manager, SERVICE_START.0)
        .context("Failed to open service for start")?;

    let result = unsafe {
        StartServiceW(service.handle(), None)
    };

    if result.is_err() {
        anyhow::bail!("Failed to start service");
    }

    Ok(())  // ← Returns immediately, service may not be running yet!
}
```

## The Problem

### StartServiceW Behavior
From Microsoft documentation:
> "The StartServiceW function returns when the service control manager has received the request to start the service, **not when the service has finished starting**."

### Service Start States
After `StartServiceW` succeeds, the service goes through these states:
1. **SERVICE_START_PENDING** - Service is starting
2. **SERVICE_RUNNING** - Service started successfully
3. **SERVICE_STOPPED** - Service failed to start

The current code returns at state 1, not waiting for state 2 or 3.

### What Can Go Wrong

#### Scenario 1: Service fails immediately
```
T+0ms:   StartServiceW succeeds
T+0ms:   start_daemon() returns Ok(())
T+10ms:  Caller thinks service is running
T+50ms:  Service fails (missing file, port conflict, etc.)
T+50ms:  Service enters SERVICE_STOPPED state
Result:  Caller thinks service is running, but it's stopped
```

#### Scenario 2: Caller tries to use service immediately
```
T+0ms:   StartServiceW succeeds
T+0ms:   start_daemon() returns Ok(())
T+1ms:   Caller tries to connect to service (e.g., HTTP endpoint)
T+1ms:   Connection fails - service not ready yet
T+100ms: Service finishes initialization
```

## Real-World Impact

### Issue 1: False Success Reports
```rust
// User code:
match start_daemon() {
    Ok(()) => println!("Daemon started successfully!"),
    Err(e) => println!("Failed to start: {}", e),
}
// Prints "started successfully" even if service crashes 100ms later
```

### Issue 2: Restart Race Condition
`restart_daemon()` does:
```rust
stop_daemon()?;
std::thread::sleep(Duration::from_secs(1));
start_daemon()?;  // Returns before service is running
// Caller may immediately try to use service → fails
```

### Issue 3: Integration Tests Fail
```rust
start_daemon()?;
let response = http_client.get("http://localhost:8080/health")?;  // Fails!
// Service started but not ready yet
```

## Recommendation

### Wait for SERVICE_RUNNING state with timeout:
```rust
pub fn start_daemon() -> Result<()> {
    let sc_manager = ScManagerHandle::new()
        .context("Failed to open Service Control Manager for start")?;

    let service = open_service(&sc_manager, SERVICE_START.0)
        .context("Failed to open service for start")?;

    let result = unsafe {
        StartServiceW(service.handle(), None)
    };

    if result.is_err() {
        let win_error = WindowsError::from_win32();
        let error_code = win_error.code().0 as u32;
        
        // ERROR_SERVICE_ALREADY_RUNNING is OK
        if error_code != 1056 {
            anyhow::bail!("Failed to start service: {}", win_error.message());
        }
        // Already running, just verify and return
    }

    // Wait for service to reach SERVICE_RUNNING state
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
            
            match status.dwCurrentState {
                x if x == SERVICE_RUNNING.0 => {
                    // Success!
                    return Ok(());
                }
                x if x == SERVICE_STOPPED.0 => {
                    // Service failed to start
                    anyhow::bail!(
                        "Service failed to start (exit code: {})",
                        status.dwWin32ExitCode
                    );
                }
                // SERVICE_START_PENDING or other transitional state
                _ => {
                    // Keep waiting
                }
            }
        }

        // Check timeout
        if start_time.elapsed() > timeout {
            anyhow::bail!(
                "Timeout waiting for service to start ({}s)",
                timeout.as_secs()
            );
        }

        // Sleep before next poll
        std::thread::sleep(Duration::from_millis(100));
    }
}
```

## Alternative: Return immediately but document behavior
If waiting is not desired, at minimum document the behavior:
```rust
/// Start the daemon service
///
/// Note: This function returns after initiating the start, not after
/// the service is fully running. Use check_status() to verify the
/// service reached SERVICE_RUNNING state.
pub fn start_daemon() -> Result<()> {
    // ... current implementation ...
}
```

But this puts the burden on every caller to poll status, which is error-prone.

## Microsoft Documentation
- [StartServiceW function](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-startservicew)
- [Service Status Transitions](https://learn.microsoft.com/en-us/windows/win32/services/service-status-transitions)
