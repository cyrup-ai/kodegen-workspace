# HIGH: Status Command Doesn't Return Results to Caller

## Priority
**MEDIUM** - Functionality broken, poor UX

## Location
`packages/kodegend/src/manager.rs` - `handle_command()` Status (lines 180-186)

## Issue Description

The Status command only logs the service status but doesn't return it to the caller. This makes the command essentially useless for programmatic access.

### Current Implementation

```rust
// Lines 180-186
ServiceCommand::Status { service_name } => {
    if let Some(state) = self.services.get(service_name) {
        info!("Service {} status: {:?}", service_name, state);
    } else {
        info!("Service {} not found", service_name);
    }
}
```

### Problems

1. **Information only logged**
   - Goes to log file (if configured)
   - Not visible to command sender
   - User sees nothing when running `kodegend status service-name`

2. **No response channel**
   - ServiceCommand enum has no response field
   - No way to send result back to caller
   - Fire-and-forget command

3. **Unusable for programmatic access**
   - CLI can't display status to user
   - Scripts can't query service state
   - Monitoring tools can't check status

4. **Only works via log tailing**
   - User must: tail -f /var/log/kodegend.log
   - Then: send status command
   - Then: grep log for status line
   - Terrible UX

### Current ServiceCommand Enum

```rust
// Lines 29-35
pub enum ServiceCommand {
    Start { service_name: String },
    Stop { service_name: String },
    Restart { service_name: String },
    Status { service_name: String },  // No response field!
}
```

### How Other Commands Are Handled

For Start/Stop/Restart:
- User sends command
- Command executes
- User checks logs for result
- Also terrible UX, but at least state changes are observable

For Status:
- User sends command  
- Command executes
- User... has no way to see result?
- Completely broken

## Impact

### User Experience

```bash
# User runs:
$ kodegend status my-service

# Expecting:
Service: my-service
Status: Running
PID: 1234
Uptime: 5m 23s

# Actually sees:
<nothing>

# Must instead:
$ tail -f /var/log/kodegend.log | grep "Service my-service status"
```

**This is unacceptable UX**.

### Programmatic Access

```python
# Script tries to check if service is running
result = subprocess.run(['kodegend', 'status', 'my-service'], 
                       capture_output=True)
# result.stdout is empty
# No way to get the status!

# Must instead parse logs:
result = subprocess.run(['grep', 'my-service status', '/var/log/kodegend.log'],
                       capture_output=True)
# Brittle, race-prone, terrible
```

### Monitoring Integration

- Nagios/Prometheus can't query service status
- No health check endpoint
- Monitoring must parse log files (eww)

## Root Cause

ServiceCommand was designed as fire-and-forget. No thought given to returning results.

## Solution

### Add Response Channel to Commands

```rust
use tokio::sync::oneshot;

pub enum ServiceCommand {
    Start { 
        service_name: String,
    },
    Stop { 
        service_name: String,
    },
    Restart { 
        service_name: String,
    },
    Status { 
        service_name: String,
        response: oneshot::Sender<ServiceStatusResponse>,  // NEW
    },
}

#[derive(Debug, Clone, Serialize)]
pub struct ServiceStatusResponse {
    pub service_name: String,
    pub state: ServiceStateInfo,
    pub pid: Option<u32>,
    pub uptime: Option<Duration>,
    pub restart_count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub enum ServiceStateInfo {
    Running,
    Stopped,
    Failed { reason: String },
    Restarting,
}

// In handle_command:
ServiceCommand::Status { service_name, response } => {
    let status_response = if let Some(state) = self.services.get(&service_name) {
        ServiceStatusResponse {
            service_name: service_name.clone(),
            state: state.to_info(),
            pid: state.pid(),
            uptime: state.uptime(),
            restart_count: state.restart_count(),
        }
    } else {
        // Service not found
        ServiceStatusResponse {
            service_name,
            state: ServiceStateInfo::Stopped,
            pid: None,
            uptime: None,
            restart_count: 0,
        }
    };
    
    let _ = response.send(status_response);
}

// In CLI:
pub async fn get_status(service_name: &str) -> Result<ServiceStatusResponse> {
    let (tx, rx) = oneshot::channel();
    
    send_command(ServiceCommand::Status {
        service_name: service_name.to_string(),
        response: tx,
    }).await?;
    
    let status = rx.await?;
    Ok(status)
}
```

### Better: Status Query API

Instead of Status command, provide query method:

```rust
impl ServiceManager {
    pub fn get_status(&self, service_name: &str) -> Option<ServiceStatusResponse> {
        self.services.get(service_name).map(|state| {
            ServiceStatusResponse {
                service_name: service_name.to_string(),
                state: state.to_info(),
                pid: state.pid(),
                uptime: state.uptime(),
                restart_count: state.restart_count(),
            }
        })
    }
    
    pub fn get_all_statuses(&self) -> Vec<ServiceStatusResponse> {
        self.services.iter()
            .map(|(name, state)| ServiceStatusResponse {
                service_name: name.clone(),
                state: state.to_info(),
                pid: state.pid(),
                uptime: state.uptime(),
                restart_count: state.restart_count(),
            })
            .collect()
    }
}
```

If ServiceManager is behind async barrier, use channel:

```rust
pub enum ServiceQuery {
    GetStatus { 
        service_name: String,
        response: oneshot::Sender<Option<ServiceStatusResponse>>,
    },
    GetAllStatuses {
        response: oneshot::Sender<Vec<ServiceStatusResponse>>,
    },
}
```

## Recommended Solution

**Hybrid approach**:

1. **Add response channels** to ServiceCommand::Status
2. **Add helper methods** on ServiceManager for direct queries (if accessible)
3. **Extend ServiceState** with methods to extract info:
   - `to_info()` - convert to serializable form
   - `pid()` - get child PID if running
   - `uptime()` - calculate uptime
   - `restart_count()` - from Issue #8 fix

```rust
impl ServiceState {
    pub fn to_info(&self) -> ServiceStateInfo {
        match self {
            ServiceState::Running { .. } => ServiceStateInfo::Running,
            ServiceState::Stopped => ServiceStateInfo::Stopped,
            ServiceState::Failed { reason, .. } => 
                ServiceStateInfo::Failed { reason: reason.clone() },
            ServiceState::Restarting { .. } => ServiceStateInfo::Restarting,
        }
    }
    
    pub fn pid(&self) -> Option<u32> {
        match self {
            ServiceState::Running { child, .. } => Some(child.id()),
            _ => None,
        }
    }
    
    pub fn uptime(&self) -> Option<Duration> {
        match self {
            ServiceState::Running { last_start, .. } => {
                last_start.elapsed().ok()
            }
            _ => None,
        }
    }
    
    pub fn restart_count(&self) -> usize {
        match self {
            ServiceState::Running { restart_count, .. } |
            ServiceState::Failed { restart_count, .. } |
            ServiceState::Restarting { restart_count } => *restart_count,
            _ => 0,
        }
    }
}
```

## Required Changes

1. Add `ServiceStatusResponse` and `ServiceStateInfo` types
2. Add response channel to `ServiceCommand::Status`
3. Update `handle_command()` to send response
4. Add helper methods to `ServiceState`: `to_info()`, `pid()`, `uptime()`, `restart_count()`
5. Update CLI to:
   - Create oneshot channel
   - Send Status command
   - Await response
   - Display formatted result
6. Add serialization support (for JSON output)

## CLI Output Examples

### Basic Status

```bash
$ kodegend status my-service
Service: my-service
Status: Running
PID: 12345
Uptime: 2h 15m 30s
Restarts: 0
```

### Failed Service

```bash
$ kodegend status broken-service
Service: broken-service
Status: Failed
Reason: Command not found: /bad/path
Restarts: 5 (max reached)
```

### JSON Output

```bash
$ kodegend status my-service --json
{
  "service_name": "my-service",
  "state": "Running",
  "pid": 12345,
  "uptime_secs": 8130,
  "restart_count": 0
}
```

### All Services

```bash
$ kodegend status --all
my-service      Running   PID: 12345  Uptime: 2h 15m
other-service   Stopped   -           -
broken-service  Failed    -           Command not found
```

## Testing

```rust
#[tokio::test]
async fn test_status_command_returns_result() {
    let mut manager = ServiceManager::new(config);
    manager.spawn_service("test").await.unwrap();
    
    let (tx, rx) = oneshot::channel();
    let cmd = ServiceCommand::Status {
        service_name: "test".to_string(),
        response: tx,
    };
    
    manager.handle_command(cmd).await;
    
    let status = rx.await.unwrap();
    assert_eq!(status.service_name, "test");
    assert!(matches!(status.state, ServiceStateInfo::Running));
    assert!(status.pid.is_some());
}
```

## Related Issues

- Issue #8: restart_count needs to be in ServiceState (required for status)
- None of the other command types return results either (could extend this fix)

## Future Enhancements

- Add response channels to Start/Stop/Restart commands
- Return success/failure instead of fire-and-forget
- Better error messages via response channel
- Streaming status updates (watch mode)
