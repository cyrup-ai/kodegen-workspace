# CRITICAL: Predictable Temporary Paths (TOCTOU Race Conditions)

## Severity: CRITICAL

## Locations
1. `packages/kodegend/src/install/binary_staging.rs:20` - Staging directory
2. `packages/kodegend/src/install/privilege.rs:199` - Certificate temp file
3. `packages/kodegend/src/install/installer/config/certificates.rs:288` - macOS cert import
4. `packages/kodegend/src/install/installer/config/certificates.rs:387` - Windows cert import
5. `packages/kodegend/src/install/installer/windows/privileges.rs:67` - Windows helper exe

## Issue Description

The installer uses `std::process::id()` to generate "unique" temporary paths. Process IDs are:
- **Predictable**: Sequential, incremental
- **Low entropy**: Limited range (typically 1-65535)
- **Reused quickly**: OS recycles PIDs

This creates **Time-of-Check-Time-of-Use (TOCTOU)** race conditions where an attacker can pre-create malicious files/directories at predicted paths.

## Vulnerable Code

### 1. Binary Staging Directory (binary_staging.rs:20)
```rust
let staging_dir = std::env::temp_dir()
    .join(format!("kodegen_install_{}", std::process::id()));

fs::create_dir_all(&staging_dir)?;  // Fails if already exists
```

**Attack**: Attacker creates `/tmp/kodegen_install_12346` (predicted next PID) with malicious binaries inside. Installer copies these to system as root.

### 2. Certificate Temporary Files (privilege.rs:199)
```rust
let temp_cert_path = std::path::PathBuf::from(
    format!("/tmp/kodegen_cert_import_{}.crt", std::process::id())
);

tokio::fs::write(&temp_cert_path, cert_only).await?;  // Overwrites if exists!
```

**Attack**: Attacker creates symlink `/tmp/kodegen_cert_import_12346.crt -> /etc/shadow`. Installer overwrites `/etc/shadow` with certificate data.

### 3. Windows Helper (windows/privileges.rs:67)
```rust
let helper_name = format!("KodegenHelper_{}.exe", std::process::id());
let helper_path = temp_dir.join(helper_name);

std::fs::write(&helper_path, HELPER_EXE_DATA)?;  // Overwrites if exists!
```

**Attack**: Attacker pre-creates malicious helper at predicted path. Signature verification runs on attacker's file instead of embedded helper.

## Attack Scenario: Complete Compromise

```bash
#!/bin/bash
# Attacker runs this before installer starts

# Predict next PIDs (installer likely to get one of these)
for pid in {12340..12400}; do
  # Pre-create staging directory with backdoored binary
  mkdir -p "/tmp/kodegen_install_$pid"
  cp /tmp/malicious_kodegen "/tmp/kodegen_install_$pid/kodegen"

  # Pre-create cert symlink to overwrite sensitive file
  ln -sf /root/.ssh/authorized_keys "/tmp/kodegen_cert_import_$pid.crt"

  # Pre-create malicious Windows helper
  cp /tmp/malicious_helper.exe "/tmp/KodegenHelper_$pid.exe"
done

# Wait for installer to run...
# One of these PIDs will match, attacker wins
```

## PID Prediction Techniques

### Linux/macOS
PIDs are sequential and predictable:
```bash
# Get current max PID
cat /proc/sys/kernel/pid_max  # Usually 32768 or 65536

# Spawn processes until PID wraps
while true; do
  sleep 0.001 &
done

# Now can predict next available PIDs with high accuracy
```

### Windows
Similar predictability, PIDs increment by 4:
```powershell
# Current PIDs
Get-Process | Select-Object Id | Sort-Object Id

# Next available likely: max + 4
```

## Real-World Exploit Timeline

1. **T-10s**: Attacker starts PID prediction/pre-creation script
2. **T-0s**: User runs installer (gets PID 12345)
3. **T+1s**: Installer creates staging dir `/tmp/kodegen_install_12345`
4. **T+1s**: Directory already exists (attacker created it)
5. **T+2s**: `create_dir_all()` succeeds (doesn't fail if exists)
6. **T+3s**: Installer reads attacker's malicious files from directory
7. **T+10s**: Installer copies malicious files to `/usr/local/bin` as root
8. **Game over**: Attacker has root-level persistent backdoor

## Additional TOCTOU Issues

### Staging Directory Race (binary_staging.rs:22-23)
```rust
fs::create_dir_all(&staging_dir)?;  // Doesn't fail if exists
// RACE WINDOW: Attacker can modify directory contents here
for binary_path in binary_paths {
    let dest_path = staging_dir.join(binary_name);
    fs::copy(binary_path, &dest_path)?;  // Copies into attacker-controlled dir
}
```

### Certificate File Overwrite (privilege.rs:202-204)
```rust
tokio::fs::write(&temp_cert_path, cert_only).await?;  // Overwrites existing files!
// RACE WINDOW: File permissions not set until line 215
#[cfg(unix)]
{
    let mut perms = tokio::fs::metadata(&temp_cert_path).await?.permissions();
    perms.set_mode(0o600);
    tokio::fs::set_permissions(&temp_cert_path, perms).await?;
}
```

## Remediation

### Immediate Fix: Use Cryptographically Secure Random Paths

```rust
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

fn secure_temp_path(prefix: &str, suffix: &str) -> PathBuf {
    let random_suffix: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)  // 32 chars = 190 bits entropy
        .map(char::from)
        .collect();

    std::env::temp_dir().join(format!("{}_{}{}", prefix, random_suffix, suffix))
}

// Usage:
let staging_dir = secure_temp_path("kodegen_install", "");
let temp_cert = secure_temp_path("kodegen_cert", ".crt");
```

### Better Fix: Use tempfile Crate (Already in Dependencies!)

```rust
// For directories:
let staging_dir = tempfile::Builder::new()
    .prefix("kodegen_install_")
    .tempdir()?;  // Creates with secure permissions, fails if exists

// For files:
let temp_cert = tempfile::Builder::new()
    .prefix("kodegen_cert_")
    .suffix(".crt")
    .tempfile()?;  // Creates with 0600 permissions, unique name guaranteed
```

### Best Fix: Atomic Operations with Exclusive Creation

```rust
use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;

// Create file atomically with O_EXCL (fails if exists)
let file = OpenOptions::new()
    .write(true)
    .create_new(true)  // Fails if file exists (prevents race)
    .mode(0o600)       // Set permissions atomically
    .open(&temp_cert_path)?;
```

## Testing for TOCTOU

```rust
#[test]
fn test_no_predictable_paths() {
    // Generate 1000 temp paths, ensure no duplicates and high entropy
    let mut paths = std::collections::HashSet::new();

    for _ in 0..1000 {
        let path = secure_temp_path("test", ".tmp");
        assert!(paths.insert(path), "Duplicate temp path generated!");
    }
}
```

## References
- CWE-377: Insecure Temporary File
- CWE-379: Creation of Temporary File in Directory with Insecure Permissions
- CWE-367: Time-of-Check Time-of-Use (TOCTOU) Race Condition
- OWASP: Insecure Temporary File
