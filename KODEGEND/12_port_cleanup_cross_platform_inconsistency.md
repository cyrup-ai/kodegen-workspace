# Cross-Platform Inconsistency: Different Error Handling on Unix vs Windows

## Severity
**MEDIUM** - Portability issue, different behavior on different platforms

## Location
`packages/kodegend/src/service/port_cleanup.rs`
- Lines 40-82: Unix implementation
- Lines 84-121: Windows implementation

## Issue Description
The Unix and Windows implementations of `find_process_by_port` handle command failures differently:

### Unix Version (Lines 45-80)
```rust
let output = Command::new("lsof")
    .args(["-ti", &format!(":{}", port)])
    .output()
    .await;

match output {
    Ok(output) if output.status.success() => { /* parse PID */ }
    Ok(_) => {
        // lsof returned non-zero (no process found or lsof not available)
        Ok(None)  // SILENT FAILURE
    }
    Err(e) => {
        log::warn!("lsof command failed: {}", e);
        Ok(None)  // SILENT FAILURE
    }
}
```

### Windows Version (Lines 89-96)
```rust
let output = Command::new("netstat")
    .args(["-ano"])
    .output()
    .await?;  // PROPAGATES ERROR

if !output.status.success() {
    return Ok(None);
}
```

## Production Impact

### Unix Behavior
If `lsof` command fails (not installed, permission denied, etc.):
1. Warning is logged
2. Function returns `Ok(None)`
3. Caller thinks "no process found"
4. Cleanup continues as if port is free
5. Server bind fails mysteriously

### Windows Behavior
If `netstat` command fails:
1. Error propagates with `?`
2. Caller gets error
3. Cleanup fails explicitly
4. User sees clear error message

### Result: Platform-Dependent UX
- **Unix users**: Silent failures, confusing "port in use" errors
- **Windows users**: Clear error messages about command failures
- This is a **PORTABILITY BUG** - same operation, different outcomes

## Example Scenarios

### Scenario 1: Command Not Installed
**Unix**: `lsof` not in PATH → returns `Ok(None)` → cleanup fails silently
**Windows**: `netstat` not in PATH → returns `Err()` → cleanup fails with error

### Scenario 2: Permission Denied
**Unix**: SELinux blocks `lsof` → returns `Ok(None)` → confusing behavior
**Windows**: UAC blocks `netstat` → returns `Err()` → clear error

## Root Cause
Inconsistent error handling philosophy between platform implementations.

## Recommended Solution

### Standardize Error Handling
Both platforms should behave the same way. Two options:

#### Option A: Both Silent (Not Recommended)
```rust
// Windows version matches Unix
let output = Command::new("netstat")
    .args(["-ano"])
    .output()
    .await
    .map_err(|e| {
        log::warn!("netstat command failed: {}", e);
        return Ok(None);
    })?;
```

**Problem**: Silent failures are bad UX.

#### Option B: Both Propagate Errors (Recommended)
```rust
// Unix version matches Windows
#[cfg(unix)]
async fn find_process_by_port_unix(port: u16) -> Result<Option<u32>> {
    let output = Command::new("lsof")
        .args(["-ti", &format!(":{}", port)])
        .output()
        .await
        .context("Failed to execute lsof command")?;  // PROPAGATE
    
    if !output.status.success() {
        // Distinguish "not found" from "command failed"
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("command not found") || stderr.contains("No such file") {
            return Err(anyhow::anyhow!(
                "lsof command not available - cannot check port usage"
            ));
        }
        // Non-zero exit but no error = no process found
        return Ok(None);
    }
    
    // ... parse PID ...
}
```

### Add Fallback Commands
Since `lsof` might not be available on all Unix systems:

```rust
#[cfg(unix)]
async fn find_process_by_port_unix(port: u16) -> Result<Option<u32>> {
    // Try lsof first
    match try_lsof(port).await {
        Ok(pid) => return Ok(pid),
        Err(e) if is_command_not_found(&e) => {
            log::debug!("lsof not available, trying ss");
        }
        Err(e) => return Err(e),
    }
    
    // Fallback to ss (more common on modern Linux)
    try_ss(port).await
}
```

## Files to Modify
- `packages/kodegend/src/service/port_cleanup.rs`

## Testing Considerations
- Test on both Unix and Windows
- Test with command not available
- Test with permission denied
- Verify consistent behavior across platforms
- Add unit tests that mock command failures
