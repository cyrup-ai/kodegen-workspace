# CRITICAL: Windows Helper Signature Verification is Broken

## Severity: CRITICAL

## Location
- `packages/kodegend/src/install/installer/windows/privileges.rs:76` - Signature verification call
- `packages/kodegend/src/install/installer/windows/privileges.rs:90` - References non-existent module
- `packages/kodegend/src/install/build/signing.rs` - macOS-only signing (wrong platform)

## Issue Description

The Windows privilege escalation helper attempts to verify its embedded executable signature by calling `crate::signing::verify_signature()`. However:

1. **The signing module doesn't exist at crate root** for Windows builds
2. **The only signing module is macOS-specific** (`install/build/signing.rs`)
3. **This code likely doesn't compile** on Windows, or uses wrong verification
4. **If stubbed out, malicious helper executes with UAC elevation**

## Vulnerable Code

```rust
// windows/privileges.rs:87-94
fn verify_helper_signature(helper_path: &Path) -> Result<(), InstallerError> {
    // Use the signing module to verify the helper
    crate::signing::verify_signature(helper_path).map_err(|e| {  // LINE 90
        InstallerError::System(format!("Helper signature verification failed: {}", e))
    })?;
    Ok(())
}
```

**Problem**: `crate::signing::verify_signature` doesn't exist for Windows or calls wrong function.

## What Should Happen

Windows helper signature verification should:
1. Use Windows Authenticode API to verify embedded signature
2. Check certificate chain against trusted roots
3. Validate certificate hasn't expired
4. Ensure binary hasn't been tampered with

## What Actually Happens (Most Likely)

### Scenario 1: Compilation Failure
```
error[E0433]: failed to resolve: could not find `signing` in the crate root
  --> src/install/installer/windows/privileges.rs:90:11
   |
90 |     crate::signing::verify_signature(helper_path).map_err(|e| {
   |           ^^^^^^^ could not find `signing` in the crate root
```

### Scenario 2: Wrong Function Called
If `signing` module is somehow available:
```rust
// install/build/signing.rs:112 (macOS-specific)
fn verify_signature(app_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("codesign")  // macOS command!
        .args(["--verify", "--deep", "--strict"])
        .arg(app_path)
        .output()?;
    // ...
}
```
This fails on Windows (no `codesign` command) and returns error.

### Scenario 3: Verification Stubbed Out
Developer may have added conditional compilation stub:
```rust
#[cfg(windows)]
fn verify_signature(_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Implement Windows signature verification
    Ok(())  // ALWAYS PASSES!
}
```

## Attack Vector

If signature verification is broken/bypassed:

### Build-Time Attack
Attacker modifies build process to embed malicious helper:
```rust
// Build script injects malicious exe
const HELPER_EXE_DATA: &[u8] = include_bytes!("malicious_helper.exe");
```

### Runtime Attack (Combined with TOCTOU)
1. Attacker predicts helper path: `/tmp/KodegenHelper_12345.exe`
2. Pre-creates malicious helper at that path
3. `ensure_helper_path()` writes embedded data, but attacker's file already there
4. Signature verification runs on attacker's malicious file
5. If verification broken, malicious helper passes
6. Helper executes with UAC elevation (full admin privileges)

## Execution Flow Leading to Compromise

```rust
// privilege.rs:298-301 - Spawns unverified helper
tokio::task::spawn_blocking(|| ensure_helper_path())
    .await?
    .context("Failed to extract Windows helper executable")?;

// Lines 310-344 - Passes script to helper via ShellExecuteExW
let mut sei = SHELLEXECUTEINFOW {
    lpFile: PCWSTR(helper_path_wide.as_ptr()),
    lpParameters: PCWSTR(script_wide.as_ptr()),  // Attacker-controlled script
    lpVerb: PCWSTR("runas\0".encode_utf16()),    // UAC elevation!
    // ...
};

unsafe { ShellExecuteExW(&mut sei) }  // Executes malicious helper as admin
```

## Proper Windows Signature Verification

### Using WinVerifyTrust API

```rust
#[cfg(windows)]
fn verify_helper_signature(helper_path: &Path) -> Result<(), InstallerError> {
    use windows::Win32::Security::WinTrust::{
        WinVerifyTrust, WINTRUST_DATA, WINTRUST_FILE_INFO,
        WTD_UI_NONE, WTD_REVOKE_WHOLECHAIN, WTD_CHOICE_FILE,
        WINTRUST_ACTION_GENERIC_VERIFY_V2,
    };

    let file_path: Vec<u16> = helper_path
        .to_str()
        .ok_or(InstallerError::System("Invalid path".into()))?
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();

    let mut file_info = WINTRUST_FILE_INFO {
        cbStruct: std::mem::size_of::<WINTRUST_FILE_INFO>() as u32,
        pcwszFilePath: PCWSTR(file_path.as_ptr()),
        hFile: None,
        pgKnownSubject: std::ptr::null(),
    };

    let mut trust_data = WINTRUST_DATA {
        cbStruct: std::mem::size_of::<WINTRUST_DATA>() as u32,
        dwUIChoice: WTD_UI_NONE,  // No UI prompts
        fdwRevocationChecks: WTD_REVOKE_WHOLECHAIN,
        dwUnionChoice: WTD_CHOICE_FILE,
        pFile: &mut file_info as *mut _,
        // ... other fields
    };

    unsafe {
        let status = WinVerifyTrust(
            None,  // Use default verification
            &WINTRUST_ACTION_GENERIC_VERIFY_V2,
            &mut trust_data as *mut _ as *mut _,
        );

        if status != 0 {
            return Err(InstallerError::System(format!(
                "Helper signature verification failed. Status: 0x{:08X}. \
                 Binary may be unsigned, tampered with, or from untrusted publisher.",
                status
            )));
        }
    }

    // Additionally verify publisher certificate matches expected
    verify_publisher_certificate(helper_path)?;

    Ok(())
}

fn verify_publisher_certificate(helper_path: &Path) -> Result<(), InstallerError> {
    // Extract certificate from signature
    // Verify subject matches "Kodegen" or expected publisher
    // Verify certificate chain to trusted root
    // Verify certificate hasn't been revoked
    // TODO: Implement using CryptQueryObject API
    Ok(())
}
```

### Using certutil Command (Simpler but less robust)

```rust
#[cfg(windows)]
fn verify_helper_signature(helper_path: &Path) -> Result<(), InstallerError> {
    use std::process::Command;

    // Verify signature using certutil
    let output = Command::new("certutil")
        .args(["-verify", "-v"])
        .arg(helper_path)
        .output()
        .map_err(|e| InstallerError::System(format!("certutil failed: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(InstallerError::System(format!(
            "Helper signature invalid: {}",
            stderr
        )));
    }

    // Parse output to verify publisher
    let stdout = String::from_utf8_lossy(&output.stdout);
    if !stdout.contains("Kodegen") {
        return Err(InstallerError::System(
            "Helper signed by unknown publisher".into()
        ));
    }

    Ok(())
}
```

## Immediate Action Required

1. **Verify if this code compiles on Windows** - check CI/CD logs
2. **If it doesn't compile**, fix compilation before next release
3. **If it does compile**, check what `crate::signing` resolves to
4. **Add integration test** that actually runs signature verification
5. **Implement proper WinVerifyTrust** before next Windows release

## Testing

```rust
#[test]
#[cfg(windows)]
fn test_helper_signature_verification() {
    // Extract helper to temp path
    let temp = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(temp.path(), HELPER_EXE_DATA).unwrap();

    // Verification should succeed for embedded helper
    assert!(verify_helper_signature(temp.path()).is_ok());

    // Verification should fail for unsigned binary
    let unsigned = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(unsigned.path(), b"fake exe").unwrap();
    assert!(verify_helper_signature(unsigned.path()).is_err());
}
```

## References
- Microsoft Docs: WinVerifyTrust function
- Microsoft Docs: Authenticode Digital Signatures
- CWE-494: Download of Code Without Integrity Check
- CWE-353: Missing Support for Integrity Check
