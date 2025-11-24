# LOW: Polling Overhead in Graceful Shutdown (Unavoidable with Current Rust std)

**Priority:** LOW  
**Component:** `packages/kodegend/src/service.rs`  
**Lines:** 206-232  
**Impact:** Performance - Minor CPU/syscall overhead during shutdown

## Problem

The graceful shutdown implementation uses a polling loop that checks process status every 100ms, resulting in up to 100 syscalls during a 10-second shutdown timeout. While this works correctly, it's not the most efficient approach.

### Current Implementation

**Lines 206-232:**
```rust
// Wait for graceful exit with configurable timeout
let grace_period = Duration::from_secs(
    self.def.shutdown_timeout_secs.unwrap_or(10)
);
let start = Instant::now();

// Poll for process exit
while start.elapsed() < grace_period {
    match ch.try_wait() {
        Ok(Some(status)) => {
            // Process exited gracefully
            info!(
                "{} exited gracefully in {:.1}s with status: {:?}",
                self.name,
                start.elapsed().as_secs_f64(),
                status
            );
            self.send_stopped_event(pid)?;
            return Ok(());
        }
        Ok(None) => {
            // Still running, wait a bit
            thread::sleep(Duration::from_millis(100));  // ← 100ms polling
        }
        Err(e) => {
            // Error checking status
            warn!(
                "{} error checking process status: {}, forcing SIGKILL",
                self.name, e
            );
            break;
        }
    }
}
```

### Overhead Analysis

**For default 10-second timeout:**
- Polling interval: 100ms
- Maximum iterations: 10,000ms / 100ms = 100
- Syscalls: Up to 100 `try_wait()` calls (maps to `waitpid()` with `WNOHANG`)
- Thread wakeups: Up to 100 (scheduler overhead)

**Best case:** Process exits immediately, 1 iteration  
**Average case:** Process exits in 2-3 seconds, 20-30 iterations  
**Worst case:** Full timeout, 100 iterations

### Why Polling Is Currently Necessary

As of Rust 1.83 (and edition 2024), the standard library `std::process::Child` does **not** provide:
- `wait_timeout()` - blocking wait with timeout
- Signal-based notification when child exits
- Async/await support for child process waits

The only available methods are:
- `wait()` - blocks forever
- `try_wait()` - non-blocking check

Therefore, polling with `try_wait()` + `thread::sleep()` is the **only portable solution** in stable Rust.

### Platform-Specific Alternatives (Not Currently Used)

**Unix:** Could use `waitpid()` with signals or `select()`/`poll()` on process file descriptors (not exposed by std)

**Linux-specific:** Could use pidfd and epoll (requires nightly or external crates)

**Cross-platform crates:** Some crates like `wait-timeout` provide this functionality, but add dependencies

## Is This Actually A Problem?

### Arguments for "Not Really A Problem"

1. **Shutdown is infrequent:** Services stop rarely (restarts, daemon shutdown)
2. **100ms is reasonable:** Not aggressive polling (some systems use 10ms)
3. **Overhead is minimal:** 100 syscalls over 10 seconds = 10 syscalls/second (negligible)
4. **Works correctly:** Processes exit gracefully as expected
5. **Portable:** Works on all platforms (Unix, Windows, macOS)

### Arguments for "Could Be Better"

1. **Unnecessary wakeups:** Thread wakes up 10 times/second just to check
2. **Power efficiency:** On laptops/embedded, frequent wakeups waste battery
3. **Scale issues:** If stopping 100 services simultaneously, 10,000 syscalls/second
4. **Sleep drift:** Polling interval can drift if syscalls take time
5. **Aesthetic:** Polling feels inelegant compared to event-driven

## Potential Improvements

### Option 1: Increase Polling Interval (Simple)

Change 100ms to 250ms or 500ms:

```rust
Ok(None) => {
    // Still running, wait a bit
    thread::sleep(Duration::from_millis(250));  // ← Reduced frequency
}
```

**Impact:**
- 10-second timeout: 40 iterations instead of 100 (60% reduction)
- Still responsive enough (process that exits in 1s detected within 250ms)
- Trade-off: Slightly less responsive to process exit

**Recommendation:** Change to 200-250ms as a simple optimization

### Option 2: Adaptive Polling (Medium Complexity)

Poll frequently at first, then back off:

```rust
let mut poll_interval = Duration::from_millis(50);   // Fast at first
const MAX_INTERVAL: Duration = Duration::from_millis(500);

while start.elapsed() < grace_period {
    match ch.try_wait() {
        Ok(Some(status)) => { /* exited */ }
        Ok(None) => {
            thread::sleep(poll_interval);
            
            // Exponential backoff, but cap at MAX_INTERVAL
            poll_interval = std::cmp::min(
                poll_interval * 2,
                MAX_INTERVAL
            );
        }
        Err(e) => { /* error */ }
    }
}
```

**Benefits:**
- Fast response for processes that exit quickly (50ms, 100ms, 200ms)
- Low overhead for stubborn processes (500ms after 1 second)
- Adaptive to process behavior

**Example timeline:**
- 0-50ms: Poll at 50ms
- 50-150ms: Poll at 100ms  
- 150-350ms: Poll at 200ms
- 350ms+: Poll at 500ms

Total iterations for 10s: ~25 instead of 100

### Option 3: Use wait-timeout Crate (Add Dependency)

```toml
[dependencies]
wait-timeout = "0.2"
```

```rust
use wait_timeout::ChildExt;

let grace_period = Duration::from_secs(
    self.def.shutdown_timeout_secs.unwrap_or(10)
);

match ch.wait_timeout(grace_period)? {
    Some(status) => {
        info!("{} exited gracefully: {:?}", self.name, status);
        self.send_stopped_event(pid)?;
        return Ok(());
    }
    None => {
        warn!("{} did not exit within {}s, sending SIGKILL",
            self.name, grace_period.as_secs()
        );
    }
}
```

**Benefits:**
- Zero polling overhead
- Uses platform-specific efficient mechanisms
- Cleaner code

**Cons:**
- Adds external dependency
- Crate is small but unmaintained (last update 2019)

### Option 4: Wait for Rust std Support

The Rust project has discussed adding `Child::wait_timeout()` but it's not yet stabilized. Monitor these issues:
- [rust-lang/rust#41618](https://github.com/rust-lang/rust/issues/41618) - wait_timeout tracking issue

When stabilized, update to use it.

## Recommended Solution

**Short-term:** Option 1 - Change polling interval to 200-250ms  
**Medium-term:** Option 2 - Implement adaptive polling if profiling shows benefit  
**Long-term:** Option 4 - Use std::Child::wait_timeout() when available

**Reasoning:**
- Option 1 is trivial (change one number)
- 60% reduction in syscalls for almost no cost
- Don't add dependencies for marginal benefit
- Wait for std library support

## Implementation

### Immediate Fix (Option 1)

**One-line change at line 221:**
```rust
Ok(None) => {
    // Still running, wait a bit
    thread::sleep(Duration::from_millis(200));  // ← Changed from 100
}
```

### Configuration Option (If Needed)

Allow users to tune polling interval:

```yaml
services:
  - name: my-service
    shutdown_timeout_secs: 10
    shutdown_poll_interval_ms: 250  # Optional, default 200
```

## Testing

### Responsiveness Test

1. Create test service that exits after 1 second on SIGTERM
2. Stop service and measure time to detection
3. Verify detected within 1.2 seconds (1s exit + 200ms poll)

### Overhead Test

Before and after:

```bash
# Monitor syscalls during shutdown
strace -c kodegend stop my-service
```

**Expected:** Reduction in waitpid() calls from ~100 to ~50 for 10s timeout

### Battery Impact (Laptop)

Run for 1 hour with services stopping every minute:

```bash
# Before: 60 stops × 100 iterations = 6000 wakeups/hour
# After:  60 stops × 50 iterations = 3000 wakeups/hour
```

Measure with `powertop` to verify reduced wakeup frequency.

## Impact Assessment

**Priority: LOW** because:
- Works correctly as-is
- Overhead is minimal in practice
- Shutdown is infrequent operation
- Simple fix available (change one number)
- Not a bug, just suboptimal

**Fix anyway?** Yes, because:
- One-line change has no downside
- 50-60% reduction in overhead
- Better power efficiency
- More professional implementation

## References

- Line 206-232: Graceful shutdown loop in `stop()`
- Line 200-202: shutdown_timeout_secs configuration
- Line 221: `thread::sleep(Duration::from_millis(100))` - the polling interval
- Rust issue tracking wait_timeout: rust-lang/rust#41618
