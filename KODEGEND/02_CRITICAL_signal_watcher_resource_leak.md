# CRITICAL: Signal Watcher Thread Resource Leak

## Severity
**CRITICAL - RESOURCE LEAK**

## Location
`packages/kodegend/src/platform/signal.rs:30-40, 47-114, 120-201`

## Issue Description
The `watch_signals()` function spawns background threads that run indefinitely but provides no mechanism to join or clean up these threads. Thread handles are immediately dropped after spawn, creating orphaned threads.

### Problems
1. **Thread leak**: Each call to `watch_signals()` creates a new thread that cannot be stopped
2. **No cleanup**: Thread handles dropped at line 111 (Unix) and 198 (Windows)
3. **Multiple calls**: If `watch_signals()` called multiple times, creates multiple orphaned threads
4. **No graceful shutdown**: Threads run until process exit

### Code Analysis
```rust
pub fn watch_signals() -> Result<Receiver<SignalKind>> {
    let (tx, rx) = bounded::<SignalKind>(16);
    
    #[cfg(unix)]
    spawn_unix_watcher(tx)?;  // ← Thread handle dropped
    
    #[cfg(windows)]
    spawn_windows_watcher(tx)?;  // ← Thread handle dropped
    
    Ok(rx)  // ← Returns only Receiver, no way to stop thread
}

fn spawn_unix_watcher(tx: Sender<SignalKind>) -> Result<()> {
    thread::Builder::new()
        .name("signal-watcher-unix".to_string())
        .spawn(move || {  // ← Thread handle dropped here
            // ... infinite loop ...
        })
        .context("Failed to spawn Unix signal watcher thread")?;
    
    Ok(())  // ← No thread handle returned
}
```

### Resource Impact
- Each watcher consumes: 1 thread + 1 tokio runtime + signal handlers
- Threads never join, only exit when process terminates
- Multiple calls = multiple leaked threads

## Recommended Fix

### Option 1: Return cleanup handle
```rust
pub struct SignalWatcher {
    rx: Receiver<SignalKind>,
    thread_handle: Option<JoinHandle<()>>,
}

impl SignalWatcher {
    pub fn receiver(&self) -> &Receiver<SignalKind> {
        &self.rx
    }
    
    pub fn shutdown(mut self) -> Result<()> {
        drop(self.rx); // Close channel to signal thread to exit
        if let Some(handle) = self.thread_handle.take() {
            handle.join().map_err(|_| anyhow!("Thread panic"))?;
        }
        Ok(())
    }
}

pub fn watch_signals() -> Result<SignalWatcher> {
    let (tx, rx) = bounded::<SignalKind>(16);
    
    #[cfg(unix)]
    let handle = spawn_unix_watcher(tx)?;
    
    #[cfg(windows)]
    let handle = spawn_windows_watcher(tx)?;
    
    Ok(SignalWatcher {
        rx,
        thread_handle: Some(handle),
    })
}

fn spawn_unix_watcher(tx: Sender<SignalKind>) -> Result<JoinHandle<()>> {
    let handle = thread::Builder::new()
        .name("signal-watcher-unix".to_string())
        .spawn(move || {
            // ... existing code ...
        })
        .context("Failed to spawn Unix signal watcher thread")?;
    
    Ok(handle)  // Return handle
}
```

### Option 2: Global singleton watcher
- Create single global signal watcher
- Use lazy_static or OnceCell
- All callers share same watcher

## Current Behavior
- Dropping `Receiver` closes channel, causing thread to exit
- But thread handle is already dropped, so cannot join
- Thread exits cleanly but asynchronously

## Files to Modify
- `packages/kodegend/src/platform/signal.rs` - Add cleanup mechanism
- Call sites using `watch_signals()` - Update to use new API

## Impact
- **Severity**: HIGH - Resource leak
- **Frequency**: Depends on how often `watch_signals()` is called
- **Mitigation**: Currently only called once at daemon startup
- **Risk**: If called repeatedly, accumulates threads

## Testing
1. Call `watch_signals()` 100 times in loop
2. Check thread count with `ps -T` or `/proc/{pid}/status`
3. Verify threads are cleaned up after stopping watcher
