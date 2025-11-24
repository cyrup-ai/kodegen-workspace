# SEVERE: Contradictory Documentation and Error Handling

## Location
`packages/kodegend/src/logging/windows.rs:47-50`

## Issue Type
**SEVERE** - Code behavior contradicts documentation

## Description
The comment claims graceful handling of eventlog initialization failure, but the code actually propagates errors and fails hard.

## Code Analysis

```rust
// Lines 46-50
// Initialize eventlog for Windows Event Log
// Note: eventlog::init() will gracefully handle missing registration
// If not registered, events may appear under generic "Application" source
eventlog::init("kodegend", log::Level::Info)
    .context("Failed to initialize Windows Event Log")?;  // ❌ Propagates error!
```

## The Contradiction

### What the comment promises:
- "**will gracefully handle** missing registration"
- "events may appear under generic 'Application' source"
- Implies: works anyway, just with degraded functionality

### What the code actually does:
- `.context("Failed to initialize Windows Event Log")?`
- The `?` operator propagates the error up
- Returns `Err` from the entire `platform_init_logging()` function
- Causes service startup to **fail completely**

## Impact

### Actual Behavior
If eventlog::init() fails (e.g., missing registry entry, permissions issue):
1. Error propagates from `platform_init_logging()`
2. Error propagates from `init_logging()` in mod.rs
3. Error reaches `main()`
4. **Service fails to start at all**

### Expected Behavior (based on comment)
If eventlog::init() fails:
1. Log a warning
2. Continue with just env_logger
3. Service starts successfully with console-only logging

## Real-World Scenario

### Problem Case
1. Administrator forgets to run `kodegend install` (which registers Event Log source)
2. Service is started manually or via other means
3. eventlog::init() fails because registry entry missing
4. **Entire service fails to start** - not just Event Log
5. Critical daemon is down because of optional feature failure

### Expected Case
1. Administrator forgets to run `kodegend install`
2. Service starts anyway
3. Warning logged: "Windows Event Log unavailable, using console logging"
4. Service runs normally, logs go to stdout/stderr
5. Systemd/service manager can still capture logs

## Root Cause Analysis

Two possibilities:
1. **Comment is wrong** - Should say "will fail if not registered"
2. **Code is wrong** - Should handle error gracefully as comment suggests

Given that this is a daemon/service, **graceful degradation is the correct behavior**. The comment is right, the code is wrong.

## Correct Implementation

```rust
// Initialize eventlog for Windows Event Log
// Note: eventlog::init() will gracefully handle missing registration
// If not registered, events may appear under generic "Application" source
match eventlog::init("kodegend", log::Level::Info) {
    Ok(_) => log::debug!("Windows Event Log initialized"),
    Err(e) => {
        // Don't fail service startup just because Event Log unavailable
        eprintln!("Warning: Windows Event Log unavailable: {}", e);
        eprintln!("Continuing with console-only logging");
        // Continue without Event Log - not a fatal error
    }
}
```

Or even simpler:
```rust
// Try to initialize Event Log, but don't fail if unavailable
let _ = eventlog::init("kodegend", log::Level::Info)
    .map_err(|e| eprintln!("Warning: Event Log unavailable: {}", e));
```

## Service Reliability Impact

This is a **reliability bug**. A daemon should be robust and handle optional feature failures gracefully. Windows Event Log is a "nice to have" for monitoring, but not essential for the service to function.

Current code makes Event Log a hard requirement, which is wrong for a production service.

## Priority
**P1 - Severe** - Affects service reliability and contradicts documented behavior

## Related Issues
- See task 05 for broader fault tolerance discussion
- Blocks proper testing of Event Log integration

## Testing After Fix
1. On Windows, ensure Event Log source is NOT registered
2. Start kodegend service
3. Verify service **starts successfully** despite Event Log unavailable
4. Verify warning is logged about Event Log failure
5. Verify console logging works
6. Then register Event Log source and restart
7. Verify Event Log now works in addition to console
