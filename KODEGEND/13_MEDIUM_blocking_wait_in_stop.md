# MEDIUM: Blocking wait() in Stop Command Freezes Manager

## Priority
**MEDIUM** - Performance degradation, unresponsive daemon

## Location
`packages/kodegend/src/manager.rs` - `handle_command()` Stop command (lines 161-173)

## Issue Description

The Stop command handler uses synchronous `child.wait()` which blocks the entire command handler task. This prevents other commands from being processed during the wait.

### Current Code

```rust
// Lines 161-173
ServiceCommand::Stop { service_name } => {
    if let Some(state) = self.services.get_mut(&service_name) {
        if let ServiceState::Running(mut child) = 
            std::mem::replace(state, ServiceState::Stopped) 
        {
            if let Err(e) = child.kill() {
                error!("Failed to kill service {}: {}", service_name, e);
            }
            
            // BLOCKING WAIT - freezes entire command handler!
            if let Err(e) = child.wait() {
                error!("Failed to wait for service {}: {}", service_name, e);
            }
        }
        *state = ServiceState::Stopped;
    }
}
```

### The Problem

`std::process::Child::wait()` is **synchronous** and **blocking**:
- Waits for process to exit
- Blocks current thread
- No timeout

In async context, this blocks the tokio task that handles ALL commands:
- No other commands processed while waiting
- Manager appears unresponsive
- Status commands queue up
- Start commands queue up

### Blocking Duration

How long does `wait()` block?

**Normal case** (well-behaved process):
- SIGKILL sent via `kill()`
- Process exits in ~1-100ms
- Short block, usually acceptable

**Problem cases**:

1. **Uninterruptible sleep (D state)**:
   - Process stuck in I/O wait
   - SIGKILL can't interrupt
   - Can block for 30+ seconds
   - Common with network FS, hung devices

2. **Zombie already created**:
   - Process already exited before `kill()`
   - `wait()` just reaps zombie (fast)
   - But we don't know if process is zombie or running

3. **Process not responding**:
   - Process hung in kernel space
   - `wait()` blocks until kernel releases it
   - Can be indefinite

### Impact Scenarios

#### Scenario 1: Stop During I/O

```
User: Stop database service
Database: Flushing 10GB buffer to slow NFS
Process: Uninterruptible sleep (D state)
Manager: Blocks for 30 seconds in wait()
User: Tries to check status of other service
Status command: Queued, no response for 30 seconds
User: "Daemon is frozen!"
```

#### Scenario 2: Multiple Stops

```
User: Stop all 10 services
Stop commands: All queued
Manager: Processes Stop for service 1
Service 1: Blocks for 5 seconds
Manager: Processes Stop for service 2  
Service 2: Blocks for 5 seconds
...
Total time: 50 seconds (sequential!)
Could be parallel: 5 seconds total
```

#### Scenario 3: Stop During Shutdown

```
User: systemctl stop kodegend
Shutdown: Sends Stop to all services
First service: Blocks for 30 seconds
Systemd: Waits for DefaultTimeoutStopSec (90s default)
If total Stop time > 90s: systemd sends SIGKILL
Kodegend: Killed while still stopping services
Result: Some services orphaned
```

## Impact

### User Experience
- Commands seem to hang
- Daemon appears unresponsive
- No feedback during long stops
- Frustrating UX

### System Impact
- Service stops are serialized (could be parallel)
- Slow shutdown times
- Risk of systemd SIGKILL during shutdown
- Orphaned processes if killed mid-shutdown

### Monitoring
- Health check timeouts
- False alerts
- Appears as daemon failure

## Root Cause

Using `std::process::Child` instead of `tokio::process::Child`:
- `std::process::Child::wait()` is blocking
- `tokio::process::Child::wait()` is async

## Solution

### Use tokio::process::Child

```rust
use tokio::process::{Child, Command};

ServiceCommand::Stop { service_name } => {
    if let Some(state) = self.services.get_mut(&service_name) {
        if let ServiceState::Running(mut child) = 
            std::mem::replace(state, ServiceState::Stopped) 
        {
            // Kill the process
            if let Err(e) = child.kill().await {
                error!("Failed to kill service {}: {}", service_name, e);
            }
            
            // ASYNC WAIT with timeout
            match tokio::time::timeout(
                Duration::from_secs(10),
                child.wait()
            ).await {
                Ok(Ok(status)) => {
                    info!("Service {} exited with status: {}", service_name, status);
                }
                Ok(Err(e)) => {
                    error!("Error waiting for service {}: {}", service_name, e);
                }
                Err(_) => {
                    warn!("Service {} didn't exit within timeout, may be zombie", 
                          service_name);
                }
            }
        }
        *state = ServiceState::Stopped;
    }
}
```

### Or: Spawn Wait in Background

If we can't switch to tokio::process yet:

```rust
ServiceCommand::Stop { service_name } => {
    if let Some(state) = self.services.get_mut(&service_name) {
        if let ServiceState::Running(mut child) = 
            std::mem::replace(state, ServiceState::Stopped) 
        {
            if let Err(e) = child.kill() {
                error!("Failed to kill service {}: {}", service_name, e);
            }
            
            // Spawn wait in background thread
            let name = service_name.clone();
            tokio::task::spawn_blocking(move || {
                match child.wait() {
                    Ok(status) => info!("Service {} exited: {}", name, status),
                    Err(e) => error!("Error waiting for {}: {}", name, e),
                }
            });
        }
        *state = ServiceState::Stopped;
    }
}
```

**Issue with this**: Zombie not reaped immediately, but at least doesn't block.

## Recommended Solution

**Switch to tokio::process::Child** throughout codebase:

Benefits:
1. Async wait - doesn't block command handler
2. Can use tokio::select! for better shutdown
3. Consistent async API
4. Better timeout handling
5. Solves Issue #9 (zombie leak) too

## Required Changes

1. Replace `use std::process::{Child, Command}` with `use tokio::process::{Child, Command}`
2. Update `spawn_service()` to use `tokio::process::Command` (line 372)
3. Update Stop command handler to use async `child.wait()` with timeout
4. Update `shutdown()` to use async wait (lines 206-234)
5. Update `monitor_service()` to use async wait instead of `try_wait()` polling
6. Update ServiceState to hold `tokio::process::Child`

### Breaking Changes

`tokio::process::Child` differences from `std::process::Child`:
- `kill()` is `async` (requires `.await`)
- Must call `start_kill()` for sync kill
- Stdio handling slightly different (already using tokio::io)

## Testing

### Unit Test
```rust
#[tokio::test]
async fn test_stop_command_non_blocking() {
    let start = Instant::now();
    
    // Create service that sleeps for 10 seconds before exiting
    // Send Stop command
    // Send Status command for different service
    // Status should respond in <100ms, not 10 seconds
    
    let duration = start.elapsed();
    assert!(duration < Duration::from_millis(200));
}
```

### Integration Test
```bash
# Terminal 1: Start daemon with service that's hard to kill
kodegend start

# Terminal 2: Send stop for slow service
time kodegend stop slow-service &

# Terminal 3: Immediately check status of other service
time kodegend status fast-service

# Should respond quickly, not wait for slow-service
```

## Performance Improvement

**Before**:
- Stop 10 services: 10 × wait_time (serial)
- If each takes 5s: 50 seconds total

**After**:
- Stop 10 services: max(all wait_times) (parallel)
- If each takes 5s: 5 seconds total
- **10x faster**

## Related Issues

- Issue #9: Zombie process leak (both need tokio::process)
- Issue #4: Shutdown incomplete (would benefit from async wait)

## Migration Path

1. **Phase 1**: Update spawn_service to use tokio::process::Command
2. **Phase 2**: Update Stop handler to async wait
3. **Phase 3**: Update shutdown to async wait
4. **Phase 4**: Update monitor to use async wait (optional, can keep try_wait)

Can be done incrementally without breaking existing code.
