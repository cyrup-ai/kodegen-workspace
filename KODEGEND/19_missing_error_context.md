# Code Quality: Missing Error Context Throughout Service Module

## Severity
**MEDIUM** - Debugging and operational difficulty

## Location
Multiple locations across all three files:
- `packages/kodegend/src/service/autoconfig.rs`
- `packages/kodegend/src/service/embedded_servers.rs`
- `packages/kodegend/src/service/port_cleanup.rs`

## Issue Description
Error messages throughout the service module lack critical context needed for debugging production issues.

## Examples of Missing Context

### Example 1: autoconfig.rs Lines 63-67
```rust
if let Err(e) = result {
    error!("Auto-config watcher failed: {e}");
    let _ = bus.send(Evt::Fatal {
        service: service_name.clone(),
        kind: "running".into(),
        msg: "Watcher error occurred".into(),  // ❌ Generic message!
        ts: chrono::Utc::now(),
    });
}
```

**Problem**: The actual error `e` is logged but NOT included in the Fatal event. The event just says "Watcher error occurred" - a developer looking at events can't see what actually failed.

**Better**:
```rust
let _ = bus.send(Evt::Fatal {
    service: service_name.clone(),
    kind: "running".into(),
    msg: format!("Auto-config watcher failed: {}", e),  // ✓ Include actual error
    ts: chrono::Utc::now(),
});
```

### Example 2: embedded_servers.rs Lines 88-93
```rust
Err(e) => {
    log::error!("✗ Failed to start {} server: {}", config.name, e);
    
    rollback_servers(servers).await;
    
    return Err(e).context(format!("Failed to start {} server", config.name));
}
```

**Problem**: Error lacks:
- Port number (which port failed?)
- Whether cleanup succeeded or failed
- How many servers were rolled back

**Better**:
```rust
Err(e) => {
    log::error!(
        "✗ Failed to start {} server on port {}: {}",
        config.name, config.port, e
    );
    
    let rollback_count = servers.len();
    log::warn!("Rolling back {} previously started servers", rollback_count);
    rollback_servers(servers).await;
    
    return Err(e).context(format!(
        "Failed to start {} server on port {} (rolled back {} servers)",
        config.name, config.port, rollback_count
    ));
}
```

### Example 3: port_cleanup.rs Lines 213-227
```rust
let pid = match find_process_by_port(port).await? {
    Some(pid) => pid,
    None => {
        if check_port_available(port).await {
            log::info!("Port {} became available during lookup", port);
            return Ok(());
        }
        return Err(anyhow::anyhow!(
            "Port {} in use but no process found (system command may have failed)",
            port
        ));
    }
};
```

**Problem**: Error message says "system command may have failed" but doesn't say:
- Which command failed (lsof? netstat?)
- What the error was
- What platform we're on
- How to debug further

**Better**:
```rust
return Err(anyhow::anyhow!(
    "Port {} is in use but cannot find the process. \
     This could indicate:\n\
     - {} command failed or is not available\n\
     - Permission denied to query process information\n\
     - Process exited between checks\n\
     Please check manually with: {} -ti :{}",
    port,
    if cfg!(unix) { "lsof" } else { "netstat" },
    if cfg!(unix) { "lsof" } else { "netstat -ano" },
    port
));
```

### Example 4: No Timing Information
Throughout all files, errors don't include timing information:
- How long did the operation take before failing?
- Did it timeout?
- Is this a transient or persistent failure?

**Better**:
```rust
let start = std::time::Instant::now();
// ... operation ...
if let Err(e) = result {
    log::error!(
        "Operation failed after {:?}: {}",
        start.elapsed(),
        e
    );
}
```

### Example 5: No State Information
Errors don't include current state:
- How many servers were running?
- What was the configuration?
- What mode are we in (startup, shutdown, reload)?

## Production Impact

### Debugging Scenario
Production log shows:
```
ERROR Failed to start filesystem server: Address already in use
ERROR Failed to start terminal server: Address already in use
ERROR Failed to start process server: Address already in use
```

**Questions that can't be answered**:
1. Which ports were these?
2. Did cleanup run? Did it succeed?
3. Were previous servers rolled back?
4. How many servers started successfully before failure?
5. What processes are occupying the ports?

With better context:
```
ERROR Failed to start filesystem server on port 30438: Address already in use (cleanup failed: permission denied to kill PID 12345)
WARN  Rolling back 0 previously started servers
ERROR Server startup failed: filesystem on port 30438 (cleanup failed, rollback complete)
```

Now we know:
- Specific port (30438)
- Cleanup was attempted but failed
- Why it failed (permission denied)
- Which PID is blocking
- No rollback needed (0 servers)

## Recommended Solutions

### 1. Structured Error Context
Create a context type:
```rust
#[derive(Debug)]
struct ServerStartContext {
    name: String,
    port: u16,
    cleanup_attempted: bool,
    cleanup_result: Option<Result<(), String>>,
    servers_started: usize,
}

impl fmt::Display for ServerStartContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Server: {} | Port: {} | Cleanup: {} | Started: {}",
            self.name,
            self.port,
            self.cleanup_status(),
            self.servers_started
        )
    }
}
```

### 2. Timing Wrapper
```rust
async fn with_timing<F, T>(name: &str, f: F) -> Result<T>
where
    F: Future<Output = Result<T>>,
{
    let start = Instant::now();
    match f.await {
        Ok(v) => {
            log::debug!("{} completed in {:?}", name, start.elapsed());
            Ok(v)
        }
        Err(e) => {
            log::error!("{} failed after {:?}: {}", name, start.elapsed(), e);
            Err(e)
        }
    }
}
```

### 3. Error Context Macro
```rust
macro_rules! context_error {
    ($result:expr, $($arg:tt)*) => {
        $result.with_context(|| {
            format!(
                "{} [{}:{}]",
                format_args!($($arg)*),
                file!(),
                line!()
            )
        })
    };
}
```

### 4. Enhanced Logging
```rust
// Include more context in every log
log::error!(
    target: "kodegend::service",
    "Failed to start server | name={} port={} cleanup_ok={} error={}",
    config.name,
    config.port,
    cleanup_succeeded,
    e
);
```

## Recommended Approach
1. Add port numbers to all server-related errors
2. Include cleanup status in startup errors
3. Add timing information for operations >100ms
4. Include specific next steps in error messages
5. Use structured logging where available

## Files to Modify
- `packages/kodegend/src/service/autoconfig.rs`
- `packages/kodegend/src/service/embedded_servers.rs`
- `packages/kodegend/src/service/port_cleanup.rs`

## Testing Considerations
- Trigger various error conditions
- Verify error messages contain sufficient context
- Check that logs are correlatable
- Ensure errors suggest actionable next steps
