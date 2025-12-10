# Task: Verify ServiceManager::shutdown() Usage

## Priority: P2 (Investigation)

## Related Error
- `manager.rs:1905` - method `shutdown` is never used

## Problem Statement

The `ServiceManager::shutdown()` method is marked as potentially dead code:
```rust
#[cfg_attr(not(windows), allow(dead_code))]
pub fn shutdown(&self, _timeout: Duration) -> Result<()> {
    info!("Sending shutdown signal to ServiceManager");
    self.shutdown_tx
        .send(())
        .context("Failed to send shutdown signal - channel disconnected")?;
    info!("Shutdown signal sent successfully");
    Ok(())
}
```

The `#[cfg_attr(not(windows), allow(dead_code))]` indicates it's intended for Windows use.

## Analysis

### How Windows Service Shutdown Works

Looking at `src/platform/windows_service.rs`:

1. **Line 134**: `get_shutdown_sender()` is called to get a clone of the shutdown channel
2. **Line 205**: The cloned sender is used directly: `mgr_shutdown_tx.send(())`

The code uses `get_shutdown_sender()` and then calls `send()` directly, rather than calling `shutdown()` method.

### The shutdown() Method

```rust
pub fn shutdown(&self, _timeout: Duration) -> Result<()> {
    self.shutdown_tx.send(()).context("...")?;
    Ok(())
}
```

This method:
- Takes a timeout parameter (currently ignored)
- Sends on the shutdown channel
- Returns Result for error handling

### get_shutdown_sender() Method

```rust
#[cfg(windows)]
pub fn get_shutdown_sender(&self) -> crossbeam_channel::Sender<()> {
    self.shutdown_tx.clone()
}
```

This method:
- Returns a clone of the sender
- Allows external code to send shutdown signals

## Investigation Required

### Question 1: Why Two Methods?

The `shutdown()` method exists but isn't used. Instead, Windows code:
1. Gets the sender clone via `get_shutdown_sender()`
2. Sends directly on the channel

Is this intentional? Possible reasons:
- `shutdown()` was added later and integration wasn't updated
- Direct send is simpler and timeout isn't needed
- Code was refactored and `shutdown()` became orphaned

### Question 2: Should timeout Be Implemented?

The `_timeout` parameter is ignored. If shutdown timeout is important:
```rust
pub fn shutdown(&self, timeout: Duration) -> Result<()> {
    self.shutdown_tx.send(()).context("...")?;

    // Wait for shutdown to complete
    match self.shutdown_complete_rx.recv_timeout(timeout) {
        Ok(_) => Ok(()),
        Err(_) => bail!("Shutdown timed out after {:?}", timeout),
    }
}
```

This would require adding a completion channel.

## Required Action

### Option A: Use shutdown() Method (Recommended)

Update `windows_service.rs` to use the method:

Before (line 205):
```rust
if let Err(e) = mgr_shutdown_tx.send(()) {
    error!("Failed to send shutdown signal: {}", e);
}
```

After:
```rust
if let Err(e) = service_manager.shutdown(Duration::from_secs(5)) {
    error!("Failed to send shutdown signal: {}", e);
}
```

**Problem**: `service_manager` is moved into thread at line 141, so we can't call methods on it.

### Option B: Remove shutdown() Method

If `get_shutdown_sender()` is the intended pattern:
1. Remove `shutdown()` method
2. Document that `get_shutdown_sender()` is the correct approach

### Option C: Redesign for External Shutdown

Keep `shutdown()` for non-Windows use cases where the ServiceManager isn't moved:
```rust
// Unix daemon control
pub fn stop_daemon() {
    let manager = get_running_manager();
    manager.shutdown(Duration::from_secs(5))?;
}
```

## Files to Investigate

- `src/manager.rs` - ServiceManager implementation
- `src/platform/windows_service.rs` - Windows service integration
- `src/main.rs` - CLI stop command

## Acceptance Criteria

- [ ] Determine if `shutdown()` is needed
- [ ] Either use it or remove it
- [ ] No dead code warning
- [ ] Windows service shutdown still works correctly
