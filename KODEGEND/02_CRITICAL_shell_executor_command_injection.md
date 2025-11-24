# CRITICAL: Incomplete Shell Command Injection Protection

## Priority: CRITICAL
## Category: Security Vulnerability
## File: `packages/kodegend/src/security/shell_executor.rs`

## Issue Description

The `ShellExecutor` attempts to block dangerous shell commands but the regex patterns are incomplete and easily bypassed. Multiple command injection vectors are not blocked, allowing arbitrary command execution.

## Location

- **File**: `packages/kodegend/src/security/shell_executor.rs`
- **Lines**: 38-87 (validation patterns and logic)

## Vulnerabilities

### 1. Missing Command Chaining Operators (HIGH SEVERITY)

The validator blocks backticks and `$()` but misses basic command chaining:

```bash
# All of these bypass the validator:
safe_command; rm -rf /home
safe_command && malicious_command
safe_command || malicious_command
safe_command | malicious_command
safe_command & background_malicious &
```

### 2. Insufficient rm Protection (HIGH SEVERITY)

Lines 41-46 only block `rm -rf /` and `rm -rf /*`:

```rust
// Only matches root directory deletion
if let Ok(pattern) = Regex::new(r"rm\s+(-[rfRF]*\s+)*/*\s*$") {
    blocked_patterns.push(pattern);
}
```

**Easily bypassed:**
```bash
rm -rf /etc          # Blocks critical system files
rm -rf /home         # Deletes user data
rm -rf /usr          # Breaks system
rm -rf .             # Deletes current directory
rm -rf ..            # Deletes parent directory
rm -rf "$HOME"       # Variable expansion bypass
rm -rf "/"           # Quote bypass
```

### 3. Missing Redirect Protection (MEDIUM SEVERITY)

No blocking of file redirects which can overwrite critical files:

```bash
cat malicious > /etc/passwd
echo "attacker_key" >> ~/.ssh/authorized_keys
command 2>&1 > /dev/null  # Hide evidence
```

### 4. Null Byte and Special Character Injection

No validation against:
- Null bytes (`\0`) which can truncate commands
- CRLF injection (`\r\n`)
- Unicode exploitation
- Newlines within the command string

## Risk Assessment

**Severity**: CRITICAL  
**Exploitability**: HIGH (trivial to bypass)  
**Impact**: CRITICAL (arbitrary command execution, system compromise)

If this executor is exposed to untrusted input (e.g., from MCP clients, web requests, or user configuration), an attacker can:
1. Execute arbitrary commands
2. Delete critical system files
3. Exfiltrate data
4. Establish persistence
5. Pivot to other systems

## Recommended Solutions

### Option 1: Use Whitelist Approach (Safest)

Only allow specific, pre-approved commands:

```rust
impl ShellExecutor {
    pub fn new_with_whitelist(allowed_commands: Vec<String>) -> Self {
        Self {
            timeout_duration: Duration::from_secs(30),
            blocked_patterns: Vec::new(),  // Don't rely on blocklist
            allowed_commands: Some(allowed_commands),
        }
    }
    
    fn validate_command(&self, cmd: &str) -> Result<(), String> {
        // Parse command to get the base executable
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.is_empty() {
            return Err("Empty command".to_string());
        }
        
        let base_cmd = parts[0];
        
        // ONLY allow whitelisted commands
        if let Some(allowed) = &self.allowed_commands {
            if !allowed.contains(&base_cmd.to_string()) {
                return Err(format!("Command not in whitelist: {}", base_cmd));
            }
        } else {
            return Err("No whitelist configured - rejecting all commands".to_string());
        }
        
        // Check for command chaining attempts
        let dangerous_chars = [';', '|', '&', '\n', '\r', '\0', '>', '<', '`'];
        if cmd.chars().any(|c| dangerous_chars.contains(&c)) {
            return Err("Command contains dangerous characters".to_string());
        }
        
        // Check for command substitution
        if cmd.contains("$(") || cmd.contains("${") {
            return Err("Command substitution not allowed".to_string());
        }
        
        Ok(())
    }
}
```

### Option 2: Use Parsed Arguments (Better)

Don't use `sh -c` at all. Instead, parse and pass arguments directly:

```rust
pub async fn execute_safe(&self, program: &str, args: &[&str]) -> ShellExecuteResponse {
    // Validate program is in whitelist
    if !self.is_allowed_program(program) {
        return ShellExecuteResponse {
            stdout: String::new(),
            stderr: format!("Program not allowed: {}", program),
            exit_code: Some(1),
            is_error: true,
        };
    }
    
    // Execute without shell - no injection possible!
    let child = tokio::process::Command::new(program)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();
    
    // ... rest of execution
}
```

### Option 3: Enhanced Blocklist (Least Recommended)

If you must use a blocklist, it needs to be comprehensive:

```rust
fn create_blocked_patterns() -> Vec<Regex> {
    let patterns = vec![
        // Command chaining
        r";",                              // Command separator
        r"\|\|",                           // OR operator
        r"&&",                             // AND operator  
        r"\|",                             // Pipe
        r"&\s*$",                          // Background execution
        
        // Redirects
        r">",                              // Output redirect
        r"<",                              // Input redirect
        r">>",                             // Append redirect
        
        // Command substitution
        r"`[^`]*`",                        // Backticks
        r"\$\([^)]*\)",                    // Command substitution
        r"\$\{[^}]*\}",                    // Variable expansion
        
        // Dangerous commands (base)
        r"^\s*rm\s+",                      // ANY rm command
        r"^\s*dd\s+",                      // Disk operations
        r"^\s*mkfs",                       // Format filesystems
        r"^\s*kill",                       // Process killing
        r"^\s*sudo\s+",                    // Privilege escalation
        r"^\s*su\s+",                      // User switching
        
        // Special characters
        r"[\x00-\x08\x0B-\x0C\x0E-\x1F]", // Control characters
        r"\r\n|\r|\n",                     // Line breaks
    ];
    
    patterns.iter()
        .filter_map(|p| Regex::new(p).ok())
        .collect()
}
```

## Action Items

1. **Immediate**: Add validation for `;`, `|`, `&&`, `||`, `&`
2. **Immediate**: Block ALL `rm` commands or restrict to specific safe paths
3. **Short-term**: Implement whitelist approach
4. **Long-term**: Refactor to not use `sh -c` at all

## Testing

Create tests for bypass attempts:

```rust
#[test]
fn test_command_injection_blocked() {
    let executor = ShellExecutor::new();
    
    let injections = vec![
        "ls; rm -rf /home",
        "ls && malicious",
        "ls | grep secret",
        "ls || rm file",
        "ls & background &",
        "echo > /etc/passwd",
        "cat `whoami`",
        "echo $(malicious)",
    ];
    
    for injection in injections {
        assert!(executor.validate_command(injection).is_err(),
                "Failed to block: {}", injection);
    }
}
```
