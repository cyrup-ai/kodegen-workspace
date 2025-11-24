# Resource Leak: Channel Error Orphans Async Task

## Severity
**HIGH** - Active task left running on error path, causes resource leaks and prevents clean shutdown

## Location
- **File**: [`packages/kodegend/src/service/autoconfig.rs`](../packages/kodegend/src/service/autoconfig.rs)
- **Line 88**: `cmd_rx.recv()?` - The problematic error propagation

## Architectural Context

### Kodegend Service Architecture

Kodegend uses a **manager-worker architecture** with crossbeam channels for IPC:

- **Manager Thread** ([`src/manager.rs`](../packages/kodegend/src/manager.rs)): Main daemon process that spawns and controls service threads
- **Worker Threads**: Individual service implementations (autoconfig, embedded servers, port cleanup)
- **IPC Protocol** ([`src/ipc.rs`](../packages/kodegend/src/ipc.rs)):
  - `Cmd` enum: Commands sent FROM manager TO workers (`Start`, `Stop`, `Restart`, `Shutdown`, `TickHealth`, `TickLogRotate`)
  - `Evt` enum: Events sent FROM workers TO manager (`State`, `Health`, `LogRotate`, `Fatal`)
  - Channels created with `crossbeam_channel::bounded(16)`

### AutoConfig Service Specifics

The autoconfig service ([`src/service/autoconfig.rs`](../packages/kodegend/src/service/autoconfig.rs)) has a unique architecture:

1. **Thread-based Worker**: Runs in dedicated OS thread (spawned by `spawn_autoconfig()` at line 161)
2. **Embedded Tokio Runtime**: Creates its own tokio runtime (line 32) for async file watching
3. **Async Watcher Task**: Spawns a tokio task (line 43) that monitors MCP client installations
4. **Synchronous Command Loop**: Uses blocking `recv()` to process commands from manager (line 87-146)

This hybrid sync/async architecture requires careful cleanup coordination.

## Issue Description

The command receive loop uses the `?` operator to propagate channel errors, causing early return:

```rust
// Line 43-84: Spawn async watcher task
let watcher_handle = rt.spawn({ ... });

// Line 87-146: Command loop
loop {
    match cmd_rx.recv()? {  // ⚠️ ERROR PATH: Early return here bypasses cleanup!
        Cmd::Start => { ... }
        Cmd::Stop => { /* cleanup */ break; }
        Cmd::Shutdown => { /* cleanup */ break; }
        _ => {}
    }
}

// Lines 148-156: Cleanup ONLY reached if loop breaks normally
if !shutdown_complete.load(Ordering::Acquire) {
    watcher_handle.abort();
}
let _ = rt.block_on(watcher_handle);
```

### The Control Flow Bug

**Happy Path** (Normal shutdown via `Cmd::Stop` or `Cmd::Shutdown`):
1. Manager sends Stop/Shutdown command → `cmd_rx.recv()` returns `Ok(Cmd::Stop)`
2. Cleanup logic runs (lines 93-116 or 118-143)
3. Loop breaks with `break`
4. Final cleanup runs (lines 148-154)
5. Tokio runtime shuts down cleanly

**Broken Path** (Channel error):
1. Manager process dies or closes channel → `cmd_rx.recv()` returns `Err(RecvError)`
2. The `?` operator converts error to function return → **EARLY EXIT**
3. Lines 148-154 **NEVER EXECUTE**
4. Watcher task continues running orphaned
5. Tokio runtime never shuts down
6. Service thread exits but async work continues

## Production Impact

### Scenario: IPC Channel Disconnects

**Trigger Conditions:**
- Parent daemon process crashes or is killed (SIGKILL)
- Manager thread panics before graceful shutdown
- IPC bus closes unexpectedly
- Channel sender is dropped due to programming error

**Consequence Chain:**
1. `cmd_rx.recv()` returns `Err(crossbeam_channel::RecvError)`
2. The `?` operator propagates error, causing `run()` to return early
3. **Watcher task (`JoinHandle`) is NEVER cancelled or aborted**
4. **Tokio runtime is NEVER shut down** (no `rt.block_on(watcher_handle)`)
5. Task continues running orphaned in background
6. Service thread exits cleanly (from manager's perspective)
7. Background I/O and file system watches remain active

### Resource Leaks
- **Tokio Runtime**: Remains alive with active thread pool
- **Watcher Task**: Continues running `AutoConfigWatcher::run()`
- **File System Watches**: inotify/FSEvents/ReadDirectoryChangesW handles remain open
- **Memory**: Watcher state, runtime buffers, task stack frames
- **Thread Resources**: Tokio worker threads don't terminate
- **Process Exit**: Clean process termination may hang

## Root Cause Analysis

### Crossbeam Channel Behavior

From `crossbeam_channel` documentation:

> `Receiver::recv()` returns `Err(RecvError)` when all senders have been dropped, indicating the channel is disconnected and no more messages will be received.

This is a **permanent failure condition** - the channel cannot recover. The autoconfig service MUST shut down.

### Error Handling Anti-Pattern

The code exhibits a classic error handling anti-pattern:

**Implicit Assumption**: All error paths are equivalent to success paths
**Reality**: Error paths bypass critical cleanup code

This violates the **RAII principle** (Resource Acquisition Is Initialization) - resources acquired in function scope should be cleaned up before function exit, **regardless of how the function exits**.

## Solution Analysis

### Two Viable Approaches

Both approaches are valid; they represent different error handling philosophies:

#### Option 1: Explicit Error Handling (RECOMMENDED)

**Philosophy**: Treat channel error as a distinct, expected shutdown scenario

**Pros**:
- ✅ Explicit and readable - clear what happens on channel error
- ✅ Matches existing code patterns (`Cmd::Stop` and `Cmd::Shutdown` do the same thing)
- ✅ No new dependencies (uses existing crossbeam pattern)
- ✅ Can log appropriate message for debugging
- ✅ Reuses existing shutdown coordination logic

**Cons**:
- ⚠️ Code duplication across 3 paths (Stop, Shutdown, channel error)
- ⚠️ Requires extracting shutdown logic to helper function

**When to Use**:
- When error cases are expected and need specific handling
- When you want explicit control flow visibility
- When matching existing code patterns is important

#### Option 2: Defer-Based Cleanup (ALTERNATIVE)

**Philosophy**: Use RAII-style cleanup guards to ensure cleanup on all paths

**Pros**:
- ✅ Guarantees cleanup on **all** exit paths (including panics!)
- ✅ Uses established pattern (scopeguard already in [`Cargo.toml:56`](../packages/kodegend/Cargo.toml#L56), used in [`src/install/installer/config/certificates.rs:83`](../packages/kodegend/src/install/installer/config/certificates.rs#L83))
- ✅ Less code duplication
- ✅ Simpler to verify correctness

**Cons**:
- ⚠️ Slightly less explicit about when cleanup happens
- ⚠️ Cleanup always runs even in happy path (but checks `shutdown_complete` flag, so no actual work)
- ⚠️ Less idiomatic for Rust (defer is more common in Go/Zig)

**When to Use**:
- When you have complex control flow with multiple exit paths
- When panic-safety is critical
- When cleanup logic is complex and error-prone

### Recommendation: Option 1 with Helper Function

**Rationale**:
1. **Code Clarity**: Explicit handling makes the shutdown scenario obvious
2. **Consistency**: Matches the existing `Cmd::Stop` and `Cmd::Shutdown` patterns
3. **Maintainability**: Future developers will understand the shutdown logic immediately
4. **Debugging**: Can log specific message for channel error case

The code duplication is easily solved by extracting the shutdown logic to a helper function.

## Implementation: Option 1 (Recommended)

### Step 1: Extract Shutdown Logic to Helper Function

Add this new function **before** the `run()` method (around line 27):

```rust
impl AutoConfigService {
    pub fn new(def: ServiceDefinition, bus: Sender<Evt>) -> Self {
        Self {
            name: def.name,
            bus,
        }
    }

    /// Gracefully shutdown the watcher task with timeout
    ///
    /// This function coordinates cancellation using atomic flags and exponential backoff.
    /// If graceful shutdown times out, the task is forcefully aborted.
    fn shutdown_watcher(
        cancel_token: &CancellationToken,
        shutdown_complete: &Arc<AtomicBool>,
        watcher_handle: &tokio::task::JoinHandle<()>,
    ) {
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
    }

    pub fn run(self, cmd_rx: Receiver<Cmd>) -> Result<()> {
        // ... rest of the implementation
```

### Step 2: Modify Command Loop to Handle Channel Errors

**Replace lines 87-146** with:

```rust
        // Handle control commands with lock-free coordination
        loop {
            // Convert channel error to graceful shutdown
            let cmd = match cmd_rx.recv() {
                Ok(cmd) => cmd,
                Err(_) => {
                    // Channel closed (parent process died or manager stopped)
                    // Treat as graceful shutdown request
                    info!("Control channel closed, initiating shutdown");
                    Self::shutdown_watcher(&cancel_token, &shutdown_complete, &watcher_handle);
                    break;
                }
            };

            match cmd {
                Cmd::Start => {
                    info!("Auto-config service already started");
                }
                Cmd::Stop => {
                    info!("Stopping auto-config service");
                    Self::shutdown_watcher(&cancel_token, &shutdown_complete, &watcher_handle);
                    break;
                }
                Cmd::Shutdown => {
                    info!("Shutting down auto-config service");
                    Self::shutdown_watcher(&cancel_token, &shutdown_complete, &watcher_handle);
                    break;
                }
                _ => {}
            }
        }
```

### Step 3: Simplify Final Cleanup

**Replace lines 148-154** with:

```rust
        // Ensure task is fully cleaned up (in case shutdown_watcher wasn't called)
        if !shutdown_complete.load(Ordering::Acquire) {
            watcher_handle.abort();
        }

        // Wait for final task completion
        let _ = rt.block_on(watcher_handle);

        Ok(())
    }
}
```

### Code Changes Summary

**File**: `packages/kodegend/src/service/autoconfig.rs`

**Changes**:
1. **Lines 27-61** (INSERT): Add `shutdown_watcher()` helper function
2. **Lines 87-146** (REPLACE): Replace command loop with error-handling version
3. **Lines 148-154** (SIMPLIFY): Keep final cleanup as safety net

**Net Change**: ~10 lines added, code duplication eliminated, bug fixed

## Implementation: Option 2 (Alternative)

If you prefer the defer-based approach:

### Add Import

At the top of the file (after line 9):

```rust
use scopeguard::defer;
```

### Wrap Cleanup in Defer Guard

**Replace lines 28-156** (the entire `run()` function body) with:

```rust
    pub fn run(self, cmd_rx: Receiver<Cmd>) -> Result<()> {
        info!("🍯 Starting MCP client auto-configuration service");

        // Create tokio runtime for the watcher
        let rt = tokio::runtime::Runtime::new()?;

        // Create cancellation token for graceful shutdown
        let cancel_token = CancellationToken::new();
        let shutdown_complete = Arc::new(AtomicBool::new(false));

        // Create the watcher with all client plugins
        let clients = all_clients();
        let watcher = AutoConfigWatcher::new(clients)?;

        // Spawn the watcher task with graceful cancellation
        let watcher_handle = rt.spawn({
            let bus = self.bus.clone();
            let service_name = self.name.clone();
            let cancel_token = cancel_token.clone();
            let shutdown_complete = shutdown_complete.clone();

            async move {
                // Notify daemon we're starting
                let _ = bus.send(Evt::State {
                    service: service_name.clone(),
                    kind: "running".into(),
                    ts: chrono::Utc::now(),
                    pid: Some(std::process::id()),
                });

                // Run watcher with cancellation support
                tokio::select! {
                    result = watcher.run() => {
                        if let Err(e) = result {
                            error!("Auto-config watcher failed: {e}");
                            let _ = bus.send(Evt::Fatal {
                                service: service_name.clone(),
                                msg: "Watcher error occurred".into(),
                                ts: chrono::Utc::now(),
                            });
                        }
                    }
                    () = cancel_token.cancelled() => {
                        info!("Auto-config watcher cancelled gracefully");
                        let _ = bus.send(Evt::State {
                            service: service_name.clone(),
                            kind: "stopped-clean".into(),
                            ts: chrono::Utc::now(),
                            pid: Some(std::process::id()),
                        });
                    }
                }

                // Signal shutdown completion atomically
                shutdown_complete.store(true, Ordering::Release);
            }
        });

        // CLEANUP GUARD: Ensures cleanup happens on ALL exit paths
        defer! {
            // Only cleanup if task hasn't already shut down gracefully
            if !shutdown_complete.load(Ordering::Acquire) {
                cancel_token.cancel();
                
                // Wait briefly for graceful shutdown
                let timeout = std::time::Duration::from_secs(5);
                let start = std::time::Instant::now();
                let mut backoff_ms = 1;
                
                while !shutdown_complete.load(Ordering::Acquire) && start.elapsed() < timeout {
                    std::thread::sleep(std::time::Duration::from_millis(backoff_ms));
                    backoff_ms = (backoff_ms * 2).min(100);
                }
                
                // Force abort if still running
                if !shutdown_complete.load(Ordering::Acquire) {
                    watcher_handle.abort();
                }
            }
            
            // Always wait for task completion
            let _ = rt.block_on(watcher_handle);
        }

        // Handle control commands with lock-free coordination
        loop {
            match cmd_rx.recv()? {  // ✅ Now safe - defer guard ensures cleanup
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

        Ok(())
    }
```

**Note**: With the defer guard at line 67, the `?` operator on line 96 is now safe. The cleanup will execute even on early return.

## Related Files

Files that might need review for similar patterns:

- [`src/manager.rs`](../packages/kodegend/src/manager.rs) - Manager thread that creates channels
- [`src/service.rs`](../packages/kodegend/src/service.rs) - Service trait and utilities
- [`src/platform/signal.rs`](../packages/kodegend/src/platform/signal.rs) - Signal handling with channels
- [`src/platform/windows_service.rs`](../packages/kodegend/src/platform/windows_service.rs) - Windows service integration with channels

Search pattern to find similar issues: `\.recv\(\)\?` in channel receive loops

## Definition of Done

This issue is resolved when:

1. ✅ **Channel error is handled explicitly** - `cmd_rx.recv()` errors are caught and converted to graceful shutdown
2. ✅ **Cleanup logic is deduplicated** - Shutdown coordination is extracted to `shutdown_watcher()` helper function (Option 1) OR wrapped in defer guard (Option 2)
3. ✅ **Watcher task is always cleaned up** - `watcher_handle` is cancelled/aborted on all exit paths
4. ✅ **Tokio runtime is always shut down** - `rt.block_on(watcher_handle)` executes on all exit paths
5. ✅ **Code compiles without warnings** - `cargo clippy` passes for kodegend package
6. ✅ **No orphaned processes** - Manually verify by killing parent process and checking `ps` output

## Verification Steps

**Manual Verification**:

1. Build kodegend: `cd packages/kodegend && cargo build --release`
2. Start daemon: `./target/release/kodegend start`
3. Kill parent process: `pkill -9 kodegend` (find correct PID)
4. Check for orphaned processes: `ps aux | grep kodegend`
5. Expected: No autoconfig processes remain running
6. Check logs for "Control channel closed, initiating shutdown" message

**Code Review Checklist**:

- [ ] `cmd_rx.recv()?` is removed or wrapped in proper error handling
- [ ] `shutdown_watcher()` helper function exists and is called on all shutdown paths (Option 1)
- [ ] `defer!` guard wraps cleanup logic (Option 2)
- [ ] `watcher_handle.abort()` is called if graceful shutdown times out
- [ ] `rt.block_on(watcher_handle)` is always executed before function return
- [ ] No clippy warnings in autoconfig.rs

## References

### Crossbeam Channel Documentation
- RecvError: https://docs.rs/crossbeam-channel/latest/crossbeam_channel/struct.RecvError.html
- Channel disconnection semantics: All senders dropped → `recv()` returns permanent `Err`

### Scopeguard Documentation
- defer! macro: https://docs.rs/scopeguard/latest/scopeguard/macro.defer.html
- guard function: https://docs.rs/scopeguard/latest/scopeguard/fn.guard.html
- Existing usage: [`packages/kodegend/src/install/installer/config/certificates.rs:83`](../packages/kodegend/src/install/installer/config/certificates.rs#L83)

### Tokio Task Cancellation
- JoinHandle::abort: https://docs.rs/tokio/latest/tokio/task/struct.JoinHandle.html#method.abort
- CancellationToken: https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html
- select! for graceful cancellation: https://docs.rs/tokio/latest/tokio/macro.select.html

### Architecture Documentation
- IPC Protocol: [`packages/kodegend/src/ipc.rs`](../packages/kodegend/src/ipc.rs) - Cmd and Evt definitions
- Service Manager: [`packages/kodegend/src/manager.rs`](../packages/kodegend/src/manager.rs) - Channel creation and lifecycle
- AutoConfig Watcher: Check `kodegen_bundler_autoconfig` package for watcher implementation details
