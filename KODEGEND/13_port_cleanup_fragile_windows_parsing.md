# Fragile Parsing: Windows netstat Output Processing

## Severity
**HIGH** - Can break on non-English Windows or with netstat format changes

## Location
`packages/kodegend/src/service/port_cleanup.rs`
- Lines 100-118

## Issue Description
Windows netstat output parsing makes several fragile assumptions:

```rust
let stdout = String::from_utf8_lossy(&output.stdout);

// Parse netstat output
// Format: "  TCP    127.0.0.1:30438    0.0.0.0:0    LISTENING       12345"
for line in stdout.lines() {
    if !line.contains("LISTENING") {  // PROBLEM 1: Locale-dependent
        continue;
    }

    if !line.contains(&format!(":{}", port)) {  // PROBLEM 2: Substring match
        continue;
    }

    // Extract PID from last column
    let parts: Vec<&str> = line.split_whitespace().collect();
    if let Some(pid_str) = parts.last() {  // PROBLEM 3: Assumes PID is last
        if let Ok(pid) = pid_str.parse::<u32>() {
            return Ok(Some(pid));
        }
    }
}
```

## Production Issues

### Issue 1: Locale Dependence
On non-English Windows, "LISTENING" might be:
- German: "ABHÖREN"
- French: "ÉCOUTE"
- Spanish: "ESCUCHANDO"
- Chinese: "监听"

**Result**: Function returns `Ok(None)` on non-English systems, even if port is occupied.

### Issue 2: Substring Match False Positives
```rust
if !line.contains(&format!(":{}", port)) {
```

Examples of false positives:
- Looking for port **30**: Matches "127.0.0.1:304**30**:0" (port 30430)
- Looking for port **80**: Matches "192.168.1.**80**:8080" (IP address)
- Looking for port **438**: Matches "127.0.0.1:30**438**" (correct) but also "127.0.0.1:**438**0"

### Issue 3: No Address Validation
The code doesn't verify the address is 127.0.0.1. It could match:
- Remote connections on the same port
- Other local addresses
- IPv6 addresses

### Issue 4: Assumes PID is Last Field
If netstat format changes or includes additional columns, parsing breaks.

### Issue 5: No Protocol Validation
Doesn't verify it's TCP vs UDP.

## Real-World Failure Scenarios

### Scenario 1: German Windows
```
  TCP    127.0.0.1:30438    0.0.0.0:0    ABHÖREN       12345
```
Line doesn't contain "LISTENING" → skipped → returns `Ok(None)` → port cleanup fails

### Scenario 2: Port Number Collision
Looking for port 438:
```
  TCP    127.0.0.1:4380     0.0.0.0:0    LISTENING     12345
```
Substring ":438" matches → returns wrong PID → kills wrong process!

### Scenario 3: Future Windows Version
```
  TCP    127.0.0.1:30438    0.0.0.0:0    LISTENING     12345    SomeNewColumn
```
`parts.last()` returns "SomeNewColumn" → parse fails → returns `Ok(None)`

## Recommended Solutions

### Option 1: Use `-ano` with Locale-Independent Parsing
```rust
async fn find_process_by_port_windows(port: u16) -> Result<Option<u32>> {
    let output = Command::new("netstat")
        .args(["-ano", "-p", "TCP"])  // Specify protocol
        .output()
        .await?;

    if !output.status.success() {
        return Ok(None);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    for line in stdout.lines() {
        // More robust parsing with regex
        // Match: <IP>:<PORT><whitespace><IP>:<PORT><whitespace><STATE><whitespace><PID>
        
        // Simple approach: parse fields by position
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        // Expected format: [Proto, LocalAddr, ForeignAddr, State, PID]
        if parts.len() < 5 {
            continue;
        }
        
        // Check local address matches our port on localhost
        if !parts[1].starts_with("127.0.0.1:") && !parts[1].starts_with("[::1]:") {
            continue;  // Not localhost
        }
        
        // Extract port from local address (format: "127.0.0.1:PORT")
        let local_port = parts[1]
            .split(':')
            .last()
            .and_then(|p| p.parse::<u16>().ok());
        
        if local_port != Some(port) {
            continue;  // Not our port
        }
        
        // State checking (position 3) - accept any state that's not empty
        // This works across locales as we're just checking position
        
        // PID is in position 4
        if let Ok(pid) = parts[4].parse::<u32>() {
            return Ok(Some(pid));
        }
    }

    Ok(None)
}
```

### Option 2: Use PowerShell Instead (Windows-Native)
```rust
async fn find_process_by_port_windows(port: u16) -> Result<Option<u32>> {
    let script = format!(
        "Get-NetTCPConnection -LocalAddress 127.0.0.1 -LocalPort {} -ErrorAction SilentlyContinue | Select-Object -ExpandProperty OwningProcess",
        port
    );
    
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", &script])
        .output()
        .await?;
    
    if !output.status.success() {
        return Ok(None);
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let pid_str = stdout.trim();
    
    if pid_str.is_empty() {
        return Ok(None);
    }
    
    pid_str.parse::<u32>()
        .map(Some)
        .context("Failed to parse PID from PowerShell output")
}
```

Benefits:
- Locale-independent
- Built-in to Windows
- More reliable parsing
- No substring matching issues

### Option 3: Use Windows API Directly (Best but Complex)
Use `windows-rs` crate to call `GetTcpTable2` directly:
- No external commands
- No parsing
- No locale issues
- Fastest

## Recommended Approach
**Option 2 (PowerShell)** for quick fix with high reliability.
**Option 3 (Windows API)** for production-grade solution.

## Files to Modify
- `packages/kodegend/src/service/port_cleanup.rs`
- Potentially `Cargo.toml` (if using windows-rs)

## Testing Considerations
- Test on English Windows
- Test on non-English Windows (German, Japanese, etc.)
- Test with ports that are substrings (30, 438, 80)
- Test with high port numbers (65535)
- Test with no process on port
- Add unit tests with mocked netstat output
