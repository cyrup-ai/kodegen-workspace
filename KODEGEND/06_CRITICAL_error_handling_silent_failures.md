# CRITICAL: Silent Failures in spawn_service Error Handling

## Priority
**HIGH** - Incorrect state, difficult debugging

## Location  
`packages/kodegend/src/manager.rs` - `spawn_service()` method (lines 358-457)

## Issue Description

The `spawn_service()` method has multiple error handling issues where failures are logged but not propagated, leading to incorrect service states and silent failures.

### Issue 1: Command Spawn Failure Returns Ok

```rust
// Line 391-395
let mut child = match Command::new(&service.command)
    .args(&service.args)
    // ... setup
    .spawn() {
        Ok(child) => child,
        Err(e) => {
            error!("Failed to spawn service {}: {}", service_name, e);
            return Ok(());  // <-- WRONG: Returns Ok when it failed!
        }
    };
```

**Problem**: Spawn fails (command not found, permissions, etc.) but function returns `Ok(())`. Caller thinks service started successfully.

**Impact**:
- Service state not updated to Failed
- No retry attempts
- User thinks service is running
- `status` command shows incorrect state

### Issue 2: Systemd Detection Failure Silent

```rust
// Line 432-436
if let Ok(true) = self.is_systemd_service().await {
    info!("Running under systemd for service {}", service_name);
}
```

**Problem**: `is_systemd_service()` errors are silently ignored. If detection fails due to I/O error, we assume not systemd.

**Impact**:
- Might miss systemd environment
- Could affect logging/notification behavior
- Hard to debug when systemd integration fails

### Issue 3: Reader Tasks Panic Handling

```rust
// Line 404-418
tokio::spawn(async move {
    let reader = BufReader::new(stdout);
    let mut lines = reader.lines();
    while let Ok(Some(line)) = lines.next_line().await {
        info!("[{}] {}", name.clone(), line);
    }
    // No error handling - what if next_line() returns Err?
});
```

**Problem**: 
- If `next_line()` fails, error is silently ignored
- Task exits without logging why
- No notification to ServiceManager

**Impact**:
- Lost log output
- Silent task death
- No debugging info

### Issue 4: No Verification After Spawn

```rust
// Line 449-453
if let Some(state) = self.services.get_mut(service_name) {
    *state = ServiceState::Running(child);
}
```

**Problem**: Service marked as Running immediately after spawn, without verifying process actually started.

**Impact**:
- Process might have already exited
- Race condition: marked Running, then immediately crashes
- Status shows Running for dead process

## Solutions

### Fix 1: Return Error on Spawn Failure

```rust
let mut child = Command::new(&service.command)
    .args(&service.args)
    .spawn()
    .map_err(|e| {
        error!("Failed to spawn service {}: {}", service_name, e);
        
        // Update state to Failed
        if let Some(state) = self.services.get_mut(service_name) {
            *state = ServiceState::Failed(format!("Spawn failed: {}", e));
        }
        
        e
    })?;  // Propagate error
```

### Fix 2: Log Systemd Detection Errors

```rust
match self.is_systemd_service().await {
    Ok(true) => {
        info!("Running under systemd for service {}", service_name);
    }
    Ok(false) => {
        debug!("Not running under systemd for service {}", service_name);
    }
    Err(e) => {
        warn!("Failed to detect systemd environment: {}", e);
        // Assume not systemd, but log the error
    }
}
```

### Fix 3: Handle Reader Task Errors

```rust
tokio::spawn(async move {
    let reader = BufReader::new(stdout);
    let mut lines = reader.lines();
    loop {
        match lines.next_line().await {
            Ok(Some(line)) => {
                info!("[{}] {}", name, line);
            }
            Ok(None) => {
                debug!("[{}] stdout closed", name);
                break;
            }
            Err(e) => {
                error!("[{}] Error reading stdout: {}", name, e);
                break;
            }
        }
    }
});
```

### Fix 4: Verify Process Started

```rust
// After spawn, give it a moment to fail fast
tokio::time::sleep(Duration::from_millis(100)).await;

// Check if process already exited
match child.try_wait() {
    Ok(Some(status)) => {
        // Already exited - spawn failed
        let msg = format!("Process exited immediately with status: {}", status);
        if let Some(state) = self.services.get_mut(service_name) {
            *state = ServiceState::Failed(msg.clone());
        }
        return Err(anyhow!(msg));
    }
    Ok(None) => {
        // Still running - good!
        if let Some(state) = self.services.get_mut(service_name) {
            *state = ServiceState::Running(child);
        }
    }
    Err(e) => {
        return Err(anyhow!("Failed to check process status: {}", e));
    }
}
```

## Required Changes

1. Change line 394 from `return Ok(())` to propagate error
2. Update callers to handle spawn_service errors:
   - `handle_command()` line 158: Update state to Failed
   - `monitor_service()` line 335: Update state to Failed
3. Add error handling to reader tasks (lines 404-425)
4. Add systemd detection error logging (line 432-436)
5. Add process start verification after spawn
6. Update all `spawn_service()` error paths to update service state

## Testing Strategy

- Test with non-existent command - verify state becomes Failed
- Test with command that exits immediately - verify detected
- Test with command that fails permission check
- Mock I/O errors in reader tasks - verify logged
- Verify all error paths update service state correctly

## Related Issues

- Issue #12: State not updated when spawn_service fails in monitor
- Issue #3: Reader task leak
