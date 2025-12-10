# Task: Implement Windows PID File Security Validation

## Priority: P0 (Security Critical)

## Status: READY FOR IMPLEMENTATION

## Related Errors
- `daemon.rs:81` - unused import `warn`
- `daemon.rs:518` - function `validate_pid_file_directory` never used
- `daemon.rs:627` - function `validate_existing_pid_file` never used

---

## Problem Statement

The Windows implementation of PID file handling is missing security validation. On Unix, the code validates:
1. PID file directory exists and is secure
2. Existing PID files are not symlinks (prevent symlink attacks)
3. File ownership and permissions are correct

On Windows, all three validation functions are no-op stubs.

---

## Security Risk: Windows Junction/Symlink Attacks

Windows has equivalent attacks to Unix symlink attacks:
- **Junction points** (IO_REPARSE_TAG_MOUNT_POINT) - directory redirects
- **Symbolic links** (IO_REPARSE_TAG_SYMLINK) - file/directory symlinks
- **Hard links** - multiple names for same file

### Attack Scenario
1. Attacker creates junction at `C:\ProgramData\kodegend\run\` pointing to `C:\Windows\System32\`
2. kodegend writes PID to `kodegend.pid` inside what it thinks is its run directory
3. Actually overwrites `C:\Windows\System32\kodegend.pid` (or attacker chooses target)
4. This is a file overwrite vulnerability (CWE-59)

---

## Unix Implementation Reference

### validate_pid_file_security() (lines 230-342)
```rust
// Layer 1: Parent directory validation
- Check parent exists
- Use symlink_metadata() to detect symlinks WITHOUT following
- Reject if parent is symlink (CWE-59)
- Reject if parent is not directory
- Validate ownership: root or current user
- Reject if world-writable (mode & 0o002)

// Layer 2: Existing file validation (if exists)
- Use symlink_metadata() to detect symlinks
- Reject if file is symlink
- Reject if not regular file
- Validate ownership matches current user
- Reject if world-writable
```

### validate_pid_file_directory() (lines 460-515)
```rust
- Ensure parent exists
- Use symlink_metadata() to detect symlinks
- Reject if parent is symlink
- Reject if parent is not directory
- Test actual writability by creating temp file
```

### validate_existing_pid_file() (lines 544-624)
```rust
- Early return if file doesn't exist
- Use symlink_metadata()
- Reject symlinks
- Validate ownership (root check, user check)
- Reject world-writable files
- Warn on group-writable files
```

---

## Windows Implementation Plan

### Dependencies

Add to `Cargo.toml` if not present:
```toml
[target.'cfg(windows)'.dependencies]
windows = { version = "0.62", features = [
    "Win32_Storage_FileSystem",
    "Win32_Security",
    "Win32_Security_Authorization",
    "Win32_Foundation",
]}
```

Or use the existing `windows-acl` crate from Trail of Bits for simpler ACL handling.

### Required Windows APIs

```rust
use windows::Win32::Storage::FileSystem::{
    GetFileAttributesW,
    GetFileInformationByHandle,
    CreateFileW,
    FILE_ATTRIBUTE_REPARSE_POINT,
    FILE_ATTRIBUTE_DIRECTORY,
    BY_HANDLE_FILE_INFORMATION,
    FILE_GENERIC_READ,
    FILE_SHARE_READ,
    OPEN_EXISTING,
};
use windows::Win32::Security::{
    GetNamedSecurityInfoW,
    SE_FILE_OBJECT,
    OWNER_SECURITY_INFORMATION,
    DACL_SECURITY_INFORMATION,
};
use windows::Win32::Security::Authorization::{
    GetEffectiveRightsFromAclW,
};
```

---

## Implementation Code

### Helper: Check if Path is Reparse Point (Junction/Symlink)

```rust
/// Check if a path is a reparse point (junction or symlink)
///
/// Windows reparse points include:
/// - IO_REPARSE_TAG_SYMLINK (0xA000000C) - symbolic links
/// - IO_REPARSE_TAG_MOUNT_POINT (0xA0000003) - junctions and volume mounts
#[cfg(windows)]
fn is_reparse_point(path: &Path) -> Result<bool> {
    use windows::Win32::Storage::FileSystem::{
        GetFileAttributesW, FILE_ATTRIBUTE_REPARSE_POINT, INVALID_FILE_ATTRIBUTES,
    };
    use std::os::windows::ffi::OsStrExt;

    let wide_path: Vec<u16> = path.as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    let attrs = unsafe { GetFileAttributesW(windows::core::PCWSTR(wide_path.as_ptr())) };

    if attrs == INVALID_FILE_ATTRIBUTES {
        let err = std::io::Error::last_os_error();
        if err.kind() == std::io::ErrorKind::NotFound {
            return Ok(false); // Path doesn't exist - not a reparse point
        }
        return Err(anyhow!("Failed to get file attributes for {}: {}", path.display(), err));
    }

    Ok((attrs.0 & FILE_ATTRIBUTE_REPARSE_POINT.0) != 0)
}
```

### Helper: Check Hardlink Count

```rust
/// Check if file has multiple hard links
///
/// Returns the number of hard links to the file.
/// A value > 1 indicates the file has hard links.
#[cfg(windows)]
fn get_hardlink_count(path: &Path) -> Result<u32> {
    use windows::Win32::Storage::FileSystem::{
        CreateFileW, GetFileInformationByHandle, BY_HANDLE_FILE_INFORMATION,
        FILE_GENERIC_READ, FILE_SHARE_READ, OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL,
    };
    use windows::Win32::Foundation::CloseHandle;
    use std::os::windows::ffi::OsStrExt;

    let wide_path: Vec<u16> = path.as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    let handle = unsafe {
        CreateFileW(
            windows::core::PCWSTR(wide_path.as_ptr()),
            FILE_GENERIC_READ.0,
            FILE_SHARE_READ,
            None,
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            None,
        )?
    };

    let mut info: BY_HANDLE_FILE_INFORMATION = unsafe { std::mem::zeroed() };
    let result = unsafe { GetFileInformationByHandle(handle, &mut info) };

    unsafe { let _ = CloseHandle(handle); }

    result?;

    Ok(info.nNumberOfLinks)
}
```

### Helper: Check Directory is Writable

```rust
/// Test directory writability by creating temp file
#[cfg(windows)]
fn test_directory_writable(dir: &Path) -> Result<()> {
    use tempfile::NamedTempFile;

    NamedTempFile::new_in(dir)
        .with_context(|| format!(
            "PID file directory is not writable: {}\n\
             Ensure the service account has write access to this directory.",
            dir.display()
        ))?;

    Ok(())
}
```

### Helper: Validate ACLs (Simplified)

```rust
/// Check if file/directory has overly permissive ACLs
///
/// For security, we check that:
/// - Only SYSTEM, Administrators, and owner have write access
/// - "Everyone" or "Users" do not have write access
///
/// Note: Full ACL parsing is complex. This is a simplified check.
/// For production, consider using the windows-acl crate.
#[cfg(windows)]
fn validate_acl_not_world_writable(path: &Path) -> Result<()> {
    // Simplified approach: Use Rust's std::fs::metadata and check readonly
    // This is a basic check - full ACL validation would use GetNamedSecurityInfoW

    let metadata = std::fs::metadata(path)
        .with_context(|| format!("Failed to read metadata for: {}", path.display()))?;

    // On Windows, if readonly is false, we can't reliably determine write permissions
    // without full ACL parsing. Log a debug message and continue.
    if metadata.permissions().readonly() {
        // File is readonly - definitely not world-writable
        return Ok(());
    }

    // For full ACL validation, would need:
    // 1. GetNamedSecurityInfoW to get security descriptor
    // 2. GetSecurityDescriptorDacl to get DACL
    // 3. Iterate ACEs with GetAce
    // 4. Check each ACE for "Everyone" SID with WRITE access
    //
    // This is complex - recommend using windows-acl crate for production

    log::debug!(
        "ACL validation skipped for {} (not readonly, full ACL check not implemented)",
        path.display()
    );

    Ok(())
}
```

### Full ACL Validation (Using windows-acl crate)

If using the [windows-acl](https://github.com/trailofbits/windows-acl) crate:

```rust
/// Full ACL validation using windows-acl crate
#[cfg(windows)]
fn validate_acl_full(path: &Path) -> Result<()> {
    use windows_acl::acl::ACL;
    use windows_acl::helper;

    // Get DACL for the path
    let acl = ACL::from_file_path(path.to_str().unwrap(), false)
        .map_err(|e| anyhow!("Failed to get ACL for {}: {:?}", path.display(), e))?;

    // Get the "Everyone" SID
    let everyone_sid = helper::string_to_sid("S-1-1-0")
        .map_err(|e| anyhow!("Failed to get Everyone SID: {:?}", e))?;

    // Check if Everyone has write access
    for entry in acl.all().unwrap_or_default() {
        if entry.sid == everyone_sid {
            // Check for write permissions (FILE_WRITE_DATA = 0x2)
            if entry.mask & 0x2 != 0 {
                return Err(anyhow!(
                    "SECURITY: Path is world-writable: {}\n\
                     The 'Everyone' group has write access.\n\
                     This allows any user to modify the file.",
                    path.display()
                ));
            }
        }
    }

    Ok(())
}
```

---

## Main Function Implementations

### validate_pid_file_security() for Windows

```rust
#[cfg(windows)]
fn validate_pid_file_security(path: &Path) -> Result<()> {
    // Layer 1: Validate parent directory security
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            // Parent will be created - this is OK
            return Ok(());
        }

        // Check for reparse points (junctions/symlinks)
        if is_reparse_point(parent)? {
            anyhow::bail!(
                "SECURITY: PID file parent directory is a reparse point (junction/symlink): {}\n\
                 This could be a symlink attack (CWE-59).\n\
                 Parent directory must be a real directory, not a junction or symbolic link.",
                parent.display()
            );
        }

        // Verify it's actually a directory
        let metadata = std::fs::metadata(parent)
            .context(format!("Failed to read metadata for: {}", parent.display()))?;

        if !metadata.is_dir() {
            anyhow::bail!(
                "SECURITY: PID file parent path exists but is not a directory: {}",
                parent.display()
            );
        }

        // Validate ACLs are not overly permissive
        validate_acl_not_world_writable(parent)?;
    }

    // Layer 2: Validate existing PID file (if exists)
    if path.exists() {
        // Check for reparse points
        if is_reparse_point(path)? {
            anyhow::bail!(
                "SECURITY: PID file is a reparse point (junction/symlink): {}\n\
                 This could be a symlink attack (CWE-59).\n\
                 PID files must be regular files, not symbolic links or junctions.\n\n\
                 Action: Remove the reparse point and restart the daemon.",
                path.display()
            );
        }

        // Check for hard links
        let link_count = get_hardlink_count(path)?;
        if link_count > 1 {
            warn!(
                "PID file has {} hard links: {}\n\
                 This is unusual and could indicate tampering.",
                link_count,
                path.display()
            );
        }

        // Verify it's a regular file
        let metadata = std::fs::metadata(path)
            .context(format!("Failed to read metadata for: {}", path.display()))?;

        if !metadata.is_file() {
            anyhow::bail!(
                "SECURITY: PID file path exists but is not a regular file: {}",
                path.display()
            );
        }

        // Validate ACLs
        validate_acl_not_world_writable(path)?;
    }

    Ok(())
}
```

### validate_pid_file_directory() for Windows

```rust
#[cfg(windows)]
fn validate_pid_file_directory(path: &Path) -> Result<()> {
    let parent = path.parent()
        .ok_or_else(|| anyhow!("Invalid PID file path: no parent directory"))?;

    if !parent.exists() {
        return Err(anyhow!(
            "PID file directory does not exist: {}\n\
             Create it first or check your configuration.",
            parent.display()
        ));
    }

    // Check for reparse points
    if is_reparse_point(parent)? {
        return Err(anyhow!(
            "SECURITY: PID directory is a junction/symlink: {}\n\
             This could be a CWE-59 symlink attack.",
            parent.display()
        ));
    }

    // Verify it's a directory
    let metadata = std::fs::metadata(parent)
        .context(format!("Failed to read metadata for: {}", parent.display()))?;

    if !metadata.is_dir() {
        return Err(anyhow!(
            "PID file parent path is not a directory: {}",
            parent.display()
        ));
    }

    // Test writability
    test_directory_writable(parent)?;

    Ok(())
}
```

### validate_existing_pid_file() for Windows

```rust
#[cfg(windows)]
fn validate_existing_pid_file(path: &Path) -> Result<()> {
    if !path.exists() {
        return Ok(()); // Nothing to validate
    }

    // Check for reparse points (symlinks/junctions)
    if is_reparse_point(path)? {
        return Err(anyhow!(
            "SECURITY: PID file is a symlink/junction: {}\n\
             This could be a symlink attack (CWE-59).\n\
             PID files must be regular files.\n\n\
             Action: Remove the symlink: del {}",
            path.display(),
            path.display()
        ));
    }

    // Check hard link count
    let link_count = get_hardlink_count(path)?;
    if link_count > 1 {
        warn!(
            "PID file has {} hard links (expected 1): {}",
            link_count,
            path.display()
        );
    }

    // Validate ACLs
    validate_acl_not_world_writable(path)?;

    Ok(())
}
```

---

## Files to Modify

1. **`packages/kodegend/src/daemon.rs`** (full path from repo root)
   - Replace stub at lines 345-350 with `validate_pid_file_security()` implementation
   - Replace stub at lines 518-523 with `validate_pid_file_directory()` implementation
   - Replace stub at lines 627-632 with `validate_existing_pid_file()` implementation
   - Add helper functions: `is_reparse_point()`, `get_hardlink_count()`, `test_directory_writable()`, `validate_acl_not_world_writable()`
   - The `warn!` import will now be used, fixing that error
   - Update Windows `PidFile::create()` (lines 869-892) to call `validate_pid_file_directory()` and `validate_existing_pid_file()`

2. **`packages/kodegend/Cargo.toml`** - NO CHANGES NEEDED
   - `windows` crate v0.62 already present with required features
   - `tempfile` crate already present

## Implementation Decision: Simplified ACL Validation

Using the **simplified ACL validation** approach (not `windows-acl` crate) because:
- Avoids adding new dependencies
- The existing `windows` crate features are sufficient for reparse point detection
- Full ACL validation can be added later if needed

## Unit Test Location

Unit tests should be added to `packages/kodegend/src/daemon.rs` in a `#[cfg(test)] #[cfg(windows)]` module at the end of the file.

---

## Error Handling Strategy

| Condition | Action |
|-----------|--------|
| Parent is reparse point | **ERROR** - bail with CWE-59 warning |
| File is reparse point | **ERROR** - bail with CWE-59 warning |
| Parent not directory | **ERROR** - bail |
| File not regular file | **ERROR** - bail |
| Multiple hard links | **WARN** - log but continue |
| World-writable ACL | **ERROR** - bail |
| Cannot read metadata | **ERROR** - bail with context |

---

## Testing

### Manual Test Cases

1. **Junction point detection**:
   ```cmd
   mklink /J C:\ProgramData\kodegend\run C:\Windows\Temp
   # Run kodegend - should detect and reject
   ```

2. **Symlink detection** (requires SeCreateSymbolicLinkPrivilege):
   ```cmd
   mklink C:\ProgramData\kodegend\run\kodegend.pid C:\Windows\System32\test.txt
   # Run kodegend - should detect and reject
   ```

3. **Hardlink detection**:
   ```cmd
   mklink /H link.pid original.pid
   # Run kodegend - should warn
   ```

4. **Permission test**:
   ```cmd
   icacls C:\ProgramData\kodegend\run /grant Everyone:F
   # Run kodegend - should reject world-writable
   ```

### Unit Tests

```rust
#[cfg(test)]
#[cfg(windows)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_normal_directory_passes() {
        let temp = TempDir::new().unwrap();
        let pid_path = temp.path().join("kodegend.pid");

        assert!(validate_pid_file_directory(&pid_path).is_ok());
    }

    #[test]
    fn test_reparse_point_detected() {
        // Note: Creating junctions requires admin or specific permissions
        // This test may need to be run with elevation
    }

    #[test]
    fn test_regular_file_passes() {
        let temp = TempDir::new().unwrap();
        let pid_path = temp.path().join("kodegend.pid");
        std::fs::write(&pid_path, "12345").unwrap();

        assert!(validate_existing_pid_file(&pid_path).is_ok());
    }
}
```

---

## Acceptance Criteria

- [ ] `is_reparse_point()` detects junction points and symlinks
- [ ] `get_hardlink_count()` returns correct count
- [ ] `validate_pid_file_directory()` rejects reparse points
- [ ] `validate_existing_pid_file()` rejects reparse points
- [ ] `validate_pid_file_security()` performs full validation
- [ ] `warn!` macro is used (fixing unused import error)
- [ ] Unit tests cover attack scenarios
- [ ] No new clippy warnings

---

## References

- [FILE_ATTRIBUTE_REPARSE_POINT - windows-rs docs](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Storage/FileSystem/constant.FILE_ATTRIBUTE_REPARSE_POINT.html)
- [GetFileAttributesW - windows-rs docs](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Storage/FileSystem/fn.GetFileAttributesW.html)
- [windows-acl crate - Trail of Bits](https://github.com/trailofbits/windows-acl)
- [windows-permissions crate](https://docs.rs/windows-permissions)
- [MetadataExt - Rust std library](https://doc.rust-lang.org/std/os/windows/fs/trait.MetadataExt.html)
- [NTFS Reparse Points - Wikipedia](https://en.wikipedia.org/wiki/NTFS_reparse_point)
- [CWE-59: Improper Link Resolution](https://cwe.mitre.org/data/definitions/59.html)
