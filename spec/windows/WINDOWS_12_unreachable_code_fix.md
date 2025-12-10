# Task: Fix Unreachable Code in main.rs Status Command

## Priority: P1 (Bug Fix)

## Related Error
- `main.rs:361` - unreachable expression

## Problem Statement

The Windows status display code has unreachable code due to logic error:

```rust
#[cfg(windows)]
{
    match connect_named_pipe(path_str) {
        Ok(mut stream) => {
            // ... display status ...
            std::process::exit(0);  // Line 347 - EXITS
        }
        Err(_) => {
            // ... display basic status ...
            std::process::exit(0);  // Line 355 - EXITS
        }
    }
}

#[cfg(not(unix))]  // This includes Windows!
{
    // THIS CODE IS UNREACHABLE ON WINDOWS
    cli_output::info("● kodegend.service - KODEGEN Daemon");
    cli_output::info("   Active: active (running)");
    cli_output::info(&format!("   Main PID: {}", pid));
    std::process::exit(0);
}
```

Both branches of the `#[cfg(windows)]` match exit, so the `#[cfg(not(unix))]` block can never execute on Windows.

## Analysis

The `#[cfg(not(unix))]` block was likely intended as a fallback for platforms that are neither Unix nor Windows. However:
1. Windows is covered by `#[cfg(windows)]` block above
2. The `not(unix)` condition includes Windows
3. Since Windows exits in the match, the fallback is unreachable

## Required Implementation

### Option A: Remove Unreachable Code (Recommended)

The `#[cfg(windows)]` block handles all Windows cases. Remove the unreachable fallback:

```rust
#[cfg(windows)]
{
    match connect_named_pipe(path_str) {
        Ok(mut stream) => {
            // ... detailed status ...
            std::process::exit(0);
        }
        Err(_) => {
            // Named pipe not available, fall back to basic status
            cli_output::info("● kodegend.service - KODEGEN Daemon");
            cli_output::info("   Active: active (running)");
            cli_output::info(&format!("   Main PID: {}", pid));
            cli_output::info("   Note: Detailed status not available (named pipe unavailable)");
            std::process::exit(0);
        }
    }
}

// Remove the #[cfg(not(unix))] block entirely
```

### Option B: Fix CFG Condition

If a fallback is needed for other platforms (WASM, etc.), fix the condition:

```rust
#[cfg(not(any(unix, windows)))]
{
    // Fallback for exotic platforms (WASM, etc.)
    cli_output::info("● kodegend.service - KODEGEN Daemon");
    cli_output::info(&format!("   Main PID: {}", pid));
    std::process::exit(0);
}
```

## Context: Full Code Structure

```rust
// Check daemon status
let status = check_daemon_status()?;

if let ServiceStatus::Running { pid } = &status {
    // Unix detailed status via Unix socket
    #[cfg(unix)]
    {
        // ... Unix socket connection and status display ...
        std::process::exit(0);
    }

    // Windows detailed status via Named Pipe
    #[cfg(windows)]
    {
        // ... Named pipe connection and status display ...
        // BOTH branches exit(0)
    }

    // UNREACHABLE on both Unix and Windows:
    #[cfg(not(unix))]
    {
        // Basic fallback (never runs)
    }
}
```

## Files to Modify

- `src/main.rs` around lines 360-367

## Testing

1. Run `kodegend status` when daemon is running - verify output
2. Run `kodegend status` when named pipe unavailable - verify fallback output
3. Compile with `--target x86_64-pc-windows-msvc` - no warnings

## Acceptance Criteria

- [ ] No unreachable code warning
- [ ] Windows status command still works
- [ ] Named pipe failure fallback still works
- [ ] Code compiles cleanly for Windows target
