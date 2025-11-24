# LOW: Windows Service Checkpoints Not Incremented

## Severity
**LOW - SERVICE CONTROL MANAGER INTEGRATION**

## Location
`packages/kodegend/src/platform/windows_service.rs:209-233`

## Issue Description
The `report_service_status()` function always sets the checkpoint field to 0, even during pending states (StartPending, StopPending). The Service Control Manager (SCM) uses checkpoints to track progress during long operations and detect hung services.

### Current Code
```rust
fn report_service_status(
    status_handle: &ServiceStatusHandle,
    current_state: ServiceState,
    wait_hint: Duration,
    exit_code: u32,
) -> Result<()> {
    let controls_accepted = if current_state == ServiceState::Running {
        ServiceControlAccept::STOP
    } else {
        ServiceControlAccept::empty()
    };
    
    let status = ServiceStatus {
        service_type: ServiceType::OWN_PROCESS,
        current_state,
        controls_accepted,
        exit_code: ServiceExitCode::Win32(exit_code),
        checkpoint: 0,  // ← Always 0, never incremented
        wait_hint,
        process_id: None,
    };
    
    status_handle
        .set_service_status(status)
        .context("Failed to set service status")
}
```

### How SCM Uses Checkpoints

From Microsoft documentation:
> "During lengthy operations, the service should call SetServiceStatus periodically with an **incremented checkpoint** value to report progress. This prevents SCM from timing out and killing the service."

**Expected behavior:**
1. StartPending/StopPending: Increment checkpoint on each status report
2. Running/Stopped: Checkpoint should be 0
3. Each report with same state should have checkpoint + 1

**Current behavior:**
- Always reports checkpoint = 0
- SCM cannot distinguish "hung" from "making progress"
- If wait_hint expires without checkpoint change, SCM may think service is hung

### Real-World Impact

**Low impact because:**
- kodegend startup is fast (< 5 seconds)
- Shutdown is also quick
- wait_hint is generous (3s for startup, 5s for shutdown)

**Could matter if:**
- ServiceManager initialization is slow
- Many MCP servers to start/stop
- Database migrations during startup
- Wait_hint expires before completion

## Recommended Fix

### Option 1: Pass checkpoint as parameter
```rust
fn report_service_status(
    status_handle: &ServiceStatusHandle,
    current_state: ServiceState,
    wait_hint: Duration,
    exit_code: u32,
    checkpoint: u32,  // ← New parameter
) -> Result<()> {
    let status = ServiceStatus {
        service_type: ServiceType::OWN_PROCESS,
        current_state,
        controls_accepted: if current_state == ServiceState::Running {
            ServiceControlAccept::STOP
        } else {
            ServiceControlAccept::empty()
        },
        exit_code: ServiceExitCode::Win32(exit_code),
        checkpoint,  // ← Use provided value
        wait_hint,
        process_id: None,
    };
    
    status_handle
        .set_service_status(status)
        .context("Failed to set service status")
}

// Usage:
report_service_status(&status_handle, ServiceState::StartPending, Duration::from_secs(3), 0, 1)?;
// ... do some work ...
report_service_status(&status_handle, ServiceState::StartPending, Duration::from_secs(3), 0, 2)?;
// ... more work ...
report_service_status(&status_handle, ServiceState::Running, Duration::from_secs(0), 0, 0)?;
```

### Option 2: Auto-increment with state tracking
```rust
struct ServiceStatusReporter {
    handle: ServiceStatusHandle,
    checkpoint: u32,
    current_state: ServiceState,
}

impl ServiceStatusReporter {
    fn new(handle: ServiceStatusHandle) -> Self {
        Self {
            handle,
            checkpoint: 0,
            current_state: ServiceState::Stopped,
        }
    }
    
    fn report(
        &mut self,
        state: ServiceState,
        wait_hint: Duration,
        exit_code: u32,
    ) -> Result<()> {
        // Auto-increment checkpoint if still in same pending state
        if state == self.current_state 
            && (state == ServiceState::StartPending || state == ServiceState::StopPending) {
            self.checkpoint += 1;
        } else {
            // New state - reset checkpoint
            self.checkpoint = if state == ServiceState::Running || state == ServiceState::Stopped {
                0
            } else {
                1
            };
            self.current_state = state;
        }
        
        let status = ServiceStatus {
            service_type: ServiceType::OWN_PROCESS,
            current_state: state,
            controls_accepted: if state == ServiceState::Running {
                ServiceControlAccept::STOP
            } else {
                ServiceControlAccept::empty()
            },
            exit_code: ServiceExitCode::Win32(exit_code),
            checkpoint: self.checkpoint,
            wait_hint,
            process_id: None,
        };
        
        self.handle
            .set_service_status(status)
            .context("Failed to set service status")
    }
}

// Usage:
let mut reporter = ServiceStatusReporter::new(status_handle);
reporter.report(ServiceState::StartPending, Duration::from_secs(3), 0)?;
// ... do work ...
reporter.report(ServiceState::StartPending, Duration::from_secs(3), 0)?;  // checkpoint auto-increments
reporter.report(ServiceState::Running, Duration::from_secs(0), 0)?;  // checkpoint resets to 0
```

### Option 3: Only increment during long operations
```rust
fn run_service() -> Result<()> {
    let mut checkpoint = 1;
    
    report_service_status(&status_handle, ServiceState::StartPending, Duration::from_secs(3), 0, checkpoint)?;
    
    // Long operation: initialize ServiceManager
    let service_manager = match ServiceManager::new() {
        Ok(mgr) => {
            checkpoint += 1;
            info!("ServiceManager initialized");
            mgr
        }
        Err(e) => {
            // ... error handling ...
        }
    };
    
    // If initialization takes > 3 seconds, report progress
    checkpoint += 1;
    report_service_status(&status_handle, ServiceState::StartPending, Duration::from_secs(3), 0, checkpoint)?;
    
    // Transition to running
    report_service_status(&status_handle, ServiceState::Running, Duration::from_secs(0), 0, 0)?;
    
    // ... similar for shutdown ...
}
```

## Recommended Approach

**Option 1 (Pass as parameter)** is simplest:
- Minimal code change
- Explicit control over checkpoints
- Easy to understand

**Only implement if:**
- ServiceManager startup takes > 3 seconds
- Shutdown takes > 5 seconds
- SCM timeout issues observed

Otherwise, current code is acceptable for fast-starting services.

## Testing
1. Add delays to simulate slow startup:
   ```rust
   thread::sleep(Duration::from_secs(10));
   ```
2. Start service and check Event Viewer for timeout warnings
3. Verify checkpoints increment in service control panel

## Impact
- **Severity**: LOW - Only matters for slow operations
- **Current**: Startup/shutdown are fast enough
- **Risk**: Low - wait_hint is generous
- **Priority**: LOW - Nice to have, not critical

## Files to Modify
- `packages/kodegend/src/platform/windows_service.rs`

## References
- SERVICE_STATUS structure: https://learn.microsoft.com/en-us/windows/win32/api/winsvc/ns-winsvc-service_status
- SetServiceStatus: https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-setservicestatus
- Service checkpoint best practices: https://learn.microsoft.com/en-us/windows/win32/services/service-control-requests

## Notes
This is a polish issue, not a bug. Current implementation works fine for fast-starting services. Only optimize if profiling shows startup/shutdown takes longer than wait_hint.
