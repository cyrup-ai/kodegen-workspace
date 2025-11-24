# CRITICAL: Silent Error Swallowing in Log Rotation Operations

**Priority:** CRITICAL  
**Component:** [`packages/kodegend/src/service.rs`](../packages/kodegend/src/service.rs)  
**Lines:** 362, 451-464, 507-508, 533  
**Impact:** Production - Data loss and silent failures

## Executive Summary

Multiple critical operations in the kodegend service worker use `.ok()` to discard errors, causing silent failures that can lead to data loss and service unavailability. This task eliminates all instances of silent error swallowing in log rotation and auto-restart operations.

## Architectural Context

### Service Worker Thread Model

The `ServiceWorker` runs in its own dedicated thread (spawned at [line 50-67](../packages/kodegend/src/service.rs#L50-L67)) with a bounded crossbeam channel for commands:

```rust
let (tx, rx) = bounded::<Cmd>(16);  // Line 45: Capacity of 16 commands
```

The worker's `run()` loop ([line 72-92](../packages/kodegend/src/service.rs#L72-L92)) uses `select!` to receive commands from three sources:
1. External commands via `self.rx` (Start, Stop, Restart, Shutdown)
2. Health check ticks every 60 seconds ([line 73](../packages/kodegend/src/service.rs#L73))
3. Log rotation ticks every 3600 seconds ([line 74](../packages/kodegend/src/service.rs#L74))

Errors in the run loop are caught at [line 60-62](../packages/kodegend/src/service.rs#L60-L62) and logged, but the thread terminates on error.

### Event Bus Architecture

The worker communicates back to the manager via an event bus using the `Evt` enum defined in [`ipc.rs`](../packages/kodegend/src/ipc.rs):

```rust
pub enum Evt {
    State { service: String, kind: Cow<'static, str>, ts: DateTime<Utc>, pid: Option<u32> },
    Health { service: String, healthy: bool, ts: DateTime<Utc> },
    LogRotate { service: String, ts: DateTime<Utc> },
    Fatal { service: String, msg: Cow<'static, str>, ts: DateTime<Utc> },  // ← Available for critical errors
}
```

**Key Pattern:** All existing `bus.send()` calls use `?` to propagate errors ([lines 173, 295, 343, 353, 372, 402](../packages/kodegend/src/service.rs)). Line 362 is the **only exception** that uses `.ok()`.

## Problem #1: Auto-Restart Channel Send Failure (Line 362)

### Current Code

[**Line 362**](../packages/kodegend/src/service.rs#L362) in `health_check()`:
```rust
if !healthy && self.def.auto_restart {
    warn!("{} unhealthy → restart", self.name);
    self.tx.send(Cmd::Restart).ok();  // ← ERROR SWALLOWED
}
```

### Why This Is Critical

The channel `self.tx` is bounded with capacity 16. The `send()` method on crossbeam's `Sender` can fail with `SendError<T>` if:
1. The channel is disconnected (receiver dropped)
2. The channel is full (16+ commands pending)

**Channel Full Scenario:**
- Multiple rapid health check failures → multiple queued restarts
- Manual commands queue up during slow operations
- Stop/Start/Restart commands pile up faster than they're processed

**Impact When Fails:**
- Service stays down permanently
- Auto-restart never triggers
- Health checks continue firing every 60 seconds, each failing silently
- No error logged, no event sent, administrator unaware

### Crossbeam Channel API

From [crossbeam-channel documentation](https://docs.rs/crossbeam/latest/crossbeam/channel/struct.Sender.html):

```rust
// Blocking send - fails only on disconnect
pub fn send(&self, msg: T) -> Result<(), SendError<T>>

// Timeout-based send - fails on timeout or disconnect  
pub fn send_timeout(&self, msg: T, timeout: Duration) -> Result<(), SendTimeoutError<T>>
```

`SendTimeoutError<T>` variants:
- `Timeout(T)` - Channel full, operation timed out
- `Disconnected(T)` - Receiver dropped, channel closed

### Solution for Line 362

Replace the silent `.ok()` with explicit error handling using `send_timeout`:

```rust
if !healthy && self.def.auto_restart {
    warn!("{} unhealthy → restart", self.name);
    
    match self.tx.send_timeout(Cmd::Restart, Duration::from_secs(2)) {
        Ok(_) => {
            // Successfully queued restart command
        }
        Err(e) => {
            // CRITICAL: Auto-restart failed to queue
            error!("{} CRITICAL: Auto-restart failed to queue command: {}", self.name, e);
            
            // Send fatal event to alert monitoring systems
            self.bus.send(Evt::Fatal {
                service: self.name.to_string(),
                msg: format!("Auto-restart channel send failed: {}", e).into(),
                ts: Utc::now(),
            })?;
        }
    }
}
```

**Key Changes:**
1. Use `send_timeout` instead of `send` to prevent indefinite blocking
2. 2-second timeout is reasonable (much longer than typical command processing)
3. Log critical error with `error!` macro for immediate visibility
4. Send `Evt::Fatal` on the bus to alert external monitoring systems
5. Propagate bus send errors with `?` (consistent with existing pattern)

## Problem #2: File Rename Errors During Log Shifting (Lines 457, 462)

### Current Code

[**Lines 451-464**](../packages/kodegend/src/service.rs#L451-L464) in `rotate_single_log()`:
```rust
// Numbered strategy: shift .1 → .2, .2 → .3, etc.
for i in (1..max_files).rev() {
    let old = format!("{}.{}", log_path, i);
    let new = format!("{}.{}", log_path, i + 1);
    
    if Path::new(&old).exists() {
        fs::rename(&old, &new).ok();  // ← ERROR SWALLOWED (Line 457)
    }
    let old_gz = format!("{}.gz", old);
    let new_gz = format!("{}.gz", new);
    if Path::new(&old_gz).exists() {
        fs::rename(&old_gz, &new_gz).ok();  // ← ERROR SWALLOWED (Line 462)
    }
}
```

### Why This Is Dangerous

When `fs::rename()` fails, the error is silently discarded. Common failure causes:
1. **Permission denied** - Process lacks permission to rename files
2. **Disk full** - No space to create directory entry
3. **Cross-device link** - Source and dest on different filesystems (EXDEV error)
4. **File system errors** - I/O errors, read-only filesystem
5. **File in use** - On some systems, files can't be renamed while open

### Data Loss Scenario

1. Log rotation triggers for `service.log` (60MB, exceeds 50MB limit)
2. Attempts to shift `service.log.1` → `service.log.2`
3. Rename fails (disk full, permissions, etc.)
4. **Error is swallowed with `.ok()`** - no indication of failure
5. Current log gets renamed to `service.log.1` ([line 471](../packages/kodegend/src/service.rs#L471))
6. **Old `service.log.1` is overwritten** - previous rotated logs destroyed
7. Process appears successful but data is permanently lost

### Existing Error Handling Pattern

The `rotate_single_log()` function **already returns `Result<()>`** and is called with `?`:
- [Line 381](../packages/kodegend/src/service.rs#L381): `rotate_single_log(...)?;`
- [Line 392](../packages/kodegend/src/service.rs#L392): `rotate_single_log(...)?;`

Other file operations in the same function use `.context()` and `?`:
- [Line 436](../packages/kodegend/src/service.rs#L436): `fs::metadata(path)?`
- [Line 471](../packages/kodegend/src/service.rs#L471): `fs::rename(path, &rotated_name)?`
- [Line 481](../packages/kodegend/src/service.rs#L481): `fs::read(&rotated_name)?`
- [Line 491](../packages/kodegend/src/service.rs#L491): `fs::remove_file(&rotated_name)?`

### Solution for Lines 457 and 462

Replace `.ok()` with `.context()?` to propagate errors with context:

```rust
// Numbered strategy: shift .1 → .2, .2 → .3, etc.
for i in (1..max_files).rev() {
    let old = format!("{}.{}", log_path, i);
    let new = format!("{}.{}", log_path, i + 1);
    
    if Path::new(&old).exists() {
        fs::rename(&old, &new)
            .with_context(|| format!("Failed to shift rotated log {} → {}", old, new))?;
    }
    
    let old_gz = format!("{}.gz", old);
    let new_gz = format!("{}.gz", new);
    if Path::new(&old_gz).exists() {
        fs::rename(&old_gz, &new_gz)
            .with_context(|| format!("Failed to shift compressed log {} → {}", old_gz, new_gz))?;
    }
}
```

**Why This Approach:**
1. **Prevents data loss** - Rotation fails loudly if shifting fails, preventing overwrite
2. **Consistent with existing code** - Uses same `.context()?` pattern as lines 471, 481, 491
3. **Provides context** - Error message includes source and destination paths
4. **Proper propagation** - Error bubbles up through `?` to caller, gets logged in `rotate_logs()`

## Problem #3: File Deletion Errors During Cleanup (Lines 507, 508, 533)

### Current Code

[**Lines 507-508**](../packages/kodegend/src/service.rs#L507-L508) - Numbered rotation cleanup:
```rust
// Remove both compressed and uncompressed versions
fs::remove_file(&old_file).ok();  // ← ERROR SWALLOWED (Line 507)
fs::remove_file(&old_gz).ok();    // ← ERROR SWALLOWED (Line 508)
```

[**Line 533**](../packages/kodegend/src/service.rs#L533) - Timestamped rotation cleanup:
```rust
for entry in archives.iter().take(to_delete) {
    fs::remove_file(entry.path()).ok();  // ← ERROR SWALLOWED (Line 533)
}
```

### Why Deletion Failures Are Different

Unlike rename failures, deletion failures are **less critical**:
- **No data loss** - If deletion fails, old files remain (data still exists)
- **Gradual degradation** - Disk fills slowly over time, not immediate failure
- **Operational resilience** - Rotation should succeed even if cleanup fails

However, accumulated failures can cause:
- Disk space exhaustion (long-term)
- `max_files` limit becomes meaningless
- No visibility into cleanup problems

### Solution for Lines 507, 508, 533

Replace `.ok()` with explicit error logging using `warn!`:

**Lines 507-508** (numbered rotation cleanup):
```rust
// Remove both compressed and uncompressed versions
if let Err(e) = fs::remove_file(&old_file) {
    // Ignore NotFound (file didn't exist), warn on other errors
    if e.kind() != std::io::ErrorKind::NotFound {
        warn!("{} Failed to delete old log file {}: {}", 
              self.name, old_file, e);
    }
}
if let Err(e) = fs::remove_file(&old_gz) {
    if e.kind() != std::io::ErrorKind::NotFound {
        warn!("{} Failed to delete compressed log {}: {}", 
              self.name, old_gz, e);
    }
}
```

**Line 533** (timestamped rotation cleanup):
```rust
for entry in archives.iter().take(to_delete) {
    if let Err(e) = fs::remove_file(entry.path()) {
        warn!("Failed to delete old timestamped log {:?}: {}", 
              entry.path(), e);
    }
}
```

**Why This Approach:**
1. **Non-blocking** - Cleanup failures don't stop rotation from succeeding
2. **Observable** - Warnings appear in logs for monitoring/alerting
3. **Pragmatic** - Ignores `NotFound` errors (race condition: file already deleted)
4. **Operational** - Allows rotation to continue even with permission/disk issues

**Note:** The cleanup code at lines 507-508 doesn't have access to `self.name` because it's inside a standalone function `rotate_single_log()`. The warning should use `log_path` instead:

```rust
warn!("Failed to delete old log file {} (beyond max_files limit): {}", old_file, e);
```

## Summary of Changes

### File: `packages/kodegend/src/service.rs`

| Line | Current Code | Change Required | Reason |
|------|-------------|-----------------|--------|
| 362 | `self.tx.send(Cmd::Restart).ok();` | Replace with `send_timeout` + error handling + `Evt::Fatal` | **CRITICAL** - Auto-restart silently fails, service stays down |
| 457 | `fs::rename(&old, &new).ok();` | Replace with `.with_context()?.` | **DATA LOSS** - Old logs overwritten if shift fails |
| 462 | `fs::rename(&old_gz, &new_gz).ok();` | Replace with `.with_context()?.` | **DATA LOSS** - Compressed logs overwritten if shift fails |
| 507 | `fs::remove_file(&old_file).ok();` | Replace with `if let Err(e)` + `warn!` | **OBSERVABLE** - Cleanup failures should be visible |
| 508 | `fs::remove_file(&old_gz).ok();` | Replace with `if let Err(e)` + `warn!` | **OBSERVABLE** - Cleanup failures should be visible |
| 533 | `fs::remove_file(entry.path()).ok();` | Replace with `if let Err(e)` + `warn!` | **OBSERVABLE** - Cleanup failures should be visible |

## Implementation Checklist

### Phase 1: Auto-Restart Fix (Line 362)
- [ ] Import `Duration` from `std::time` (already imported at line 8)
- [ ] Replace `self.tx.send(Cmd::Restart).ok()` with `send_timeout` match block
- [ ] Add error logging with `error!` macro
- [ ] Send `Evt::Fatal` on bus for monitoring integration
- [ ] Verify error propagation with `?` operator

### Phase 2: Log Shift Fix (Lines 457, 462)  
- [ ] Replace `fs::rename(&old, &new).ok()` with `.with_context()?`
- [ ] Add descriptive context message with source and destination paths
- [ ] Replace `fs::rename(&old_gz, &new_gz).ok()` with `.with_context()?`
- [ ] Add descriptive context message for compressed file shift
- [ ] Verify consistency with existing error handling pattern (lines 471, 481, 491)

### Phase 3: Cleanup Logging (Lines 507, 508, 533)
- [ ] Replace line 507 `.ok()` with `if let Err(e)` + `warn!` + `NotFound` filter
- [ ] Replace line 508 `.ok()` with `if let Err(e)` + `warn!` + `NotFound` filter  
- [ ] Replace line 533 `.ok()` with `if let Err(e)` + `warn!`
- [ ] Ensure warning messages include file paths for debugging
- [ ] Verify cleanup errors don't propagate (warnings only)

## Definition of Done

1. **All `.ok()` calls removed** from lines 362, 457, 462, 507, 508, 533
2. **Auto-restart failures** emit `error!` log and `Evt::Fatal` event
3. **Log shift failures** propagate with context via `.with_context()?`
4. **Cleanup failures** emit `warn!` logs but don't stop rotation
5. **No compilation errors** - code compiles cleanly with `cargo check`
6. **No clippy warnings** - code passes `cargo clippy`
7. **Error handling consistent** with existing patterns in service.rs

## References and Citations

### Source Files
- Primary file: [`packages/kodegend/src/service.rs`](../packages/kodegend/src/service.rs)
- Event definitions: [`packages/kodegend/src/ipc.rs`](../packages/kodegend/src/ipc.rs)
- Command definitions: [`packages/kodegend/src/ipc.rs`](../packages/kodegend/src/ipc.rs)

### External Documentation
- [Crossbeam Channel Sender API](https://docs.rs/crossbeam/latest/crossbeam/channel/struct.Sender.html)
- [Crossbeam Bounded Channels](https://docs.rs/crossbeam/latest/crossbeam/channel/fn.bounded.html)
- [anyhow Context Trait](https://docs.rs/anyhow/latest/anyhow/trait.Context.html)

### Code Patterns in kodegend
- Worker thread model: [service.rs lines 50-67](../packages/kodegend/src/service.rs#L50-L67)
- Event bus usage: [service.rs lines 173, 295, 343, 353, 372, 402](../packages/kodegend/src/service.rs)
- File operation error handling: [service.rs lines 108, 116, 168, 436, 471, 481, 491](../packages/kodegend/src/service.rs)
- Health check loop: [service.rs lines 304-366](../packages/kodegend/src/service.rs#L304-L366)
- Log rotation function: [service.rs lines 411-538](../packages/kodegend/src/service.rs#L411-L538)

### Architecture Notes
- Bounded channel capacity: [service.rs line 45](../packages/kodegend/src/service.rs#L45) - `bounded::<Cmd>(16)`
- Health check interval: [service.rs line 73](../packages/kodegend/src/service.rs#L73) - 60 seconds
- Rotation check interval: [service.rs line 74](../packages/kodegend/src/service.rs#L74) - 3600 seconds (1 hour)
- Worker thread naming: [service.rs line 51](../packages/kodegend/src/service.rs#L51) - `svc-{service_name}`
