# MEDIUM: Incomplete Cleanup on Installation Failure

## Severity: MEDIUM

## Locations
- `packages/kodegend/src/install/privilege.rs:393-395` - Cleanup after success only
- `packages/kodegend/src/install/orchestration.rs:184-219` - No cleanup on Chromium failure
- `packages/kodegend/src/install/binary_staging.rs:22-48` - No cleanup on copy errors

## Issue Description

The installer creates temporary directories, downloads files, and performs privileged operations, but doesn't properly clean up on failure. This can leave:
- Sensitive data in temp directories
- Partial installations
- Certificate files with private keys
- Downloaded binaries

## Vulnerable Code

### 1. Staging Directory Cleanup (privilege.rs:393-395)
```rust
// Cleanup staging directory
std::fs::remove_dir_all(staging_dir)
    .with_context(|| format!("Failed to cleanup staging directory: {}", staging_dir.display()))?;

Ok(())  // Only reached if all privileged operations succeed
```

**Problem**: If ANY privileged operation fails (lines 280-391), staging directory is never cleaned up.

### 2. Chromium Installation Failure (orchestration.rs:198-220)
```rust
match chromium::install_chromium().await {
    Ok(chromium_path) => {
        // Success path...
    }
    Err(e) => {
        // Chromium is REQUIRED - fail installation
        pb_overall.set_message("Chromium installation FAILED");
        pb_overall.finish_and_clear();
        pb_download.finish_and_clear();

        // ERROR: Doesn't clean up:
        // - Downloaded binaries
        // - Staging directory
        // - Certificate files
        // - Partially installed services

        return Err(e);
    }
}
```

### 3. Binary Staging Copy Errors (binary_staging.rs:37-40)
```rust
fs::copy(binary_path, &dest_path).with_context(|| {
    format!("Failed to copy {} to staging: {}", binary_path.display(), dest_path.display())
})?;
```

**Problem**: If copy fails mid-way through binary list, staging dir contains partial binaries but is never cleaned up.

## Security Impact

### 1. Information Disclosure
Temp directories may contain:
```
/tmp/kodegen_install_12345/
├── kodegen (downloaded binary)
├── kodegend (downloaded binary)
└── ...

/tmp/kodegen_cert_import_12345.crt (contains private key!)
```

### 2. Disk Space Exhaustion
Repeated failed installations accumulate:
```bash
/tmp/kodegen_install_12340/  # 50 MB
/tmp/kodegen_install_12341/  # 50 MB
/tmp/kodegen_install_12342/  # 50 MB
# ... user retries installation many times
# Hundreds of MB in /tmp never cleaned up
```

### 3. Private Key Exposure
If certificate generation succeeds but installation fails:
```
/tmp/kodegen_cert_import_12345.crt
# Contains:
# -----BEGIN CERTIFICATE-----
# ...
# -----END CERTIFICATE-----
# -----BEGIN PRIVATE KEY-----  ← SENSITIVE!
# ...
# -----END PRIVATE KEY-----
```

File sits in `/tmp` with world-readable permissions (before chmod on line 215).

## Failure Scenarios Leaving Artifacts

### Scenario 1: User Cancels UAC Prompt
```rust
// privilege.rs:349-352
if error_code.0 == 1223 {
    return Err(anyhow::anyhow!(
        "UAC elevation cancelled by user..."
    ));
}
```

Leaves behind:
- Downloaded binaries in tempfile::tempdir
- Staging directory with binaries
- Certificate file in /tmp

### Scenario 2: Network Failure During Chromium Download
```rust
// chromium.rs (assumed to exist)
chromium::install_chromium().await?;  // Fails
```

Leaves behind:
- All downloaded binaries
- Staged binaries
- Generated certificate
- Possibly partial Chromium download

### Scenario 3: Insufficient Disk Space
```rust
// binary_staging.rs:37
fs::copy(binary_path, &dest_path)?;  // Fails on 3rd binary
```

Leaves behind:
- Staging directory with 2 successfully copied binaries
- Original download directory intact

## Remediation

### Use RAII Cleanup Guards

```rust
use scopeguard::defer;

pub async fn install_with_elevated_privileges(
    staging_dir: &std::path::Path,
    cert_content: Option<&str>,
    data_dir: &std::path::Path,
) -> Result<()> {
    // RAII guard: cleanup staging_dir on ANY exit (success or failure)
    let staging_dir_cleanup = scopeguard::guard(staging_dir.to_path_buf(), |dir| {
        if let Err(e) = std::fs::remove_dir_all(&dir) {
            log::warn!("Failed to cleanup staging directory: {}", e);
        }
    });

    // RAII guard: cleanup temp cert file
    let temp_cert_cleanup = if let Some(_) = cert_content {
        let temp_path = get_temp_cert_path();
        Some(scopeguard::guard(temp_path.clone(), |path| {
            if let Err(e) = std::fs::remove_file(&path) {
                log::warn!("Failed to cleanup temp certificate: {}", e);
            }
        }))
    } else {
        None
    };

    // ... perform installation ...
    // Cleanup happens automatically on ANY return (Ok or Err)

    // Defuse guard only on complete success
    scopeguard::ScopeGuard::into_inner(staging_dir_cleanup);
    if let Some(guard) = temp_cert_cleanup {
        scopeguard::ScopeGuard::into_inner(guard);
    }

    Ok(())
}
```

### Centralized Cleanup Function

```rust
struct InstallationContext {
    staging_dir: Option<PathBuf>,
    temp_cert: Option<PathBuf>,
    downloaded_binaries: Option<PathBuf>,
    partial_service: bool,
}

impl Drop for InstallationContext {
    fn drop(&mut self) {
        // Always cleanup on drop (even on panic)
        if let Some(dir) = &self.staging_dir {
            let _ = std::fs::remove_dir_all(dir);
        }
        if let Some(cert) = &self.temp_cert {
            let _ = std::fs::remove_file(cert);
        }
        if let Some(bins) = &self.downloaded_binaries {
            let _ = std::fs::remove_dir_all(bins);
        }
        if self.partial_service {
            // Uninstall partial service
            let _ = self.cleanup_partial_service();
        }
    }
}

pub async fn run_install_with_options(options: &wizard::InstallOptions, cli: &Cli) -> Result<()> {
    let mut ctx = InstallationContext::default();

    // Set paths as they're created
    ctx.downloaded_binaries = Some(output_dir_guard.path().to_path_buf());
    ctx.staging_dir = Some(staging_dir);
    ctx.temp_cert = Some(temp_cert_path);

    // On success, clear context to prevent cleanup
    ctx.staging_dir = None;  // Don't cleanup on success
    ctx.temp_cert = None;

    Ok(())
}
```

### Explicit Cleanup in Error Paths

```rust
match chromium::install_chromium().await {
    Ok(chromium_path) => { /* success */ }
    Err(e) => {
        // CLEANUP BEFORE RETURNING ERROR
        let _ = cleanup_partial_installation(&ctx);
        return Err(e);
    }
}

fn cleanup_partial_installation(ctx: &InstallContext) {
    // Remove staging directory
    if let Some(staging_dir) = &ctx.staging_dir {
        if let Err(e) = std::fs::remove_dir_all(staging_dir) {
            log::error!("Failed to cleanup staging directory: {}", e);
        }
    }

    // Remove temp certificate
    if let Some(temp_cert) = &ctx.temp_cert {
        if let Err(e) = std::fs::remove_file(temp_cert) {
            log::error!("Failed to cleanup temp certificate: {}", e);
        }
    }

    // Remove downloaded binaries
    if let Some(download_dir) = &ctx.download_dir {
        if let Err(e) = std::fs::remove_dir_all(download_dir) {
            log::error!("Failed to cleanup downloads: {}", e);
        }
    }

    // Uninstall partial service if registered
    if ctx.service_registered {
        let _ = uninstall_service();
    }
}
```

## Testing

```rust
#[tokio::test]
async fn test_cleanup_on_failure() {
    let staging_dir = std::env::temp_dir().join("test_staging");
    std::fs::create_dir_all(&staging_dir).unwrap();
    std::fs::write(staging_dir.join("test.bin"), b"test").unwrap();

    // Simulate installation failure
    let result = run_install_with_injected_failure(&staging_dir).await;
    assert!(result.is_err());

    // Verify cleanup happened
    assert!(!staging_dir.exists(), "Staging directory should be cleaned up on failure");
}
```

## References
- CWE-459: Incomplete Cleanup
- OWASP: Sensitive Data Exposure
- Rust RAII Pattern Best Practices
