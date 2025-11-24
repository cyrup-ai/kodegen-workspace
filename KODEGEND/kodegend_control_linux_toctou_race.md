# Linux: TOCTOU Race Condition in is_root() Check

## Location
`packages/kodegend/src/control/linux_control.rs`

## Issue Type
Race Condition

## Severity
Low (but worth fixing)

## Description
Each function (`check_status`, `start_daemon`, `stop_daemon`, `restart_daemon`) calls `is_root()` separately to determine whether to use `--user` flag for systemctl. There's a theoretical TOCTOU (Time-Of-Check-Time-Of-Use) vulnerability where the effective UID could change between the `is_root()` check and the actual `Command::new("systemctl")` execution.

While this is unlikely in normal operation, it could occur in environments with:
- Setuid binaries
- Linux capabilities (CAP_SETUID)
- Privilege dropping during execution
- Security modules that change effective UID

## Current Code Pattern
```rust
pub fn check_status() -> Result<bool> {
    let service_name = format!("{}.service", SERVICE_NAME);
    let args = if is_root() {  // ← Check happens here
        vec!["is-active", &service_name]
    } else {
        vec!["--user", "is-active", &service_name]
    };
    
    let output = Command::new("systemctl")  // ← Use happens here
        .args(&args)
        // ...
```

## Impact
- Potential for incorrect systemctl command if UID changes mid-execution
- Could lead to permission errors or wrong systemd scope (user vs system)
- Difficult to debug in production

## Recommendation
Capture `is_root()` result once at the start of each function and use that value consistently:

```rust
pub fn check_status() -> Result<bool> {
    let root = is_root();  // Capture once
    let service_name = format!("{}.service", SERVICE_NAME);
    let args = if root {
        vec!["is-active", &service_name]
    } else {
        vec!["--user", "is-active", &service_name]
    };
    // ... rest of function
}
```

Even better, consider caching at module level if the daemon's privilege level doesn't change during runtime.
