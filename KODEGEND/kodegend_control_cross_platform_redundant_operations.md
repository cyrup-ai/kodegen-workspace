# Cross-Platform: Redundant Status Checks and Process Spawns

## Location
All platform implementations (Linux, macOS)

## Issue Type
Performance Bottleneck

## Severity
Low-Medium

## Description
Typical daemon management workflows spawn many redundant processes due to lack of state caching and batched operations. Each function call spawns a new process, even when operations could be combined.

## Current Workflow Example

```rust
// Typical user code to restart daemon:
if check_status()? {           // Spawn 1: systemctl is-active
    stop_daemon()?;            // Spawn 2: systemctl stop
}
start_daemon()?;               // Spawn 3: systemctl start
if !check_status()? {          // Spawn 4: systemctl is-active
    return Err("Failed to start");
}
```

**Total: 4 process spawns for a simple restart operation**

## Process Spawn Overhead

### Linux: systemctl
Each `systemctl` command involves:
- Fork + exec (expensive)
- D-Bus connection to systemd
- Authentication check
- Service lookup
- Command execution
- Response marshaling

**Approximate overhead: 20-50ms per spawn**

### macOS: launchctl  
Each `launchctl` command involves:
- Fork + exec
- XPC connection to launchd
- Service lookup
- Command execution
- Response parsing

**Approximate overhead: 30-60ms per spawn**

### Cumulative Impact
With 4 spawns per operation:
- **Best case**: 80ms (4 × 20ms)
- **Typical case**: 160ms (4 × 40ms)
- **Worst case**: 240ms (4 × 60ms)

For a simple restart operation, spending 160ms+ on process overhead is excessive.

## Real-World Scenarios

### Scenario 1: Health check loop
```rust
loop {
    if !check_status()? {
        start_daemon()?;
    }
    sleep(Duration::from_secs(5));
}
// Spawns process every 5 seconds
```

### Scenario 2: Automated deployment
```bash
#!/bin/bash
kodegend stop      # 1 spawn
# Update binary
kodegend start     # 1 spawn
kodegend status    # 1 spawn
# Total: 3 spawns (60-180ms overhead)
```

### Scenario 3: Web UI dashboard
```rust
// HTTP endpoint that shows daemon status
async fn status_handler() -> Result<Response> {
    let running = check_status()?;  // Spawns process on every request
    Ok(Response::json(StatusResponse { running }))
}
// If dashboard polls every 2s, spawns process every 2s
```

## Problems

### 1. Unnecessary Process Spawns
Most workflows spawn 2-6 processes when 1-2 would suffice.

### 2. No State Caching
`check_status()` results aren't cached even briefly, so consecutive calls spawn multiple processes.

### 3. No Batched Operations
Can't combine operations like "stop if running, then start" in a single command.

### 4. Windows Handles Not Reused
Windows implementation opens new SCM handle for each operation, even though the same handle could be reused.

## Impact
- **Latency**: Operations take 100-200ms longer than necessary
- **Resource Usage**: Unnecessary fork/exec overhead
- **Scalability**: Doesn't scale to high-frequency status checks
- **Battery Impact**: On laptops, frequent process spawns drain battery

## Recommendation

### Option 1: Add batched operations API
```rust
/// Execute multiple daemon control operations with minimal overhead
pub struct DaemonControlBatch {
    operations: Vec<Operation>,
}

enum Operation {
    CheckStatus,
    Start,
    Stop,
    Restart,
}

impl DaemonControlBatch {
    pub fn new() -> Self {
        Self { operations: Vec::new() }
    }
    
    pub fn check_status(mut self) -> Self {
        self.operations.push(Operation::CheckStatus);
        self
    }
    
    pub fn start_if_not_running(mut self) -> Self {
        self.operations.push(Operation::CheckStatus);
        self.operations.push(Operation::Start);
        self
    }
    
    pub fn execute(self) -> Result<BatchResult> {
        // Execute all operations with minimal process spawns
        // For systemctl, can combine commands: systemctl is-active && systemctl start
        // Or use systemctl's built-in conditionals
    }
}

// Usage:
let result = DaemonControlBatch::new()
    .check_status()
    .start_if_not_running()
    .execute()?;
```

### Option 2: Cache status for short duration
```rust
use std::sync::Mutex;
use std::time::{Duration, Instant};

struct StatusCache {
    status: Option<bool>,
    timestamp: Instant,
    ttl: Duration,
}

static STATUS_CACHE: Mutex<StatusCache> = Mutex::new(StatusCache {
    status: None,
    timestamp: Instant::now(),
    ttl: Duration::from_millis(500),
});

pub fn check_status() -> Result<bool> {
    let mut cache = STATUS_CACHE.lock().unwrap();
    
    // Return cached value if still fresh
    if let Some(status) = cache.status {
        if cache.timestamp.elapsed() < cache.ttl {
            return Ok(status);
        }
    }
    
    // Cache expired, fetch fresh status
    let status = platform::check_status()?;
    cache.status = Some(status);
    cache.timestamp = Instant::now();
    
    Ok(status)
}
```

### Option 3: Reuse Windows handles
```rust
// For Windows, keep SCM handle open across operations
thread_local! {
    static SCM_HANDLE: RefCell<Option<ScManagerHandle>> = RefCell::new(None);
}

fn get_scm_handle() -> Result<ScManagerHandle> {
    SCM_HANDLE.with(|handle| {
        let mut h = handle.borrow_mut();
        if h.is_none() {
            *h = Some(ScManagerHandle::new()?);
        }
        Ok(h.as_ref().unwrap().clone())
    })
}
```

### Option 4: Use systemctl's batch features
```rust
// systemctl supports multiple services in one call:
// systemctl is-active service1 service2 service3

// For complex workflows, use systemctl show for rich info in one call:
pub fn get_detailed_status() -> Result<ServiceInfo> {
    let output = Command::new("systemctl")
        .args(["show", SERVICE_NAME, "--no-pager"])
        .output()?;
    
    // Parse output which includes:
    // - ActiveState=active
    // - SubState=running
    // - MainPID=1234
    // - ExecMainStatus=0
    // All in one process spawn
}
```

## Performance Improvement

### Before (4 spawns):
- Latency: 160ms (4 × 40ms)
- CPU: 4 × fork/exec overhead

### After (1-2 spawns):
- Latency: 40-80ms (1-2 × 40ms)  
- CPU: 1-2 × fork/exec overhead
- **Speedup: 2-4x**

## Related Issues
- No timeout protection (makes redundant spawns worse if they hang)
- No async support (could parallelize independent operations)
