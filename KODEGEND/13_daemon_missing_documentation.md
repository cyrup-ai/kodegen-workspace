# Code Quality: Missing Documentation on Public Functions

## Severity: LOW

## Location
All public functions in `packages/kodegend/src/daemon.rs`

## Issue Description
The module has no doc comments explaining what functions do, their safety considerations, or how to use them properly. This is especially critical for daemon management code which has subtle requirements.

## Current State
```rust
pub fn daemonize() -> Result<i32> {
    // No documentation!
    // Users don't know:
    // - What this does
    // - When to call it
    // - Side effects
    // - Platform support
}
```

## Recommended Documentation

### daemonize()
```rust
/// Daemonize the current process using the traditional Unix double-fork pattern.
///
/// This function performs the following operations:
/// 1. First fork() to background the process
/// 2. Create new session with setsid() to detach from terminal
/// 3. Second fork() to prevent acquiring a controlling terminal
/// 4. Change working directory to "/" to avoid blocking unmounts
/// 5. Close stdin/stdout/stderr and redirect to /dev/null
///
/// # Returns
///
/// Returns the PID of the final daemon process (the grandchild).
///
/// # Platform Support
///
/// This function is only available on Unix-like systems. On Windows, it will
/// return an error.
///
/// # Side Effects
///
/// - **Parent and intermediate processes exit**: Only the grandchild continues
/// - **Working directory changed** to "/"
/// - **File descriptors closed**: stdin, stdout, stderr redirected to /dev/null
/// - **New session created**: Process becomes session leader
/// - **Process reparented**: Final process adopted by init (PID 1)
///
/// # Safety
///
/// This function calls `fork()` which is unsafe in multi-threaded programs.
/// It should only be called very early in the program before any threads
/// are spawned.
///
/// # Errors
///
/// Returns an error if:
/// - Called on non-Unix platform
/// - fork() fails
/// - setsid() fails
/// - Unable to open /dev/null
/// - Child process fails to start
///
/// # Example
///
/// ```no_run
/// use kodegend::daemon::daemonize;
///
/// fn main() -> Result<()> {
///     // Must be called early, before any threads
///     let daemon_pid = daemonize()?;
///     
///     // At this point, we're running as a daemon
///     // Parent and intermediate processes have exited
///     // We're the grandchild with PID = daemon_pid
///     
///     // Run your daemon logic here
///     run_daemon_loop()?;
///     
///     Ok(())
/// }
/// ```
///
/// # See Also
///
/// - [`create_pid_file`] - Create PID file after daemonizing
/// - [`is_systemd_available`] - Check if running under systemd (may not need daemonization)
#[cfg(unix)]
pub fn daemonize() -> Result<i32> {
    // ... implementation
}
```

### create_pid_file()
```rust
/// Create a PID file containing the current process's PID.
///
/// This function:
/// 1. Checks if a PID file already exists
/// 2. If it exists, verifies the process is not running
/// 3. Creates a new PID file with the current PID
///
/// # Arguments
///
/// * `path` - Path where the PID file should be created
///
/// # Errors
///
/// Returns an error if:
/// - A PID file exists and the process is still running
/// - Unable to read existing PID file
/// - Unable to create new PID file
/// - Unable to write PID to file
///
/// # Security Considerations
///
/// This function is vulnerable to race conditions and symlink attacks.
/// See issue #2 for details. In production:
/// - Ensure PID file directory has restrictive permissions
/// - Consider using file locking for mutual exclusion
/// - Validate path is not a symlink
///
/// # Example
///
/// ```no_run
/// use std::path::Path;
/// use kodegend::daemon::create_pid_file;
///
/// let pid_file = Path::new("/var/run/kodegend.pid");
/// create_pid_file(pid_file)?;
/// ```
///
/// # See Also
///
/// - [`read_pid_file`] - Read PID from file
/// - [`remove_pid_file`] - Clean up PID file
/// - [`is_process_running`] - Check if process is still alive
pub fn create_pid_file(path: &Path) -> Result<()> {
    // ... implementation
}
```

### is_process_running()
```rust
/// Check if a process with the given PID is running.
///
/// # Platform Differences
///
/// - **Unix**: Uses kill(pid, 0) to check process existence
/// - **Windows**: Uses OpenProcess() with PROCESS_QUERY_INFORMATION
///
/// # Arguments
///
/// * `pid` - Process ID to check
///
/// # Returns
///
/// - `Ok(true)` if process is running
/// - `Ok(false)` if process does not exist or is not accessible
/// - `Err(_)` on system errors
///
/// # Limitations
///
/// This function has several important limitations:
///
/// 1. **No process identity verification**: Returns true for ANY process
///    with the given PID, not just kodegend processes. PIDs can be reused.
///
/// 2. **Zombie processes**: On Unix, returns true for zombie (defunct)
///    processes that have exited but not been reaped.
///
/// 3. **Permission issues**: May return false for processes that exist
///    but are owned by a different user.
///
/// 4. **Race conditions**: Process could exit between check and use.
///
/// For production use, consider using `is_kodegend_running()` which
/// verifies process identity.
///
/// # Example
///
/// ```no_run
/// use kodegend::daemon::is_process_running;
///
/// let pid = 12345;
/// if is_process_running(pid)? {
///     println!("Process {} is running", pid);
/// } else {
///     println!("Process {} is not running", pid);
/// }
/// ```
///
/// # See Also
///
/// - [`get_service_status`] - Higher-level status check with PID file
pub fn is_process_running(pid: i32) -> Result<bool> {
    // ... implementation
}
```

### stop_service()
```rust
/// Stop the service by sending a termination signal to the process.
///
/// This function:
/// 1. Reads the PID from the PID file
/// 2. Checks if the process is running
/// 3. Sends SIGTERM (Unix) or TerminateProcess (Windows)
/// 4. Removes the PID file
///
/// # Arguments
///
/// * `pid_file` - Path to the PID file
///
/// # Errors
///
/// Returns an error if:
/// - Unable to read PID file
/// - Unable to send termination signal
/// - Unable to remove PID file
///
/// # Important Limitations
///
/// **This function does NOT wait for the process to terminate!**
///
/// It sends the signal and immediately returns. The process may still be
/// running when this function returns. This can cause issues:
/// - PID file removed while process is still cleaning up
/// - restart_service() may fail due to resource conflicts
/// - Port binding failures if process hasn't released ports
///
/// For production use, see task #4 for a version that waits for termination.
///
/// # Security Considerations
///
/// - No verification that PID belongs to kodegend (PID reuse vulnerability)
/// - Sends signal to arbitrary PID from file (could terminate wrong process)
/// - Should validate process identity before terminating
///
/// # Example
///
/// ```no_run
/// use std::path::Path;
/// use kodegend::daemon::stop_service;
///
/// let pid_file = Path::new("/var/run/kodegend.pid");
/// stop_service(pid_file)?;
///
/// // WARNING: Process may still be running!
/// // Use `get_service_status` to verify termination
/// ```
///
/// # See Also
///
/// - [`restart_service`] - Stop and start service
/// - [`get_service_status`] - Check if service has actually stopped
pub fn stop_service(pid_file: &Path) -> Result<()> {
    // ... implementation
}
```

### get_service_status()
```rust
/// Get the current status of the service by checking the PID file.
///
/// This function checks:
/// 1. Whether a PID file exists
/// 2. Whether the PID in the file is valid
/// 3. Whether the process is running
///
/// # Arguments
///
/// * `pid_file` - Path to the PID file
///
/// # Returns
///
/// A string describing the service status:
/// - `"stopped"` - No PID file exists
/// - `"running (PID: N)"` - Process is running
/// - `"stopped (stale PID file)"` - PID file exists but process is not running
/// - `"unknown (invalid PID file)"` - PID file is corrupted
///
/// # Note
///
/// The string-based return type is not ideal. See task #11 for a proposal
/// to return a structured enum instead.
///
/// # Example
///
/// ```no_run
/// use std::path::Path;
/// use kodegend::daemon::get_service_status;
///
/// let pid_file = Path::new("/var/run/kodegend.pid");
/// let status = get_service_status(pid_file)?;
/// println!("Service status: {}", status);
/// ```
pub fn get_service_status(pid_file: &Path) -> Result<String> {
    // ... implementation
}
```

### restart_service()
```rust
/// Restart the service by stopping it and calling a start function.
///
/// This function:
/// 1. Checks if service is running via PID file
/// 2. If running, calls `stop_service()` to stop it
/// 3. Calls the provided `start_fn` to start it again
///
/// # Arguments
///
/// * `pid_file` - Path to the PID file
/// * `start_fn` - Closure that starts the service
///
/// # Errors
///
/// Returns an error if:
/// - `stop_service()` fails
/// - `start_fn()` fails
///
/// # Critical Issues
///
/// **This function has a serious race condition!**
///
/// `stop_service()` doesn't wait for the process to terminate, so this
/// function immediately calls `start_fn()` while the old process may still
/// be running. This causes:
/// - Port binding conflicts
/// - Resource contention
/// - Failed restarts
///
/// See task #3 for a fix that waits for termination.
///
/// # Example
///
/// ```no_run
/// use std::path::Path;
/// use kodegend::daemon::restart_service;
///
/// let pid_file = Path::new("/var/run/kodegend.pid");
/// restart_service(pid_file, || {
///     // Start the service
///     start_daemon()
/// })?;
/// ```
///
/// # See Also
///
/// - [`stop_service`] - Stop the service
pub fn restart_service(pid_file: &Path, start_fn: impl FnOnce() -> Result<()>) -> Result<()> {
    // ... implementation
}
```

### is_systemd_available()
```rust
/// Check if systemd is available on the current system.
///
/// On Unix systems, this checks if systemd is running by executing
/// `pgrep systemd`. On non-Unix systems, always returns false.
///
/// # Returns
///
/// - `true` if systemd appears to be available
/// - `false` if systemd is not available or check failed
///
/// # Performance
///
/// **This function spawns a subprocess every call!**
///
/// For better performance, see task #8 which suggests:
/// - Checking `/run/systemd/system` directory instead
/// - Caching the result since systemd availability doesn't change at runtime
///
/// # Example
///
/// ```no_run
/// use kodegend::daemon::is_systemd_available;
///
/// if is_systemd_available() {
///     println!("Running under systemd");
///     // May not need to daemonize
/// } else {
///     println!("Not running under systemd");
///     // Should daemonize manually
/// }
/// ```
pub fn is_systemd_available() -> bool {
    // ... implementation
}
```

## Module-Level Documentation

```rust
//! Daemon process management utilities.
//!
//! This module provides functions for:
//! - Daemonizing processes using the traditional Unix double-fork pattern
//! - Managing PID files
//! - Checking process status
//! - Starting, stopping, and restarting daemon services
//!
//! # Platform Support
//!
//! Most functionality is Unix-only. Windows support is limited to:
//! - PID file management
//! - Process status checking
//! - Process termination
//!
//! # Security Considerations
//!
//! This module has several known security issues:
//! - PID file race conditions (task #2)
//! - Symlink attacks (task #9)
//! - PID reuse vulnerabilities (task #5)
//!
//! For production use, ensure:
//! - PID files are in directories with restrictive permissions
//! - Daemon runs with appropriate user privileges
//! - PID file paths are not user-controlled
//!
//! # Usage Example
//!
//! ```no_run
//! use std::path::Path;
//! use kodegend::daemon::*;
//!
//! fn main() -> anyhow::Result<()> {
//!     let pid_file = Path::new("/var/run/myservice.pid");
//!     
//!     // Daemonize the process
//!     if !is_systemd_available() {
//!         let daemon_pid = daemonize()?;
//!         create_pid_file(pid_file)?;
//!     }
//!     
//!     // Run daemon loop
//!     run_service()?;
//!     
//!     // Clean up
//!     remove_pid_file(pid_file)?;
//!     Ok(())
//! }
//! ```
//!
//! # See Also
//!
//! - [daemon(3)](https://man7.org/linux/man-pages/man3/daemon.3.html) - Linux daemon creation
//! - [systemd.service(5)](https://www.freedesktop.org/software/systemd/man/systemd.service.html) - systemd service files
//! - W. Richard Stevens, "Advanced Programming in the UNIX Environment", Chapter 13
```

## Benefits of Documentation

### For Users
- Understand what functions do without reading implementation
- Know when to use each function
- Aware of security issues and limitations
- See working examples

### For Maintainers
- Remember why code was written a certain way
- Document known issues inline
- Cross-reference related tasks/issues
- Preserve institutional knowledge

### For IDEs
- Better autocomplete suggestions
- Inline documentation in tooltips
- Link to related functions

## Implementation Checklist

- [ ] Add module-level doc comment
- [ ] Document `daemonize()`
- [ ] Document `create_pid_file()`
- [ ] Document `read_pid_file()`
- [ ] Document `remove_pid_file()`
- [ ] Document `is_process_running()`
- [ ] Document `get_service_status()`
- [ ] Document `stop_service()`
- [ ] Document `restart_service()`
- [ ] Document `is_systemd_available()`
- [ ] Add examples to each function
- [ ] Link to related tasks for known issues
- [ ] Run `cargo doc` and verify output

## References
- Rust API Guidelines: "All public items should have documentation"
- RFC 1574: More API documentation conventions
- "The Rustdoc Book": https://doc.rust-lang.org/rustdoc/
