# Restart Race Condition and Timing Issues

## Location
- `packages/kodegend/src/control/macos_control.rs:117-134` (fallback path)
- `packages/kodegend/src/control/windows_control.rs:161-172` (entire implementation)

## Severity
🟠 **MEDIUM - RELIABILITY ISSUE**

## Current Implementation Analysis

### Linux (ALREADY CORRECT ✅)
**File:** [`packages/kodegend/src/control/linux_control.rs`](../packages/kodegend/src/control/linux_control.rs)

Linux implementation is **already correct** - it uses systemd's native restart command:

```rust
// Lines 80-101
pub fn restart_daemon() -> Result<()> {
    let service_name = format!("{}.service", SERVICE_NAME);
    let args = if is_root() {
        vec!["restart", &service_name]
    } else {
        vec!["--user", "restart", &service_name]
    };

    let output = Command::new("systemctl")
        .args(&args)
        .output()
        .context("Failed to execute systemctl restart")?;

    if !output.status.success() {
        anyhow::bail!(
            "Failed to restart daemon: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}
```

**Why this is correct:** systemd's `restart` command is atomic and handles:
- Waiting for clean shutdown with timeout
- Verifying process fully stopped
- Handling failure cases (SIGKILL if needed)
- Ensuring no race conditions between stop and start

**No changes needed for Linux.**

---

### macOS (NEEDS IMPROVEMENT ⚠️)
**File:** [`packages/kodegend/src/control/macos_control.rs`](../packages/kodegend/src/control/macos_control.rs)

Current implementation has a good primary path but problematic fallback:

```rust
// Lines 117-134
pub fn restart_daemon() -> Result<()> {
    // PRIMARY PATH: Uses kickstart -k (kill flag) which restarts the service
    let output = Command::new("launchctl")
        .args(["kickstart", "-k", SERVICE_LABEL])
        .output()
        .context("Failed to execute launchctl kickstart -k")?;

    if !output.status.success() {
        // FALLBACK PATH (PROBLEMATIC): manual stop + start
        stop_daemon()?;
        std::thread::sleep(Duration::from_secs(1));  // ⚠️ RACE CONDITION
        start_daemon()?;
    }

    Ok(())
}
```

**Problems with fallback path:**
1. Fixed 1-second sleep is arbitrary - daemon might not be fully stopped
2. No verification that service actually stopped
3. No verification that service successfully restarted
4. If daemon takes >1s to stop, port binding fails on restart

---

### Windows (NEEDS COMPLETE REWRITE ⚠️)
**File:** [`packages/kodegend/src/control/windows_control.rs`](../packages/kodegend/src/control/windows_control.rs)

Current implementation is entirely naive:

```rust
// Lines 161-172
pub fn restart_daemon() -> Result<()> {
    // Stop the service
    stop_daemon()?;

    // Wait for service to fully stop
    std::thread::sleep(Duration::from_secs(1));  // ⚠️ RACE CONDITION

    // Start the service
    start_daemon()?;

    Ok(())
}
```

**Problems:**
1. Same issues as macOS fallback - no verification
2. Windows Service Control Manager can report status - should be used
3. Services might take longer than 1s to stop (HTTP servers, database connections)

---

## Real-World Impact

**kodegend manages MCP tool servers** - each running HTTP servers on ports 30438-30452. When restart occurs:

1. User runs `kodegend restart`
2. Stop command kills all HTTP servers immediately
3. MCP clients (Claude Desktop, Cline, etc.) get connection errors
4. If daemon hasn't fully stopped:
   - Port still bound (e.g., 30438)
   - `start_daemon()` tries to bind same port
   - Fails with "address already in use"
5. **BROKEN STATE**: daemon stopped but won't start, all MCP tools offline
6. Manual intervention required (kill processes, wait, retry)

---

## Existing Code Patterns to Follow

The kodegend codebase already has proper wait loop patterns we should reuse:

### Pattern 1: Synchronous Wait with Exponential Backoff
**File:** [`packages/kodegend/src/service/autoconfig.rs`](../packages/kodegend/src/service/autoconfig.rs) (lines 98-114)

```rust
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
```

**This is the pattern to use** because:
- Control modules use `std::process::Command` (synchronous)
- Exponential backoff is efficient (starts at 1ms, caps at 100ms)
- Timeout prevents hanging indefinitely
- Already proven in production code

### Pattern 2: Async Wait (DO NOT USE for control modules)
**File:** [`packages/kodegend/src/service/port_cleanup.rs`](../packages/kodegend/src/service/port_cleanup.rs) (lines 177-192)

```rust
pub async fn wait_for_port_release(port: u16, timeout: Duration) -> Result<()> {
    let start = tokio::time::Instant::now();
    
    while start.elapsed() < timeout {
        if check_port_available(port).await {
            return Ok(());
        }
        sleep(Duration::from_millis(100)).await;
    }

    Err(anyhow::anyhow!(
        "Timeout waiting for port {} to be released after {:?}",
        port,
        timeout
    ))
}
```

**Why NOT to use this pattern:**
- Control modules are synchronous (use `std::process::Command`, not `tokio::process::Command`)
- Would require adding async/await throughout control modules
- Unnecessary complexity

---

## Implementation Requirements

### For macOS (`packages/kodegend/src/control/macos_control.rs`)

**Add two helper functions:**

```rust
/// Wait for daemon to fully stop (check_status returns false)
///
/// Uses exponential backoff pattern from autoconfig.rs for efficiency.
fn wait_for_stopped(timeout: Duration) -> Result<()> {
    let start_time = std::time::Instant::now();
    let mut backoff_ms = 1;

    loop {
        // Check if already stopped
        match check_status() {
            Ok(false) => return Ok(()), // Stopped successfully
            Ok(true) => {
                // Still running, continue waiting
                if start_time.elapsed() > timeout {
                    anyhow::bail!(
                        "Daemon did not stop within {:?}. Manual intervention may be required.",
                        timeout
                    );
                }

                // Exponential backoff sleep
                std::thread::sleep(Duration::from_millis(backoff_ms));
                backoff_ms = (backoff_ms * 2).min(100); // Cap at 100ms
            }
            Err(e) => {
                // If check_status fails, we can't determine state
                // Log warning but continue - might be already stopped
                log::warn!("Failed to check daemon status during shutdown: {}", e);
                std::thread::sleep(Duration::from_millis(backoff_ms));
                backoff_ms = (backoff_ms * 2).min(100);
                
                if start_time.elapsed() > timeout {
                    return Err(e);
                }
            }
        }
    }
}

/// Wait for daemon to become active (check_status returns true)
///
/// Uses exponential backoff pattern from autoconfig.rs for efficiency.
fn wait_for_active(timeout: Duration) -> Result<()> {
    let start_time = std::time::Instant::now();
    let mut backoff_ms = 1;

    loop {
        // Check if active
        match check_status() {
            Ok(true) => return Ok(()), // Active successfully
            Ok(false) => {
                // Not running yet, continue waiting
                if start_time.elapsed() > timeout {
                    anyhow::bail!(
                        "Daemon did not become active within {:?}. Check logs for startup errors.",
                        timeout
                    );
                }

                // Exponential backoff sleep
                std::thread::sleep(Duration::from_millis(backoff_ms));
                backoff_ms = (backoff_ms * 2).min(100); // Cap at 100ms
            }
            Err(e) => {
                // If check_status fails during startup, that's a problem
                if start_time.elapsed() > timeout {
                    anyhow::bail!(
                        "Daemon startup verification failed: {}",
                        e
                    );
                }
                
                std::thread::sleep(Duration::from_millis(backoff_ms));
                backoff_ms = (backoff_ms * 2).min(100);
            }
        }
    }
}
```

**Update restart_daemon() fallback path:**

```rust
pub fn restart_daemon() -> Result<()> {
    // Try modern kickstart with -k (kill) flag which restarts the service
    let output = Command::new("launchctl")
        .args(["kickstart", "-k", SERVICE_LABEL])
        .output()
        .context("Failed to execute launchctl kickstart -k")?;

    if !output.status.success() {
        // Fallback: manual stop + start with proper verification
        log::warn!("launchctl kickstart -k failed, using manual stop+start fallback");
        
        stop_daemon()
            .context("Failed to stop daemon during restart")?;
        
        wait_for_stopped(Duration::from_secs(10))
            .context("Daemon did not stop cleanly within 10 seconds")?;
        
        // Small delay to ensure port release (launchd-specific timing)
        std::thread::sleep(Duration::from_millis(500));
        
        start_daemon()
            .context("Failed to start daemon after stop")?;
        
        wait_for_active(Duration::from_secs(30))
            .context("Daemon started but did not become active within 30 seconds")?;
    }

    Ok(())
}
```

**Key timeouts chosen:**
- **10 seconds for stop**: MCP servers need to close HTTP connections gracefully
- **30 seconds for start**: Some tool servers (browser, citescrape) initialize heavy resources
- **500ms port release delay**: launchd-specific - ensures port fully released by kernel

---

### For Windows (`packages/kodegend/src/control/windows_control.rs`)

**Add the same two helper functions** (wait_for_stopped and wait_for_active) with identical implementation.

**Replace entire restart_daemon() implementation:**

```rust
/// Restart daemon (Windows doesn't have native restart - stop + start with verification)
pub fn restart_daemon() -> Result<()> {
    // Stop the service
    stop_daemon()
        .context("Failed to stop daemon during restart")?;

    // Wait for service to fully stop (poll Service Control Manager)
    wait_for_stopped(Duration::from_secs(10))
        .context("Service did not stop cleanly within 10 seconds")?;

    // Small delay to ensure resource cleanup
    std::thread::sleep(Duration::from_millis(500));

    // Start the service
    start_daemon()
        .context("Failed to start daemon after stop")?;

    // Verify startup succeeded
    wait_for_active(Duration::from_secs(30))
        .context("Service started but did not become active within 30 seconds")?;

    Ok(())
}
```

**Key timeouts chosen:**
- **10 seconds for stop**: Windows services can have dependencies, cleanup tasks
- **30 seconds for start**: Tool servers may need to initialize COM objects, HTTP servers
- **500ms resource delay**: Windows-specific - ensures handles, sockets fully released

---

## Files to Modify

### 1. `packages/kodegend/src/control/macos_control.rs`

**Changes:**
- Add `wait_for_stopped()` helper function (use exponential backoff pattern from [autoconfig.rs](../packages/kodegend/src/service/autoconfig.rs))
- Add `wait_for_active()` helper function (use exponential backoff pattern from [autoconfig.rs](../packages/kodegend/src/service/autoconfig.rs))
- Update `restart_daemon()` fallback path to use these helpers
- Keep primary path (`launchctl kickstart -k`) unchanged

**Imports needed:**
```rust
use std::time::Duration; // Already present at line 5
```

**Lines to modify:** 117-134

---

### 2. `packages/kodegend/src/control/windows_control.rs`

**Changes:**
- Add `wait_for_stopped()` helper function (identical to macOS version)
- Add `wait_for_active()` helper function (identical to macOS version)
- Replace entire `restart_daemon()` implementation with verified stop+start

**Imports needed:**
```rust
use std::time::Duration; // Already present at line 5
```

**Lines to modify:** 161-172

---

### 3. `packages/kodegend/src/control/linux_control.rs`

**No changes needed** - already correct.

---

## Edge Cases Handled

### 1. Service Already Stopped
- `wait_for_stopped()` checks status immediately
- Returns `Ok(())` if already false (no waiting)
- Restart proceeds normally

### 2. Service Fails to Start
- `wait_for_active()` times out after 30 seconds
- Returns clear error: "Daemon started but did not become active within 30 seconds"
- User knows to check logs for startup errors

### 3. Concurrent Restart Calls
- Service managers (systemd/launchd/SCM) handle this internally
- Each restart call polls the actual service manager state
- No cached state - always authoritative

### 4. Stale State (PID file exists, service dead)
- `check_status()` queries service manager, not PID file
- Linux: `systemctl is-active` checks actual process
- macOS: `launchctl list` checks actual PID
- Windows: `QueryServiceStatusEx` checks SCM state
- Stale PID files don't affect restart logic

### 5. System Under Heavy Load
- Exponential backoff starts at 1ms, caps at 100ms
- Reduces CPU usage during polling
- Timeout ensures eventual failure (not infinite loop)
- Clear error messages guide user

---

## Performance Considerations

### Exponential Backoff Efficiency

**From [autoconfig.rs pattern](../packages/kodegend/src/service/autoconfig.rs#L103-L113):**

```rust
let mut backoff_ms = 1;
while !condition {
    std::thread::sleep(Duration::from_millis(backoff_ms));
    backoff_ms = (backoff_ms * 2).min(100); // Cap at 100ms
}
```

**Polling sequence:** 1ms, 2ms, 4ms, 8ms, 16ms, 32ms, 64ms, 100ms, 100ms...

**Why this is optimal:**
- Fast response for quick operations (1-2ms initial checks)
- Minimal CPU usage for slow operations (100ms max polling)
- Total CPU time for 10s timeout: ~100ms (1% CPU usage)
- Compare to fixed 100ms polling: ~10s of CPU time (100% wasted)

### Impact Analysis

**Restart frequency:** Infrequent operation (config changes, updates)
**User expectations:** "It just works" > Speed
**Typical restart time:**
- Fast path (service restarts in 2s): ~2.05s (adds 50ms verification)
- Slow path (service restarts in 8s): ~8.1s (adds 100ms verification)

**Trade-off:** +50-100ms latency for **100% reliability** is acceptable.

---

## Definition of Done

### Functional Requirements

1. **macOS restart_daemon() fallback path:**
   - ✅ Calls `stop_daemon()`
   - ✅ Calls `wait_for_stopped(Duration::from_secs(10))`
   - ✅ Sleeps 500ms for port release
   - ✅ Calls `start_daemon()`
   - ✅ Calls `wait_for_active(Duration::from_secs(30))`
   - ✅ Returns proper errors with context on timeout

2. **Windows restart_daemon():**
   - ✅ Calls `stop_daemon()`
   - ✅ Calls `wait_for_stopped(Duration::from_secs(10))`
   - ✅ Sleeps 500ms for resource cleanup
   - ✅ Calls `start_daemon()`
   - ✅ Calls `wait_for_active(Duration::from_secs(30))`
   - ✅ Returns proper errors with context on timeout

3. **Helper functions (both platforms):**
   - ✅ `wait_for_stopped()` uses exponential backoff (1ms->100ms)
   - ✅ `wait_for_active()` uses exponential backoff (1ms->100ms)
   - ✅ Both respect timeout parameter
   - ✅ Both return `Result<()>` with descriptive error messages
   - ✅ Both handle `check_status()` errors gracefully

### Code Quality

1. ✅ Follows existing patterns from [autoconfig.rs](../packages/kodegend/src/service/autoconfig.rs)
2. ✅ Error messages include context (which step failed, how long waited)
3. ✅ Uses `anyhow::Context` for error propagation
4. ✅ No unwrap() or panic!() - all errors handled
5. ✅ Comments explain timeout choices and platform-specific behavior

### Integration

1. ✅ No changes to public API (`pub fn restart_daemon() -> Result<()>`)
2. ✅ Helper functions are private (`fn wait_for_stopped`, `fn wait_for_active`)
3. ✅ Works with existing `check_status()`, `stop_daemon()`, `start_daemon()`
4. ✅ No new dependencies added

---

## Implementation Notes

### DO NOT Add Async

The control modules are **intentionally synchronous**:
- CLI commands are blocking operations
- User expects immediate feedback
- No benefit from async (waiting for external process)
- Adding async would require refactoring entire control module hierarchy

Use `std::thread::sleep`, not `tokio::sleep`.

### DO NOT Use Fixed Sleep

```rust
// ❌ BAD - arbitrary wait time
std::thread::sleep(Duration::from_secs(1));
start_daemon()?;

// ✅ GOOD - poll until verified
wait_for_stopped(Duration::from_secs(10))?;
start_daemon()?;
```

### DO Use Clear Error Messages

```rust
// ❌ BAD - vague error
anyhow::bail!("Failed to restart");

// ✅ GOOD - actionable guidance
anyhow::bail!(
    "Daemon did not become active within 30 seconds. Check logs for startup errors."
);
```

---

## Related Code References

### Existing Implementations
- [`linux_control.rs`](../packages/kodegend/src/control/linux_control.rs) - Correct restart pattern
- [`macos_control.rs`](../packages/kodegend/src/control/macos_control.rs) - Needs fallback improvement
- [`windows_control.rs`](../packages/kodegend/src/control/windows_control.rs) - Needs complete rewrite

### Existing Wait Patterns
- [`autoconfig.rs`](../packages/kodegend/src/service/autoconfig.rs#L98-114) - Sync exponential backoff (USE THIS)
- [`port_cleanup.rs`](../packages/kodegend/src/service/port_cleanup.rs#L177-192) - Async wait pattern (DO NOT USE)

### Service Management
- [`daemon.rs`](../packages/kodegend/src/daemon.rs) - Daemon lifecycle
- [`service.rs`](../packages/kodegend/src/service.rs) - Service orchestration
- [`manager.rs`](../packages/kodegend/src/manager.rs) - Service manager

---

## Priority

**MEDIUM** - Restart is less frequent than start/stop but critical when needed. Users expect restart to "just work" even when:
- System is under load
- Services are slow to shutdown
- Multiple tool servers are running
- Network connections are active

A failed restart leaves the system in a broken state requiring manual intervention, which is unacceptable for a daemon manager.
