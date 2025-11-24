# Code Quality: Magic Numbers Should Be Named Constants

## Severity: LOW

## Location
`packages/kodegend/src/daemon.rs:71`

## Issue Description
The code contains magic numbers without explanation, making them hard to understand and maintain.

## Current Code
```rust
std::thread::sleep(Duration::from_millis(100));
```

## Problem
The 100ms timeout is hardcoded with no explanation of why this specific value was chosen. Questions that arise:
- Why 100ms specifically?
- Is it sufficient for all systems?
- Can it be configured?
- What happens if it needs to change?

## Recommended Solution

### Define Named Constants
```rust
/// Timeout to wait for child process to start after fork
/// 
/// This is a conservative value that should work on most systems.
/// On slower systems or under heavy load, the child may need more time,
/// but we can't wait indefinitely as we need to report startup failures.
const CHILD_START_CHECK_DELAY_MS: u64 = 100;

/// Maximum time to wait for graceful shutdown before using SIGKILL
const GRACEFUL_SHUTDOWN_TIMEOUT_SECS: u64 = 10;

/// Interval for polling process status during shutdown
const SHUTDOWN_POLL_INTERVAL_MS: u64 = 50;

/// Maximum time to wait for process to fully terminate
const FORCE_KILL_TIMEOUT_SECS: u64 = 30;
```

### Use Named Constants
```rust
// Instead of:
std::thread::sleep(Duration::from_millis(100));

// Use:
std::thread::sleep(Duration::from_millis(CHILD_START_CHECK_DELAY_MS));
```

## Other Magic Numbers in Module

### File Permissions
```rust
// Current (if implemented)
std::fs::set_permissions(path, Permissions::from_mode(0o644))?;

// Better:
const PID_FILE_MODE: u32 = 0o644;  // rw-r--r--
std::fs::set_permissions(path, Permissions::from_mode(PID_FILE_MODE))?;
```

### Buffer Sizes
```rust
// Current
let mut buf = [0u8; 1];

// Better:
const STDIN_DRAIN_BUFFER_SIZE: usize = 1;
let mut buf = [0u8; STDIN_DRAIN_BUFFER_SIZE];
```

## Complete Constants Module

```rust
/// Configuration constants for daemon operations
mod constants {
    use std::time::Duration;
    
    /// Delay before checking if child process started successfully
    /// 
    /// After forking, we wait this amount of time before checking if the
    /// child process is still alive. This is a heuristic to catch immediate
    /// startup failures. Ideally, we'd use proper IPC for synchronization.
    pub const CHILD_START_CHECK_DELAY: Duration = Duration::from_millis(100);
    
    /// How long to wait for graceful shutdown before forcing termination
    /// 
    /// After sending SIGTERM, we poll the process status for this duration
    /// before escalating to SIGKILL.
    pub const GRACEFUL_SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(10);
    
    /// Interval between process status checks during shutdown
    /// 
    /// We poll at this interval to detect when the process has exited.
    /// Lower values = faster response, higher values = less CPU overhead.
    pub const SHUTDOWN_POLL_INTERVAL: Duration = Duration::from_millis(50);
    
    /// Maximum time to wait for forced termination (SIGKILL)
    /// 
    /// Even SIGKILL can take time if the process is in uninterruptible sleep.
    /// This is the absolute maximum we'll wait.
    pub const FORCE_KILL_TIMEOUT: Duration = Duration::from_secs(5);
    
    /// File permissions for PID files (rw-r--r--)
    /// 
    /// Owner can read/write, group and others can read.
    pub const PID_FILE_MODE: u32 = 0o644;
    
    /// File permissions for secure PID files (rw-------)
    /// 
    /// Only owner can read/write, more secure but may break some monitoring tools.
    pub const PID_FILE_MODE_SECURE: u32 = 0o600;
    
    /// Buffer size for draining stdin during daemonization
    pub const STDIN_DRAIN_BUFFER_SIZE: usize = 1;
}

use constants::*;
```

## Usage Examples

### Before
```rust
std::thread::sleep(Duration::from_millis(100));
// Why 100? What if it's not enough?
```

### After
```rust
std::thread::sleep(CHILD_START_CHECK_DELAY);
// Clear intent: waiting for child to start
// Can easily adjust if needed
// Can make configurable later
```

## Making Constants Configurable

For production flexibility, these could be made configurable:

```rust
pub struct DaemonConfig {
    /// How long to wait for child process startup verification
    pub child_start_check_delay: Duration,
    
    /// Graceful shutdown timeout before SIGKILL
    pub graceful_shutdown_timeout: Duration,
    
    /// Interval for polling process status
    pub shutdown_poll_interval: Duration,
    
    /// PID file permissions
    pub pid_file_mode: u32,
}

impl Default for DaemonConfig {
    fn default() -> Self {
        Self {
            child_start_check_delay: Duration::from_millis(100),
            graceful_shutdown_timeout: Duration::from_secs(10),
            shutdown_poll_interval: Duration::from_millis(50),
            pid_file_mode: 0o644,
        }
    }
}

// Load from config file
impl DaemonConfig {
    pub fn from_file(path: &Path) -> Result<Self> {
        // Parse TOML/JSON config
        // Fall back to defaults for missing values
        todo!()
    }
}
```

## Benefits

### Maintainability
- Easy to find and update timeout values
- Can tune for different environments
- Clear documentation of each value's purpose

### Testing
```rust
#[cfg(test)]
mod tests {
    // Can override constants for faster tests
    const TEST_SHUTDOWN_TIMEOUT: Duration = Duration::from_millis(100);
    
    #[test]
    fn test_shutdown() {
        // Use shorter timeout for tests
        // ...
    }
}
```

### Configurability
Later can add config file support:
```toml
[daemon]
graceful_shutdown_timeout_secs = 30  # Override default 10s
shutdown_poll_interval_ms = 100      # Less frequent polling
```

## List of All Magic Numbers

Audit of current magic numbers in daemon.rs:

| Line | Value | Purpose | Suggested Constant Name |
|------|-------|---------|------------------------|
| 71 | 100ms | Child start check delay | `CHILD_START_CHECK_DELAY_MS` |
| Future | 10s | Graceful shutdown timeout | `GRACEFUL_SHUTDOWN_TIMEOUT_SECS` |
| Future | 50ms | Shutdown poll interval | `SHUTDOWN_POLL_INTERVAL_MS` |
| Future | 0o644 | PID file permissions | `PID_FILE_MODE` |
| Future | 1 | stdin buffer size | `STDIN_DRAIN_BUFFER_SIZE` |

## Implementation Priority

1. **High**: Define constants for all timeout values
2. **Medium**: Document each constant's purpose
3. **Low**: Make constants configurable from file

## Testing

No specific tests needed, but:
- Verify code still compiles
- Verify behavior unchanged
- Verify constants are used consistently

## References
- "Clean Code" - Chapter 17: Smells and Heuristics (G25: Replace Magic Numbers with Named Constants)
- Rust API Guidelines: "Constants should be named with SCREAMING_SNAKE_CASE"
