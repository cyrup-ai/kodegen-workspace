# Error Handling: Missing Context and Silent Failures

## Severity: MEDIUM

## Location
Multiple locations throughout `packages/kodegend/src/daemon.rs`

## Issue Description
Error messages throughout the module lack sufficient context, and some errors are silently ignored, making debugging production issues difficult.

## Examples of Insufficient Error Context

### Example 1: PID File Errors
```rust
// Current (line 109)
.map_err(|_| anyhow!("Invalid PID in file"))?;

// Should be:
.with_context(|| format!(
    "Invalid PID in file {} (contents: {:?})",
    path.display(),
    std::fs::read_to_string(path).unwrap_or_else(|_| "<unreadable>".to_string())
))?;
```

### Example 2: Process Already Running
```rust
// Current (line 92)
return Err(anyhow!("Service already running with PID {}", existing_pid));

// Should be:
return Err(anyhow!(
    "Service already running with PID {} (PID file: {})",
    existing_pid,
    path.display()
));
```

### Example 3: Failed to Create PID File
```rust
// Current (line 100)
let mut file = File::create(path)?;

// Should be:
let mut file = File::create(path)
    .with_context(|| format!("Failed to create PID file at {}", path.display()))?;
```

## Silent Error Ignoring

### Example 1: stdin Read (line 58)
```rust
// Current
let _ = std::io::stdin().read(&mut [0u8; 1]);
```

**Problem**: Explicitly ignores errors. If stdin is closed or unavailable, this silently continues.

**Fix**:
```rust
if let Err(e) = std::io::stdin().read(&mut [0u8; 1]) {
    log::warn!("Failed to read from stdin during daemonization: {}", e);
}
```

### Example 2: Windows Error Codes Not Checked
```rust
// Current (line 207)
TerminateProcess(handle, 1);
```

**Problem**: Return value not checked. Process termination might fail silently.

**Fix**:
```rust
if TerminateProcess(handle, 1) == 0 {
    let error = std::io::Error::last_os_error();
    return Err(anyhow!("Failed to terminate process {}: {}", pid, error));
}
```

## Recommended Improvements

### 1. Add Structured Error Types
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DaemonError {
    #[error("Failed to create PID file at {path}: {source}")]
    PidFileCreate {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    
    #[error("Invalid PID in file {path}: {content:?}")]
    InvalidPid {
        path: PathBuf,
        content: String,
    },
    
    #[error("Service already running with PID {pid} (PID file: {path})")]
    AlreadyRunning {
        pid: i32,
        path: PathBuf,
    },
    
    #[error("Failed to stop process {pid}: {reason}")]
    StopFailed {
        pid: i32,
        reason: String,
    },
    
    #[error("Daemonization failed: {0}")]
    DaemonizeFailed(String),
    
    #[error("Process {pid} failed to start within timeout")]
    StartTimeout {
        pid: i32,
    },
}

pub type Result<T> = std::result::Result<T, DaemonError>;
```

### 2. Enhanced Error Context
```rust
pub fn read_pid_file(path: &Path) -> Result<i32> {
    let pid_str = fs::read_to_string(path)
        .map_err(|e| DaemonError::PidFileCreate {
            path: path.to_path_buf(),
            source: e,
        })?;
    
    let pid = pid_str.trim().parse::<i32>()
        .map_err(|_| DaemonError::InvalidPid {
            path: path.to_path_buf(),
            content: pid_str.clone(),
        })?;
    
    if pid <= 0 {
        return Err(DaemonError::InvalidPid {
            path: path.to_path_buf(),
            content: format!("{} (must be > 0)", pid),
        });
    }
    
    Ok(pid)
}
```

### 3. Add Logging Throughout
```rust
pub fn stop_service(pid_file: &Path) -> Result<()> {
    log::info!("Stopping service with PID file: {}", pid_file.display());
    
    let pid = read_pid_file(pid_file)?;
    log::debug!("Read PID {} from file", pid);
    
    if !is_process_running(pid)? {
        log::info!("Process {} not running, cleaning up PID file", pid);
        remove_pid_file(pid_file)?;
        return Ok(());
    }
    
    log::info!("Sending SIGTERM to process {}", pid);
    
    #[cfg(unix)]
    {
        use nix::sys::signal::{kill, Signal};
        kill(Pid::from_raw(pid), Signal::SIGTERM)
            .map_err(|e| DaemonError::StopFailed {
                pid,
                reason: format!("Failed to send SIGTERM: {}", e),
            })?;
    }
    
    log::info!("Successfully sent termination signal to process {}", pid);
    remove_pid_file(pid_file)?;
    Ok(())
}
```

### 4. Better Windows Error Handling
```rust
#[cfg(windows)]
pub fn stop_service_windows(pid: i32) -> Result<()> {
    use windows_sys::Win32::Foundation::GetLastError;
    
    unsafe {
        let handle = OpenProcess(PROCESS_TERMINATE, 0, pid as u32);
        if handle.is_null() {
            let error_code = GetLastError();
            return Err(DaemonError::StopFailed {
                pid,
                reason: format!(
                    "OpenProcess failed with error code {}: {}",
                    error_code,
                    std::io::Error::from_raw_os_error(error_code as i32)
                ),
            });
        }
        
        if TerminateProcess(handle, 1) == 0 {
            let error_code = GetLastError();
            CloseHandle(handle);
            return Err(DaemonError::StopFailed {
                pid,
                reason: format!(
                    "TerminateProcess failed with error code {}: {}",
                    error_code,
                    std::io::Error::from_raw_os_error(error_code as i32)
                ),
            });
        }
        
        CloseHandle(handle);
    }
    
    Ok(())
}
```

## Specific Issues to Fix

### Line 58: Silent stdin read failure
```rust
// Before
let _ = std::io::stdin().read(&mut [0u8; 1]);

// After
if let Err(e) = std::io::stdin().read(&mut [0u8; 1]) {
    log::warn!("Failed to read from stdin: {}", e);
}
```

### Line 92: Add PID file path to error
```rust
// Before
return Err(anyhow!("Service already running with PID {}", existing_pid));

// After
return Err(DaemonError::AlreadyRunning {
    pid: existing_pid,
    path: path.to_path_buf(),
});
```

### Line 109: Show invalid content
```rust
// Before
.map_err(|_| anyhow!("Invalid PID in file"))?;

// After
.map_err(|_| DaemonError::InvalidPid {
    path: path.to_path_buf(),
    content: pid_str.clone(),
})?;
```

### Line 160-168: Check Windows API return values
```rust
// Add error checking for all Windows API calls
if OpenProcess(...) == null { /* check GetLastError */ }
if TerminateProcess(...) == 0 { /* check GetLastError */ }
```

## Benefits of Improved Error Handling

### Before (unhelpful)
```
Error: Invalid PID in file

Error: Service already running with PID 12345
```

### After (actionable)
```
Error: Invalid PID in file /var/run/kodegend.pid: "abc123" (expected integer)

Error: Service already running with PID 12345
  PID file: /var/run/kodegend.pid
  Process name: kodegend
  Started: 2024-01-15 10:30:45
  
Suggestion: Use 'kodegend stop' to stop the running instance
```

## Implementation Priority

1. **High**: Add context to all error paths
2. **High**: Fix silent error ignoring (stdin, Windows APIs)
3. **Medium**: Add structured error types
4. **Medium**: Add comprehensive logging
5. **Low**: Pretty error formatting

## Testing Strategy

### Error Message Tests
```rust
#[test]
fn test_error_messages_include_path() {
    let bad_pid_file = Path::new("/tmp/bad.pid");
    std::fs::write(bad_pid_file, "not_a_number").unwrap();
    
    let err = read_pid_file(bad_pid_file).unwrap_err();
    let msg = format!("{}", err);
    
    // Should include both path and content
    assert!(msg.contains("/tmp/bad.pid"));
    assert!(msg.contains("not_a_number"));
}
```

## Dependencies
```toml
[dependencies]
thiserror = "1.0"  # For structured errors
log = "0.4"        # For logging
```

## References
- Rust Error Handling best practices
- "Error Handling in Rust" - Rust Book Chapter 9
- thiserror documentation
