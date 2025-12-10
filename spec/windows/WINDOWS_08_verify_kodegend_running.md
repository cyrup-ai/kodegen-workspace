# Task: Integrate verify_kodegend_running() into Windows Code Paths

## Priority: P0 (Security Critical)

## Related Errors
- `platform/mod.rs:184` - function `verify_kodegend_running` never used
- `platform/windows/mod.rs:370` - function `platform_verify_kodegend_running` never used

## Problem Statement

The `verify_kodegend_running()` function is implemented but never called on Windows. This function is **security critical** - it prevents PID reuse attacks (CVE-2020-14977 class).

## Security Risk: PID Reuse Attack

### Attack Scenario

1. kodegend starts with PID 1234, writes PID file
2. kodegend crashes or is killed
3. Another process starts and gets PID 1234 (OS reuses PIDs)
4. User runs `kodegend stop` which reads PID 1234 from file
5. `kodegend stop` sends termination signal to PID 1234
6. **WRONG PROCESS IS TERMINATED**

### Real-World Impact

- Could kill critical system processes
- Could kill user applications with unsaved data
- Attacker could intentionally trigger this for DoS

## Current Implementation

### Public Wrapper (platform/mod.rs:184)
```rust
pub fn verify_kodegend_running(pid: ProcessId) -> Result<bool, std::io::Error> {
    platform_verify_kodegend_running(pid)
}
```

### Windows Implementation (platform/windows/mod.rs:370)
```rust
pub(super) fn platform_verify_kodegend_running(pid: u32) -> Result<bool, std::io::Error> {
    // 1. Check if process exists (fast path)
    if !platform_is_process_running(pid)? {
        return Ok(false);
    }

    // 2. Use sysinfo to verify it's actually kodegend
    let mut system = System::new();
    system.refresh_processes(ProcessesToUpdate::All, true);

    // 3. Check executable name matches "kodegend.exe"
    match process.exe() {
        Some(exe_path) => {
            let exe_name = exe_path.file_name()...;
            let is_kodegend = exe_lower == "kodegend.exe" || ...;
            Ok(is_kodegend)
        }
        None => Err(PermissionDenied)
    }
}
```

## Required Implementation

### 1. Find All PID Usage Points

Search for code that:
- Reads from PID file
- Sends signals/terminates processes
- Assumes a PID belongs to kodegend

Likely locations:
- `src/main.rs` - `stop` and `status` commands
- `src/daemon.rs` - PID file handling
- `src/control/` - Daemon control operations

### 2. Add Verification Before Process Operations

Before any process termination:
```rust
// Read PID from file
let pid = read_pid_file()?;

// CRITICAL: Verify it's actually kodegend
if !verify_kodegend_running(pid)? {
    return Err(anyhow!("PID {} is not a kodegend process", pid));
}

// Safe to terminate
terminate_process(pid)?;
```

### 3. Handle Verification Failures

```rust
match verify_kodegend_running(pid) {
    Ok(true) => {
        // PID belongs to kodegend, safe to proceed
        terminate_process(pid)?;
    }
    Ok(false) => {
        // PID exists but is NOT kodegend
        warn!("Stale PID file: {} is not kodegend", pid);
        cleanup_stale_pid_file()?;
        return Err(anyhow!("kodegend is not running"));
    }
    Err(e) if e.kind() == ErrorKind::PermissionDenied => {
        // Cannot verify - fail safe by refusing
        return Err(anyhow!("Cannot verify PID {} belongs to kodegend: {}", pid, e));
    }
    Err(e) => {
        return Err(e.into());
    }
}
```

### 4. Add to Status Command

The `status` command should also verify:
```rust
let pid = read_pid_file()?;

if verify_kodegend_running(pid)? {
    println!("kodegend is running (PID: {})", pid);
} else {
    println!("kodegend is NOT running (stale PID file)");
}
```

## Files to Modify

- `src/main.rs` - `stop` and `status` command handlers
- `src/daemon.rs` - Any place that acts on PID from file
- Potentially `src/control/windows_control.rs` if it exists

## Testing

1. **Normal case**: Start kodegend, verify stop works
2. **Stale PID**: Kill kodegend with `taskkill /F`, verify stop detects stale PID
3. **PID reuse**:
   - Start kodegend, note PID
   - Kill kodegend
   - Start another process (loop until it gets same PID)
   - Verify `kodegend stop` refuses to kill wrong process
4. **Permission denied**: Try to verify system process PID

## Acceptance Criteria

- [ ] `verify_kodegend_running()` is called before process termination
- [ ] Stale PID files are detected and handled
- [ ] PID reuse attacks are prevented
- [ ] No dead code warnings for verification functions
- [ ] Security-critical paths are logged for audit
