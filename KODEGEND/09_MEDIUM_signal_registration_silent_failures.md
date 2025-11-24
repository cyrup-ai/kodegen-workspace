# MEDIUM: Silent Signal Handler Registration Failures

## Severity
**MEDIUM - RELIABILITY**

## Location
- `packages/kodegend/src/platform/signal.rs:66-88` (Unix)
- `packages/kodegend/src/platform/signal.rs:140-170` (Windows)

## Issue Description
When signal handler registration fails (e.g., `signal()` returns error), the code logs an error and continues with partial signal handling. This means some signals work while others silently fail, with no indication to the caller.

### Problem Code - Unix
```rust
let mut sigterm = match signal(TokioSignalKind::terminate()) {
    Ok(s) => s,
    Err(e) => {
        log::error!("Failed to install SIGTERM handler: {}", e);
        return;  // ← Exits thread, but watch_signals() already returned Ok
    }
};

let mut sigint = match signal(TokioSignalKind::interrupt()) {
    Ok(s) => s,
    Err(e) => {
        log::error!("Failed to install SIGINT handler: {}", e);
        return;  // ← SIGTERM works but SIGINT doesn't
    }
};
```

### Problem Code - Windows
Similar pattern at lines 140-170 for CTRL+C, CTRL+BREAK, CTRL+CLOSE, CTRL+SHUTDOWN.

### Issues

1. **Partial functionality**: Some signals work, others don't
2. **No caller notification**: `watch_signals()` returns `Ok(rx)` even if registration failed
3. **Race condition**: Thread might fail after `watch_signals()` returns
4. **Hard to debug**: Caller assumes all signals registered, but SIGTERM might not work
5. **Silent degradation**: Daemon appears to start successfully but can't be stopped

### Real-World Scenario
1. System has signal handler limit (uncommon but possible)
2. SIGTERM registration fails
3. `watch_signals()` returns `Ok(rx)`
4. Daemon starts successfully
5. User sends SIGTERM to stop daemon
6. Daemon doesn't respond (handler not installed)
7. User must use SIGKILL

## Recommended Fixes

### Option 1: Wait for confirmation before returning
```rust
use crossbeam_channel::{bounded, Sender, Receiver};

enum SignalWatcherStatus {
    Ready,
    Error(String),
}

pub fn watch_signals() -> Result<Receiver<SignalKind>> {
    let (tx, rx) = bounded::<SignalKind>(16);
    let (status_tx, status_rx) = bounded::<SignalWatcherStatus>(1);
    
    #[cfg(unix)]
    spawn_unix_watcher(tx, status_tx)?;
    
    // Wait for watcher to confirm it's ready
    match status_rx.recv_timeout(Duration::from_secs(5)) {
        Ok(SignalWatcherStatus::Ready) => Ok(rx),
        Ok(SignalWatcherStatus::Error(e)) => {
            Err(anyhow!("Signal watcher failed to start: {}", e))
        }
        Err(_) => {
            Err(anyhow!("Signal watcher timed out during startup"))
        }
    }
}

fn spawn_unix_watcher(
    tx: Sender<SignalKind>,
    status_tx: Sender<SignalWatcherStatus>,
) -> Result<()> {
    thread::Builder::new()
        .name("signal-watcher-unix".to_string())
        .spawn(move || {
            let rt = match tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            {
                Ok(rt) => rt,
                Err(e) => {
                    let _ = status_tx.send(SignalWatcherStatus::Error(
                        format!("Failed to create runtime: {}", e)
                    ));
                    return;
                }
            };
            
            rt.block_on(async {
                // Register all signals
                let mut sigterm = match signal(TokioSignalKind::terminate()) {
                    Ok(s) => s,
                    Err(e) => {
                        let _ = status_tx.send(SignalWatcherStatus::Error(
                            format!("Failed to install SIGTERM: {}", e)
                        ));
                        return;
                    }
                };
                
                // ... register other signals ...
                
                // All signals registered successfully
                let _ = status_tx.send(SignalWatcherStatus::Ready);
                
                // Run signal loop
                loop {
                    // ... existing signal handling ...
                }
            });
        })
        .context("Failed to spawn Unix signal watcher thread")?;
    
    Ok(())
}
```

### Option 2: Collect registration errors and proceed with best effort
```rust
fn spawn_unix_watcher(tx: Sender<SignalKind>) -> Result<()> {
    thread::Builder::new()
        .name("signal-watcher-unix".to_string())
        .spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Failed to create tokio runtime");
            
            rt.block_on(async {
                let mut registered_signals = Vec::new();
                let mut failed_signals = Vec::new();
                
                // Try to register each signal
                let sigterm = match signal(TokioSignalKind::terminate()) {
                    Ok(s) => {
                        registered_signals.push("SIGTERM");
                        Some(s)
                    }
                    Err(e) => {
                        failed_signals.push(format!("SIGTERM: {}", e));
                        None
                    }
                };
                
                // ... register other signals similarly ...
                
                if !failed_signals.is_empty() {
                    log::warn!(
                        "Failed to register signals: {:?}. Continuing with: {:?}",
                        failed_signals,
                        registered_signals
                    );
                }
                
                if registered_signals.is_empty() {
                    log::error!("No signals registered successfully - signal handling disabled");
                    return;
                }
                
                // Build select! with only successfully registered signals
                loop {
                    tokio::select! {
                        _ = sigterm.as_mut().unwrap().recv(), if sigterm.is_some() => {
                            let _ = tx.send(SignalKind::Terminate);
                        }
                        // ... other signals ...
                    }
                }
            });
        })
        .context("Failed to spawn Unix signal watcher thread")?;
    
    Ok(())
}
```

### Option 3: Fail fast if critical signals can't be registered
```rust
fn spawn_unix_watcher(tx: Sender<SignalKind>) -> Result<()> {
    thread::Builder::new()
        .name("signal-watcher-unix".to_string())
        .spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Failed to create tokio runtime");
            
            rt.block_on(async {
                // Critical signals - must succeed
                let mut sigterm = signal(TokioSignalKind::terminate())
                    .expect("CRITICAL: Failed to install SIGTERM handler");
                
                let mut sigint = signal(TokioSignalKind::interrupt())
                    .expect("CRITICAL: Failed to install SIGINT handler");
                
                // Non-critical signals - log but continue
                let mut sighup = match signal(TokioSignalKind::hangup()) {
                    Ok(s) => Some(s),
                    Err(e) => {
                        log::warn!("Failed to install SIGHUP handler: {}", e);
                        None
                    }
                };
                
                log::info!("Signal handlers installed successfully");
                
                loop {
                    tokio::select! {
                        _ = sigterm.recv() => {
                            let _ = tx.send(SignalKind::Terminate);
                        }
                        _ = sigint.recv() => {
                            let _ = tx.send(SignalKind::Interrupt);
                        }
                        _ = sighup.as_mut().unwrap().recv(), if sighup.is_some() => {
                            let _ = tx.send(SignalKind::Hangup);
                        }
                    }
                }
            });
        })
        .context("Failed to spawn Unix signal watcher thread")?;
    
    Ok(())
}
```

## Recommended Approach
**Option 3 (Fail fast)** is recommended:
- SIGTERM/SIGINT are critical - daemon must respond to these
- SIGHUP is optional (config reload)
- Clear failure mode - daemon won't start if critical signals fail
- Better than silent degradation

## Testing
1. Exhaust signal handler limit (if possible on test system)
2. Verify daemon fails to start with clear error
3. Verify log shows which signal failed to register

## Impact
- **Severity**: MEDIUM - Daemon may not respond to signals
- **Probability**: LOW - Signal registration rarely fails
- **User Impact**: Cannot stop daemon gracefully
- **Debugging**: Difficult to diagnose

## Files to Modify
- `packages/kodegend/src/platform/signal.rs`

## Related Issues
- Related to #02 (thread leak) and #03 (panic handling)
- All three should be addressed together for robust signal handling
