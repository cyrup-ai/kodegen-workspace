# Edge Case: Silent Errors in wait_for_port_release

## Severity
**LOW** - Edge case but confusing when it occurs

## Location
`packages/kodegend/src/service/port_cleanup.rs`
- Lines 174-192: `wait_for_port_release()`

## Issue Description
The function polls port availability in a loop, but silently ignores errors from `check_port_available`:

```rust
pub async fn wait_for_port_release(port: u16, timeout: Duration) -> Result<()> {
    let start = tokio::time::Instant::now();
    
    while start.elapsed() < timeout {
        if check_port_available(port).await {  // Returns bool, not Result
            return Ok(());
        }
        sleep(Duration::from_millis(100)).await;
    }

    Err(anyhow::anyhow!(
        "Timeout waiting for port {} to be released after {:?}",
        port,
        timeout
    ))
}

// check_port_available implementation:
pub async fn check_port_available(port: u16) -> bool {
    tokio::net::TcpListener::bind(("127.0.0.1", port))
        .await
        .is_ok()  // Converts Result to bool - loses error information
}
```

## The Problem

`check_port_available` returns `bool`, which means:
- `true` = port is free
- `false` = port is NOT free **OR** bind failed for other reasons

### Bind Can Fail For Reasons Other Than "Port In Use"

Examples:
1. **Permission denied**: User lacks CAP_NET_BIND_SERVICE (Linux ports < 1024)
2. **Network unreachable**: localhost misconfigured
3. **Address family not supported**: IPv4/IPv6 issues
4. **Out of file descriptors**: System resource exhaustion
5. **Firewall blocking**: Security software interference

## Production Impact

### Scenario: Permission Denied
1. User tries to bind privileged port (e.g., 80 or 443)
2. `check_port_available(80)` returns `false` (permission denied)
3. `wait_for_port_release` keeps retrying every 100ms
4. All retries return `false` (still permission denied)
5. After 3 seconds, timeout error: "Timeout waiting for port 80 to be released"
6. **User thinks port is occupied when it's actually a permission issue**

### Scenario: System Resource Exhaustion
1. System runs out of file descriptors
2. `TcpListener::bind()` fails with EMFILE
3. Function returns `false`, keeps retrying
4. Timeout after 3s with misleading error
5. **User doesn't know system is out of resources**

### Debugging Difficulty
The timeout error message says:
```
"Timeout waiting for port 30438 to be released after 3s"
```

But the REAL error might be:
- Permission denied
- Network unreachable
- Out of file descriptors

This makes debugging very difficult.

## Root Cause
Using `bool` instead of `Result<bool>` loses critical error information.

## Recommended Solution

### Option 1: Return Result<bool>
```rust
pub async fn check_port_available(port: u16) -> Result<bool> {
    match tokio::net::TcpListener::bind(("127.0.0.1", port)).await {
        Ok(_) => Ok(true),  // Port is free
        Err(e) if e.kind() == std::io::ErrorKind::AddrInUse => {
            Ok(false)  // Port occupied (expected case)
        }
        Err(e) => {
            // Other errors (permission, network, resources, etc.)
            Err(anyhow::anyhow!("Failed to check port {}: {}", port, e))
        }
    }
}

pub async fn wait_for_port_release(port: u16, timeout: Duration) -> Result<()> {
    let start = tokio::time::Instant::now();
    
    while start.elapsed() < timeout {
        match check_port_available(port).await {
            Ok(true) => return Ok(()),  // Port freed
            Ok(false) => {
                // Port still occupied, keep waiting
                sleep(Duration::from_millis(100)).await;
            }
            Err(e) => {
                // Error checking port - propagate immediately
                return Err(e).context(format!(
                    "Error while waiting for port {} to be released",
                    port
                ));
            }
        }
    }

    Err(anyhow::anyhow!(
        "Timeout waiting for port {} to be released after {:?}",
        port,
        timeout
    ))
}
```

### Option 2: Distinguish Errors at Call Site
Keep `check_port_available` as bool but add detailed error checking:

```rust
pub async fn wait_for_port_release(port: u16, timeout: Duration) -> Result<()> {
    let start = tokio::time::Instant::now();
    let mut last_error = None;
    
    while start.elapsed() < timeout {
        // Try to bind to get detailed error
        match tokio::net::TcpListener::bind(("127.0.0.1", port)).await {
            Ok(_) => return Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::AddrInUse => {
                // Port occupied, expected - keep waiting
                sleep(Duration::from_millis(100)).await;
            }
            Err(e) => {
                // Unexpected error - store it but keep trying
                // (might be transient)
                last_error = Some(e);
                sleep(Duration::from_millis(100)).await;
            }
        }
    }

    // Timeout - include last error if we saw one
    if let Some(e) = last_error {
        Err(anyhow::anyhow!(
            "Timeout waiting for port {} (last error: {})",
            port, e
        ))
    } else {
        Err(anyhow::anyhow!(
            "Timeout waiting for port {} to be released",
            port
        ))
    }
}
```

### Option 3: Early Exit on Persistent Errors
If we see the same non-AddrInUse error 3 times in a row, fail fast:

```rust
pub async fn wait_for_port_release(port: u16, timeout: Duration) -> Result<()> {
    let start = tokio::time::Instant::now();
    let mut consecutive_errors = 0;
    let mut last_error_kind = None;
    
    while start.elapsed() < timeout {
        match tokio::net::TcpListener::bind(("127.0.0.1", port)).await {
            Ok(_) => return Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::AddrInUse => {
                // Reset error counter - port is just occupied
                consecutive_errors = 0;
                last_error_kind = None;
                sleep(Duration::from_millis(100)).await;
            }
            Err(e) => {
                // Non-AddrInUse error
                let kind = e.kind();
                if Some(kind) == last_error_kind {
                    consecutive_errors += 1;
                } else {
                    consecutive_errors = 1;
                    last_error_kind = Some(kind);
                }
                
                // If same error 3 times in a row, fail fast
                if consecutive_errors >= 3 {
                    return Err(anyhow::anyhow!(
                        "Persistent error checking port {}: {}",
                        port, e
                    ));
                }
                
                sleep(Duration::from_millis(100)).await;
            }
        }
    }

    Err(anyhow::anyhow!(
        "Timeout waiting for port {} to be released",
        port
    ))
}
```

## Recommended Approach
**Option 1** is cleanest - properly distinguish between:
- Port available (Ok(true))
- Port occupied (Ok(false))
- Error checking port (Err)

## Files to Modify
- `packages/kodegend/src/service/port_cleanup.rs`

## Testing Considerations
- Test with permission denied (try port 80 as non-root)
- Test with invalid port (0 or > 65535)
- Test with network unavailable
- Verify error messages are clear and actionable
- Test normal case (port occupied then freed) still works
