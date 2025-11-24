# No Command Visibility for Debugging

## Location
`packages/kodegend/src/control.rs` (entire module)

## Severity
🟡 **LOW - DEBUGGABILITY ISSUE**

## Issue Description
When control operations fail, users have **no visibility** into what commands were executed:

```rust
pub fn start_daemon() -> Result<()> {
    platform::start_daemon()  // ⚠️ What command actually ran?
}
```

Platform implementations will execute system commands like:
- `systemctl --user start kodegend`
- `launchctl load ~/Library/LaunchAgents/ai.kodegen.kodegend.plist`
- `sc start kodegend`

But **users never see these commands** in:
- Error messages
- Logs
- Debug output

## Real-World Debugging Scenario

### User Experience (Current)
```bash
$ kodegend start
Failed to start: Permission denied (os error 13)
```

User thinks:
- "What does permission denied mean?"
- "Permission for what file?"
- "What command is failing?"
- "How do I debug this?"

They have to:
1. Read source code to find what command is used
2. Guess the arguments
3. Manually try the command
4. Compare behavior

### User Experience (Improved)
```bash
$ kodegend start
Executing: systemctl --user start kodegend
Failed to start: Permission denied (os error 13)

Debug info:
  Command: systemctl --user start kodegend
  Working directory: /home/user
  PATH: /usr/local/bin:/usr/bin:/bin
  
Troubleshooting:
  - Check if service is installed: systemctl --user list-unit-files | grep kodegend
  - Check service file permissions: ls -la ~/.config/systemd/user/kodegend.service
  - Try with verbose output: systemctl --user start kodegend -v
```

User can immediately:
1. See exact command
2. Try it manually
3. Understand what's failing
4. Get actionable next steps

## Implementation Options

### Option 1: Logging (Recommended)
Use Rust `log` crate to log commands before execution:

```rust
// In linux_control.rs
use log::info;

pub async fn start_daemon() -> Result<()> {
    let args = &["--user", "start", "kodegend"];
    
    info!("Executing: systemctl {}", args.join(" "));
    
    let output = Command::new("systemctl")
        .args(args)
        .output()
        .await
        .context("Failed to execute systemctl command")?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!("systemctl failed with exit code {:?}", output.status.code());
        error!("stderr: {}", stderr);
        bail!("systemctl start failed: {}", stderr);
    }
    
    info!("systemctl start completed successfully");
    Ok(())
}
```

**Benefits:**
- Standard approach for daemons
- Configurable via RUST_LOG environment variable
- Easy to filter (RUST_LOG=kodegend::control=debug)
- Production-safe (can disable in release)

**Usage:**
```bash
RUST_LOG=kodegend::control=debug kodegend start
# Output:
# [DEBUG kodegend::control::linux] Executing: systemctl --user start kodegend
# [INFO  kodegend::control::linux] systemctl start completed successfully
# kodegend started successfully
```

### Option 2: Verbose Flag
Add --verbose CLI flag that enables command echoing:

```rust
// In cli.rs
#[derive(Parser)]
pub struct Args {
    #[arg(short, long)]
    pub verbose: bool,
    
    #[command(subcommand)]
    pub sub: Option<Cmd>,
}

// In main.rs
if args.verbose {
    std::env::set_var("RUST_LOG", "debug");
}
```

**Benefits:**
- User control over verbosity
- Familiar UX (many CLIs use --verbose)
- Can combine with logging

**Usage:**
```bash
kodegend --verbose start
# Shows all debug info
```

### Option 3: Include in Error Messages
Add command info directly to error messages:

```rust
pub async fn start_daemon() -> Result<()> {
    let cmd = "systemctl";
    let args = &["--user", "start", "kodegend"];
    
    let output = Command::new(cmd)
        .args(args)
        .output()
        .await
        .with_context(|| format!("Failed to execute: {} {}", cmd, args.join(" ")))?;
    
    if !output.status.success() {
        bail!(
            "Command failed: {} {}\nExit code: {:?}\nStderr: {}",
            cmd,
            args.join(" "),
            output.status.code(),
            String::from_utf8_lossy(&output.stderr)
        );
    }
    
    Ok(())
}
```

**Benefits:**
- Always visible on errors
- No configuration needed
- Immediate debugging info

**Drawbacks:**
- Error messages can be verbose
- No visibility on success cases

## Recommendation
Use **combination of Options 1 and 3**:
- Log commands at DEBUG level (always)
- Include commands in error messages (on failure)

This gives:
- Optional verbose mode for troubleshooting
- Automatic error context when things fail
- Production-friendly (errors have context, but success is quiet)

## Example Implementation

```rust
// In linux_control.rs
use log::{debug, info, error};
use anyhow::{Context, Result, bail};

pub async fn start_daemon() -> Result<()> {
    let cmd = "systemctl";
    let args = vec!["--user", "start", "kodegend"];
    let args_str = args.join(" ");
    
    debug!("Executing: {} {}", cmd, args_str);
    
    let output = Command::new(cmd)
        .args(&args)
        .output()
        .await
        .with_context(|| format!("Failed to execute: {} {}", cmd, args_str))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        error!("Command failed: {} {}", cmd, args_str);
        error!("Exit code: {:?}", output.status.code());
        error!("Stderr: {}", stderr);
        
        bail!(
            "Failed to start daemon via systemd\n\
             \n\
             Command: {} {}\n\
             Exit code: {:?}\n\
             Error: {}\n\
             \n\
             Troubleshooting:\n\
             - Verify service is installed: systemctl --user list-unit-files | grep kodegend\n\
             - Check service status: systemctl --user status kodegend\n\
             - View service logs: journalctl --user -u kodegend -n 50\n\
             - Reinstall service: kodegend install",
            cmd, args_str, output.status.code(), stderr
        );
    }
    
    info!("Daemon started successfully via {}", cmd);
    Ok(())
}
```

## Testing
1. Test successful command (verify logging works)
2. Test failed command (verify error includes command)
3. Test with RUST_LOG=debug (verify debug output)
4. Test with invalid systemctl path (verify "command not found" is clear)

## User Experience Impact
**Before:**
- Cryptic errors
- Manual source code reading required
- Trial and error debugging

**After:**
- Clear command visibility
- Self-service debugging
- Faster issue resolution

## Priority
**LOW** - Nice to have for debugging, but not critical for functionality. Becomes more valuable as user base grows and support requests increase.

## Related Issues
- Task 03: Command visibility complements error context
- Task 01: Should be implemented in platform modules from the start
