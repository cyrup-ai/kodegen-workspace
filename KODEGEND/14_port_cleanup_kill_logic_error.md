# Logic Error: Misleading Flow in kill_process_graceful

## Severity
**LOW** - Code clarity issue with misleading logs

## Location
`packages/kodegend/src/service/port_cleanup.rs`
- Lines 147-159

## Issue Description
The graceful kill logic has confusing control flow with misleading log message:

```rust
#[cfg(unix)]
let graceful_result = process.kill_with(Signal::Term);

if graceful_result.is_some() {
    // Wait a moment for graceful shutdown
    std::thread::sleep(Duration::from_secs(2));
    
    // Refresh and check if still running
    system.refresh_processes(ProcessesToUpdate::All, true);
    if system.process(sysinfo_pid).is_some() {
        log::warn!("Process {} did not terminate gracefully, force killing", pid);
        // NO RETURN HERE - falls through!
    } else {
        log::info!("Process {} terminated gracefully", pid);
        return Ok(());
    }
}

// Force kill with SIGKILL
system.refresh_processes(ProcessesToUpdate::All, true);
if let Some(process) = system.process(sysinfo_pid) {
    process.kill_with(Signal::Kill) // Actually happens here
        .ok_or_else(|| anyhow::anyhow!("Failed to kill process {}", pid))?;
    log::info!("Process {} force killed", pid);
}
```

## The Problem

When graceful termination fails (process still running after 2 seconds):

1. Line 154: Log says "**force killing**" (present continuous tense)
2. No return statement
3. Falls through to line 162
4. Refreshes process table AGAIN (redundant)
5. Line 164: Actually force kills the process
6. Line 166: Log says "Process {} **force killed**" (past tense)

**Result**: The log message at line 154 is MISLEADING. It says "force killing" but doesn't actually kill yet. The kill happens later.

## Why This Is Confusing

### For Developers
Reading the code, it appears the force kill happens at line 154 because:
- The message says "force killing"
- There's no obvious continuation
- The actual kill is in a separate block

### For Users
Log output shows:
```
Process 12345 did not terminate gracefully, force killing
Process 12345 force killed
```

This seems redundant - why two messages for the same operation?

### For Debugging
If a panic or error occurs between lines 154-164, logs show "force killing" but the kill never happened.

## Root Cause
Poor control flow structure - the "graceful check" block should contain the force kill fallback.

## Recommended Solution

### Option 1: Restructure with Early Returns
```rust
pub async fn kill_process_graceful(pid: u32) -> Result<()> {
    tokio::task::spawn_blocking(move || {
        let mut system = System::new();
        system.refresh_processes(ProcessesToUpdate::All, true);
        
        let sysinfo_pid = Pid::from(pid as usize);
        let process = system.process(sysinfo_pid)
            .ok_or_else(|| anyhow::anyhow!("Process {} not found", pid))?;
        
        // Try graceful termination first
        #[cfg(unix)]
        if process.kill_with(Signal::Term).is_some() {
            log::debug!("Sent SIGTERM to process {}, waiting 2s", pid);
            std::thread::sleep(Duration::from_secs(2));
            
            system.refresh_processes(ProcessesToUpdate::All, true);
            if system.process(sysinfo_pid).is_none() {
                log::info!("Process {} terminated gracefully", pid);
                return Ok(());
            }
            
            log::warn!("Process {} did not respond to SIGTERM", pid);
        }
        
        // Graceful failed or not available, force kill
        log::info!("Force killing process {} with SIGKILL", pid);
        system.refresh_processes(ProcessesToUpdate::All, true);
        
        if let Some(process) = system.process(sysinfo_pid) {
            process.kill_with(Signal::Kill)
                .ok_or_else(|| anyhow::anyhow!("Failed to send SIGKILL to process {}", pid))?;
            log::info!("Process {} force killed successfully", pid);
        } else {
            log::info!("Process {} already exited", pid);
        }
        
        Ok(())
    })
    .await?
}
```

### Option 2: Explicit State Tracking
```rust
let graceful_success = if graceful_result.is_some() {
    std::thread::sleep(Duration::from_secs(2));
    system.refresh_processes(ProcessesToUpdate::All, true);
    
    if system.process(sysinfo_pid).is_none() {
        log::info!("Process {} terminated gracefully", pid);
        true
    } else {
        log::warn!("Process {} did not respond to SIGTERM, will force kill", pid);
        false
    }
} else {
    false
};

if !graceful_success {
    system.refresh_processes(ProcessesToUpdate::All, true);
    if let Some(process) = system.process(sysinfo_pid) {
        log::info!("Sending SIGKILL to process {}", pid);
        process.kill_with(Signal::Kill)
            .ok_or_else(|| anyhow::anyhow!("Failed to kill process {}", pid))?;
        log::info!("Process {} force killed", pid);
    }
}
```

## Improved Log Messages

Instead of:
```
Process 12345 did not terminate gracefully, force killing
Process 12345 force killed
```

Use:
```
Process 12345 did not respond to SIGTERM
Sending SIGKILL to process 12345
Process 12345 force killed successfully
```

Or more concise:
```
Process 12345 did not respond to SIGTERM, escalating to SIGKILL
Process 12345 terminated with SIGKILL
```

## Files to Modify
- `packages/kodegend/src/service/port_cleanup.rs`

## Testing Considerations
- Test graceful kill (process exits on SIGTERM)
- Test force kill (process ignores SIGTERM)
- Verify log messages are clear and accurate
- Check that no redundant operations occur
