# Blocking I/O in Async Context

## Location
`packages/kodegend/src/control.rs:27-44`

## Severity
🟠 **MEDIUM - PERFORMANCE ISSUE**

## Issue Description
All control module functions are **synchronous** (use `fn`, not `async fn`):

```rust
pub fn check_status() -> Result<bool> {
    platform::check_status()  // ⚠️ Synchronous, blocks thread
}

pub fn start_daemon() -> Result<()> {
    platform::start_daemon()  // ⚠️ Synchronous, blocks thread
}
// ... etc
```

These are called from `main.rs` in an **async context** (tokio runtime):
```rust
async fn real_main() -> Result<()> {
    let args = cli::Args::parse();
    match args.sub {
        // ...
        cli::Cmd::Status => handle_status(),   // ⚠️ Blocks tokio thread!
        cli::Cmd::Start => handle_start(),     // ⚠️ Blocks tokio thread!
        cli::Cmd::Stop => handle_stop(),       // ⚠️ Blocks tokio thread!
        cli::Cmd::Restart => handle_restart(), // ⚠️ Blocks tokio thread!
    }
}
```

## The Problem

### Current Platform Implementations

The platform control modules are **fully implemented** and use blocking I/O:

**Linux** ([`src/control/linux_control.rs`](../../packages/kodegend/src/control/linux_control.rs)):
```rust
pub fn start_daemon() -> Result<()> {
    let service_name = format!("{}.service", SERVICE_NAME);
    let args = if is_root() {
        vec!["start", &service_name]
    } else {
        vec!["--user", "start", &service_name]
    };

    let output = Command::new("systemctl")  // ⚠️ std::process::Command (blocking)
        .args(&args)
        .output()
        .context("Failed to execute systemctl start")?;  // Blocks thread!

    if !output.status.success() {
        anyhow::bail!(
            "Failed to start daemon: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}
```

**macOS** ([`src/control/macos_control.rs`](../../packages/kodegend/src/control/macos_control.rs)):
```rust
pub fn stop_daemon() -> Result<()> {
    // Try to kill the service first (graceful shutdown)
    let _ = Command::new("launchctl")  // ⚠️ std::process::Command (blocking)
        .args(["kill", "SIGTERM", SERVICE_LABEL])
        .output();

    // Give it a moment to shutdown gracefully
    std::thread::sleep(Duration::from_millis(500));  // ⚠️ Also blocks thread!

    // Then bootout
    let output = Command::new("launchctl")  // ⚠️ std::process::Command (blocking)
        .args(["bootout", "system", PLIST_PATH])
        .output()
        .context("Failed to execute launchctl bootout")?;  // Blocks thread!

    // ... error handling
}
```

### Blocking Issues

When run on a tokio thread:
1. **Thread blocking**: One of tokio's worker threads is blocked waiting for system commands
2. **Reduced parallelism**: Other async tasks can't use this thread
3. **Potential deadlock**: If many tasks block, runtime may stall
4. **Poor responsiveness**: UI feels sluggish if CLI is used interactively

## Impact Analysis

### Current Mitigation
In [`src/main.rs`](../../packages/kodegend/src/main.rs), the CLI commands immediately exit after calling control functions:
```rust
fn handle_start() -> Result<()> {
    match control::start_daemon() {
        Ok(()) => {
            println!("kodegend started successfully");
            std::process::exit(0);  // ← Exits immediately
        }
        // ...
    }
}
```

Since the process exits right after, **this is NOT currently a production issue**. The tokio runtime only runs for ~100ms for these commands.

### Future Risk
If control functions are ever used in a **long-running context**, this becomes problematic:
- Web API endpoint to control daemon
- IPC server receiving control commands
- Monitoring loop that checks status periodically
- Integration with other async tasks

## Recommended Solution: Async Functions with `tokio::process::Command`

### Why This Approach?

The codebase **already uses this pattern extensively** in the bundler packages. This is the established standard.

**Existing Pattern Examples:**

[`packages/kodegen-bundler-bundle/src/bundler/platform/linux/appimage.rs:138`](../../packages/kodegen-bundler-bundle/src/bundler/platform/linux/appimage.rs#L138):
```rust
let status = tokio::process::Command::new(&linuxdeploy)
    .env("OUTPUT", &appimage_path)
    .env("ARCH", arch)
    .args(["--appdir", app_dir_str, "--output", "appimage"])
    .status()
    .await
    .map_err(|e| {
        crate::bundler::Error::GenericError(format!("Failed to execute linuxdeploy: {}", e))
    })?;

if !status.success() {
    bail!("linuxdeploy failed with exit code: {:?}", status.code());
}
```

[`packages/kodegen-bundler-bundle/src/bundler/platform/linux/appimage.rs:203`](../../packages/kodegen-bundler-bundle/src/bundler/platform/linux/appimage.rs#L203):
```rust
let extract_status = tokio::process::Command::new(&appimage_path)
    .arg("--appimage-extract")
    .current_dir(tools_dir)
    .status()
    .await
    .map_err(|e| {
        crate::bundler::Error::GenericError(format!("Failed to run AppImage extraction: {}", e))
    })?;
```

**50+ more examples** across the bundler packages use this exact pattern.

### Benefits
- ✅ Non-blocking I/O
- ✅ Plays nice with tokio runtime  
- ✅ Future-proof for long-running contexts
- ✅ Consistent with existing codebase patterns
- ✅ No artificial delays or worker thread exhaustion

### Reference Documentation
- [Tokio Process Documentation](https://docs.rs/tokio/latest/tokio/process/index.html)
- [Tokio spawn_blocking vs tokio::process::Command](https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html) - spawn_blocking is for CPU-bound work, use tokio::process::Command for subprocess I/O

## Implementation Guide

### Step 1: Update Control Module Interface

**File:** [`packages/kodegend/src/control.rs`](../../packages/kodegend/src/control.rs)

**Current (lines 27-44):**
```rust
/// Check if daemon is running
///
/// Returns: Ok(true) if running, Ok(false) if stopped
pub fn check_status() -> Result<bool> {
    platform::check_status()
}

/// Start the daemon service
pub fn start_daemon() -> Result<()> {
    platform::start_daemon()
}

/// Stop the daemon service
pub fn stop_daemon() -> Result<()> {
    platform::stop_daemon()
}

/// Restart the daemon service
pub fn restart_daemon() -> Result<()> {
    platform::restart_daemon()
}
```

**Updated:**
```rust
/// Check if daemon is running
///
/// Returns: Ok(true) if running, Ok(false) if stopped
pub async fn check_status() -> Result<bool> {
    platform::check_status().await
}

/// Start the daemon service
pub async fn start_daemon() -> Result<()> {
    platform::start_daemon().await
}

/// Stop the daemon service
pub async fn stop_daemon() -> Result<()> {
    platform::stop_daemon().await
}

/// Restart the daemon service
pub async fn restart_daemon() -> Result<()> {
    platform::restart_daemon().await
}
```

**Changes:**
- Add `async` keyword to all function signatures
- Add `.await` to all platform function calls

### Step 2: Update Linux Platform Implementation

**File:** [`packages/kodegend/src/control/linux_control.rs`](../../packages/kodegend/src/control/linux_control.rs)

**Key Changes:**

1. **Import change** (line 1-2):
```rust
// OLD:
use std::process::Command;

// NEW:
use tokio::process::Command;
```

2. **check_status function** (lines 8-26):
```rust
// OLD:
pub fn check_status() -> Result<bool> {
    let service_name = format!("{}.service", SERVICE_NAME);
    let args = if is_root() {
        vec!["is-active", &service_name]
    } else {
        vec!["--user", "is-active", &service_name]
    };

    let output = Command::new("systemctl")
        .args(&args)
        .output()
        .context("Failed to execute systemctl is-active")?;

    Ok(output.status.success())
}

// NEW:
pub async fn check_status() -> Result<bool> {
    let service_name = format!("{}.service", SERVICE_NAME);
    let args = if is_root() {
        vec!["is-active", &service_name]
    } else {
        vec!["--user", "is-active", &service_name]
    };

    let output = Command::new("systemctl")
        .args(&args)
        .output()
        .await  // ← Add .await
        .context("Failed to execute systemctl is-active")?;

    Ok(output.status.success())
}
```

3. **start_daemon function** (lines 28-46):
```rust
// OLD:
pub fn start_daemon() -> Result<()> {
    // ... setup code ...
    
    let output = Command::new("systemctl")
        .args(&args)
        .output()
        .context("Failed to execute systemctl start")?;
    
    // ... error handling ...
}

// NEW:
pub async fn start_daemon() -> Result<()> {
    // ... setup code ...
    
    let output = Command::new("systemctl")
        .args(&args)
        .output()
        .await  // ← Add .await
        .context("Failed to execute systemctl start")?;
    
    // ... error handling ...
}
```

4. **stop_daemon function** (lines 48-66):
```rust
// OLD:
pub fn stop_daemon() -> Result<()> {
    // ... setup code ...
    
    let output = Command::new("systemctl")
        .args(&args)
        .output()
        .context("Failed to execute systemctl stop")?;
    
    // ... error handling ...
}

// NEW:
pub async fn stop_daemon() -> Result<()> {
    // ... setup code ...
    
    let output = Command::new("systemctl")
        .args(&args)
        .output()
        .await  // ← Add .await
        .context("Failed to execute systemctl stop")?;
    
    // ... error handling ...
}
```

5. **restart_daemon function** (lines 68-86):
```rust
// OLD:
pub fn restart_daemon() -> Result<()> {
    // ... setup code ...
    
    let output = Command::new("systemctl")
        .args(&args)
        .output()
        .context("Failed to execute systemctl restart")?;
    
    // ... error handling ...
}

// NEW:
pub async fn restart_daemon() -> Result<()> {
    // ... setup code ...
    
    let output = Command::new("systemctl")
        .args(&args)
        .output()
        .await  // ← Add .await
        .context("Failed to execute systemctl restart")?;
    
    // ... error handling ...
}
```

**Summary of Linux changes:**
- Change import: `std::process::Command` → `tokio::process::Command`
- Add `async` to all 4 function signatures
- Add `.await` after every `.output()` call (4 locations)

### Step 3: Update macOS Platform Implementation

**File:** [`packages/kodegend/src/control/macos_control.rs`](../../packages/kodegend/src/control/macos_control.rs)

**Key Changes:**

1. **Import changes** (lines 1-3):
```rust
// OLD:
use anyhow::{Context, Result};
use std::process::Command;
use std::time::Duration;

// NEW:
use anyhow::{Context, Result};
use tokio::process::Command;
use tokio::time::{sleep, Duration};
```

2. **check_status function** (lines 9-40):
```rust
// OLD:
pub fn check_status() -> Result<bool> {
    let output = Command::new("launchctl")
        .args(["list", SERVICE_LABEL])
        .output()
        .context("Failed to execute launchctl list")?;
    
    // ... parsing logic ...
}

// NEW:
pub async fn check_status() -> Result<bool> {
    let output = Command::new("launchctl")
        .args(["list", SERVICE_LABEL])
        .output()
        .await  // ← Add .await
        .context("Failed to execute launchctl list")?;
    
    // ... parsing logic (unchanged) ...
}
```

3. **start_daemon function** (lines 42-72):
```rust
// OLD:
pub fn start_daemon() -> Result<()> {
    let _ = Command::new("launchctl")
        .args(["bootstrap", "system", PLIST_PATH])
        .output();

    let output = Command::new("launchctl")
        .args(["kickstart", SERVICE_LABEL])
        .output()
        .context("Failed to execute launchctl kickstart")?;

    if !output.status.success() {
        let load_output = Command::new("launchctl")
            .args(["load", "-w", PLIST_PATH])
            .output()
            .context("Failed to execute launchctl load")?;
        // ... error handling ...
    }
    Ok(())
}

// NEW:
pub async fn start_daemon() -> Result<()> {
    let _ = Command::new("launchctl")
        .args(["bootstrap", "system", PLIST_PATH])
        .output()
        .await;  // ← Add .await

    let output = Command::new("launchctl")
        .args(["kickstart", SERVICE_LABEL])
        .output()
        .await  // ← Add .await
        .context("Failed to execute launchctl kickstart")?;

    if !output.status.success() {
        let load_output = Command::new("launchctl")
            .args(["load", "-w", PLIST_PATH])
            .output()
            .await  // ← Add .await
            .context("Failed to execute launchctl load")?;
        // ... error handling ...
    }
    Ok(())
}
```

4. **stop_daemon function** (lines 74-102):
```rust
// OLD:
pub fn stop_daemon() -> Result<()> {
    let _ = Command::new("launchctl")
        .args(["kill", "SIGTERM", SERVICE_LABEL])
        .output();

    std::thread::sleep(Duration::from_millis(500));  // ⚠️ Blocking sleep!

    let output = Command::new("launchctl")
        .args(["bootout", "system", PLIST_PATH])
        .output()
        .context("Failed to execute launchctl bootout")?;

    if !output.status.success() {
        let unload_output = Command::new("launchctl")
            .args(["unload", "-w", PLIST_PATH])
            .output()
            .context("Failed to execute launchctl unload")?;
        // ... error handling ...
    }
    Ok(())
}

// NEW:
pub async fn stop_daemon() -> Result<()> {
    let _ = Command::new("launchctl")
        .args(["kill", "SIGTERM", SERVICE_LABEL])
        .output()
        .await;  // ← Add .await

    sleep(Duration::from_millis(500)).await;  // ← Use tokio::time::sleep!

    let output = Command::new("launchctl")
        .args(["bootout", "system", PLIST_PATH])
        .output()
        .await  // ← Add .await
        .context("Failed to execute launchctl bootout")?;

    if !output.status.success() {
        let unload_output = Command::new("launchctl")
            .args(["unload", "-w", PLIST_PATH])
            .output()
            .await  // ← Add .await
            .context("Failed to execute launchctl unload")?;
        // ... error handling ...
    }
    Ok(())
}
```

5. **restart_daemon function** (lines 104-121):
```rust
// OLD:
pub fn restart_daemon() -> Result<()> {
    let output = Command::new("launchctl")
        .args(["kickstart", "-k", SERVICE_LABEL])
        .output()
        .context("Failed to execute launchctl kickstart -k")?;

    if !output.status.success() {
        stop_daemon()?;
        std::thread::sleep(Duration::from_secs(1));  // ⚠️ Blocking sleep!
        start_daemon()?;
    }

    Ok(())
}

// NEW:
pub async fn restart_daemon() -> Result<()> {
    let output = Command::new("launchctl")
        .args(["kickstart", "-k", SERVICE_LABEL])
        .output()
        .await  // ← Add .await
        .context("Failed to execute launchctl kickstart -k")?;

    if !output.status.success() {
        stop_daemon().await?;  // ← Add .await
        sleep(Duration::from_secs(1)).await;  // ← Use tokio::time::sleep!
        start_daemon().await?;  // ← Add .await
    }

    Ok(())
}
```

**Summary of macOS changes:**
- Change imports: `std::process::Command` → `tokio::process::Command`, add `tokio::time::sleep`
- Add `async` to all 4 function signatures
- Add `.await` after every `.output()` call (7 locations total)
- Replace `std::thread::sleep()` with `tokio::time::sleep().await` (2 locations)
- Add `.await` to recursive function calls in `restart_daemon()`

### Step 4: Update Windows Platform Implementation

**File:** [`packages/kodegend/src/control/windows_control.rs`](../../packages/kodegend/src/control/windows_control.rs)

**Note:** Windows implementation uses Windows API directly (not Command), but functions should still be marked `async` for consistency.

**Changes:**
- Add `async` to all 4 function signatures
- If any blocking operations exist, wrap in `tokio::task::spawn_blocking` or convert to async equivalents
- (Review the file to identify specific blocking operations in Windows API calls)

### Step 5: Update Main.rs Handlers

**File:** [`packages/kodegend/src/main.rs`](../../packages/kodegend/src/main.rs)

**Current handlers (lines ~208-254):**
```rust
fn handle_status() -> Result<()> {
    match control::check_status() {
        Ok(true) => {
            println!("kodegend is running");
            std::process::exit(0);
        }
        Ok(false) => {
            println!("kodegend is stopped");
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("Error checking status: {e:#}");
            std::process::exit(1);
        }
    }
}

fn handle_start() -> Result<()> {
    match control::start_daemon() {
        Ok(()) => {
            println!("kodegend started successfully");
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("Failed to start: {e:#}");
            std::process::exit(1);
        }
    }
}

fn handle_stop() -> Result<()> {
    match control::stop_daemon() {
        Ok(()) => {
            println!("kodegend stopped successfully");
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("Failed to stop: {e:#}");
            std::process::exit(1);
        }
    }
}

fn handle_restart() -> Result<()> {
    match control::restart_daemon() {
        Ok(()) => {
            println!("kodegend restarted successfully");
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("Failed to restart: {e:#}");
            std::process::exit(1);
        }
    }
}
```

**Updated handlers:**
```rust
async fn handle_status() -> Result<()> {
    match control::check_status().await {  // ← Add .await
        Ok(true) => {
            println!("kodegend is running");
            std::process::exit(0);
        }
        Ok(false) => {
            println!("kodegend is stopped");
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("Error checking status: {e:#}");
            std::process::exit(1);
        }
    }
}

async fn handle_start() -> Result<()> {
    match control::start_daemon().await {  // ← Add .await
        Ok(()) => {
            println!("kodegend started successfully");
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("Failed to start: {e:#}");
            std::process::exit(1);
        }
    }
}

async fn handle_stop() -> Result<()> {
    match control::stop_daemon().await {  // ← Add .await
        Ok(()) => {
            println!("kodegend stopped successfully");
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("Failed to stop: {e:#}");
            std::process::exit(1);
        }
    }
}

async fn handle_restart() -> Result<()> {
    match control::restart_daemon().await {  // ← Add .await
        Ok(()) => {
            println!("kodegend restarted successfully");
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("Failed to restart: {e:#}");
            std::process::exit(1);
        }
    }
}
```

**Summary of main.rs handler changes:**
- Add `async` to all 4 handler function signatures
- Add `.await` to all control function calls

### Step 6: Update real_main Match Statement

**File:** [`packages/kodegend/src/main.rs`](../../packages/kodegend/src/main.rs)

**Current (lines ~61-68):**
```rust
async fn real_main() -> Result<()> {
    let args = cli::Args::parse();

    match args.sub.unwrap_or(cli::Cmd::Run {
        foreground: false,
        config: None,
        system: false,
    }) {
        cli::Cmd::Run {
            foreground,
            config,
            system,
        } => run_daemon(foreground, config, system).await,
        cli::Cmd::Status => handle_status(),
        cli::Cmd::Start => handle_start(),
        cli::Cmd::Stop => handle_stop(),
        cli::Cmd::Restart => handle_restart(),
    }
}
```

**Updated:**
```rust
async fn real_main() -> Result<()> {
    let args = cli::Args::parse();

    match args.sub.unwrap_or(cli::Cmd::Run {
        foreground: false,
        config: None,
        system: false,
    }) {
        cli::Cmd::Run {
            foreground,
            config,
            system,
        } => run_daemon(foreground, config, system).await,
        cli::Cmd::Status => handle_status().await,     // ← Add .await
        cli::Cmd::Start => handle_start().await,       // ← Add .await
        cli::Cmd::Stop => handle_stop().await,         // ← Add .await
        cli::Cmd::Restart => handle_restart().await,   // ← Add .await
    }
}
```

## File-by-File Summary

| File | Changes Required |
|------|-----------------|
| [`control.rs`](../../packages/kodegend/src/control.rs) | Add `async` to 4 functions, add `.await` to 4 platform calls |
| [`control/linux_control.rs`](../../packages/kodegend/src/control/linux_control.rs) | Change import to `tokio::process::Command`, add `async` to 4 functions, add `.await` to 4 Command calls |
| [`control/macos_control.rs`](../../packages/kodegend/src/control/macos_control.rs) | Change imports to `tokio::process::Command` and `tokio::time::sleep`, add `async` to 4 functions, add `.await` to 7 Command calls and 2 sleep calls |
| [`control/windows_control.rs`](../../packages/kodegend/src/control/windows_control.rs) | Add `async` to 4 functions, review for blocking API calls |
| [`main.rs`](../../packages/kodegend/src/main.rs) | Add `async` to 4 handler functions, add `.await` to 8 function calls (4 in handlers + 4 in match) |

**Total Changes:**
- **5 files** to modify
- **20 function signatures** to make async
- **23+ `.await` calls** to add
- **3 import statements** to change
- **2 blocking sleep calls** to convert

## Definition of Done

The task is complete when:

1. ✅ All control module functions (`check_status`, `start_daemon`, `stop_daemon`, `restart_daemon`) are `async fn`
2. ✅ All platform implementations use `tokio::process::Command` instead of `std::process::Command`
3. ✅ All blocking sleeps use `tokio::time::sleep().await` instead of `std::thread::sleep()`
4. ✅ All handler functions in main.rs are `async fn` and call control functions with `.await`
5. ✅ The code compiles without errors: `cargo check -p kodegend`
6. ✅ The CLI commands work correctly:
   - `kodegend status` returns correct daemon status
   - `kodegend start` starts the daemon
   - `kodegend stop` stops the daemon
   - `kodegend restart` restarts the daemon

**Verification Commands:**
```bash
# Compile check
cd packages/kodegend
cargo check

# Build
cargo build --release

# Manual functional test (run each and verify output)
cargo run --release -- status
cargo run --release -- start
cargo run --release -- status
cargo run --release -- stop
cargo run --release -- status
```

## Performance Impact
- **Current**: Negligible (process exits immediately)
- **After fix**: Still negligible for CLI usage, but properly non-blocking for future async integrations

## Priority
**MEDIUM** - Important for:
- Code quality and best practices alignment
- Consistency with existing codebase patterns (bundler uses tokio::process::Command)
- Future extensibility for long-running daemon control scenarios
- Setting good architectural patterns

## Related Tasks
- When implementing platform modules, use `tokio::process::Command` from the start
- Any wait loops in restart logic should use `tokio::time::sleep` not `std::thread::sleep`

## References
- [Tokio Process Documentation](https://docs.rs/tokio/latest/tokio/process/index.html)
- [Tokio spawn_blocking Documentation](https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html)
- [Existing codebase pattern: bundler appimage.rs](../../packages/kodegen-bundler-bundle/src/bundler/platform/linux/appimage.rs#L138)
- [Existing codebase pattern: bundler dmg creation](../../packages/kodegen-bundler-bundle/src/bundler/platform/macos/dmg/creation.rs#L154)
