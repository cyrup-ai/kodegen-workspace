# HIGH: restart_count Not Persistent or Thread-Safe

## Priority
**MEDIUM** - Incorrect behavior, max_restarts not enforced correctly

## Location
`packages/kodegend/src/manager.rs` - `monitor_service()` method (line 242)

## Issue Description

The `restart_count` variable is a local variable in the `monitor_service()` async task. This means it's neither persistent nor shared across different code paths, leading to incorrect restart limit enforcement.

### Current Implementation

```rust
// Line 236-242
async fn monitor_service(
    service_name: &str,
    shutdown: Arc<AtomicBool>,
) {
    let config = /* ... */;
    let mut restart_count = 0;  // <-- Local to this task
    
    loop {
        // Check if service crashed
        // If crashed and restart_count < max_restarts, restart
        // ...
    }
}
```

### Problems

#### Problem 1: Per-Task Variable

Each time `monitor_service()` is spawned, it gets a new `restart_count` starting at 0.

Scenarios where monitor is spawned multiple times:
- Service manually stopped and restarted via commands
- Monitor task crashes (panic, etc.) and is recreated
- Bug in Issue #8 (multiple monitors per service)

**Result**: restart_count resets to 0, max_restarts never actually enforced.

#### Problem 2: Not Shared Across Code Paths

- **Automatic restarts** (via monitor): increment restart_count
- **Manual restarts** (via Restart command): don't increment restart_count
- **Stop + Start sequence**: restart_count stays the same

**Result**: Inconsistent tracking - a service can be manually restarted unlimited times even if it's exceeded automatic restart limit.

#### Problem 3: No Cooldown Reset

Once a service reaches max_restarts, restart_count stays at max forever (or until monitor task recreated).

**Expected behavior**: After service runs successfully for X minutes, reset restart_count to 0.

**Current behavior**: Counter never resets, service permanently marked as failed.

#### Problem 4: Not Visible

There's no way to query current restart_count:
- Status command doesn't show it
- Not logged anywhere  
- Can't debug restart loops

## Impact

### User Experience
- max_restarts setting appears ineffective
- Services restart more times than configured
- Confusion about why service keeps restarting
- No visibility into restart history

### System Impact
- Restart storms not prevented
- Resource exhaustion from infinite restarts
- No circuit breaker functionality

## Root Cause

restart_count stored as local variable instead of:
- Part of ServiceState
- Part of a separate monitoring state structure
- Persistent storage

## Solution

### Option 1: Add to ServiceState (Recommended)

```rust
#[derive(Debug)]
pub enum ServiceState {
    Running {
        child: Child,
        restart_count: usize,
        last_start: SystemTime,
    },
    Stopped,
    Failed {
        reason: String,
        restart_count: usize,
    },
    Restarting {
        restart_count: usize,
    },
}

impl ServiceState {
    fn increment_restart_count(&mut self) {
        match self {
            ServiceState::Running { restart_count, .. } |
            ServiceState::Failed { restart_count, .. } |
            ServiceState::Restarting { restart_count } => {
                *restart_count += 1;
            }
            _ => {}
        }
    }
    
    fn reset_restart_count_if_stable(&mut self, min_uptime: Duration) {
        if let ServiceState::Running { restart_count, last_start, .. } = self {
            if last_start.elapsed().unwrap_or(Duration::ZERO) > min_uptime {
                *restart_count = 0;
            }
        }
    }
    
    fn restart_count(&self) -> usize {
        match self {
            ServiceState::Running { restart_count, .. } |
            ServiceState::Failed { restart_count, .. } |
            ServiceState::Restarting { restart_count } => *restart_count,
            _ => 0,
        }
    }
}
```

**Usage in monitor_service()**:

```rust
async fn monitor_service(/* ... */) {
    loop {
        // ...
        if let Some(state) = self.services.get_mut(service_name) {
            match child.try_wait() {
                Ok(Some(status)) => {
                    // Service crashed
                    state.increment_restart_count();
                    let count = state.restart_count();
                    
                    if count >= max_restarts {
                        *state = ServiceState::Failed {
                            reason: format!("Exceeded max restarts: {}", max_restarts),
                            restart_count: count,
                        };
                    } else {
                        *state = ServiceState::Restarting { 
                            restart_count: count 
                        };
                        // Restart...
                    }
                }
                Ok(None) => {
                    // Still running - reset if stable
                    state.reset_restart_count_if_stable(Duration::from_secs(300));
                }
                Err(e) => { /* ... */ }
            }
        }
    }
}
```

### Option 2: Separate Monitoring State

```rust
pub struct ServiceMonitorState {
    pub restart_count: usize,
    pub last_start: SystemTime,
    pub last_crash: Option<SystemTime>,
    pub total_crashes: usize,
}

pub struct ServiceManager {
    services: HashMap<String, ServiceState>,
    monitor_state: HashMap<String, ServiceMonitorState>,
    // ...
}
```

**Pros**: Clean separation of concerns  
**Cons**: Two HashMaps to keep in sync

### Option 3: Persistent Storage

Store restart counts in a file/database:

```rust
// On crash:
let count = db.get_restart_count(service_name).unwrap_or(0) + 1;
db.set_restart_count(service_name, count);

// On successful uptime:
db.reset_restart_count(service_name);
```

**Pros**: Survives daemon restarts  
**Cons**: I/O overhead, more complex

## Recommended Approach

**Option 1** (add to ServiceState) because:
- Restart count is logically part of service state
- No synchronization needed (once Issue #1 fixed)
- Visible in Status command
- Survives monitor task recreation
- Simple to implement

Plus **stability-based reset**: After 5 minutes of successful running, reset restart_count to 0.

## Required Changes

1. Update `ServiceState` enum to include restart_count in all variants
2. Add helper methods: `increment_restart_count()`, `reset_restart_count_if_stable()`, `restart_count()`
3. Update `monitor_service()` to use state-based restart_count
4. Update `handle_command()` Restart to increment restart_count
5. Update `spawn_service()` to initialize restart_count
6. Add last_start timestamp to track uptime
7. Update Status command to show restart_count
8. Add config option for stability_duration (default 300s)

## Configuration

```rust
pub struct ServiceConfig {
    // ... existing fields
    pub max_restarts: usize,              // Existing
    pub check_interval: Duration,         // Existing
    pub stability_duration_secs: u64,     // NEW: default 300
}
```

## Testing Strategy

- Service crashes 5 times rapidly - verify stopped after max_restarts
- Service crashes, then runs for 6 minutes, then crashes - verify restart_count reset
- Manual restart via command - verify restart_count incremented
- Status command shows restart_count
- Monitor task recreated - verify restart_count preserved

## Related Issues

- Issue #1: HashMap synchronization needed
- Issue #8: Multiple monitor tasks (makes this worse)
- Issue #16: Status command doesn't return data
