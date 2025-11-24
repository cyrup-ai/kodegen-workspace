# Code Quality: String-Based Status Returns Should Be Enum

## Severity: MEDIUM

## Location
`packages/kodegend/src/daemon.rs:173-185`

## Issue Description
The `get_service_status()` function returns a `String` that must be parsed by callers, instead of a structured enum. This is error-prone and not type-safe.

## Current Implementation
```rust
pub fn get_service_status(pid_file: &Path) -> Result<String> {
    if !pid_file.exists() {
        return Ok("stopped".to_string());
    }
    
    match read_pid_file(pid_file) {
        Ok(pid) => {
            if is_process_running(pid)? {
                Ok(format!("running (PID: {})", pid))
            } else {
                Ok("stopped (stale PID file)".to_string())
            }
        }
        Err(_) => Ok("unknown (invalid PID file)".to_string()),
    }
}
```

## Problems

### 1. Fragile String Matching
Callers must parse strings:
```rust
// Fragile!
let status = get_service_status(&pid_file)?;
if status.starts_with("running") {
    // Extract PID from string? Yikes!
    let pid = status.split("PID: ")
        .nth(1)?
        .trim_end_matches(')')
        .parse()?;
}
```

### 2. Not Exhaustive
Compiler can't verify all cases handled:
```rust
match get_service_status(&pid_file)?.as_str() {
    "stopped" => { /* ... */ },
    "running" => { /* ... */ },  // Wrong! Format is "running (PID: 123)"
    // Missing cases: stale file, invalid file
}
```

### 3. Internationalization Nightmare
If status messages need translation, all string matching breaks.

### 4. Lost Information
Important details buried in strings:
- PID is embedded in string, not separate field
- Can't distinguish "never started" from "crashed"
- Can't access structured data programmatically

## Recommended Solution

### Define Status Enum
```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceStatus {
    /// Service is running with the given PID
    Running { pid: i32 },
    
    /// Service is stopped, no PID file exists
    Stopped,
    
    /// Service is stopped, but PID file exists with given PID
    StaleFile { pid: i32 },
    
    /// PID file exists but is corrupted/invalid
    InvalidFile { error: String },
    
    /// Service is running but appears to be a zombie process
    Zombie { pid: i32 },
}

impl ServiceStatus {
    /// Returns true if service is actively running
    pub fn is_running(&self) -> bool {
        matches!(self, ServiceStatus::Running { .. })
    }
    
    /// Returns the PID if available
    pub fn pid(&self) -> Option<i32> {
        match self {
            ServiceStatus::Running { pid }
            | ServiceStatus::StaleFile { pid }
            | ServiceStatus::Zombie { pid } => Some(*pid),
            _ => None,
        }
    }
    
    /// Returns true if cleanup is needed
    pub fn needs_cleanup(&self) -> bool {
        matches!(
            self,
            ServiceStatus::StaleFile { .. }
            | ServiceStatus::InvalidFile { .. }
            | ServiceStatus::Zombie { .. }
        )
    }
}

impl std::fmt::Display for ServiceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceStatus::Running { pid } => {
                write!(f, "running (PID: {})", pid)
            }
            ServiceStatus::Stopped => {
                write!(f, "stopped")
            }
            ServiceStatus::StaleFile { pid } => {
                write!(f, "stopped (stale PID file for PID {})", pid)
            }
            ServiceStatus::InvalidFile { error } => {
                write!(f, "unknown (invalid PID file: {})", error)
            }
            ServiceStatus::Zombie { pid } => {
                write!(f, "zombie (PID: {})", pid)
            }
        }
    }
}
```

### Improved Implementation
```rust
pub fn get_service_status(pid_file: &Path) -> Result<ServiceStatus> {
    // No PID file = definitely stopped
    if !pid_file.exists() {
        return Ok(ServiceStatus::Stopped);
    }
    
    // Try to read PID from file
    let pid = match read_pid_file(pid_file) {
        Ok(pid) => pid,
        Err(e) => {
            return Ok(ServiceStatus::InvalidFile {
                error: format!("{}", e),
            });
        }
    };
    
    // Check if process is running
    match is_process_running(pid)? {
        true => {
            // Further check: is it a zombie?
            if is_zombie_process(pid)? {
                Ok(ServiceStatus::Zombie { pid })
            } else {
                Ok(ServiceStatus::Running { pid })
            }
        }
        false => {
            Ok(ServiceStatus::StaleFile { pid })
        }
    }
}
```

### Type-Safe Usage
```rust
// Now callers can use pattern matching
match get_service_status(&pid_file)? {
    ServiceStatus::Running { pid } => {
        println!("Service is running with PID {}", pid);
        // Can directly use pid as i32
    }
    ServiceStatus::Stopped => {
        println!("Service is stopped");
    }
    ServiceStatus::StaleFile { pid } => {
        println!("Cleaning up stale PID file for PID {}", pid);
        remove_pid_file(&pid_file)?;
    }
    ServiceStatus::InvalidFile { error } => {
        println!("Warning: invalid PID file: {}", error);
        remove_pid_file(&pid_file)?;
    }
    ServiceStatus::Zombie { pid } => {
        println!("Warning: process {} is a zombie", pid);
        // Can decide how to handle zombies
    }
}

// Or use helper methods
let status = get_service_status(&pid_file)?;
if status.is_running() {
    println!("Service is up!");
}

if status.needs_cleanup() {
    cleanup_service(&pid_file)?;
}

if let Some(pid) = status.pid() {
    println!("Associated PID: {}", pid);
}
```

## Additional Benefits

### Serialization Support
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", content = "data")]
pub enum ServiceStatus {
    Running { pid: i32 },
    Stopped,
    StaleFile { pid: i32 },
    InvalidFile { error: String },
    Zombie { pid: i32 },
}

// Now can output JSON for APIs:
// {"status": "Running", "data": {"pid": 12345}}
```

### Better CLI Output
```rust
pub fn format_status_detailed(status: &ServiceStatus) -> String {
    match status {
        ServiceStatus::Running { pid } => {
            format!(
                "● kodegend.service - KODEGEN Daemon\n\
                 Loaded: loaded\n\
                 Active: active (running) since {}\n\
                 Main PID: {}",
                get_process_start_time(*pid).unwrap_or_else(|_| "unknown".into()),
                pid
            )
        }
        ServiceStatus::Stopped => {
            "● kodegend.service - KODEGEN Daemon\n\
             Loaded: loaded\n\
             Active: inactive (dead)".to_string()
        }
        // ... etc
    }
}
```

## Migration Path

### Phase 1: Add enum alongside string
```rust
// Keep old function for compatibility
pub fn get_service_status_string(pid_file: &Path) -> Result<String> {
    Ok(get_service_status(pid_file)?.to_string())
}

// New function returns enum
pub fn get_service_status(pid_file: &Path) -> Result<ServiceStatus> {
    // ... new implementation
}
```

### Phase 2: Update callers
Update all call sites to use the new enum-based API.

### Phase 3: Remove string version
After all callers migrated, remove the string-based function.

## Testing Strategy

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_status_no_file() {
        let temp = tempfile::NamedTempFile::new().unwrap();
        let path = temp.path().to_path_buf();
        drop(temp);  // Delete file
        
        let status = get_service_status(&path).unwrap();
        assert_eq!(status, ServiceStatus::Stopped);
        assert!(!status.is_running());
        assert_eq!(status.pid(), None);
    }
    
    #[test]
    fn test_status_invalid_pid() {
        let mut temp = tempfile::NamedTempFile::new().unwrap();
        write!(temp, "not_a_number").unwrap();
        
        let status = get_service_status(temp.path()).unwrap();
        assert!(matches!(status, ServiceStatus::InvalidFile { .. }));
        assert!(status.needs_cleanup());
    }
    
    #[test]
    fn test_status_running() {
        let mut temp = tempfile::NamedTempFile::new().unwrap();
        let current_pid = std::process::id() as i32;
        write!(temp, "{}", current_pid).unwrap();
        
        let status = get_service_status(temp.path()).unwrap();
        assert_eq!(status, ServiceStatus::Running { pid: current_pid });
        assert!(status.is_running());
        assert_eq!(status.pid(), Some(current_pid));
    }
}
```

## Comparison

### Before
```rust
let status = get_service_status(&pid_file)?;
// status is "running (PID: 12345)" - must parse!
if status.contains("running") {
    // How to get PID? String manipulation...
}
```

### After
```rust
let status = get_service_status(&pid_file)?;
if let ServiceStatus::Running { pid } = status {
    // pid is directly available as i32
    println!("PID: {}", pid);
}
```

## Dependencies
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"], optional = true }
```

## References
- "Effective Rust" - Prefer enums over strings for known states
- Rust API Guidelines - Type safety
- "Parse, don't validate" principle
