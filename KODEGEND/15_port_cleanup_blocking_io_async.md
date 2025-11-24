# Performance Issue: Blocking Thread Pool with Long Sleep

## Severity
**MEDIUM** - Can exhaust tokio blocking thread pool

## Location
`packages/kodegend/src/service/port_cleanup.rs`
- Line 131: `spawn_blocking()`
- Line 149: `std::thread::sleep(Duration::from_secs(2))`

## Issue Description
The `kill_process_graceful` function blocks a tokio worker thread for 2 full seconds:

```rust
pub async fn kill_process_graceful(pid: u32) -> Result<()> {
    use sysinfo::{Pid, ProcessesToUpdate, Signal, System};

    // Spawn blocking task for sysinfo operations
    tokio::task::spawn_blocking(move || {
        let mut system = System::new();
        system.refresh_processes(ProcessesToUpdate::All, true);

        // ... graceful kill attempt ...

        if graceful_result.is_some() {
            // PROBLEM: Blocks thread for 2 seconds!
            std::thread::sleep(Duration::from_secs(2));
            
            // ... check if process terminated ...
        }
        
        // ... force kill ...
    })
    .await?  // Wait for blocking task
}
```

## Production Impact

### Scenario: Multiple Ports Being Cleaned
During daemon startup with 15 servers:
1. Embedded_servers.rs starts servers sequentially (currently)
2. Each server's port may need cleanup
3. If 5 ports need cleanup → 5 concurrent `spawn_blocking` tasks
4. Each blocks a thread for 2 seconds
5. **Requires 5 threads from blocking pool**

### Tokio Blocking Pool Limits
By default, tokio's blocking pool has:
- **Max threads**: 512 (or `core_count * 10`)
- **Thread spawn rate**: Limited to prevent thread explosion

With 5 concurrent 2-second sleeps:
- Uses 5 threads for 2 seconds each
- Not catastrophic, but wasteful

### Future Risk
If startup becomes parallel (as recommended in task #9):
- All 15 servers start concurrently
- If 10 need port cleanup → **10 threads blocked for 2s each**
- With multiple daemon restarts → pool exhaustion possible

## Root Cause
Mixing blocking sleep inside async context. The sleep portion doesn't need to be blocking - only the sysinfo operations do.

## Recommended Solution

### Option 1: Split Blocking and Async Work
```rust
pub async fn kill_process_graceful(pid: u32) -> Result<()> {
    use sysinfo::{Pid, ProcessesToUpdate, Signal, System};

    // First blocking section: send signal
    let graceful_sent = tokio::task::spawn_blocking(move || {
        let mut system = System::new();
        system.refresh_processes(ProcessesToUpdate::All, true);

        let sysinfo_pid = Pid::from(pid as usize);
        let process = system.process(sysinfo_pid)
            .ok_or_else(|| anyhow::anyhow!("Process {} not found", pid))?;

        #[cfg(unix)]
        let sent = process.kill_with(Signal::Term).is_some();
        
        #[cfg(windows)]
        let sent = true;

        Ok::<bool, anyhow::Error>(sent)
    })
    .await??;

    if graceful_sent {
        // ASYNC SLEEP - doesn't block thread pool!
        tokio::time::sleep(Duration::from_secs(2)).await;
        
        // Second blocking section: check if alive
        let still_running = tokio::task::spawn_blocking(move || {
            let mut system = System::new();
            system.refresh_processes(ProcessesToUpdate::All, true);
            
            let sysinfo_pid = Pid::from(pid as usize);
            Ok::<bool, anyhow::Error>(system.process(sysinfo_pid).is_some())
        })
        .await??;

        if !still_running {
            log::info!("Process {} terminated gracefully", pid);
            return Ok(());
        }
        
        log::warn!("Process {} did not respond to SIGTERM", pid);
    }

    // Third blocking section: force kill
    tokio::task::spawn_blocking(move || {
        let mut system = System::new();
        system.refresh_processes(ProcessesToUpdate::All, true);

        let sysinfo_pid = Pid::from(pid as usize);
        if let Some(process) = system.process(sysinfo_pid) {
            process.kill_with(Signal::Kill)
                .ok_or_else(|| anyhow::anyhow!("Failed to kill process {}", pid))?;
            log::info!("Process {} force killed", pid);
        }
        
        Ok::<(), anyhow::Error>(())
    })
    .await??;

    Ok(())
}
```

**Benefits**:
- Only blocks thread pool for sysinfo operations (milliseconds)
- Sleep happens in async context (zero thread cost)
- Can handle hundreds of concurrent cleanups

### Option 2: Use Async sysinfo Alternative
If available, use an async process management crate that doesn't require blocking.

### Option 3: Reduce Sleep Duration
Simple fix: reduce from 2s to 500ms or 1s:
```rust
std::thread::sleep(Duration::from_millis(500));
```

Less blocking time, but still wasteful.

## Performance Improvement

### Before (Current)
- 10 concurrent cleanups = 10 threads blocked for 2s
- Total thread-seconds: 20
- Pool capacity: -10 threads for 2 seconds

### After (Option 1)
- 10 concurrent cleanups = brief blocking for sysinfo only
- Total thread-seconds: ~0.1 (10 × 10ms)
- Pool capacity: minimal impact

### Speedup
- 200x reduction in thread pool usage
- Allows thousands of concurrent port cleanups

## Files to Modify
- `packages/kodegend/src/service/port_cleanup.rs`

## Testing Considerations
- Test concurrent port cleanup (spawn 10 simultaneously)
- Monitor thread pool usage
- Verify graceful termination still works correctly
- Test that async sleep doesn't affect kill logic
