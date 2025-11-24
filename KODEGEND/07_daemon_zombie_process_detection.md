# Zombie Processes Not Detected as Dead

## Severity: HIGH

## Location
`packages/kodegend/src/daemon.rs:147-171` (is_process_running)

## Issue Description
The Unix implementation of `is_process_running()` uses `kill(pid, 0)` which returns success for zombie (defunct) processes. Zombies appear as "running" even though they're effectively dead.

## Current Implementation
```rust
#[cfg(unix)]
{
    use nix::sys::signal::{kill, Signal};
    let pid = Pid::from_raw(pid);
    match kill(pid, None) {
        Ok(_) => Ok(true),   // Returns true for zombies!
        Err(_) => Ok(false),
    }
}
```

## What Are Zombie Processes?

A zombie process is a process that has:
- Completed execution (`exit()` called)
- Still has an entry in the process table
- Waiting for parent to call `wait()` to collect exit status
- Shows as `<defunct>` or `Z` state in `ps`

```bash
$ ps aux | grep kodegend
root     1234  0.0  0.0      0     0 ?        Z    12:00   0:00 [kodegend] <defunct>
```

## Problems

### 1. False "Running" Status
```bash
$ kodegend stop
# Process exits but becomes zombie
$ kodegend status
Service running (PID: 1234)
# LIE! Process is dead zombie
```

### 2. Prevents Restart
```bash
$ kodegend restart
Error: Service already running with PID 1234
# Can't restart because zombie appears alive!
```

### 3. Zombie Accumulation
If `stop_service()` doesn't properly wait for children:
- Each stop creates a zombie
- Zombies accumulate over time
- Eventually exhaust process table
- System-wide impact

### 4. Misleading Monitoring
Monitoring systems check `kodegend status`:
- Reports "healthy" for zombie
- Alerts don't fire
- Service is actually down
- Silent production outage

## How Zombies Are Created in Current Code

From `daemonize()`:
```rust
match unsafe { fork() } {
    Ok(ForkResult::Parent { child }) => {
        std::process::exit(0);  // First parent exits
    }
    Ok(ForkResult::Child) => {
        // Child continues...
        match unsafe { fork() } {
            Ok(ForkResult::Parent { child }) => {
                // Intermediate parent exits
                std::process::exit(0);
            }
            Ok(ForkResult::Child) => {
                // Grandchild becomes daemon
            }
        }
    }
}
```

If daemon later crashes and parent isn't PID 1 or init:
- Daemon exits and becomes zombie
- Parent never calls wait()
- Zombie persists

## Recommended Solutions

### Solution 1: Check Process State (Recommended)
```rust
#[cfg(target_os = "linux")]
pub fn is_process_running(pid: i32) -> Result<bool> {
    // First check if process exists
    use nix::sys::signal::{kill, Signal};
    let pid_obj = Pid::from_raw(pid);
    
    if kill(pid_obj, None).is_err() {
        return Ok(false);
    }
    
    // Now check if it's a zombie
    let stat_path = format!("/proc/{}/stat", pid);
    let stat = std::fs::read_to_string(&stat_path)
        .map_err(|_| anyhow!("Process {} does not exist", pid))?;
    
    // Parse state from /proc/PID/stat
    // Format: pid (comm) state ...
    // State is third field
    let state = stat
        .split_whitespace()
        .nth(2)
        .ok_or_else(|| anyhow!("Invalid stat format"))?;
    
    // Z = zombie, return false for zombies
    Ok(state != "Z")
}

#[cfg(target_os = "macos")]
pub fn is_process_running(pid: i32) -> Result<bool> {
    use libproc::libproc::proc_pid::{pidinfo, PIDInfo, ProcBSDInfo};
    
    match pidinfo::<ProcBSDInfo>(pid, 0) {
        Ok(info) => {
            // Check if process is zombie (status code 5)
            Ok(info.pbi_status != 5)
        }
        Err(_) => Ok(false),
    }
}

#[cfg(target_os = "freebsd")]
pub fn is_process_running(pid: i32) -> Result<bool> {
    // FreeBSD: use sysctl kern.proc.pid.{pid}
    // or fall back to kill(0) + parsing ps output
    let output = std::process::Command::new("ps")
        .args(&["-p", &pid.to_string(), "-o", "state="])
        .output()?;
    
    if !output.status.success() {
        return Ok(false);
    }
    
    let state = String::from_utf8_lossy(&output.stdout);
    Ok(!state.trim().starts_with('Z'))
}
```

### Solution 2: Proper Cleanup in stop_service()
From task 04, ensure `stop_service()` calls `waitpid()`:
```rust
// This prevents zombies by reaping the child
waitpid(Pid::from_raw(pid), None)?;
```

### Solution 3: Double-Fork Adoption by Init
The double-fork pattern should make init (PID 1) the parent:
- When intermediate parent exits, grandchild is re-parented to init
- Init automatically reaps zombies
- This SHOULD work, but only if intermediate parent fully exits

Verify in code:
```rust
// After second fork, verify parent is PID 1
#[cfg(unix)]
fn verify_daemonized() -> bool {
    unsafe { libc::getppid() == 1 }
}
```

## Complete is_process_running() Implementation

```rust
pub fn is_process_running(pid: i32) -> Result<bool> {
    #[cfg(target_os = "linux")]
    {
        use nix::sys::signal::{kill, Signal};
        
        // Quick check: does process exist?
        if kill(Pid::from_raw(pid), None).is_err() {
            return Ok(false);
        }
        
        // Detailed check: is it a zombie?
        let stat = match std::fs::read_to_string(format!("/proc/{}/stat", pid)) {
            Ok(s) => s,
            Err(_) => return Ok(false),  // Process disappeared
        };
        
        // Parse state (third field)
        let state = stat
            .split_whitespace()
            .nth(2)
            .unwrap_or("X");
        
        Ok(state != "Z")
    }
    
    #[cfg(target_os = "macos")]
    {
        use libproc::libproc::proc_pid::{pidinfo, PIDInfo, ProcBSDInfo};
        
        match pidinfo::<ProcBSDInfo>(pid, 0) {
            Ok(info) => Ok(info.pbi_status != 5),  // 5 = zombie
            Err(_) => Ok(false),
        }
    }
    
    #[cfg(all(unix, not(any(target_os = "linux", target_os = "macos"))))]
    {
        // Fallback: use ps command
        let output = std::process::Command::new("ps")
            .args(&["-p", &pid.to_string(), "-o", "state="])
            .output()?;
        
        if !output.status.success() {
            return Ok(false);
        }
        
        let state = String::from_utf8_lossy(&output.stdout);
        Ok(!state.trim().contains('Z'))
    }
    
    #[cfg(windows)]
    {
        // Windows doesn't have zombie processes
        // Keep existing implementation
        unsafe {
            let handle = OpenProcess(PROCESS_QUERY_INFORMATION, 0, pid as u32);
            if handle.is_null() {
                return Ok(false);
            }
            CloseHandle(handle);
            Ok(true)
        }
    }
}
```

## Testing Strategy

### Create Zombie Test
```rust
#[cfg(test)]
mod tests {
    #[test]
    #[cfg(unix)]
    fn test_zombie_detection() {
        use nix::unistd::{fork, ForkResult};
        
        match unsafe { fork() } {
            Ok(ForkResult::Parent { child }) => {
                // Parent: don't wait, create zombie
                std::thread::sleep(Duration::from_millis(100));
                
                // Child should be zombie now
                assert!(!is_process_running(child.as_raw()).unwrap());
                
                // Clean up: reap the zombie
                nix::sys::wait::waitpid(child, None).unwrap();
            }
            Ok(ForkResult::Child) => {
                // Child: exit immediately
                std::process::exit(0);
            }
            Err(_) => panic!("Fork failed"),
        }
    }
}
```

### Manual Test
```bash
# Terminal 1: Create zombie
$ perl -e 'fork and sleep 999'

# Terminal 2: Find zombie PID
$ ps aux | grep perl | grep Z
user     12345  0.0  0.0      0     0 ?        Z    12:00   0:00 [perl] <defunct>

# Terminal 3: Test detection
$ cargo test -- --nocapture test_zombie_detection
```

## Dependencies
```toml
[target.'cfg(target_os = "macos")'.dependencies]
libproc = "0.14"
```

## Performance Impact
- Linux: /proc read ~0.05-0.1ms
- macOS: pidinfo syscall ~0.1ms
- Negligible compared to network/disk I/O

## References
- Stevens, "Advanced Programming in the UNIX Environment", Chapter 8
- Linux kernel Documentation/filesystems/proc.txt
- macOS `man libproc`
