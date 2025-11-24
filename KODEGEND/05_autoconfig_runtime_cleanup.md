# Resource Leak Risk: Implicit Tokio Runtime Shutdown

## Severity
**MEDIUM** - Potential resource leak on error paths

## Location
`packages/kodegend/src/service/autoconfig.rs`
- Line 32: Runtime creation
- Line 88: Channel recv with `?` operator (early return risk)
- Lines 148-154: Cleanup logic
- Line 156: Implicit runtime drop

## Issue Description

The `AutoConfigService::run()` function creates a Tokio runtime and spawns an async task, but relies on implicit runtime drop behavior. On error paths, the function can return early before executing cleanup logic, causing the runtime to drop while still having an active spawned task. This can block indefinitely during the implicit drop.

### Current Code Pattern (Lines 28-156)

```rust
pub fn run(self, cmd_rx: Receiver<Cmd>) -> Result<()> {
    info!("🍯 Starting MCP client auto-configuration service");

    // Line 32: Create tokio runtime for the watcher
    let rt = tokio::runtime::Runtime::new()?;

    // Create cancellation token for graceful shutdown
    let cancel_token = CancellationToken::new();
    let shutdown_complete = Arc::new(AtomicBool::new(false));

    // ... watcher creation and spawning (lines 39-84) ...
    let watcher_handle = rt.spawn({ /* async task */ });

    // Handle control commands with lock-free coordination
    loop {
        match cmd_rx.recv()? {  // ⚠️ LINE 88: Early return on error!
            Cmd::Start => {
                info!("Auto-config service already started");
            }
            Cmd::Stop => {
                // ... trigger cancellation and wait ...
                break;
            }
            Cmd::Shutdown => {
                // ... trigger cancellation and wait ...
                break;
            }
            _ => {}
        }
    }

    // Lines 148-154: Cleanup logic (SKIPPED on early return!)
    if !shutdown_complete.load(Ordering::Acquire) {
        watcher_handle.abort();
    }
    let _ = rt.block_on(watcher_handle);

    Ok(())  // Line 156: Runtime drops here implicitly
}
```

## Root Cause Analysis

### The Problem: Line 88

```rust
match cmd_rx.recv()? {  // ⚠️ The `?` operator causes early return
    // ...
}
```

When `cmd_rx.recv()` returns an `Err` (e.g., channel disconnected), the `?` operator propagates the error and **returns immediately from the function**. This skips:
1. Lines 148-150: Task abortion check
2. Line 154: Final cleanup with `rt.block_on(watcher_handle)`
3. Any explicit runtime shutdown

The runtime then drops implicitly at line 156 while the spawned watcher task may still be running, causing the drop to block waiting for task completion.

### Production Impact Scenarios

#### 1. Normal Path (Happy Case)
- Loop breaks on `Cmd::Stop` or `Cmd::Shutdown`
- Falls through to cleanup logic (lines 148-154)
- Runtime drops cleanly after task completion
- ✅ No issue

#### 2. Channel Disconnect (Error Path)
- Parent thread drops `cmd_tx` (sender side)
- `cmd_rx.recv()` returns `RecvError`
- `?` operator returns early from function
- Cleanup logic (lines 148-154) is **SKIPPED**
- Runtime drops with active task still running
- ❌ **Potential indefinite block**

#### 3. Future Error Paths
- Any new code that adds early returns before cleanup
- Will inherit the same resource leak risk
- Fragile pattern for future maintenance

## Recommended Solution: Option 3 (Handle Channel Errors Gracefully)

### Why Option 3?

After analyzing the codebase architecture, **Option 3 is the recommended approach** because:

1. **Zero-allocation hot path**: Aligns with kodegen's performance philosophy (see [kodegen-simd](../packages/kodegen-simd) and [kodegen daemon patterns](../packages/kodegend/src/daemon.rs))
2. **No new dependencies**: Uses existing infrastructure (`cancel_token`, `shutdown_complete`)
3. **Idiomatic pattern**: Treating channel disconnect as shutdown signal is standard for service coordination
4. **Explicit control flow**: Clear and maintainable compared to defer guards
5. **Already used elsewhere**: Matches patterns in [service.rs](../packages/kodegend/src/service.rs) for service lifecycle management

### Alternative Options (Not Recommended)

**Option 1: Scopeguard (Available but suboptimal)**
- `scopeguard = "1.2"` is already a dependency (see [Cargo.toml line 56](../packages/kodegend/Cargo.toml#L56))
- Could use defer pattern with `scopeguard::guard()`
- ❌ Adds runtime overhead for guard allocation
- ❌ Less explicit about cleanup timing
- ❌ Doesn't match kodegen's zero-allocation philosophy

**Option 2: Manual wrapper (Redundant)**
- Wrap entire function body in closure
- ❌ More complex than needed
- ❌ Doesn't improve clarity

## Implementation Instructions

### File to Modify
`packages/kodegend/src/service/autoconfig.rs`

### Exact Changes Required

**Current code (lines 87-146):**

```rust
// Handle control commands with lock-free coordination
loop {
    match cmd_rx.recv()? {  // ⚠️ REMOVE the `?` operator
        Cmd::Start => {
            info!("Auto-config service already started");
        }
        Cmd::Stop => {
            info!("Stopping auto-config service");
            
            // Trigger graceful cancellation
            cancel_token.cancel();
            
            // Wait for shutdown completion with timeout
            let shutdown_timeout = std::time::Duration::from_secs(5);
            let start_time = std::time::Instant::now();
            
            // Spin-wait with exponential backoff for shutdown completion
            let mut backoff_ms = 1;
            while !shutdown_complete.load(Ordering::Acquire) {
                if start_time.elapsed() > shutdown_timeout {
                    info!("Graceful shutdown timeout, aborting task");
                    watcher_handle.abort();
                    break;
                }
                
                // Lock-free backoff using thread sleep
                std::thread::sleep(std::time::Duration::from_millis(backoff_ms));
                backoff_ms = (backoff_ms * 2).min(100); // Cap at 100ms
            }
            
            break;
        }
        Cmd::Shutdown => {
            // ... similar pattern ...
            break;
        }
        _ => {}
    }
}
```

**Replace with:**

```rust
// Handle control commands with lock-free coordination
loop {
    let cmd = match cmd_rx.recv() {  // ✅ CHANGED: Remove `?`, handle error explicitly
        Ok(cmd) => cmd,
        Err(_) => {
            // Channel disconnected - treat as shutdown signal
            info!("Command channel disconnected, initiating graceful shutdown");
            
            // Trigger graceful cancellation
            cancel_token.cancel();
            
            // Wait for shutdown completion with timeout
            let shutdown_timeout = std::time::Duration::from_secs(5);
            let start_time = std::time::Instant::now();
            
            // Spin-wait with exponential backoff for shutdown completion
            let mut backoff_ms = 1;
            while !shutdown_complete.load(Ordering::Acquire) {
                if start_time.elapsed() > shutdown_timeout {
                    info!("Graceful shutdown timeout, aborting task");
                    watcher_handle.abort();
                    break;
                }
                
                // Lock-free backoff using thread sleep
                std::thread::sleep(std::time::Duration::from_millis(backoff_ms));
                backoff_ms = (backoff_ms * 2).min(100); // Cap at 100ms
            }
            
            break;  // ✅ Fall through to cleanup logic
        }
    };
    
    match cmd {  // ✅ CHANGED: Now matching on the extracted cmd value
        Cmd::Start => {
            info!("Auto-config service already started");
        }
        Cmd::Stop => {
            info!("Stopping auto-config service");
            
            // Trigger graceful cancellation
            cancel_token.cancel();
            
            // Wait for shutdown completion with timeout
            let shutdown_timeout = std::time::Duration::from_secs(5);
            let start_time = std::time::Instant::now();
            
            // Spin-wait with exponential backoff for shutdown completion
            let mut backoff_ms = 1;
            while !shutdown_complete.load(Ordering::Acquire) {
                if start_time.elapsed() > shutdown_timeout {
                    info!("Graceful shutdown timeout, aborting task");
                    watcher_handle.abort();
                    break;
                }
                
                // Lock-free backoff using thread sleep
                std::thread::sleep(std::time::Duration::from_millis(backoff_ms));
                backoff_ms = (backoff_ms * 2).min(100); // Cap at 100ms
            }
            
            break;
        }
        Cmd::Shutdown => {
            info!("Shutting down auto-config service");
            
            // Trigger graceful cancellation
            cancel_token.cancel();
            
            // Wait for shutdown completion with timeout
            let shutdown_timeout = std::time::Duration::from_secs(5);
            let start_time = std::time::Instant::now();
            
            // Spin-wait with exponential backoff for shutdown completion
            let mut backoff_ms = 1;
            while !shutdown_complete.load(Ordering::Acquire) {
                if start_time.elapsed() > shutdown_timeout {
                    info!("Graceful shutdown timeout, aborting task");
                    watcher_handle.abort();
                    break;
                }
                
                // Lock-free backoff using thread sleep
                std::thread::sleep(std::time::Duration::from_millis(backoff_ms));
                backoff_ms = (backoff_ms * 2).min(100); // Cap at 100ms
            }
            
            break;
        }
        _ => {}
    }
}
```

### Key Changes Summary

1. **Line 88**: Change `match cmd_rx.recv()?` to `let cmd = match cmd_rx.recv()`
2. **Add error handling**: Handle `Err(_)` case by treating it as shutdown signal
3. **Duplicate shutdown logic**: Copy the cancellation + timeout waiting pattern into the error case
4. **Add break**: Ensure error case breaks from loop to reach cleanup
5. **Nested match**: Move command handling to a nested `match cmd` block

### Why This Works

- **Channel disconnect** → Triggers graceful shutdown sequence
- **cancel_token.cancel()** → Signals async task to stop
- **Spin-wait with timeout** → Waits for task to acknowledge cancellation
- **break** → Exits loop and falls through to existing cleanup (lines 148-154)
- **Cleanup logic** → Aborts task if needed, waits for completion
- **Runtime drops cleanly** → Only after all tasks are finished

## Definition of Done

The task is complete when:

1. ✅ Line 88 no longer uses the `?` operator
2. ✅ `cmd_rx.recv()` errors are handled with explicit `match` 
3. ✅ Error case triggers cancellation via `cancel_token.cancel()`
4. ✅ Error case waits for shutdown with timeout + backoff
5. ✅ Error case breaks from loop to reach cleanup logic (lines 148-154)
6. ✅ All command handling is moved to nested `match cmd` block
7. ✅ Code compiles without warnings: `cargo check -p kodegend`
8. ✅ Code passes clippy: `cargo clippy -p kodegend`

## Additional Context

### Related Files
- [kodegend/src/service/autoconfig.rs](../packages/kodegend/src/service/autoconfig.rs) - Main file to modify
- [kodegend/src/ipc.rs](../packages/kodegend/src/ipc.rs) - Defines `Cmd` and `Evt` types
- [kodegend/src/service.rs](../packages/kodegend/src/service.rs) - Service lifecycle patterns
- [kodegend/Cargo.toml](../packages/kodegend/Cargo.toml) - Dependencies (scopeguard available but not used)

### Architecture Notes

The autoconfig service uses a **double-runtime pattern**:
1. **Blocking thread runtime**: The `run()` function executes in a dedicated OS thread (spawned by [spawn_autoconfig](../packages/kodegend/src/service/autoconfig.rs#L161-L182))
2. **Async runtime**: Created on line 32 specifically for the async `AutoConfigWatcher`

This pattern is necessary because:
- The service must coordinate with other services via **crossbeam channels** (blocking)
- The watcher uses async file system operations (**tokio::fs**)
- The `cmd_rx` receiver is synchronous (not `async_recv()`)

### Concurrency Coordination

The code uses **lock-free atomic coordination**:
- `cancel_token: CancellationToken` - Async cancellation signal
- `shutdown_complete: Arc<AtomicBool>` - Task completion flag  
- Spin-wait with exponential backoff (1ms → 2ms → 4ms → ... → 100ms)

This avoids mutexes and condition variables for maximum performance in the daemon's hot path.

### Why Not Use `rt.shutdown_timeout()`?

The task proposes using `rt.shutdown_timeout(Duration::from_secs(10))` in the defer pattern, but this is **not used in the recommended solution** because:

1. The cleanup already handles task abortion explicitly (line 150)
2. The cleanup already waits for task completion (line 154)
3. Implicit runtime drop after cleanup is safe (no active tasks remain)
4. Adding explicit shutdown would be redundant overhead

The real fix is **ensuring we always reach the cleanup logic**, not adding more cleanup.
