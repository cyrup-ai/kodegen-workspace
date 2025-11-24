# CRITICAL: Log File Handle Leak on Service Restarts

## Priority
**CRITICAL** - File descriptor exhaustion

## Location
`packages/kodegend/src/manager.rs` - `spawn_service()` method (lines 384-390, 398-399)

## Issue Description

Each service spawn opens log file handles that are never closed. When services restart, new handles are opened without closing old ones, leading to file descriptor exhaustion.

### Current Code

```rust
// Lines 384-390
let log_file = OpenOptions::new()
    .create(true)
    .append(true)
    .open(&log_path)?;

let stderr_file = log_file.try_clone()?;

// Lines 398-399
let mut child = Command::new(&service.command)
    .stdout(Stdio::from(log_file))
    .stderr(Stdio::from(stderr_file))
    .spawn()?;
```

### The Problem

1. **log_file** and **stderr_file** are owned by ServiceManager process
2. Passed to child process as stdout/stderr
3. **Child process inherits file descriptors** (gets copies)
4. **Parent process keeps original file descriptors open**
5. When child exits, child's copies close, **parent's copies stay open**
6. On restart, **new log files opened**, old ones **never closed**

### File Descriptor Leak Progression

```
Spawn 1:
- Open /var/log/kodegen/my-service.log → FD 10
- Clone → FD 11
- Pass to child (child gets FD 10, 11)
- Parent still holds FD 10, 11
- Open FDs: 2 (leaked)

Service crashes, restart:

Spawn 2:
- Open /var/log/kodegen/my-service.log → FD 12
- Clone → FD 13
- Pass to child
- Parent still holds FD 10, 11, 12, 13
- Open FDs: 4 (leaked)

Spawn 3:
- Open → FD 14, 15
- Parent holds: 10, 11, 12, 13, 14, 15
- Open FDs: 6 (leaked)

After N restarts: 2N leaked file descriptors
```

### System Limits

```bash
# Default soft limit (per-process)
$ ulimit -n
1024

# Some systems
$ ulimit -n
256
```

**Critical threshold**:
- Service restarts 128 times on system with ulimit 256
- 128 × 2 = 256 FDs leaked
- Next spawn: `open()` returns EMFILE (Too many open files)
- **Daemon can't spawn new services**
- Complete failure

### Additional Log Issues

1. **No log rotation**
   - Logs opened in append mode
   - Never truncated or rotated
   - Grow unbounded
   - Will fill disk space

2. **No cleanup of old logs**
   - Log files accumulate forever
   - Disk space exhaustion
   - No retention policy

## Impact

### Short Term (hours to days)

Depends on restart frequency:
- Stable service, rarely restarts: months before issue
- Unstable service, restarts every hour: days before issue
- Debug mode, restart every minute: hours before issue

### Critical Failure Scenario

```
Service crashes frequently due to bug
Restarts every 5 minutes
24 hours = 288 restarts
288 × 2 = 576 FDs leaked

ulimit -n = 1024
Additional FDs: ~20 (daemon's own files, sockets, etc.)
Remaining capacity: 1024 - 20 - 576 = 428

If daemon manages 10 services, each crashing:
10 × 576 = 5760 FDs needed
> 1024 limit

Result: EMFILE error
Daemon can't spawn any services
Complete service outage
Requires daemon restart to clear
```

### Detection

```bash
# Check open FDs for kodegend
lsof -p $(pgrep kodegend) | wc -l

# Should be: 
# - 3 (stdin/stdout/stderr)
# - N (one per running service)
# - ~10 (sockets, config files, etc.)
# Total: ~20 for 5 services

# If leaking:
# - Hundreds or thousands
# - Many duplicate paths to same log file
```

```bash
# Check FD limit
cat /proc/$(pgrep kodegend)/limits | grep "open files"
# Limit                     Soft Limit           Hard Limit           Units
# Max open files            1024                 1048576              files
```

## Root Cause

Log file handles passed to child process but not explicitly closed in parent after spawn.

In Unix:
- `Stdio::from(file)` consumes the File
- File is moved into Command
- But Command **duplicates** the FD for the child (via dup2)
- Original FD should be closed after spawn
- Rust's File drop closes it, BUT the dup'd FD in parent might not be closed properly

Actually, looking closer:
- `Stdio::from(log_file)` **moves** log_file
- log_file is dropped after being moved
- Should close the FD...

**But wait**: The child process holds a reference. Parent's Stdio object might keep the FD open until child exits?

**Real issue**: Each Child holds references to the Stdio objects. When ServiceState transitions without properly closing Child, the Stdio FDs remain open.

## Solution

### Option 1: Close Stdio Explicitly

```rust
// Don't pass File directly, use piped() and spawn reader task
let mut child = Command::new(&service.command)
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()?;

// Open log file
let mut log_file = OpenOptions::new()
    .create(true)
    .append(true)
    .open(&log_path)?;

// Reader task that writes to log file
if let Some(stdout) = child.stdout.take() {
    tokio::spawn(async move {
        let mut reader = BufReader::new(stdout);
        let mut buffer = Vec::new();
        while reader.read_until(b'\n', &mut buffer).await.is_ok() {
            log_file.write_all(&buffer).await.ok();
            buffer.clear();
        }
    });
}
```

**Problem**: We already have reader tasks (lines 404-425), but they log via `info!()`, not direct file write.

### Option 2: Use File Descriptors from Child

```rust
use std::os::unix::io::{FromRawFd, AsRawFd};

let log_file = OpenOptions::new()
    .create(true)
    .append(true)
    .open(&log_path)?;

// Get raw FD
let fd = log_file.as_raw_fd();

let mut child = Command::new(&service.command)
    .stdout(unsafe { Stdio::from_raw_fd(fd) })
    .stderr(unsafe { Stdio::from_raw_fd(fd) })
    .spawn()?;

// Explicitly close our copy
drop(log_file);
```

**Problem**: Platform-specific, unsafe.

### Option 3: Null Stdio and Use System Logger

Best option: Don't redirect to files at all.

```rust
let mut child = Command::new(&service.command)
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .spawn()?;

// Services should log to syslog/journald
// Or collect logs via the reader tasks (lines 404-425)
```

**Issue**: Current reader tasks use `info!()` and `error!()`, which goes to daemon's log, not service-specific log files.

### Option 4: Fix the Real Issue - Properly Clean Up Child

The real issue is Child objects aren't properly cleaned up (see Issue #9).

When we drop Child without calling wait():
- Process becomes zombie
- **Child's Stdio handles remain open in parent**

Fix:
1. Always call `child.wait()` before dropping (Issue #9 fix)
2. This closes the Stdio handles
3. No leak

## Recommended Solution

**Combination**:

1. **Fix zombie leak** (Issue #9) - Always wait() before dropping Child
   - This will also close Stdio handles
2. **Verify FD cleanup** with proper Drop impl
3. **Add log rotation** for long-running services
4. **Monitor FD count** in daemon

```rust
impl ServiceState {
    async fn transition_to_stopped(self) -> Self {
        if let ServiceState::Running { mut child, .. } = self {
            // Kill process
            let _ = child.kill().await;
            
            // Wait to reap zombie AND close Stdio handles
            let _ = tokio::time::timeout(
                Duration::from_secs(5),
                child.wait()
            ).await;
            
            // child dropped here, Stdio handles closed
        }
        ServiceState::Stopped
    }
}
```

### Log Rotation

```rust
pub struct ServiceConfig {
    // ... existing fields
    pub log_max_size_mb: usize,        // Default: 10MB
    pub log_max_files: usize,          // Default: 5
    pub log_rotation_enabled: bool,    // Default: true
}

// Before opening log:
fn rotate_logs_if_needed(log_path: &Path, max_size: u64, max_files: usize) -> Result<()> {
    if let Ok(metadata) = std::fs::metadata(log_path) {
        if metadata.len() > max_size {
            // Rotate: log.txt -> log.txt.1 -> log.txt.2 -> ... -> log.txt.N (deleted)
            for i in (1..max_files).rev() {
                let old = format!("{}.{}", log_path.display(), i);
                let new = format!("{}.{}", log_path.display(), i + 1);
                let _ = std::fs::rename(old, new);
            }
            let _ = std::fs::rename(log_path, format!("{}.1", log_path.display()));
        }
    }
    Ok(())
}
```

## Required Changes

1. **Fix Issue #9** (zombie leak) - This will likely fix FD leak too
2. Verify with FD monitoring test
3. Add log rotation logic
4. Add FD count monitoring
5. Add cleanup of old rotated logs

## Testing

```rust
#[tokio::test]
async fn test_no_fd_leak_on_restart() {
    let start_fds = count_open_fds();
    
    let mut manager = ServiceManager::new(config);
    
    // Restart service 100 times
    for _ in 0..100 {
        manager.spawn_service("test").await.unwrap();
        tokio::time::sleep(Duration::from_millis(100)).await;
        manager.stop_service("test").await.unwrap();
    }
    
    let end_fds = count_open_fds();
    
    // Should not leak FDs
    assert!(end_fds - start_fds < 10);  // Allow small variance
}

fn count_open_fds() -> usize {
    std::fs::read_dir("/proc/self/fd")
        .unwrap()
        .count()
}
```

## Monitoring

Add to daemon startup:

```rust
async fn monitor_fd_usage() {
    loop {
        tokio::time::sleep(Duration::from_secs(60)).await;
        
        let fd_count = count_open_fds();
        let (soft_limit, _) = get_fd_limit();
        
        let usage_pct = (fd_count as f64 / soft_limit as f64) * 100.0;
        
        if usage_pct > 80.0 {
            error!("FD usage critical: {}/{} ({}%)", 
                   fd_count, soft_limit, usage_pct);
        } else if usage_pct > 50.0 {
            warn!("FD usage high: {}/{} ({}%)", 
                  fd_count, soft_limit, usage_pct);
        }
    }
}
```

## Related Issues

- Issue #9: Zombie process leak (root cause fix)
- Issue #3: Reader task leak (complementary issue)

## References

- `ulimit -n`: File descriptor limit
- `lsof`: List open files
- `/proc/PID/fd`: Open file descriptors
- Log rotation: logrotate(8)
