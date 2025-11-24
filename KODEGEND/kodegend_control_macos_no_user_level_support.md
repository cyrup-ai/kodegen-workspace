# macOS: No User-Level Daemon Support

## Location
`packages/kodegend/src/control/macos_control.rs:8`

## Issue Type
Feature Gap / Architectural Limitation

## Severity
Medium

## Description
The macOS implementation hardcodes `PLIST_PATH = "/Library/LaunchDaemons/kodegend.plist"`, which is system-wide and requires root access. Unlike the Linux implementation which supports both system-wide (`systemctl`) and user-level (`systemctl --user`) daemons, macOS has no mechanism for user-level daemon operation.

## Current Code
```rust
const SERVICE_LABEL: &str = "ai.kodegen.kodegend";
const PLIST_PATH: &str = "/Library/LaunchDaemons/kodegend.plist";  // ← System-wide only
```

## Problem
macOS launchd supports two daemon scopes:
1. **System-wide**: `/Library/LaunchDaemons/` (requires root)
2. **User-level**: `~/Library/LaunchAgents/` (runs as user, no root needed)

The current implementation only supports system-wide daemons, which means:
- Users must have root/sudo access to manage kodegend
- Cannot run kodegend as a regular user daemon
- Inconsistent with Linux implementation (which has `--user` support)

## Impact
- **User Experience**: Non-root users cannot manage the daemon
- **Security**: Forces users to run as root when user-level would be sufficient
- **Inconsistency**: Linux supports user daemons, macOS doesn't
- **Deployment**: Can't deploy to environments where users don't have root

## Use Cases Affected
1. **Developer machines**: Developers may not want to run daemons as root
2. **Corporate environments**: IT policies may restrict LaunchDaemons
3. **Multi-user systems**: Each user might want their own kodegend instance
4. **CI/CD**: Build agents often run as non-root users

## Recommendation

### Add support for user-level daemons:
```rust
const SERVICE_LABEL: &str = "ai.kodegen.kodegend";

// Determine plist path based on whether running as root
fn get_plist_path() -> PathBuf {
    if nix::unistd::getuid().is_root() {
        PathBuf::from("/Library/LaunchDaemons/kodegend.plist")
    } else {
        // User-level daemon
        let home = std::env::var("HOME")
            .expect("HOME environment variable not set");
        PathBuf::from(home)
            .join("Library")
            .join("LaunchAgents")
            .join("kodegend.plist")
    }
}

fn get_service_domain() -> &'static str {
    if nix::unistd::getuid().is_root() {
        "system"
    } else {
        "gui/501"  // User domain (UID varies, need to get actual UID)
    }
}
```

### Update all functions to use dynamic paths:
```rust
pub fn start_daemon() -> Result<()> {
    let plist_path = get_plist_path();
    let domain = get_service_domain();
    
    let _ = Command::new("launchctl")
        .args(["bootstrap", domain, plist_path.to_str().unwrap()])
        .output();
    
    // ...
}
```

### For user domain, use proper UID:
```rust
fn get_service_domain() -> String {
    if nix::unistd::getuid().is_root() {
        "system".to_string()
    } else {
        // User domain is gui/<uid> where uid is the user's UID
        let uid = nix::unistd::getuid();
        format!("gui/{}", uid)
    }
}
```

## Related Issues
- Linux implementation already has user-level support via `--user` flag
- Should unify the approach across platforms
