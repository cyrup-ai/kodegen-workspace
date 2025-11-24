# LOW: Receiver Option Consumed Without Checking if Already Running

## Priority
**LOW** - Edge case, poor design

## Location
`packages/kodegend/src/manager.rs` - `start()` method (lines 119-120, 135-139)

## Issue Description

The `start()` method consumes the receiver from an Option without checking if the command handler is already running. This allows calling `start()` multiple times, but only the first call actually starts the handler.

### Current Code

```rust
// Lines 119-120, 135-139
pub async fn start(&mut self) -> Result<()> {
    // ... spawn monitor tasks for configured services ...
    
    let receiver = self.receiver.take();  // Line 119
    if let Some(mut rx) = receiver {       // Line 120
        tokio::spawn(async move {
            while let Some(cmd) = rx.recv().await {
                // handle command
                if self.shutdown.load(Ordering::Relaxed) { break; }
            }
        });
    }
    // ...
}
```

### Problems

#### Problem 1: Multiple Calls to start()

```rust
let mut manager = ServiceManager::new(config);

manager.start().await?;  // First call - starts command handler
manager.start().await?;  // Second call - does nothing! receiver is None
manager.start().await?;  // Third call - does nothing!
```

**Issues**:
- Silent no-op on second call
- User might think they restarted the manager
- Confusing behavior

#### Problem 2: No Error on Re-Start

Second call to `start()` should either:
- Return an error: "Already started"
- Or be idempotent (no-op but intentional)

Current: Silent no-op, unclear if intentional.

#### Problem 3: No State Tracking

```rust
// Can't check if manager is running
if manager.is_running() {  // No such method
    // ...
}

// Must track externally
let mut is_started = false;
if !is_started {
    manager.start().await?;
    is_started = true;
}
```

**Issues**:
- No way to query if manager is running
- External state tracking required
- Error-prone

## Impact

### User Confusion

```rust
// CLI code
async fn cmd_start() -> Result<()> {
    let mut manager = get_manager()?;
    manager.start().await?;  // First time - works
    Ok(())
}

async fn cmd_restart() -> Result<()> {
    let mut manager = get_manager()?;
    manager.start().await?;  // Oops - does nothing if already started!
    Ok(())
}
```

User expects restart to work, but it silently fails.

### Operational Issues

If daemon crashes and auto-restarts:
- Systemd restarts process
- Calls `start()` again
- Second call does nothing (receiver already consumed)
- Daemon appears running but not processing commands
- Silent failure mode

## Root Cause

`receiver` stored as `Option<Receiver>` and consumed by `take()`. Design assumes `start()` called exactly once. No enforcement or state tracking.

## Solution

### Option 1: Return Error on Re-Start

```rust
pub struct ServiceManager {
    receiver: Option<Receiver<ServiceCommand>>,
    is_started: bool,  // NEW
    // ...
}

pub async fn start(&mut self) -> Result<()> {
    if self.is_started {
        return Err(anyhow!("ServiceManager already started"));
    }
    
    let receiver = self.receiver.take()
        .ok_or_else(|| anyhow!("ServiceManager already started (receiver consumed)"))?;
    
    tokio::spawn(async move {
        // ...
    });
    
    self.is_started = true;
    Ok(())
}

pub fn is_running(&self) -> bool {
    self.is_started && !self.shutdown.load(Ordering::Relaxed)
}
```

### Option 2: Make start() Idempotent

```rust
pub async fn start(&mut self) -> Result<()> {
    // If already started, just return Ok
    if self.is_started {
        debug!("ServiceManager already started, ignoring duplicate call");
        return Ok(());
    }
    
    // ... rest of start logic ...
    
    self.is_started = true;
    Ok(())
}
```

### Option 3: Store Task Handle

```rust
pub struct ServiceManager {
    receiver: Option<Receiver<ServiceCommand>>,
    command_handler: Option<JoinHandle<()>>,  // NEW
    // ...
}

pub async fn start(&mut self) -> Result<()> {
    if let Some(handle) = &self.command_handler {
        if !handle.is_finished() {
            return Err(anyhow!("ServiceManager already started"));
        }
    }
    
    let receiver = self.receiver.take()
        .ok_or_else(|| anyhow!("No receiver available"))?;
    
    let handle = tokio::spawn(async move {
        // ...
    });
    
    self.command_handler = Some(handle);
    Ok(())
}

pub fn is_running(&self) -> bool {
    self.command_handler
        .as_ref()
        .map(|h| !h.is_finished())
        .unwrap_or(false)
}
```

## Recommended Solution

**Option 3** (store task handle) because:
- Enables proper shutdown (can join task) - fixes Issue #4
- Provides accurate is_running() check
- Prevents duplicate starts
- Clean state management

This also helps with Issue #4 (shutdown mechanism).

## Required Changes

1. Add `command_handler: Option<JoinHandle<()>>` to ServiceManager
2. Store handle in `start()`
3. Check handle before starting
4. Add `is_running()` method
5. Use handle in `shutdown()` to join task (Issue #4)
6. Add tests for duplicate start()

## Testing

```rust
#[tokio::test]
async fn test_duplicate_start_returns_error() {
    let mut manager = ServiceManager::new(config);
    
    // First start - OK
    assert!(manager.start().await.is_ok());
    
    // Second start - Error
    assert!(manager.start().await.is_err());
}

#[tokio::test]
async fn test_is_running() {
    let mut manager = ServiceManager::new(config);
    
    // Not running initially
    assert!(!manager.is_running());
    
    // Running after start
    manager.start().await.unwrap();
    assert!(manager.is_running());
    
    // Not running after shutdown
    manager.shutdown().await.unwrap();
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert!(!manager.is_running());
}

#[tokio::test]
async fn test_restart_after_shutdown() {
    let mut manager = ServiceManager::new(config);
    
    manager.start().await.unwrap();
    manager.shutdown().await.unwrap();
    
    // Should be able to start again after shutdown
    // (requires recreating receiver - separate issue)
    // Current: Can't restart, receiver consumed
}
```

## Additional Issue: Can't Restart After Shutdown

Related problem: Once receiver is consumed, can't restart manager even after proper shutdown.

**Solution**: Don't consume receiver, or recreate it on restart.

```rust
pub struct ServiceManager {
    sender: Sender<ServiceCommand>,
    receiver: Receiver<ServiceCommand>,  // Not Option
    // ...
}

// Or:

pub async fn reset(&mut self) -> Result<()> {
    if self.is_running() {
        return Err(anyhow!("Cannot reset while running"));
    }
    
    // Recreate channel
    let (sender, receiver) = mpsc::channel(100);
    self.sender = sender;
    self.receiver = Some(receiver);
    
    Ok(())
}
```

## Related Issues

- Issue #4: Shutdown mechanism (needs task handle)
- Not storing task handles is a pattern throughout the code

## Severity

**LOW** because:
- Unlikely in practice (start() typically called once)
- No data corruption or crashes
- More of a design smell

But worth fixing when refactoring task handle management for Issue #4.
