# MEDIUM: Inefficient Tokio Runtime Creation Per Signal Watcher

## Severity
**MEDIUM - PERFORMANCE**

## Location
- `packages/kodegend/src/platform/signal.rs:52-54` (Unix)
- `packages/kodegend/src/platform/signal.rs:126-128` (Windows)

## Issue Description
Each signal watcher thread creates its own dedicated tokio runtime using `new_current_thread()`. While lighter than a multi-threaded runtime, this still has non-trivial overhead and duplicates resources if signal handling could use a shared runtime.

### Current Code
```rust
fn spawn_unix_watcher(tx: Sender<SignalKind>) -> Result<()> {
    thread::Builder::new()
        .name("signal-watcher-unix".to_string())
        .spawn(move || {
            // Create tokio runtime for signal handling
            let rt = match tokio::runtime::Builder::new_current_thread()  // ← Creates new runtime
                .enable_all()
                .build()
            {
                Ok(rt) => rt,
                Err(e) => {
                    log::error!("Failed to create tokio runtime: {}", e);
                    return;
                }
            };
            
            rt.block_on(async {
                // ... signal handling ...
            });
        })
        .context("Failed to spawn Unix signal watcher thread")?;
    
    Ok(())
}
```

### Overhead Analysis

**Runtime creation cost:**
- Thread-local storage allocation
- I/O driver initialization (epoll/kqueue/IOCP)
- Timer wheel initialization
- Signal handler setup (ironic - tokio also uses signals)
- Park/unpark synchronization primitives

**Per-runtime resources:**
- I/O reactor thread-local state
- Timer wheel data structures
- Task scheduler state
- Waker infrastructure

### Performance Impact
- Startup latency: ~1-5ms per runtime creation
- Memory: ~50-200KB per runtime (depends on platform)
- File descriptors: 1-2 per runtime (for I/O reactor)
- Ongoing overhead: Minimal once running

### When This Matters
- **Startup time**: Adds milliseconds to daemon startup
- **Resource-constrained environments**: Embedded systems, containers
- **Multiple instances**: If running many kodegend instances
- **Hot reload**: If recreating signal watchers (not current behavior)

## Recommended Fixes

### Option 1: Use global tokio runtime (if kodegend already has one)
```rust
// In main.rs or wherever global runtime is created
static RUNTIME: OnceCell<Runtime> = OnceCell::new();

pub fn get_runtime() -> &'static Runtime {
    RUNTIME.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed to create tokio runtime")
    })
}

// In signal.rs
fn spawn_unix_watcher(tx: Sender<SignalKind>) -> Result<()> {
    let runtime_handle = get_runtime().handle().clone();
    
    thread::Builder::new()
        .name("signal-watcher-unix".to_string())
        .spawn(move || {
            runtime_handle.block_on(async {
                // ... signal handling ...
            });
        })
        .context("Failed to spawn Unix signal watcher thread")?;
    
    Ok(())
}
```

### Option 2: Spawn directly on existing runtime (no dedicated thread)
```rust
pub fn watch_signals(runtime: &Runtime) -> Result<Receiver<SignalKind>> {
    let (tx, rx) = bounded::<SignalKind>(16);
    
    #[cfg(unix)]
    runtime.spawn(async move {
        run_unix_signal_watcher(tx).await
    });
    
    Ok(rx)
}

async fn run_unix_signal_watcher(tx: Sender<SignalKind>) {
    use tokio::signal::unix::{signal, SignalKind as TokioSignalKind};
    
    let mut sigterm = signal(TokioSignalKind::terminate())
        .expect("Failed to install SIGTERM");
    // ... register other signals ...
    
    loop {
        tokio::select! {
            _ = sigterm.recv() => {
                let _ = tx.send(SignalKind::Terminate);
            }
            // ... other signals ...
        }
    }
}
```

### Option 3: Lazy static runtime for signal handling only
```rust
use once_cell::sync::Lazy;

static SIGNAL_RUNTIME: Lazy<Runtime> = Lazy::new(|| {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Failed to create signal handler runtime")
});

fn spawn_unix_watcher(tx: Sender<SignalKind>) -> Result<()> {
    thread::Builder::new()
        .name("signal-watcher-unix".to_string())
        .spawn(move || {
            SIGNAL_RUNTIME.block_on(async {
                // ... signal handling ...
            });
        })
        .context("Failed to spawn Unix signal watcher thread")?;
    
    Ok(())
}
```

## Recommended Approach

**Check if kodegend has global runtime:**
- If YES: Use Option 1 (share global runtime)
- If NO: Current approach is acceptable, or use Option 3 (lazy static)

**Rationale:**
- Signal handling is async I/O, fits naturally in tokio
- Sharing runtime reduces overhead
- But dedicated thread for signals is reasonable for isolation
- Biggest win: Avoid creating runtime if one already exists

## Architectural Questions

Need to check:
1. Does kodegend main binary use tokio? (Check src/main.rs)
2. Is there a global runtime for HTTP servers or other async code?
3. Would sharing runtime affect signal handling reliability?

## Benchmark

Before fix:
```bash
time kodegend --startup-bench
# Measure runtime creation overhead
```

After fix:
```bash
time kodegend --startup-bench
# Should be ~1-5ms faster
```

## Impact
- **Severity**: MEDIUM - Performance optimization
- **Startup**: 1-5ms slower startup
- **Memory**: 50-200KB per instance
- **Priority**: LOW - Optimize after critical bugs fixed

## Files to Modify
- `packages/kodegend/src/platform/signal.rs`
- Possibly `packages/kodegend/src/main.rs` (check for existing runtime)

## Related Issues
- Related to #02 (thread management)
- Consider as part of overall signal handling refactor

## Notes
This is an optimization, not a bug. Current approach is functionally correct, just slightly inefficient. Prioritize after critical/high issues are resolved.
