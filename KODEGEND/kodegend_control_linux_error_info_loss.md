# Linux: Error Information Loss in check_status()

## Location
`packages/kodegend/src/control/linux_control.rs:28`

## Issue Type
Hidden Errors / Diagnostic Information Loss

## Severity
Medium

## Description
The `check_status()` function only returns `Ok(true)` or `Ok(false)` based on whether `systemctl is-active` exits with code 0. However, systemd uses multiple exit codes to indicate different states:

- Exit 0: Service is active
- Exit 1: Service is inactive (dead)
- Exit 3: Service is inactive (stopped)
- Exit 4: Service state is unknown
- Other codes: Various failure conditions

The current implementation treats ALL non-zero exits as "not running", which loses critical diagnostic information.

## Current Code
```rust
pub fn check_status() -> Result<bool> {
    // ...
    let output = Command::new("systemctl")
        .args(&args)
        .output()
        .context("Failed to execute systemctl is-active")?;

    // systemctl is-active returns:
    // - Exit 0 if active
    // - Exit 3 if inactive
    // - Other codes for other states
    Ok(output.status.success())  // ← Loses all state information
}
```

## Impact
- Cannot distinguish between:
  - Cleanly stopped service (exit 3)
  - Failed/crashed service (exit 1)
  - Unknown state (exit 4)
  - systemctl command error
- Users have no way to diagnose WHY their daemon isn't running
- Makes debugging production issues significantly harder
- Could mask serious problems (daemon crashed vs intentionally stopped)

## Recommendation

### Option 1: Return richer status information
```rust
pub enum DaemonStatus {
    Running,
    Stopped,
    Failed,
    Unknown,
}

pub fn check_status() -> Result<DaemonStatus> {
    let output = Command::new("systemctl")
        .args(&args)
        .output()
        .context("Failed to execute systemctl is-active")?;
    
    match output.status.code() {
        Some(0) => Ok(DaemonStatus::Running),
        Some(3) => Ok(DaemonStatus::Stopped),
        Some(1) => Ok(DaemonStatus::Failed),
        Some(4) => Ok(DaemonStatus::Unknown),
        _ => anyhow::bail!("Unexpected systemctl exit code: {:?}", output.status.code()),
    }
}
```

### Option 2: Parse stdout for detailed state
```rust
// systemctl is-active outputs the actual state as text
let state = String::from_utf8_lossy(&output.stdout).trim().to_string();
match state.as_str() {
    "active" | "activating" => Ok(true),
    "inactive" | "deactivating" | "failed" => Ok(false),
    _ => anyhow::bail!("Unknown service state: {}", state),
}
```

## Related Issues
- Similar loss of diagnostic info in macOS implementation
- Windows implementation also has generic error handling
