# Cross-Platform: No Timeout Protection on Command Execution

## Location
All platform implementations:
- `packages/kodegend/src/control/linux_control.rs` (all functions using `Command::new().output()`)
- `packages/kodegend/src/control/macos_control.rs` (all functions using `Command::new().output()`)
- Windows is not affected (uses Windows API, not Command)

## Issue Type
Logical Error / Hang Risk

## Severity
High

## Description
None of the platform implementations (Linux, macOS) have timeout protection on `Command::new().output()` calls. If `systemctl` or `launchctl` hangs, the entire control operation hangs forever, blocking the calling thread indefinitely.

## Affected Code

### Linux - All Functions
```rust
let output = Command::new("systemctl")
    .args(&args)
    .output()
    .context("Failed to execute systemctl")?;
// ↑ No timeout - if systemctl hangs, this hangs forever
```

### macOS - All Functions
```rust
let output = Command::new("launchctl")
    .args(["list", SERVICE_LABEL])
    .output()
    .context("Failed to execute launchctl")?;
// ↑ No timeout - if launchctl hangs, this hangs forever
```

## When Commands Can Hang

### systemctl (Linux)
- **Degraded systemd state**: If systemd is in a degraded state, systemctl can hang
- **D-Bus issues**: systemctl communicates with systemd via D-Bus, which can hang
- **Dependency deadlocks**: Service dependencies can cause systemctl to wait indefinitely
- **Network mounts**: If service depends on network mount that's unreachable
- **Stuck processes**: If service process is in uninterruptible sleep (D state)

Real-world example:
```bash
$ systemctl status network-mount.service
# Hangs if NFS server is unreachable
```

### launchctl (macOS)
- **System updates**: During macOS updates, launchd can be unresponsive
- **Kernel panics**: After kernel panic recovery, launchctl may hang
- **File system issues**: If plist is on failing disk
- **XPC issues**: launchctl uses XPC, which can deadlock

## Real-World Impact

### Scenario 1: CLI Hangs Forever
```bash
$ kodegend status
# User presses Ctrl+C after 30 seconds
# Process doesn't respond - stuck in .output() call
# User has to kill -9 from another terminal
```

### Scenario 2: Automated Scripts Hang
```bash
#!/bin/bash
kodegend restart  # Hangs forever
# Rest of script never runs
# CI/CD pipeline times out
```

### Scenario 3: HTTP API Deadlock
```rust
// HTTP handler
async fn restart_handler() -> Result<Response> {
    restart_daemon()?;  // Blocks async executor!
    Ok(Response::new("Restarted"))
}
// All HTTP requests hang because executor is blocked
```

## Impact
- **Availability**: Entire application hangs if systemctl/launchctl hangs
- **User Experience**: CLI becomes unresponsive
- **CI/CD**: Pipelines hang and timeout
- **Debugging**: Difficult to diagnose hanging processes
- **Resource Leaks**: Hung processes accumulate over time

## Recommendation

### Option 1: Use timeout wrapper with spawn()
```rust
use std::time::Duration;

fn execute_with_timeout(
    mut cmd: Command,
    timeout: Duration,
) -> Result<std::process::Output> {
    use std::sync::mpsc;
    use std::thread;

    let (tx, rx) = mpsc::channel();
    
    let handle = thread::spawn(move || {
        cmd.output()
    });

    match rx.recv_timeout(timeout) {
        Ok(result) => result.context("Command execution failed"),
        Err(_) => {
            // Timeout - try to kill the process
            anyhow::bail!("Command timed out after {:?}", timeout)
        }
    }
}

pub fn check_status() -> Result<bool> {
    let service_name = format!("{}.service", SERVICE_NAME);
    let args = if is_root() {
        vec!["is-active", &service_name]
    } else {
        vec!["--user", "is-active", &service_name]
    };

    let mut cmd = Command::new("systemctl");
    cmd.args(&args);
    
    let output = execute_with_timeout(cmd, Duration::from_secs(10))?;
    
    Ok(output.status.success())
}
```

### Option 2: Use tokio::time::timeout (if async)
```rust
use tokio::process::Command;
use tokio::time::{timeout, Duration};

pub async fn check_status() -> Result<bool> {
    let service_name = format!("{}.service", SERVICE_NAME);
    let args = if is_root() {
        vec!["is-active", &service_name]
    } else {
        vec!["--user", "is-active", &service_name]
    };

    let output = timeout(
        Duration::from_secs(10),
        Command::new("systemctl")
            .args(&args)
            .output()
    )
    .await
    .context("systemctl command timed out after 10s")?
    .context("Failed to execute systemctl")?;
    
    Ok(output.status.success())
}
```

### Option 3: Use wait-timeout crate
```rust
use wait_timeout::ChildExt;
use std::process::Command;
use std::time::Duration;

pub fn check_status() -> Result<bool> {
    let service_name = format!("{}.service", SERVICE_NAME);
    let args = if is_root() {
        vec!["is-active", &service_name]
    } else {
        vec!["--user", "is-active", &service_name]
    };

    let mut child = Command::new("systemctl")
        .args(&args)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .context("Failed to spawn systemctl")?;

    let timeout = Duration::from_secs(10);
    
    match child.wait_timeout(timeout)? {
        Some(status) => {
            let output = child.wait_with_output()?;
            Ok(status.success())
        }
        None => {
            // Timeout - kill the process
            child.kill()?;
            anyhow::bail!("systemctl timed out after {:?}", timeout)
        }
    }
}
```

## Recommended Timeouts
- **check_status()**: 5-10 seconds
- **start_daemon()**: 30-60 seconds
- **stop_daemon()**: 30-60 seconds  
- **restart_daemon()**: 60-120 seconds

## Related Issues
- This is a cross-cutting issue affecting both Linux and macOS
- Windows implementation doesn't have this issue (uses synchronous API calls with inherent timeouts)
