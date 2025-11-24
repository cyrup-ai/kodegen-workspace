# HIGH: Windows Script Injection via UAC Elevation

## Severity: HIGH

## Location
- `packages/kodegend/src/install/privilege.rs:120-177` - Windows script building
- `packages/kodegend/src/install/privilege.rs:318-329` - Script passed to elevated helper
- `packages/kodegend/src/install/privilege.rs:150-177` - Hosts file update

## Issue Description

Similar to the Unix shell injection issue, Windows installation builds batch scripts via string concatenation and passes them to an elevated helper process via `ShellExecuteExW`. If any paths or data contain batch script metacharacters, command injection is possible with administrator privileges.

## Vulnerable Code

### Script Building (Lines 120-136)
```rust
#[cfg(windows)]
{
    use crate::install::installer::windows::paths::{self, InstallScope};

    let install_dir = paths::install_dir(InstallScope::System);

    script.push_str(&paths::mkdir_command(&install_dir));  // LINE 126
    script.push_str("\n");

    for file in &staged_files {
        script.push_str(&paths::copy_file_command(  // LINE 130
            std::path::Path::new(file),
            &install_dir
        ));
        script.push_str("\n");
    }
}
```

### Script Execution (Lines 318-329)
```rust
// Encode script content directly as wide string (helper expects content, not path)
let script_wide: Vec<u16> = script  // Unsanitized script!
    .encode_utf16()
    .chain(std::iter::once(0))
    .collect();

let mut sei = SHELLEXECUTEINFOW {
    lpFile: PCWSTR(helper_path_wide.as_ptr()),
    lpParameters: PCWSTR(script_wide.as_ptr()),  // Attacker-controlled script
    lpVerb: PCWSTR("runas\0".encode_utf16()),    // UAC elevation!
    // ...
};
```

## Windows Batch Script Injection

### Metacharacters in Batch Scripts
```batch
& - Command separator (cmd1 & cmd2)
&& - Conditional AND (cmd1 && cmd2)
|| - Conditional OR (cmd1 || cmd2)
| - Pipe (cmd1 | cmd2)
^ - Escape character
< > - Redirection
( ) - Command grouping
% - Variable expansion
! - Delayed expansion
```

### Attack Vector: Malicious Filename
```batch
# If staged_files contains:
C:\Users\temp\kodegen_install_12345\test & net user hacker P@ss123 /add & .exe

# paths::copy_file_command() generates:
copy "C:\Users\temp\kodegen_install_12345\test & net user hacker P@ss123 /add & .exe" "C:\Program Files\Kodegen\"

# Batch interpreter sees:
1. copy "C:\Users\temp\kodegen_install_12345\test
2. net user hacker P@ss123 /add     # Executes as admin!
3. .exe" "C:\Program Files\Kodegen\"
```

### Attack Vector: Hosts File (Lines 162-170)
```rust
script.push_str(&format!(
    r#"findstr /i /c:"mcp.kodegen.ai" "{}" >nul 2>&1
if errorlevel 1 (
    echo 127.0.0.1 mcp.kodegen.ai >> "{}"
    echo Hosts entry added
) else (
    echo Hosts entry already exists
)
"#,
    hosts_path.display(), hosts_path.display()
));
```

If `hosts_path` contains `"`, injection possible:
```batch
# Malicious hosts_path:
C:\Windows\System32\drivers\etc\hosts" & calc.exe & rem "

# Generated script:
findstr /i /c:"mcp.kodegen.ai" "C:\Windows\System32\drivers\etc\hosts" & calc.exe & rem "" >nul 2>&1
```

## Checking paths Module for Injection Vulnerabilities

Need to examine these helper functions:
- `paths::mkdir_command(&install_dir)` - How is install_dir escaped?
- `paths::copy_file_command(file, dest)` - How are file/dest escaped?
- `paths::delete_file_command(&temp_cert_path)` - How is path escaped?

## Potential Impact

**With administrator privileges (UAC elevation):**
- Create admin accounts
- Install persistent backdoors
- Disable Windows Defender
- Modify system files
- Exfiltrate sensitive data
- Ransomware deployment

## Remediation

### Option 1: Proper Batch Script Escaping

```rust
fn batch_escape(s: &str) -> String {
    // Escape special characters for Windows batch
    s.replace('&', "^&")
        .replace('|', "^|")
        .replace('<', "^<")
        .replace('>', "^>")
        .replace('(', "^(")
        .replace(')', "^)")
        .replace('%', "%%")
        .replace('!', "^^!")
        .replace('"', "\"\"")  // Double quotes
}

fn batch_quote_path(path: &Path) -> String {
    format!("\"{}\"", batch_escape(&path.display().to_string()))
}
```

### Option 2: Use PowerShell with -EncodedCommand

PowerShell's `-EncodedCommand` accepts base64-encoded commands, preventing injection:

```rust
// Build PowerShell script
let ps_script = format!(
    r#"
    Copy-Item -Path '{}' -Destination '{}' -Force
    "#,
    file.display(), install_dir.display()
);

// Base64 encode the script (UTF-16LE)
let script_utf16: Vec<u16> = ps_script.encode_utf16().collect();
let script_bytes: Vec<u8> = script_utf16.iter()
    .flat_map(|&c| c.to_le_bytes())
    .collect();
let encoded_script = base64::encode(&script_bytes);

// Execute safely (no injection possible)
let output = Command::new("powershell.exe")
    .args(["-NoProfile", "-NonInteractive", "-EncodedCommand"])
    .arg(&encoded_script)
    .output()?;
```

### Option 3: Avoid Script Generation (Best)

Use Windows APIs directly instead of batch scripts:

```rust
// Instead of script, use Windows APIs
use windows::Win32::Storage::FileSystem::CopyFileW;

for file in &staged_files {
    let src_wide: Vec<u16> = file.encode_utf16().chain(once(0)).collect();
    let dst_path = install_dir.join(
        Path::new(file).file_name().unwrap()
    );
    let dst_wide: Vec<u16> = dst_path
        .to_str()
        .unwrap()
        .encode_utf16()
        .chain(once(0))
        .collect();

    unsafe {
        CopyFileW(
            PCWSTR(src_wide.as_ptr()),
            PCWSTR(dst_wide.as_ptr()),
            false  // Overwrite existing
        )?;
    }
}
```

## Testing

```rust
#[test]
#[cfg(windows)]
fn test_batch_injection_protection() {
    // Malicious filename with batch metacharacters
    let malicious = r#"test & calc.exe & rem .exe"#;
    let escaped = batch_quote_path(Path::new(malicious));

    // Should be fully quoted and escaped
    assert!(escaped.starts_with('"'));
    assert!(escaped.ends_with('"'));
    assert!(!escaped.contains("& calc.exe &"));  // Not executable
}
```

## References
- Microsoft Docs: Batch File Command Injection
- CWE-78: OS Command Injection
- Windows Batch Script Security
- UAC Bypass via Script Injection
