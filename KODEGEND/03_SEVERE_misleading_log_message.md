# SEVERE: Misleading Log Message Claims Event Log Works When It Doesn't

## Location
`packages/kodegend/src/logging/windows.rs:59`

## Issue Type
**SEVERE** - False reporting of functionality status

## Description
After the broken logger initialization, the code logs a success message claiming Windows Event Log is operational when it actually isn't.

## Code Analysis

```rust
// Line 59
log::info!("Logging initialized: console + Windows Event Log");
```

This message is:
1. **Factually incorrect** - Windows Event Log is NOT initialized (see tasks 01 & 02)
2. **Misleading to operators** - They'll believe monitoring is working when it's not
3. **Only goes to console** - Proves Event Log isn't working (this message itself doesn't appear in Event Viewer)

## Impact

### User Impact
- System administrators see this message and assume Event Viewer monitoring is active
- They won't notice that Event Viewer shows no kodegend logs
- Critical issues may go undetected because expected monitoring is silently broken

### Debugging Impact
This message actively hinders debugging because:
- It provides false confirmation of successful setup
- Developers testing the code will see this and assume it works
- Explains why the bug wasn't caught - the code lies about its own state

## Evidence of Bug
The fact that this very log message only appears in console and NOT in Windows Event Viewer is proof that Event Log integration is broken. If it were working, this message would appear in both places.

## Correct Behavior

### If Event Log Works
```rust
log::info!("Logging initialized: console + Windows Event Log");
```
This should appear in:
- Console/stdout ✓
- Windows Event Viewer Application log ✓

### Current Broken Behavior
```rust
log::info!("Logging initialized: console + Windows Event Log");
```
This appears in:
- Console/stdout ✓
- Windows Event Viewer Application log ✗ (missing!)

## Fix Options

### Option 1: Fix Event Log and keep message
Once tasks 01 & 02 are fixed, this message becomes accurate.

### Option 2: Separate success messages
```rust
log::info!("Console logging initialized");
// After confirming Event Log works:
log::info!("Windows Event Log initialized");
```

### Option 3: Test-based message
```rust
// Try to verify Event Log is actually working before claiming success
if event_log_is_working() {
    log::info!("Logging initialized: console + Windows Event Log");
} else {
    log::warn!("Logging initialized: console only (Event Log unavailable)");
}
```

## Priority
**P1 - Severe** - Actively misleads operators and developers

## Related Issues
- Depends on fixing tasks 01 & 02 first
- Part of overall Windows Event Log integration failure

## Testing After Fix
1. Fix the Event Log initialization bugs (tasks 01 & 02)
2. Restart kodegend service
3. Check Windows Event Viewer for this exact message
4. Verify it appears in BOTH console AND Event Viewer
