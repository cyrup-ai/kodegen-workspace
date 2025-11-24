# HIGH: Signal Channel Buffer May Overflow and Drop Signals

## Severity
**HIGH - SIGNAL LOSS**

## Location
`packages/kodegend/src/platform/signal.rs:31`

## Issue Description
The signal channel is created with a small bounded buffer of 16 signals. If signals arrive faster than they are consumed, the channel will block, potentially causing signal handlers to miss subsequent signals. This is particularly problematic for critical signals like SIGTERM.

### Problem Code
```rust
pub fn watch_signals() -> Result<Receiver<SignalKind>> {
    let (tx, rx) = bounded::<SignalKind>(16);  // ← Only 16 signal buffer
    
    #[cfg(unix)]
    spawn_unix_watcher(tx)?;
    
    #[cfg(windows)]
    spawn_windows_watcher(tx)?;
    
    Ok(rx)
}
```

### Signal Handler Behavior
```rust
// Unix (lines 92-96)
_ = sigterm.recv() => {
    if tx.send(SignalKind::Terminate).is_err() {
        break; // Channel closed
    }
    // ← If send() blocks here, we can't receive more signals
}
```

### Potential Issues

1. **Channel full scenario**:
   - 16 signals arrive rapidly
   - Consumer is slow (e.g., processing previous shutdown)
   - `tx.send()` blocks at line 93
   - While blocked, subsequent signals can't be received
   - Signal may be delayed or coalesced by OS

2. **Signal storms**:
   - Buggy script sends SIGHUP repeatedly (e.g., every 100ms)
   - Channel fills up
   - Later SIGTERM gets delayed
   - Daemon doesn't stop promptly

3. **Tokio select starvation**:
   - If one signal handler blocks on send()
   - Other branches in tokio::select! can't run
   - Could miss other signal types

### Real-World Scenario
```bash
# User tries to reload config repeatedly
for i in {1..20}; do kill -HUP $PID; done

# Then tries to stop daemon
kill -TERM $PID  # ← May be delayed if channel full
```

## Recommended Fixes

### Option 1: Use unbounded channel
```rust
use crossbeam_channel::unbounded;

pub fn watch_signals() -> Result<Receiver<SignalKind>> {
    let (tx, rx) = unbounded::<SignalKind>();  // ← Unbounded
    
    #[cfg(unix)]
    spawn_unix_watcher(tx)?;
    
    #[cfg(windows)]
    spawn_windows_watcher(tx)?;
    
    Ok(rx)
}
```

**Pros:**
- Never blocks on send
- Can't lose signals due to buffer full
- Simple change

**Cons:**
- Could accumulate unbounded signals if consumer is dead
- Memory usage grows if signals not consumed

### Option 2: Try_send with priority handling
```rust
// In signal handlers
_ = sigterm.recv() => {
    // Try to send, drop if full (with warning)
    if let Err(e) = tx.try_send(SignalKind::Terminate) {
        match e {
            TrySendError::Full(_) => {
                log::warn!("Signal buffer full - SIGTERM may be delayed");
                // Could force-clear buffer and retry for critical signals
            }
            TrySendError::Disconnected(_) => {
                break; // Consumer gone
            }
        }
    }
}
```

### Option 3: Larger buffer with monitoring
```rust
pub fn watch_signals() -> Result<Receiver<SignalKind>> {
    let (tx, rx) = bounded::<SignalKind>(256);  // ← Much larger buffer
    
    // Could spawn monitoring thread
    spawn_buffer_monitor(rx.clone());
    
    #[cfg(unix)]
    spawn_unix_watcher(tx)?;
    
    Ok(rx)
}

fn spawn_buffer_monitor(rx: Receiver<SignalKind>) {
    thread::spawn(move || {
        loop {
            let len = rx.len();
            if len > 200 {
                log::warn!("Signal buffer nearly full: {}/256", len);
            }
            thread::sleep(Duration::from_secs(1));
        }
    });
}
```

### Option 4: Coalescing for non-critical signals
```rust
use std::sync::atomic::{AtomicBool, Ordering};

struct SignalState {
    hangup_pending: AtomicBool,
    // ...
}

// Only send if not already pending
_ = sighup.recv() => {
    if !state.hangup_pending.swap(true, Ordering::SeqCst) {
        let _ = tx.send(SignalKind::Hangup);
    }
    // Subsequent SIGHUPs coalesced until first is processed
}
```

## Recommended Solution
**Use unbounded channel** for signals. Rationale:
- Signals are infrequent events (not data streams)
- Critical signals (SIGTERM) must never be dropped
- Risk of unbounded growth is minimal (bounded by signal rate)
- Consumer death is already handled (daemon exits)

Alternative: Use larger bounded buffer (256-1024) if unbounded is concerning.

## Testing
1. Send 100 SIGHUP signals rapidly: `for i in {1..100}; do kill -HUP $PID; sleep 0.01; done`
2. Verify all signals received (check log count)
3. Send SIGTERM immediately after: `kill -TERM $PID`
4. Verify daemon stops promptly (< 1 second)

## Impact
- **Severity**: HIGH - Could miss critical SIGTERM
- **Probability**: LOW - Requires signal storm
- **User Impact**: Daemon doesn't stop when signaled
- **Workaround**: Use SIGKILL (not graceful)

## Files to Modify
- `packages/kodegend/src/platform/signal.rs`

## References
- Crossbeam channel docs: https://docs.rs/crossbeam-channel/latest/crossbeam_channel/
- Signal handling best practices: https://www.gnu.org/software/libc/manual/html_node/Signal-Handling.html
