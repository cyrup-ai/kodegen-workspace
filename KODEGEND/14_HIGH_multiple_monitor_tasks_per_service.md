# HIGH: Multiple Monitor Tasks Can Run for Same Service

## Priority
**HIGH** - Resource waste, race conditions, duplicate processes

## Location
`packages/kodegend/src/manager.rs` - `spawn_service()` (line 455-456) and `monitor_service()` (line 335)

## Issue Description

There's no check to prevent multiple monitoring tasks from running for the same service. This leads to duplicate monitors that race to restart services.

### How Duplicates Are Created

#### Path 1: Initial Service Start

```rust
// Line 455-456 in spawn_service()
tokio::spawn(async move {
    Self::monitor_service(service_name, shutdown.clone()).await;
});
// No check if monitor already running!
```

Every call to `spawn_service()` spawns a new monitor task.

#### Path 2: Auto-Restart in monitor_service

```rust
// Line 335 in monitor_service
if let Err(e) = self.spawn_service(service_name).await {
    error!("Failed to restart service {}: {}", service_name, e);
}
```

When a service crashes, the monitor calls `spawn_service()`, which spawns **another** monitor task (line 455).

**Result**: Old monitor still running + new monitor = 2 monitors for same service.

#### Path 3: User Sends Multiple Start Commands

```
User: Send Start command
Manager: Spawns service + monitor task 1

User: Send Start command again (rapid clicking)
Manager: Spawns service + monitor task 2

Result: 2 monitor tasks for 1 service
```

#### Path 4: Restart Command

```rust
// Line 175-178
ServiceCommand::Restart { service_name } => {
    let _ = self.sender.send(ServiceCommand::Stop { ... }).await;
    let _ = self.sender.send(ServiceCommand::Start { ... }).await;
}
```

Stop doesn't kill monitor task, Start spawns new monitor = duplicate.

### Progression Example

```
T=0:    Start service → Monitor task 1 spawned
T=10:   Service crashes
T=11:   Monitor task 1 detects crash, calls spawn_service
T=11:   spawn_service → Monitor task 2 spawned
T=20:   Service crashes again
T=21:   Monitor task 1 detects, spawns Monitor task 3
T=21:   Monitor task 2 detects, spawns Monitor task 4
T=30:   Service crashes
T=31:   All 4 monitors detect, spawn 4 more = 8 monitors
...
Exponential growth!
```

## Impact

### Resource Waste

For N duplicate monitors:
- N tasks in tokio runtime
- N × polling overhead (each wakes every 1 second)
- N × HashMap lookups
- N × process checks (`try_wait()`)

**Example**:
- 10 services
- Each restarted 5 times
- = 10 × 2^5 = 320 monitor tasks
- vs expected 10 tasks
- **32x overhead**

### Race Conditions

Multiple monitors for same service:

1. **Duplicate restart attempts**:
   - Service crashes
   - Monitor 1 detects, starts restart
   - Monitor 2 detects, starts restart
   - **Result**: 2 processes spawned for 1 service

2. **State corruption**:
   - Monitors race to update ServiceState
   - Undefined which monitor wins
   - State doesn't match reality

3. **Process orphaning**:
   - Monitor 1 spawns PID 100
   - Monitor 2 spawns PID 200
   - ServiceState holds one, other is orphaned
   - Orphaned process can't be stopped via Manager

### restart_count Corruption

Each monitor has its own local `restart_count`:
- Monitor 1: count = 3
- Monitor 2: count = 1
- Monitor 3: count = 0

None reflect true restart count. max_restarts not enforced correctly.

### Debugging Nightmare

- Logs show multiple "Service crashed" messages for same event
- Hard to trace which monitor is doing what
- Non-deterministic behavior

## Root Cause

No tracking of active monitor tasks. `spawn_service()` always spawns a new monitor without checking if one exists.

## Solution

### Track Monitor Tasks

```rust
use tokio::task::JoinHandle;

pub struct ServiceManager {
    services: HashMap<String, ServiceState>,
    monitor_tasks: HashMap<String, JoinHandle<()>>,  // NEW
    // ... other fields
}

impl ServiceManager {
    async fn spawn_service(&mut self, service_name: &str) -> Result<()> {
        // ... spawn child process ...
        
        // Check if monitor already running
        if let Some(existing_handle) = self.monitor_tasks.get(service_name) {
            if !existing_handle.is_finished() {
                // Monitor still running, don't spawn new one
                debug!("Monitor already running for {}", service_name);
                return Ok(());
            } else {
                // Old monitor finished, remove it
                self.monitor_tasks.remove(service_name);
            }
        }
        
        // Spawn new monitor
        let shutdown = self.shutdown.clone();
        let name = service_name.to_string();
        let handle = tokio::spawn(async move {
            Self::monitor_service(&name, shutdown).await;
        });
        
        self.monitor_tasks.insert(service_name.to_string(), handle);
        
        Ok(())
    }
    
    async fn stop_service(&mut self, service_name: &str) -> Result<()> {
        // Stop the child process
        // ...
        
        // Stop the monitor task
        if let Some(handle) = self.monitor_tasks.remove(service_name) {
            handle.abort();  // Cancel the monitor task
        }
        
        Ok(())
    }
}
```

### Better: Use Cancellation Tokens

```rust
use tokio_util::sync::CancellationToken;

pub struct ServiceManager {
    services: HashMap<String, ServiceState>,
    monitor_cancels: HashMap<String, CancellationToken>,  // NEW
    // ... other fields
}

impl ServiceManager {
    async fn spawn_service(&mut self, service_name: &str) -> Result<()> {
        // ... spawn child ...
        
        // Cancel existing monitor if any
        if let Some(token) = self.monitor_cancels.get(service_name) {
            token.cancel();
        }
        
        // Create new cancellation token
        let cancel = CancellationToken::new();
        self.monitor_cancels.insert(service_name.to_string(), cancel.clone());
        
        // Spawn monitor with cancel token
        let name = service_name.to_string();
        tokio::spawn(async move {
            Self::monitor_service(&name, cancel).await;
        });
        
        Ok(())
    }
}

async fn monitor_service(
    service_name: &str, 
    cancel: CancellationToken
) {
    loop {
        tokio::select! {
            _ = cancel.cancelled() => {
                debug!("Monitor cancelled for {}", service_name);
                break;
            }
            _ = tokio::time::sleep(Duration::from_secs(1)) => {
                // Check service status
            }
        }
    }
}
```

### Alternative: Don't Spawn Monitor in spawn_service

Remove monitor spawn from `spawn_service()` entirely. Instead:

1. **Spawn monitors once** in `start()` for each configured service
2. **Monitors call spawn_service** when restart needed
3. **spawn_service** only spawns the child process, not monitor

```rust
impl ServiceManager {
    pub async fn start(&mut self) -> Result<()> {
        // Start command handler
        // ... existing code ...
        
        // Spawn monitor for each configured service
        for service in &self.config.services {
            let name = service.name.clone();
            let shutdown = self.shutdown.clone();
            tokio::spawn(async move {
                Self::monitor_service(&name, shutdown).await;
            });
        }
        
        Ok(())
    }
}
```

**Pros**: 
- One monitor per service, guaranteed
- Clean architecture

**Cons**:
- Monitor runs even when service is stopped
- Need to handle monitor lifecycle differently

## Recommended Solution

**Use cancellation tokens** (Option 2):

1. Store `CancellationToken` per service
2. Cancel old monitor before spawning new one
3. Monitors check cancellation token in loop
4. Clean, explicit lifecycle management

## Required Changes

1. Add `tokio_util` dependency
2. Add `monitor_cancels: HashMap<String, CancellationToken>` to ServiceManager
3. Update `spawn_service()` to:
   - Cancel existing monitor
   - Create new cancellation token
   - Pass token to monitor_service
4. Update `monitor_service()` signature to accept CancellationToken
5. Update `monitor_service()` loop to check cancellation
6. Update Stop command to cancel monitor
7. Update shutdown to cancel all monitors

## Testing

```rust
#[tokio::test]
async fn test_no_duplicate_monitors() {
    let mut manager = ServiceManager::new(config);
    
    // Spawn service
    manager.spawn_service("test").await.unwrap();
    let task_count_1 = count_tokio_tasks();
    
    // Spawn again (shouldn't create duplicate)
    manager.spawn_service("test").await.unwrap();
    let task_count_2 = count_tokio_tasks();
    
    assert_eq!(task_count_1, task_count_2);
}

#[tokio::test]
async fn test_monitor_cancelled_on_stop() {
    let mut manager = ServiceManager::new(config);
    manager.spawn_service("test").await.unwrap();
    
    // Stop service
    manager.stop_service("test").await.unwrap();
    
    // Monitor should be cancelled
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert!(manager.monitor_cancels.get("test").unwrap().is_cancelled());
}
```

## Related Issues

- Issue #8: restart_count not shared (duplicate monitors make this worse)
- Issue #2: Direct HashMap modification (duplicate monitors race)
- Issue #1: HashMap synchronization (duplicate monitors cause more races)

## Dependencies

```toml
tokio-util = { version = "0.7", features = ["sync"] }
```
