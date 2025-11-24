# macOS: Silent Failure in start_daemon()

## Location
`packages/kodegend/src/control/macos_control.rs:49-52`

## Issue Type
Hidden Errors / Silent Failure

## Severity
High

## Description
The `start_daemon()` function intentionally ignores all errors from `launchctl bootstrap` using `let _ = ...`. While the comment explains this is because the command "may fail if already loaded", this blanket error suppression hides legitimate failures.

## Current Code
```rust
pub fn start_daemon() -> Result<()> {
    // Try modern bootstrap first (may fail if already loaded - that's OK)
    let _ = Command::new("launchctl")
        .args(["bootstrap", "system", PLIST_PATH])
        .output();

    // Then kickstart to ensure it starts
    let output = Command::new("launchctl")
        .args(["kickstart", SERVICE_LABEL])
        .output()
        .context("Failed to execute launchctl kickstart")?;
    // ...
}
```

## Problems

### 1. Masks Real Errors
The `bootstrap` command can fail for many reasons:
- **Permission denied** - user doesn't have root access
- **Invalid plist file** - syntax errors, missing keys
- **File not found** - PLIST_PATH doesn't exist
- **Service already loaded** - legitimate case to ignore ✓

By ignoring ALL errors, critical issues like permission problems or invalid plist files are silently hidden until the `kickstart` command fails (which gives less useful error messages).

### 2. Cascading Failures
If `bootstrap` fails for a real reason (not "already loaded"), the subsequent `kickstart` will also fail, but with a misleading error message about kickstart rather than the root cause.

### 3. Difficult Debugging
When users report "start fails", we have no way to know if:
- The service was already loaded (harmless)
- The plist file is invalid (serious)
- Permissions are wrong (serious)
- The file doesn't exist (serious)

## Impact
- Critical errors in service configuration are hidden
- Users get misleading error messages
- Troubleshooting is extremely difficult
- May succeed silently when configuration is broken

## Recommendation

### Parse the error and only ignore specific cases:
```rust
pub fn start_daemon() -> Result<()> {
    // Try modern bootstrap first
    let bootstrap_result = Command::new("launchctl")
        .args(["bootstrap", "system", PLIST_PATH])
        .output();
    
    match bootstrap_result {
        Ok(output) => {
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                
                // Only ignore "already loaded" errors
                if !stderr.contains("service already loaded") 
                   && !stderr.contains("Already loaded") {
                    // This is a real error - don't ignore it
                    log::warn!("launchctl bootstrap failed: {}", stderr);
                    
                    // Check for critical errors
                    if stderr.contains("Permission denied") {
                        anyhow::bail!("Permission denied - need root access to start daemon");
                    } else if stderr.contains("Could not find") {
                        anyhow::bail!("Service plist not found at {}", PLIST_PATH);
                    } else if stderr.contains("Invalid property list") {
                        anyhow::bail!("Invalid plist file at {}", PLIST_PATH);
                    }
                    // For other errors, continue but warn
                }
            }
        }
        Err(e) => {
            // Failed to execute launchctl at all
            anyhow::bail!("Failed to execute launchctl: {}", e);
        }
    }

    // Then kickstart to ensure it starts
    // ...
}
```

### Alternative: Check if service is already loaded first
```rust
pub fn start_daemon() -> Result<()> {
    // Check if already loaded
    let list_output = Command::new("launchctl")
        .args(["list", SERVICE_LABEL])
        .output()?;
    
    if !list_output.status.success() {
        // Not loaded, so bootstrap it
        let bootstrap_output = Command::new("launchctl")
            .args(["bootstrap", "system", PLIST_PATH])
            .output()
            .context("Failed to bootstrap service")?;
        
        if !bootstrap_output.status.success() {
            anyhow::bail!(
                "Failed to bootstrap service: {}",
                String::from_utf8_lossy(&bootstrap_output.stderr)
            );
        }
    }
    
    // Now kickstart to ensure it's running
    // ...
}
```
