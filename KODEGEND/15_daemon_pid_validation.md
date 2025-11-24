# Edge Case: Missing PID Range and Format Validation

## Severity: LOW

## Location
`packages/kodegend/src/daemon.rs:105-112`

## Issue Description
The `read_pid_file()` function parses the PID but doesn't validate it's in a valid range or has a reasonable value.

## Current Implementation
```rust
pub fn read_pid_file(path: &Path) -> Result<i32> {
    let pid_str = fs::read_to_string(path)
        .with_context(|| format!("Failed to read PID file at {}", path.display()))?;
    
    let pid = pid_str.trim().parse::<i32>()
        .map_err(|_| anyhow!("Invalid PID in file"))?;
    
    Ok(pid)  // No validation!
}
```

## Problems

### 1. Negative PIDs Accepted
```rust
// PID file contains: "-123"
let pid = read_pid_file(path)?;  // Returns -123
is_process_running(pid)?;         // Behavior undefined!
```

PIDs should always be positive integers. Negative values are:
- Invalid on all platforms
- May cause undefined behavior in syscalls
- Sign of corrupted PID file

### 2. Zero PID Accepted
```rust
// PID file contains: "0"
let pid = read_pid_file(path)?;  // Returns 0
```

PID 0 is special:
- On Unix: represents the scheduler/swapper
- Should never be in a PID file
- `kill(0, signal)` has special meaning (sends to process group)

### 3. No Maximum Value Check
```rust
// PID file contains: "2147483647"
let pid = read_pid_file(path)?;  // i32::MAX
```

While technically valid, PIDs typically have much lower limits:
- Linux: default max PID is 32768 (can be configured up to 4194304)
- macOS: max PID is typically 99999
- Windows: uses u32, but practical limit is lower

### 4. Whitespace Not Fully Handled
```rust
// PID file contains: "  123  \n\n"
let pid = pid_str.trim().parse()?;  // Works, but...
```

While `trim()` handles leading/trailing whitespace, it doesn't:
- Reject embedded whitespace: "12 34"
- Reject multiple values: "123\n456"
- Validate file format

## Recommended Solution

### Add Comprehensive Validation
```rust
pub fn read_pid_file(path: &Path) -> Result<i32> {
    let pid_str = fs::read_to_string(path)
        .with_context(|| format!("Failed to read PID file at {}", path.display()))?;
    
    // Validate file isn't empty
    if pid_str.trim().is_empty() {
        return Err(anyhow!(
            "PID file {} is empty",
            path.display()
        ));
    }
    
    // Validate file contains only one line
    let lines: Vec<&str> = pid_str.lines().filter(|l| !l.trim().is_empty()).collect();
    if lines.len() != 1 {
        return Err(anyhow!(
            "PID file {} should contain exactly one line, found {}",
            path.display(),
            lines.len()
        ));
    }
    
    let trimmed = pid_str.trim();
    
    // Validate format: only digits (and optional sign)
    if !trimmed.chars().all(|c| c.is_ascii_digit() || c == '-' || c == '+') {
        return Err(anyhow!(
            "PID file {} contains invalid characters: {:?}",
            path.display(),
            trimmed
        ));
    }
    
    // Parse as integer
    let pid = trimmed.parse::<i32>()
        .map_err(|e| anyhow!(
            "Invalid PID in file {}: {:?} ({})",
            path.display(),
            trimmed,
            e
        ))?;
    
    // Validate PID is positive
    if pid <= 0 {
        return Err(anyhow!(
            "PID must be positive, got {} in file {}",
            pid,
            path.display()
        ));
    }
    
    // Validate PID is within reasonable range
    // Linux default max: 32768, but can be configured higher
    // Be conservative: allow up to 4194304 (Linux's absolute max)
    const MAX_REASONABLE_PID: i32 = 4_194_304;
    if pid > MAX_REASONABLE_PID {
        return Err(anyhow!(
            "PID {} in file {} exceeds maximum reasonable value {}",
            pid,
            path.display(),
            MAX_REASONABLE_PID
        ));
    }
    
    Ok(pid)
}
```

### Platform-Specific Maximum Values
```rust
#[cfg(target_os = "linux")]
const MAX_PID: i32 = 4_194_304;  // /proc/sys/kernel/pid_max absolute maximum

#[cfg(target_os = "macos")]
const MAX_PID: i32 = 99_999;      // kern.maxproc typical value

#[cfg(target_os = "freebsd")]
const MAX_PID: i32 = 99_999;      // kern.pid_max

#[cfg(windows)]
const MAX_PID: u32 = u32::MAX;    // Windows uses full u32 range

pub fn validate_pid_range(pid: i32) -> Result<()> {
    if pid <= 0 {
        return Err(anyhow!("PID must be positive, got {}", pid));
    }
    
    if pid > MAX_PID {
        return Err(anyhow!(
            "PID {} exceeds platform maximum {}",
            pid,
            MAX_PID
        ));
    }
    
    Ok(())
}
```

### Enhanced Validation with System Check
```rust
pub fn read_pid_file_validated(path: &Path) -> Result<i32> {
    let pid = read_pid_file(path)?;
    
    // Validate range
    validate_pid_range(pid)?;
    
    // On Linux, can check actual system max
    #[cfg(target_os = "linux")]
    {
        if let Ok(max_pid_str) = std::fs::read_to_string("/proc/sys/kernel/pid_max") {
            if let Ok(max_pid) = max_pid_str.trim().parse::<i32>() {
                if pid > max_pid {
                    return Err(anyhow!(
                        "PID {} exceeds system maximum {} (from /proc/sys/kernel/pid_max)",
                        pid,
                        max_pid
                    ));
                }
            }
        }
    }
    
    Ok(pid)
}
```

## Edge Cases to Handle

### Malformed Content Examples
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_rejects_negative_pid() {
        let mut file = tempfile::NamedTempFile::new().unwrap();
        write!(file, "-123").unwrap();
        assert!(read_pid_file(file.path()).is_err());
    }
    
    #[test]
    fn test_rejects_zero_pid() {
        let mut file = tempfile::NamedTempFile::new().unwrap();
        write!(file, "0").unwrap();
        assert!(read_pid_file(file.path()).is_err());
    }
    
    #[test]
    fn test_rejects_multiple_pids() {
        let mut file = tempfile::NamedTempFile::new().unwrap();
        write!(file, "123\n456").unwrap();
        assert!(read_pid_file(file.path()).is_err());
    }
    
    #[test]
    fn test_rejects_non_numeric() {
        let mut file = tempfile::NamedTempFile::new().unwrap();
        write!(file, "abc").unwrap();
        assert!(read_pid_file(file.path()).is_err());
    }
    
    #[test]
    fn test_rejects_embedded_whitespace() {
        let mut file = tempfile::NamedTempFile::new().unwrap();
        write!(file, "12 34").unwrap();
        assert!(read_pid_file(file.path()).is_err());
    }
    
    #[test]
    fn test_rejects_too_large_pid() {
        let mut file = tempfile::NamedTempFile::new().unwrap();
        write!(file, "999999999").unwrap();
        assert!(read_pid_file(file.path()).is_err());
    }
    
    #[test]
    fn test_accepts_valid_pid() {
        let mut file = tempfile::NamedTempFile::new().unwrap();
        write!(file, "12345").unwrap();
        assert_eq!(read_pid_file(file.path()).unwrap(), 12345);
    }
    
    #[test]
    fn test_accepts_pid_with_whitespace() {
        let mut file = tempfile::NamedTempFile::new().unwrap();
        write!(file, "  12345  \n").unwrap();
        assert_eq!(read_pid_file(file.path()).unwrap(), 12345);
    }
}
```

## Real-World Attack Scenarios

### Scenario 1: Integer Overflow Attack
```bash
# Attacker creates malicious PID file
$ echo "2147483648" > /var/run/kodegend.pid  # i32::MAX + 1
$ kodegend stop
# Parse fails or wraps to negative value
```

### Scenario 2: Special PID Attack
```bash
# Attacker writes PID 0
$ echo "0" > /var/run/kodegend.pid
$ kodegend stop
# kill(0, SIGTERM) sends signal to entire process group!
```

### Scenario 3: Multiple PID Confusion
```bash
# Multiple PIDs in file
$ echo -e "1234\n5678" > /var/run/kodegend.pid
$ kodegend stop
# Which PID gets killed? First? Last? Both?
```

## Benefits of Validation

### Security
- Prevents signal sending to unexpected processes
- Prevents integer overflow attacks
- Detects corrupted PID files early

### Robustness
- Clear error messages for malformed files
- Catches file corruption early
- Prevents undefined behavior

### Debugging
- Better error messages show exact problem
- Validation helps identify how file got corrupted

## Performance Impact
- Minimal: few extra checks during PID file read
- Only happens during daemon start/stop/status (not hot path)
- String validation is O(n) where n = file size (typically < 10 bytes)

## Implementation Priority
**Low** - While good practice, unlikely to cause issues in practice since:
- PID files are typically machine-generated
- Operating system limits are much higher than practical PIDs
- Most corruption would fail integer parsing anyway

But should still be fixed for:
- Defense in depth
- Better error messages
- Catching edge cases early

## References
- POSIX: PIDs are positive integers
- Linux: /proc/sys/kernel/pid_max documentation
- "Secure Coding in C and C++" - Input validation chapter
