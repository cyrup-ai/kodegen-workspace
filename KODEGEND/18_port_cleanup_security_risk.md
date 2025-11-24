# SECURITY RISK: Killing Arbitrary Processes Without Verification

## Severity
**CRITICAL** - Can kill unrelated processes, potential for service disruption

## Location
`packages/kodegend/src/service/port_cleanup.rs`
- Lines 203-243: `cleanup_port_if_needed()`

## Issue Description
The port cleanup function finds a process by port and kills it WITHOUT verifying:
- Process identity (is it actually a kodegen server?)
- Process ownership (does current user own it?)
- Process safety (is it a critical system service?)

```rust
pub async fn cleanup_port_if_needed(port: u16) -> Result<()> {
    // ... check if port is available ...
    
    // Find process using the port
    let pid = match find_process_by_port(port).await? {
        Some(pid) => pid,
        None => { /* ... */ }
    };

    log::warn!("Terminating process {} using port {}", pid, port);

    // Kill the process - NO VERIFICATION!
    kill_process_graceful(pid).await
        .context(format!("Failed to kill process {}", pid))?;
    
    // ... wait for port release ...
}
```

## Production Security Scenarios

### Scenario 1: Shared Server Environment
**Setup**:
- Multi-user Linux server
- User A runs kodegend on ports 30438-30452
- User B happens to run a web service on port 30440

**Attack Vector**:
1. User A stops kodegend (ports freed)
2. User B starts web service, binds port 30440
3. User A restarts kodegend
4. kodegend finds port 30440 occupied
5. **kodegend kills User B's web service without permission**
6. Service disruption for User B

### Scenario 2: Port Collision with System Service
**Setup**:
- Admin configures kodegend to use port 8080
- System service (nginx, apache) already using port 8080

**Attack Vector**:
1. kodegend starts up
2. Finds port 8080 occupied by nginx
3. **kodegend kills nginx** (if running as root)
4. Production web server goes down
5. Major outage

### Scenario 3: Malicious Configuration
**Setup**:
- Attacker has limited access to server
- Can modify kodegend config but can't kill root processes directly

**Attack Vector**:
1. Attacker identifies critical service PID (e.g., sshd on port 22)
2. Attacker modifies kodegend config to use port 22
3. Attacker triggers kodegend restart
4. kodegend attempts to kill sshd
5. If successful, server becomes inaccessible

### Scenario 4: Process ID Reuse
**Setup**:
- kodegen server was running as PID 12345
- Server crashes
- OS reuses PID 12345 for unrelated process

**Attack Vector**:
1. kodegend restarts
2. Checks port, finds PID 12345
3. **Kills wrong process** (PID was reused)
4. Unintended process termination

## Current Lack of Validation

The code performs ZERO validation:

❌ No process name checking
❌ No process path verification
❌ No ownership verification
❌ No user confirmation
❌ No safelist/blocklist
❌ No process age checking
❌ No command-line argument inspection

It blindly kills whatever process is bound to the port.

## Recommended Solutions

### Option 1: Verify Process Identity (Minimum Required)
```rust
pub async fn cleanup_port_if_needed(port: u16) -> Result<()> {
    // ... check if port is available ...
    
    let pid = find_process_by_port(port).await?
        .ok_or_else(|| anyhow::anyhow!("Port in use but no process found"))?;

    // VERIFY: Is this a kodegen process?
    if !is_safe_to_kill(pid).await? {
        return Err(anyhow::anyhow!(
            "Port {} is occupied by process {} which does not appear to be a kodegen server. \
             Please manually stop the process or change the configured port.",
            port, pid
        ));
    }

    log::warn!("Terminating kodegen process {} using port {}", pid, port);
    kill_process_graceful(pid).await?;
    
    // ... wait for port release ...
}

async fn is_safe_to_kill(pid: u32) -> Result<bool> {
    use sysinfo::{Pid, ProcessesToUpdate, System};
    
    tokio::task::spawn_blocking(move || {
        let mut system = System::new();
        let sysinfo_pid = Pid::from(pid as usize);
        system.refresh_processes(ProcessesToUpdate::Some(&[sysinfo_pid]), true);
        
        let process = system.process(sysinfo_pid)
            .ok_or_else(|| anyhow::anyhow!("Process not found"))?;
        
        // Get process name and path
        let name = process.name();
        let exe = process.exe();
        
        // Check if it's a kodegen binary
        let is_kodegen = name.to_string_lossy().contains("kodegen") ||
                        exe.map(|p| p.to_string_lossy().contains("kodegen"))
                           .unwrap_or(false);
        
        Ok(is_kodegen)
    })
    .await?
}
```

### Option 2: Check Process Ownership
```rust
async fn is_safe_to_kill(pid: u32) -> Result<bool> {
    use sysinfo::{Pid, ProcessesToUpdate, System};
    
    tokio::task::spawn_blocking(move || {
        let mut system = System::new();
        let sysinfo_pid = Pid::from(pid as usize);
        system.refresh_processes(ProcessesToUpdate::Some(&[sysinfo_pid]), true);
        
        let process = system.process(sysinfo_pid)
            .ok_or_else(|| anyhow::anyhow!("Process not found"))?;
        
        // Check ownership (Unix only)
        #[cfg(unix)]
        {
            let current_uid = unsafe { libc::getuid() };
            let process_uid = process.user_id()
                .ok_or_else(|| anyhow::anyhow!("Cannot determine process owner"))?;
            
            if process_uid.to_string() != current_uid.to_string() {
                log::warn!("Process {} owned by different user", pid);
                return Ok(false);
            }
        }
        
        // Check if it's a kodegen binary
        let name = process.name().to_string_lossy();
        let is_kodegen = name.contains("kodegen-tools-") || 
                        name.starts_with("kodegen");
        
        Ok(is_kodegen)
    })
    .await?
}
```

### Option 3: Interactive Confirmation (CLI mode)
```rust
pub async fn cleanup_port_if_needed(port: u16) -> Result<()> {
    // ... find process ...
    
    let process_info = get_process_info(pid).await?;
    
    log::warn!(
        "Port {} is occupied by process {} ({})",
        port, pid, process_info.name
    );
    
    if !is_safe_to_kill(pid).await? {
        log::error!(
            "Process does not appear to be a kodegen server:\n\
             Name: {}\n\
             Path: {}\n\
             Owner: {}",
            process_info.name,
            process_info.path.display(),
            process_info.user
        );
        
        // In daemon mode: fail
        // In CLI mode: could prompt user
        return Err(anyhow::anyhow!(
            "Refusing to kill non-kodegen process. Please resolve manually."
        ));
    }
    
    // ... proceed with kill ...
}
```

### Option 4: Configuration-Based Safeguards
```toml
[port_cleanup]
# Only kill processes matching these patterns
allow_kill_patterns = ["kodegen-tools-*", "kodegen*"]

# Never kill processes matching these patterns
deny_kill_patterns = ["nginx", "apache", "sshd", "systemd"]

# Require explicit confirmation in config
require_manual_cleanup = false
```

## Recommended Approach
Implement **Option 1 + Option 2**:
1. Verify process name contains "kodegen"
2. Verify process is owned by current user
3. Fail with clear error if validation fails

This prevents most common security issues while maintaining usability.

## Files to Modify
- `packages/kodegend/src/service/port_cleanup.rs`

## Testing Considerations
- Test with non-kodegen process on port (should refuse to kill)
- Test with kodegen process on port (should kill)
- Test with process owned by different user (should refuse)
- Test with system service on port (should refuse)
- Verify error messages guide user to resolution
