# macOS: Brittle String Parsing in check_status()

## Location
`packages/kodegend/src/control/macos_control.rs:27-42`

## Issue Type
Logical Error / Fragile Implementation

## Severity
High

## Description
The `check_status()` function parses the output of `launchctl list` using whitespace splitting and string searching. This parsing is extremely brittle and error-prone.

## Current Code
```rust
// Parse output to check if PID exists
let stdout = String::from_utf8_lossy(&output.stdout);

// Output format: "PID\tStatus\tLabel"
// If PID is "-", service is loaded but not running
// If PID is a number, service is running
for line in stdout.lines() {
    if line.contains(SERVICE_LABEL) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if let Some(pid) = parts.first() {
            return Ok(*pid != "-");
        }
    }
}
```

## Problems

### 1. Output Format Not Guaranteed
The comment claims format is "PID\tStatus\tLabel" but:
- launchctl's output format is not documented as stable across macOS versions
- The format varies between `launchctl list` vs `launchctl list <service>`
- Tabs vs spaces may differ between macOS versions

### 2. Incorrect Parsing Logic
- Uses `split_whitespace()` which splits on ANY whitespace, not just tabs
- Assumes first token is PID, but with `list <service>` the output can include header lines
- No validation that the line actually corresponds to the queried service

### 3. False Positives
- `line.contains(SERVICE_LABEL)` could match unrelated services
  - e.g., "ai.kodegen.kodegend-test" would match "ai.kodegen.kodegend"
- If a service's status message contains the label text, it would match incorrectly

### 4. No Error Handling
- If parsing fails (no SERVICE_LABEL found), returns `Ok(false)`
- Can't distinguish "service not loaded" from "parsing failed"

## Impact
- May incorrectly report service status on different macOS versions
- Could break when Apple changes launchctl output format
- False positives from substring matching
- Hard to debug when parsing fails silently

## Recommendation

### Option 1: Use structured output (if available)
Check if `launchctl` has JSON or plist output mode in recent macOS versions.

### Option 2: Use launchctl print
```rust
// launchctl print system/<service> gives more structured output
let output = Command::new("launchctl")
    .args(["print", &format!("system/{}", SERVICE_LABEL)])
    .output()?;

// Parse the structured output looking for "state = running"
```

### Option 3: More robust parsing
```rust
let stdout = String::from_utf8_lossy(&output.stdout);

// Look for exact label match at start or end of line
for line in stdout.lines() {
    // Skip header lines
    if line.starts_with("PID") {
        continue;
    }
    
    // Match exact label, not substring
    if line.ends_with(SERVICE_LABEL) {
        // Use split('\t') for tab-separated fields
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() >= 3 {
            let pid = parts[0].trim();
            return Ok(pid != "-" && !pid.is_empty());
        }
    }
}

// If not found, service is not loaded
Ok(false)
```

### Option 4: Verify with PID check
```rust
// After getting PID, verify the process actually exists
if let Ok(pid_num) = pid.parse::<i32>() {
    // Use kill(pid, 0) to check if process exists
    nix::sys::signal::kill(nix::unistd::Pid::from_raw(pid_num), None).is_ok()
}
```
