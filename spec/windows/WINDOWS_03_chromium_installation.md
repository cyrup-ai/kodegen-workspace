# Task: Enable Chromium Installation on Windows

## Priority: P1 (Core Functionality)

## Related Errors
- `install/chromium.rs:15` - function `get_chromium_install_timeout` never used
- `install/chromium.rs:27` - function `install_chromium` never used

## Problem Statement

The `install_chromium()` function exists and is fully implemented, but it's never called during Windows installation. This means Windows users cannot use browser automation tools.

## Current Implementation

`src/install/chromium.rs` contains:
```rust
/// Read Chromium installation timeout from environment or use default
fn get_chromium_install_timeout() -> Duration { ... }

/// Install Chromium using browser package's `download_managed_browser`
pub(super) async fn install_chromium() -> Result<PathBuf> { ... }
```

The function:
1. Reads timeout from `KODEGEN_CHROMIUM_TIMEOUT` env var (default 900s)
2. Displays progress message to user
3. Calls `kodegen_tools_browser::download_managed_browser()`
4. Verifies the download succeeded
5. Returns the Chromium executable path

## Required Implementation

### 1. Find Windows Installation Entry Point

Trace the installation flow to find where Chromium should be called:

Likely locations:
- `src/install/mod.rs` - Main installation orchestration
- `src/install/installer/mod.rs` - Installer builder
- `src/install/privilege.rs` - Elevated installation commands

### 2. Add Chromium Installation Step

In the Windows installation flow, add:
```rust
// After other installation steps
let chromium_path = install_chromium().await?;
log::info!("Chromium installed at: {}", chromium_path.display());
```

### 3. Handle Errors Appropriately

Chromium installation can fail due to:
- Network issues (timeout, DNS, firewall)
- Disk space
- Write permissions

Decide on error handling:
- **Hard failure**: Installation aborts if Chromium fails
- **Soft failure**: Continue without Chromium, warn user

Current code treats it as hard failure (returns `Result`).

### 4. Consider Offline Installation

For enterprise deployments, support pre-bundled Chromium:
```rust
// Check if Chromium is already present
let existing = kodegen_tools_browser::get_browser_path();
if existing.exists() {
    log::info!("Using existing Chromium at: {}", existing.display());
    return Ok(existing);
}

// Download if not present
install_chromium().await
```

## Files to Investigate

- `src/install/mod.rs` - Check for Unix Chromium installation call
- `src/install/installer/core/context.rs` - Installation context
- `src/main.rs` - CLI entry point for `install` command

## Files to Modify

- Wherever the Unix installation calls `install_chromium()`, add equivalent Windows call
- May need to add the call inside a `#[cfg(windows)]` block if flow differs

## Testing

1. Fresh Windows install - verify Chromium downloads
2. Reinstall - verify existing Chromium is detected
3. Network failure - verify appropriate error message
4. Timeout test - set `KODEGEN_CHROMIUM_TIMEOUT=5` and verify timeout handling

## Acceptance Criteria

- [ ] `install_chromium()` is called during Windows installation
- [ ] Browser tools work after installation
- [ ] Error handling provides clear messages
- [ ] No dead code warnings for chromium.rs functions
