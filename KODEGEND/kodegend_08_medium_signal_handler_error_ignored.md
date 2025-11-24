# MEDIUM: Signal Handler Send Error Silently Ignored

## Severity
**MEDIUM** - Masks shutdown failures and complicates debugging

## Location
`packages/kodegend/src/main.rs:122-130`

## Issue Description
The signal handler task silently ignores errors when sending shutdown signal to the main task. If the channel is closed (main task panicked or dropped receiver), the error is completely ignored, preventing proper error tracking and debugging.

## Current Code
```rust
// Spawn signal handler task
let signal_shutdown_tx = shutdown_tx.clone();
tokio::spawn(async move {
    tokio::select! {
        _ = sigterm.recv() => {
            info!("Received SIGTERM");
        }
        _ = sigint.recv() => {
            info!("Received SIGINT");
        }
    }
    let _ = signal_shutdown_tx.send(()).await;  // ← ERROR SILENTLY IGNORED
});
```

## Problem
The `let _ = ...` pattern discards the `Result<(), SendError>` without checking. This can hide critical issues:

### When Does send() Fail?
```rust
// mpsc::Sender::send returns SendError when:
// 1. All receivers have been dropped
// 2. Channel is closed
```

### Why Would Receiver Be Dropped?
1. **Main task panicked**: Receiver dropped, channel closed
2. **Early shutdown**: Main completes before signal arrives
3. **Logic error**: Receiver accidentally dropped
4. **Memory corruption**: Rare but possible

## Production Scenarios

### Scenario 1: Main Task Panic
```
Timeline:
1. Daemon starts successfully
2. Main task panics due to bug in service manager
3. shutdown_rx dropped, channel closes
4. Later, SIGTERM received
5. Signal handler tries to send shutdown signal
6. send() fails with SendError (channel closed)
7. Error silently ignored
8. No log entry about the failure
9. Daemon appears to ignore signals

Debugging difficulty: HIGH
- No indication signal was received
- No error logged
- Appears as "daemon not responding to signals"
- Actual issue: daemon already crashed
```

### Scenario 2: Race on Fast Shutdown
```
Timeline:
1. Service stops very quickly
2. Main task completes, drops shutdown_rx
3. SIGTERM arrives milliseconds later
4. Signal handler sends to closed channel
5. Error ignored
6. No visibility that signal was received

Expected: Log that signal arrived but daemon already stopped
Actual: Silent failure, no log entry
```

### Scenario 3: Multiple Signals
```
User hits Ctrl+C multiple times:
1. First SIGINT: Successfully sent to main task
2. Main task begins shutdown
3. Second SIGINT arrives during shutdown
4. Signal handler tries to send again
5. Channel might be closed or full
6. Error ignored

Expected: Log "shutdown already in progress"
Actual: Silent
```

## Impact on Debugging
When investigating why daemon didn't shut down gracefully:

```
Support: "What happened when you sent SIGTERM?"
Logs: [No mention of SIGTERM being received]
Support: "Did the daemon receive the signal?"
Answer: Unknown - error was silently ignored
```

Proper logging would show:
```
INFO Received SIGTERM
ERROR Failed to send shutdown signal: channel closed (main task may have panicked)
```

## Recommended Fix

### Option 1: Log Error (Minimal Change)
```rust
tokio::spawn(async move {
    tokio::select! {
        _ = sigterm.recv() => {
            info!("Received SIGTERM");
        }
        _ = sigint.recv() => {
            info!("Received SIGINT");
        }
    }
    
    if let Err(e) = signal_shutdown_tx.send(()).await {
        error!("Failed to send shutdown signal: {}", e);
        error!("Main task may have panicked or already exited");
        // Force exit as last resort
        std::process::exit(1);
    } else {
        debug!("Shutdown signal sent successfully");
    }
});
```

### Option 2: Try Sync Send Then Force Exit
```rust
tokio::spawn(async move {
    tokio::select! {
        _ = sigterm.recv() => {
            info!("Received SIGTERM");
        }
        _ = sigint.recv() => {
            info!("Received SIGINT");
        }
    }
    
    match signal_shutdown_tx.send(()).await {
        Ok(()) => {
            debug!("Shutdown signal sent to main task");
        }
        Err(mpsc::error::SendError(_)) => {
            warn!("Cannot send shutdown signal - channel closed");
            warn!("Main task appears to have exited or panicked");
            warn!("Forcing immediate exit");
            
            // Attempt cleanup before exit
            // Note: Config path is not available here (design issue)
            
            // Force exit
            std::process::exit(1);
        }
    }
});
```

### Option 3: Keep Task Handle for Join (Better Architecture)
```rust
// In run_service_manager():
let signal_task = tokio::spawn(async move {
    tokio::select! {
        _ = sigterm.recv() => {
            info!("Received SIGTERM");
        }
        _ = sigint.recv() => {
            info!("Received SIGINT");
        }
    }
    signal_shutdown_tx.send(()).await
});

// Later, when shutting down:
match signal_task.await {
    Ok(Ok(())) => {
        debug!("Signal handler completed successfully");
    }
    Ok(Err(e)) => {
        error!("Signal handler failed to send: {}", e);
    }
    Err(e) => {
        error!("Signal handler task panicked: {}", e);
    }
}
```

**Option 1 is simplest and addresses the immediate issue.**

## Alternative: Use oneshot Channel
For shutdown signals, `oneshot` channel is more appropriate than `mpsc`:

```rust
use tokio::sync::oneshot;

let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

tokio::spawn(async move {
    tokio::select! {
        _ = sigterm.recv() => info!("Received SIGTERM"),
        _ = sigint.recv() => info!("Received SIGINT"),
    }
    
    // oneshot can only send once, which matches our semantics
    if let Err(_) = shutdown_tx.send(()) {
        error!("Shutdown signal channel closed - main task exited");
        std::process::exit(1);
    }
});

// Wait for shutdown
match shutdown_rx.await {
    Ok(()) => info!("Shutdown signal received"),
    Err(_) => warn!("Shutdown sender dropped without sending"),
}
```

Benefits:
- Clearer semantics (shutdown happens once)
- Slightly more efficient
- Better error messages

## What About Task Lifecycle?
The spawned task is not tracked, which has issues:

```rust
tokio::spawn(async move { /* ... */ });
// Task handle discarded - task becomes "fire and forget"
```

If main task exits immediately:
- Signal handler task becomes orphaned
- No way to await its completion
- Races with process exit

Better:
```rust
let signal_handle = tokio::spawn(async move { /* ... */ });

// During shutdown:
// Allow signal handler to complete if needed
tokio::time::timeout(
    Duration::from_secs(1),
    signal_handle
).await.ok();
```

## Testing Requirements
1. **Normal signal**: Send SIGTERM, verify clean shutdown
2. **Main task panic**: Panic main task, send SIGTERM, verify error logged
3. **Multiple signals**: Send SIGTERM twice, verify second is handled gracefully
4. **Channel closed**: Drop receiver, send signal, verify error logged
5. **Fast shutdown**: Complete shutdown before signal, verify logging
6. **Signal during shutdown**: Send signal mid-shutdown, verify no panic

## Monitoring Impact
Production monitoring often looks for specific log patterns:

```python
# Alert: daemon not responding to signals
if "SIGTERM" not in logs and signal_sent:
    alert("Daemon ignoring SIGTERM")
```

Current code makes this unreliable. With proper logging:
```
2024-01-15 10:30:45 INFO Received SIGTERM
2024-01-15 10:30:45 ERROR Failed to send shutdown signal: channel closed
```

Alert can be more specific:
```python
if "Failed to send shutdown signal" in logs:
    alert("Daemon main task crashed before signal handling")
```

## Related Issues
- Shutdown receiver ignores None (task file 09) - related channel handling
- No service shutdown timeout (task file 07) - affects shutdown timing
- If main task panics, entire error handling is compromised

## Documentation
Add to operations guide:

```markdown
### Daemon Not Responding to Signals

If SIGTERM/SIGINT logs appear but shutdown doesn't happen:

1. Check for "Failed to send shutdown signal" error
2. This indicates main task crashed before signal arrived
3. Look for panic or error logs before signal timestamp
4. Main task may have exited due to unhandled error

The signal was received correctly, but couldn't be processed
because the main task was already gone.
```

## Performance Considerations
- Logging adds ~microseconds overhead (negligible)
- process::exit(1) is immediate (no cleanup)
- For production daemon, correctness > performance here

## Code Quality
The pattern `let _ =` for errors should be rare in production code:

```rust
// OK for infallible operations:
let _ = writeln!(stderr, "Error: {}", e);  // Write might fail, that's fine

// NOT OK for critical operations:
let _ = database.commit();  // ❌ Silently ignores DB errors
let _ = shutdown_tx.send(());  // ❌ Silently ignores channel errors
```

A linter rule could catch this:
```toml
# .clippy.toml
disallowed-patterns = [
    { pattern = "let _ = .*\\.send\\(", message = "Don't ignore send errors" }
]
```
