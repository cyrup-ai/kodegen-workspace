# MODERATE: No Fault Tolerance for Event Log Initialization Failure

## Location
`packages/kodegend/src/logging/windows.rs:49-50` and `windows.rs:54-57`

## Issue Type
**MODERATE** - Service reliability and fault tolerance

## Description
Both `eventlog::init()` and `MultiLogger::init()` failures cause complete service startup failure. For a critical daemon/service, this is poor fault tolerance design.

## Code Analysis

```rust
// Line 49-50: Hard failure on eventlog error
eventlog::init("kodegend", log::Level::Info)
    .context("Failed to initialize Windows Event Log")?;

// Line 54-57: Hard failure on MultiLogger error  
multi_log::MultiLogger::init(
    vec![Box::new(env_logger)],
    log::Level::Info
).context("Failed to initialize multi-logger")?;
```

Both use the `?` operator, which propagates errors and prevents service startup.

## Service Criticality Analysis

### What is kodegend?
Based on the codebase context:
- It's a Unix daemon / Windows service
- It's meant to run continuously in the background
- It manages MCP (Model Context Protocol) tools
- It's infrastructure-level software

### Logging Criticality
For such a service:
- **Console/file logging**: Essential - service blind without it
- **Windows Event Log**: Important but not essential - nice for monitoring but service can function without it

## Current Failure Modes

### Scenario 1: Event Log Registration Missing
```
User runs: net start kodegend
Result: SERVICE FAILS TO START
Reason: eventlog::init() returns Err
Impact: Critical service down due to optional monitoring feature
```

### Scenario 2: MultiLogger Initialization Failure
```
User runs: net start kodegend  
Result: SERVICE FAILS TO START
Reason: MultiLogger::init() returns Err (rare but possible)
Impact: Critical service down due to logging framework issue
```

### Scenario 3: Permission Issue
```
User runs service as non-admin user
Result: SERVICE MIGHT FAIL TO START
Reason: Event Log access might be restricted
Impact: Service unavailable when running with least privilege
```

## Fault Tolerance Best Practices

For a production daemon/service, the fault tolerance hierarchy should be:

### Level 1: Must Succeed (service fails if these fail)
- Core business logic initialization
- Critical resource allocation (if any)
- Basic console logging (stdout/stderr)

### Level 2: Should Succeed (warn but continue if these fail)
- Windows Event Log integration
- Optional monitoring features
- Enhanced logging destinations

### Level 3: Nice to Have (silent fallback if these fail)
- Performance optimizations
- Optional features

## Current Problems

1. **Event Log treated as Level 1** (must succeed) when it should be Level 2
2. **No fallback path** when Event Log unavailable
3. **No degraded mode** - all or nothing
4. **Violates least privilege** - might require elevated permissions

## Recommended Fix

### Strategy: Graceful Degradation

```rust
pub(super) fn platform_init_logging() -> Result<()> {
    // Create env_logger (this is essential, let it fail)
    let env_logger = env_logger::Builder::from_default_env()
        .format(|buf, record| {
            use std::io::Write;
            writeln!(
                buf,
                "[{} {} {}:{}] {}",
                buf.timestamp_millis(),
                record.level(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .filter_level(LevelFilter::Info)
        .build();

    // Try to add Event Log, but don't fail service if unavailable
    let mut loggers: Vec<Box<dyn log::Log>> = vec![Box::new(env_logger)];
    
    match create_eventlog_logger() {
        Ok(event_logger) => {
            loggers.push(Box::new(event_logger));
            eprintln!("Windows Event Log enabled");
        }
        Err(e) => {
            eprintln!("Warning: Windows Event Log unavailable: {}", e);
            eprintln!("Continuing with console-only logging");
            // Don't fail - service can work without Event Log
        }
    }

    // Initialize with whatever loggers we have
    multi_log::MultiLogger::init(loggers, log::Level::Info)
        .context("Failed to initialize logging")?;
    
    Ok(())
}
```

### Fallback Levels

1. **Best case**: env_logger + eventlog via MultiLogger
2. **Degraded case**: env_logger only (Event Log unavailable)
3. **Failure case**: Can't initialize ANY logging (then fail service)

## Benefits of This Approach

1. **Service availability** - Won't fail to start due to Event Log issues
2. **Least privilege** - Can run as non-admin user if Event Log registration missing
3. **Debugging** - Easier to debug Event Log issues when service is actually running
4. **Production ready** - Handles real-world deployment variations

## Priority
**P2 - Moderate** - Service reliability issue, but can be worked around by ensuring proper setup

## Related Issues
- See task 04 for contradictory error handling (same root cause)
- Blocked by tasks 01 & 02 (must fix Event Log integration first)

## Testing After Fix

### Test 1: No Event Log Registration
1. Remove Event Log registry entry
2. Start service → Should succeed with warning
3. Verify console logging works
4. Verify Event Viewer shows no logs (expected)

### Test 2: Proper Registration
1. Register Event Log source
2. Start service → Should succeed
3. Verify both console AND Event Viewer show logs

### Test 3: Non-Admin User
1. Run service as restricted user
2. Service should start even if Event Log unavailable
3. Console logging should work
