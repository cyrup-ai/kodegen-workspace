# CRITICAL: Signal Handler Thread Panic Not Caught

## Severity
**CRITICAL - DAEMON STABILITY**

## Location
`packages/kodegend/src/platform/signal.rs:48-111, 123-197`

## Issue Description
The signal watcher threads spawned by `spawn_unix_watcher()` and `spawn_windows_watcher()` have no panic handling. If a thread panics during signal handling, it silently disappears, leaving the daemon without signal handling capability.

### Problems
1. **Silent failure**: Panicking thread disappears without notification
2. **No recovery**: Once thread panics, signal handling stops permanently
3. **Hard to debug**: No panic message or stack trace captured
4. **Production impact**: Daemon cannot be stopped gracefully (SIGTERM ignored)

### Panic Scenarios
- Tokio runtime panics during signal handling
- Channel send panics (unlikely but possible with poisoned channel)
- Future panics in tokio::select! branches
- Out of memory during signal handling

### Code Analysis
```rust
fn spawn_unix_watcher(tx: Sender<SignalKind>) -> Result<()> {
    thread::Builder::new()
        .name("signal-watcher-unix".to_string())
        .spawn(move || {  // ← No panic handling
            let rt = match tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            {
                Ok(rt) => rt,
                Err(e) => {
                    log::error!("Failed to create tokio runtime: {}", e);
                    return;  // ← Exits thread silently
                }
            };
            
            rt.block_on(async {
                // ... signal handling loop ...
                // If this panics, thread exits with no trace
            });
        })
        .context("Failed to spawn Unix signal watcher thread")?;
    
    Ok(())
}
```

## Recommended Fix

### Option 1: Catch panics with std::panic::catch_unwind
```rust
use std::panic::{self, AssertUnwindSafe};

fn spawn_unix_watcher(tx: Sender<SignalKind>) -> Result<()> {
    thread::Builder::new()
        .name("signal-watcher-unix".to_string())
        .spawn(move || {
            let result = panic::catch_unwind(AssertUnwindSafe(|| {
                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .expect("Failed to create tokio runtime for signal handling");
                
                rt.block_on(async {
                    // ... signal handling ...
                });
            }));
            
            if let Err(panic_payload) = result {
                log::error!("Signal watcher thread panicked: {:?}", panic_payload);
                // Optionally: attempt restart or notify main thread
            }
        })
        .context("Failed to spawn Unix signal watcher thread")?;
    
    Ok(())
}
```

### Option 2: Set custom panic hook
```rust
fn spawn_unix_watcher(tx: Sender<SignalKind>) -> Result<()> {
    thread::Builder::new()
        .name("signal-watcher-unix".to_string())
        .spawn(move || {
            // Set thread-local panic hook
            let default_hook = panic::take_hook();
            panic::set_hook(Box::new(move |panic_info| {
                log::error!("CRITICAL: Signal watcher thread panicked: {}", panic_info);
                // Call default hook for backtrace
                default_hook(panic_info);
            }));
            
            // ... rest of signal handling ...
        })
        .context("Failed to spawn Unix signal watcher thread")?;
    
    Ok(())
}
```

### Option 3: Auto-restart on panic
```rust
fn spawn_unix_watcher(tx: Sender<SignalKind>) -> Result<()> {
    thread::Builder::new()
        .name("signal-watcher-unix".to_string())
        .spawn(move || {
            let mut restart_count = 0;
            const MAX_RESTARTS: u32 = 3;
            
            loop {
                let tx_clone = tx.clone();
                let result = panic::catch_unwind(AssertUnwindSafe(|| {
                    run_signal_watcher(tx_clone);
                }));
                
                match result {
                    Ok(()) => break, // Normal exit
                    Err(e) => {
                        restart_count += 1;
                        log::error!("Signal watcher panic #{}: {:?}", restart_count, e);
                        
                        if restart_count >= MAX_RESTARTS {
                            log::error!("Signal watcher failed {} times, giving up", MAX_RESTARTS);
                            break;
                        }
                        
                        thread::sleep(Duration::from_secs(1));
                        log::info!("Restarting signal watcher (attempt {})", restart_count);
                    }
                }
            }
        })
        .context("Failed to spawn Unix signal watcher thread")?;
    
    Ok(())
}
```

## Impact
- **Severity**: CRITICAL - Loss of signal handling
- **Probability**: LOW - Requires runtime panic
- **Consequence**: Daemon cannot be stopped gracefully
- **User Impact**: Must use SIGKILL to stop daemon

## Testing
1. Inject panic into signal handler code
2. Send SIGTERM to daemon
3. Verify panic is logged
4. Verify daemon behavior (restart or exit cleanly)

## Files to Modify
- `packages/kodegend/src/platform/signal.rs`
- Both `spawn_unix_watcher()` and `spawn_windows_watcher()`
