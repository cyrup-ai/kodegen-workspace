# CRITICAL: Log Rotation Race Condition - Rotated Files Continue Growing

**Priority:** CRITICAL  
**Component:** `packages/kodegend/src/service.rs`  
**Lines:** 368-408 (rotate_logs), 419-538 (rotate_single_log), 82 (Cmd::Restart handler)  
**Impact:** Production - Log rotation fails to achieve its purpose

## Problem Statement

The `rotate_logs()` function rotates log files while child processes are actively writing to them, causing rotated files to continue growing indefinitely. This defeats the entire purpose of log rotation and will eventually fill disk space in production.

## Root Cause Analysis

### The Race Condition

**Location:** Lines 381-388 and 391-398 in `service.rs`

The code calls `rotate_single_log()` which renames the active log file (line 471: `fs::rename(path, &rotated_name)`) while the child process still holds an open file descriptor to it.

### Unix File Descriptor Behavior

When a file is renamed while a process has it open:
1. The file descriptor remains valid and points to the **same inode**
2. All writes continue to the **renamed file** (not a new file)
3. The process does NOT automatically detect the rename
4. The process does NOT automatically start writing to a new file
5. No new file is created until the process explicitly opens a new file descriptor

This is fundamental Unix behavior documented in the POSIX standard.

### Evidence in Current Code

**Line 470 comment is INCORRECT:**
```rust
// Rename current log to rotated name
// The service will automatically create a new file on next write
fs::rename(path, &rotated_name)?;
```

This comment is **demonstrably false**. The service will NOT create a new file - it will continue writing to the rotated file via its existing file descriptor.

### What Actually Happens (Step-by-Step)

1. **T0:** Child process opens `service.log` → receives file descriptor 7 pointing to inode 12345
2. **T1:** Child writes "log entry 1\n" → written to inode 12345 (file: service.log)
3. **T2:** Log rotation executes: `fs::rename("service.log", "service.log.1")`
4. **T3:** Filesystem renames the directory entry, but **inode 12345 remains unchanged**
5. **T4:** Child writes "log entry 2\n" → written to inode 12345 (now file: service.log.1)
6. **T5:** Child writes "log entry 3\n" → written to inode 12345 (now file: service.log.1)
7. **T6:** `service.log` does not exist! No new file is created!
8. **Result:** `service.log.1` grows indefinitely, defeating rotation

## Impact Assessment

- **Disk Space:** Rotated log files continue growing without bound
- **Main Log File:** `service.log` is never recreated until process restart
- **Configuration Ignored:** `max_size_mb` and `max_files` become meaningless
- **Monitoring Failure:** Log monitoring tools watching `service.log` see no new data
- **Production Risk:** Systems will eventually fill disk and crash

## Code Analysis

### Current Implementation

**File:** `packages/kodegend/src/service.rs`

**Lines 368-408: rotate_logs() function**
```rust
fn rotate_logs(&self) -> Result<()> {
    // Only rotate if log_rotation config exists
    let Some(ref rotation_config) = self.def.log_rotation else {
        // No rotation configured - just send event and return
        self.bus.send(Evt::LogRotate {
            service: self.name.to_string(),
            ts: Utc::now(),
        })?;
        return Ok(());
    };
    
    // Rotate stdout log if configured
    if let Some(ref log_path) = self.def.log_stdout {
        rotate_single_log(
            log_path,
            rotation_config.max_size_mb,
            rotation_config.max_files,
            rotation_config.compress,
            rotation_config.timestamp,
        )?;
    }
    
    // Rotate stderr log if configured
    if let Some(ref log_path) = self.def.log_stderr {
        rotate_single_log(
            log_path,
            rotation_config.max_size_mb,
            rotation_config.max_files,
            rotation_config.compress,
            rotation_config.timestamp,
        )?;
    }
    
    // Send rotation event
    self.bus.send(Evt::LogRotate {
        service: self.name.to_string(),
        ts: Utc::now(),
    })?;
    
    // MISSING: Restart service to reopen log files!
    
    Ok(())
}
```

### Existing Infrastructure (Already Available)

**Line 82: Cmd::Restart handler in the event loop**
```rust
Cmd::Restart  => { self.stop(&mut child)?; self.start(&mut child)?; },
```

**Lines 112-119: start() opens log files with .create(true)**
```rust
let file = std::fs::OpenOptions::new()
    .create(true)   // ← Creates file if it doesn't exist!
    .append(true)
    .open(&log_path)
    .context("open stdout log")?;

Stdio::from(file)
```

**Line 362: auto_restart already uses the same pattern**
```rust
if !healthy && self.def.auto_restart {
    warn!("{} unhealthy → restart", self.name);
    self.tx.send(Cmd::Restart).ok();  // ← Same mechanism we need!
}
```

### Why Restart Solves the Problem

When `Cmd::Restart` is sent:
1. **stop()** is called → terminates child process → all file descriptors are closed
2. **start()** is called → spawns new child process → opens fresh file descriptors
3. The new file descriptor points to a **new inode** (the new `service.log` file)
4. The old inode (rotated file) is no longer written to
5. Log rotation works as intended

## Solution: Add Service Restart After Log Rotation

### Implementation

**File:** `packages/kodegend/src/service.rs`  
**Function:** `rotate_logs()` (lines 368-408)  
**Location:** After line 405 (after sending LogRotate event, before Ok(()))

Add the following code:

```rust
fn rotate_logs(&self) -> Result<()> {
    // ... existing rotation logic (lines 370-405) ...
    
    // Send rotation event
    self.bus.send(Evt::LogRotate {
        service: self.name.to_string(),
        ts: Utc::now(),
    })?;
    
    // ============ ADD THIS SECTION ============
    // Restart service to close old file descriptors and open new ones
    // This ensures the rotated file stops growing and a fresh log file is created
    if self.def.log_stdout.is_some() || self.def.log_stderr.is_some() {
        warn!("{} rotating logs, restarting service to reopen file descriptors", self.name);
        self.tx.send(Cmd::Restart).ok();
    }
    // =========================================
    
    Ok(())
}
```

### Exact Changes Required

**File:** `packages/kodegend/src/service.rs`

**Before (lines 401-408):**
```rust
    // Send rotation event
    self.bus.send(Evt::LogRotate {
        service: self.name.to_string(),
        ts: Utc::now(),
    })?;
    
    Ok(())
}
```

**After:**
```rust
    // Send rotation event
    self.bus.send(Evt::LogRotate {
        service: self.name.to_string(),
        ts: Utc::now(),
    })?;
    
    // Restart service to close old file descriptors and open new ones
    // This ensures the rotated file stops growing and a fresh log file is created
    if self.def.log_stdout.is_some() || self.def.log_stderr.is_some() {
        warn!("{} rotating logs, restarting service to reopen file descriptors", self.name);
        self.tx.send(Cmd::Restart).ok();
    }
    
    Ok(())
}
```

### Why This Solution Is Correct

1. **Leverages Existing Infrastructure:** Uses the same `Cmd::Restart` mechanism already proven to work in auto_restart (line 362)
2. **Minimal Code Changes:** Single 4-line addition, no architectural changes needed
3. **Safe:** The `.ok()` ensures the send doesn't panic even if the channel is somehow closed
4. **Conditional:** Only restarts if logs are actually configured (prevents unnecessary restarts)
5. **Self-Documenting:** Clear warning message explains why restart is happening
6. **Standard Practice:** Aligns with industry-standard log rotation (see References)

## Industry Best Practices

### Standard Approaches to Log Rotation

There are three standard approaches used in production Unix systems:

#### 1. Signal-Based Rotation (SIGHUP)
Most common for daemons. The daemon catches SIGHUP and reopens log files.
- **Used by:** nginx, Apache, syslog
- **Pros:** No downtime
- **Cons:** Requires signal handling code

#### 2. Service Restart After Rotation
Second most common. Logrotate's `postrotate` script restarts the service.
- **Used by:** Many systemd services
- **Pros:** Simple, guaranteed to work, no special daemon code needed
- **Cons:** Brief service downtime (acceptable for most daemons)

#### 3. copytruncate
Alternative approach: copy file, then truncate original.
- **Used by:** When neither restart nor signal handling is feasible
- **Pros:** No process restart needed
- **Cons:** Race condition window, requires extra disk space, can lose data

### Why Service Restart Is Best for kodegend

From [StackExchange discussion on logrotate with systemd](https://unix.stackexchange.com/questions/605121/):
> "copytruncate is the right answer when you have a proper daemon that you can signal to re-open the log file. **The alternative is to restart the service in the post-rotation script**, but that may not be convenient or desirable."

For kodegend:
- ✅ Service restart is already implemented and tested (auto_restart)
- ✅ Services are designed to handle restarts gracefully (daemon pattern)
- ✅ Brief downtime is acceptable (services restart quickly)
- ✅ No special signal handling code needed
- ✅ No risk of data loss from truncation race conditions

From [Baeldung: Rotating Logs with Logrotate](https://www.baeldung.com/linux/rotating-logs-logrotate) (May 2024):
> "To make the rotation process less disruptive, we can rotate the log by copying the old log into a different file and clearing the content. This flavor of rotation comes in the form of 2 different directives: copytruncate and copy."

However, for a daemon manager like kodegend, restart is simpler and safer than copytruncate.

## Alternative Solutions Considered (and Why They Were Rejected)

### Option A: Implement SIGHUP Signal Handling
**Description:** Catch SIGHUP in child processes and reopen log files.

**Rejected because:**
- Requires modifying every managed service to handle SIGHUP
- Child processes are not under our control (they're user services)
- Complex implementation for minimal benefit
- Not feasible for arbitrary third-party services

### Option B: Use copytruncate Instead of Rename
**Description:** Copy the log file, then truncate the original.

**Rejected because:**
- Race condition: writes during copy are lost or duplicated
- Requires more disk space (2x log size during rotation)
- More complex implementation than restart
- Doesn't leverage existing restart infrastructure

### Option C: Redirect Through a Pipe Handler
**Description:** Pipe stdout/stderr to a separate log collection process.

**Rejected because:**
- Significant architectural change (not a bug fix)
- Adds process management complexity
- Requires IPC for log rotation coordination
- Over-engineering for a simple problem

### Option D: Do Nothing (copytruncate in rotate_single_log)
**Description:** Change rotate_single_log to use copy+truncate.

**Rejected because:**
- Still has race condition issues
- Doesn't solve the fundamental problem
- More complex than restart
- Can lose log data during rotation

## Implementation Checklist

### Files to Modify

- [ ] **`packages/kodegend/src/service.rs`**
  - Lines 401-408: Add service restart after log rotation
  - Update comment on line 470 to reflect actual behavior

### Specific Changes

1. **Add restart logic after rotation (lines 406-407):**
   ```rust
   if self.def.log_stdout.is_some() || self.def.log_stderr.is_some() {
       warn!("{} rotating logs, restarting service to reopen file descriptors", self.name);
       self.tx.send(Cmd::Restart).ok();
   }
   ```

2. **Fix misleading comment (line 470):**
   ```rust
   // OLD (INCORRECT):
   // Rename current log to rotated name
   // The service will automatically create a new file on next write
   fs::rename(path, &rotated_name)?;
   
   // NEW (CORRECT):
   // Rename current log to rotated name
   // The service will be restarted to close old file descriptors and create new ones
   fs::rename(path, &rotated_name)?;
   ```

## Definition of Done

The implementation is complete when:

1. **Code Changes Applied:**
   - Service restart code added after log rotation in `rotate_logs()` function
   - Conditional check ensures restart only happens when logs are configured
   - Warning message logged when restart occurs
   - Misleading comment on line 470 corrected

2. **Behavior Verification:**
   - Start a service with log rotation enabled
   - Trigger log rotation (manually or wait for hourly tick)
   - Verify rotated file (`service.log.1`) stops receiving new writes
   - Verify new main log file (`service.log`) is created and receiving writes
   - Verify service restarts successfully after rotation
   - Check that warning message appears in daemon logs

3. **Edge Cases Handled:**
   - Services without log configuration are not restarted
   - Services with only stdout logging work correctly
   - Services with only stderr logging work correctly
   - Services with both stdout and stderr logging work correctly
   - Log rotation with compression works correctly
   - Log rotation with timestamps works correctly

## References

### Code Files
- [`packages/kodegend/src/service.rs`](../../packages/kodegend/src/service.rs) - Main implementation file
  - Lines 368-408: `rotate_logs()` function
  - Lines 419-538: `rotate_single_log()` helper
  - Line 82: `Cmd::Restart` handler in event loop
  - Lines 112-119: Log file opening with `.create(true)`
  - Line 362: `auto_restart` usage example

- [`packages/kodegend/src/ipc.rs`](../../packages/kodegend/src/ipc.rs) - Command/Event definitions
  - Line 10: `Cmd::Restart` enum variant
  - Lines 30-33: `Evt::LogRotate` event structure

### External Resources
- [Baeldung: Rotating Logs with Logrotate in Linux](https://www.baeldung.com/linux/rotating-logs-logrotate) (May 2024)
- [StackExchange: How to properly logrotate logs of service managed by systemd](https://unix.stackexchange.com/questions/605121/how-to-properly-logrotate-logs-of-service-managed-by-systemd-via-file-config)
- [BetterStack: A Complete Guide to Managing Log Files with Logrotate](https://betterstack.com/community/guides/logging/how-to-manage-log-files-with-logrotate-on-ubuntu-20-04/) (Jan 2025)
- [Dash0: Mastering Log Rotation in Linux with Logrotate](https://www.dash0.com/guides/log-rotation-linux-logrotate) (Sep 2025)

### Crawled Documentation
- [docs/www.baeldung.com](../../docs/www.baeldung.com) - Detailed logrotate behavior
- [docs/unix.stackexchange.com](../../docs/unix.stackexchange.com) - Community best practices

---

**Last Updated:** 2025-01-18  
**Status:** Ready for Implementation
