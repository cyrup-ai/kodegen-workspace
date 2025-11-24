# LOW: Potential Zip Path Traversal in Windows Extraction

## Severity: LOW (Limited exploitability but worth fixing)

## Location
- `packages/kodegend/src/install/download/extract.rs:267-289` - Windows ZIP extraction
- `packages/kodegend/src/install/download/extract.rs:272` - File name retrieval

## Issue Description

The Windows binary extraction from ZIP archives doesn't validate that extracted files stay within the intended output directory. A malicious ZIP archive could contain entries with path traversal sequences (`../`) to write files outside the extraction directory.

## Vulnerable Code

```rust
// Line 267-289: ZIP extraction loop
for i in 0..archive.len() {
    let mut file = archive.by_index(i)?;

    // Get the file name (handles both flat and nested structures)
    let file_name = file.name();  // LINE 272: No validation!

    // Check if this is the binary we're looking for
    if file_name.ends_with(&exe_name) && !file.is_dir() {
        // Extract binary to output directory
        let mut outfile = std::fs::File::create(&final_path)?;  // LINE 280

        std::io::copy(&mut file, &mut outfile)?;

        binary_found = true;
        break;
    }
}
```

## Attack Vector (Low Probability)

For this to be exploitable:
1. Attacker must compromise GitHub releases (separate issue #00)
2. Attacker creates malicious ZIP with path traversal
3. ZIP must contain valid binary name to pass filter
4. Limited impact: Can only overwrite files in output_dir parent

### Example Malicious ZIP Structure
```
malicious_archive.zip:
├── ../../../etc/cron.daily/evil_script
├── ../../../.bashrc
├── kodegen.exe  (legitimate-looking binary)
└── ../../sensitive_config.toml
```

However, current code has partial protection:
```rust
// Line 278: Only matches files ending with exe_name
if file_name.ends_with(&exe_name) && !file.is_dir() {
    // Only extracts ONE file (breaks after first match)
    ...
    break;
}
```

So only ONE file is extracted, and it must end with exact binary name (`kodegen.exe`).

## Why Severity is LOW

1. **Requires prior compromise**: Attacker needs control of GitHub release
2. **Filename constraint**: Must end with expected binary name
3. **Single file**: Only extracts one file (first match)
4. **Limited destinations**: Output directory is controlled, usually temp

## But Still Should Fix

Even though exploitability is low, path traversal is a well-known vulnerability class and should be prevented defensively.

## Remediation

### Option 1: Validate No Path Traversal

```rust
fn is_safe_zip_path(path: &str) -> Result<()> {
    // Normalize path and check for traversal
    let normalized = Path::new(path)
        .components()
        .collect::<Vec<_>>();

    for component in normalized {
        match component {
            Component::ParentDir => {
                return Err(anyhow!(
                    "Zip entry contains path traversal (..): {}",
                    path
                ));
            }
            Component::RootDir if cfg!(windows) => {
                return Err(anyhow!(
                    "Zip entry contains absolute path: {}",
                    path
                ));
            }
            _ => {}
        }
    }

    Ok(())
}

// Usage:
for i in 0..archive.len() {
    let mut file = archive.by_index(i)?;
    let file_name = file.name();

    // Validate path safety BEFORE any processing
    is_safe_zip_path(file_name)?;

    if file_name.ends_with(&exe_name) && !file.is_dir() {
        // ... extract ...
    }
}
```

### Option 2: Use zip Crate's Built-in Safety (Newer Versions)

```rust
// Newer versions of zip crate have sanitized_name()
for i in 0..archive.len() {
    let mut file = archive.by_index(i)?;

    // Get sanitized name (removes .. and absolute paths)
    let sanitized_name = file.sanitized_name();  // Returns PathBuf

    let file_name = sanitized_name
        .to_str()
        .ok_or_else(|| anyhow!("Invalid filename in ZIP"))?;

    if file_name.ends_with(&exe_name) && !file.is_dir() {
        // ... extract ...
    }
}
```

### Option 3: Verify Extracted Path (Defense in Depth)

```rust
if file_name.ends_with(&exe_name) && !file.is_dir() {
    // Build extraction path
    let extract_path = output_dir.join(file_name);

    // Verify extracted path is inside output_dir
    let canonical_output = output_dir.canonicalize()?;
    let canonical_extract = extract_path
        .parent()
        .ok_or_else(|| anyhow!("Invalid extract path"))?
        .canonicalize()?;

    if !canonical_extract.starts_with(&canonical_output) {
        return Err(anyhow!(
            "Zip path traversal detected: {} attempts to escape {}",
            file_name,
            output_dir.display()
        ));
    }

    // Safe to extract
    let mut outfile = std::fs::File::create(&extract_path)?;
    std::io::copy(&mut file, &mut outfile)?;
}
```

## Similar Issue in Other Extractors

### Check .deb Extraction (extract.rs:13-64)
Uses `tar::Archive::unpack()` which should be safe, but verify:
```rust
let mut archive = Archive::new(tar);
archive.unpack(&extract_dir)?;  // Does tar crate prevent traversal?
```

### Check .rpm Extraction (extract.rs:66-121)
Uses external `cpio` command:
```rust
let mut cpio = tokio::process::Command::new("cpio")
    .arg("-idm")  // Extract with parent dirs
    .current_dir(&extract_dir)
    .spawn()?;
```

The `-idm` flags allow directory creation. Malicious .rpm could traverse if cpio doesn't prevent it.

### Check .dmg Extraction (extract.rs:174-237)
macOS DMG mounting via `hdiutil` - should be safe as it's a filesystem image.

## Testing

```rust
#[test]
fn test_zip_path_traversal_protection() {
    use zip::ZipWriter;

    // Create malicious ZIP with path traversal
    let temp_zip = tempfile::NamedTempFile::new().unwrap();
    let mut zip = ZipWriter::new(&temp_zip);

    // Add file with traversal
    zip.start_file("../../etc/evil.exe", Default::default()).unwrap();
    zip.write_all(b"malicious").unwrap();

    zip.start_file("kodegen.exe", Default::default()).unwrap();
    zip.write_all(b"legitimate").unwrap();

    zip.finish().unwrap();
    drop(zip);

    // Attempt extraction
    let output_dir = tempfile::tempdir().unwrap();
    let result = extract_from_windows_installer(
        temp_zip.path(),
        "kodegen",
        output_dir.path()
    );

    // Should fail with path traversal error
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("traversal"));

    // Verify no file written outside output_dir
    assert!(!Path::new("../../etc/evil.exe").exists());
}
```

## References
- CWE-22: Improper Limitation of a Pathname to a Restricted Directory ('Path Traversal')
- OWASP: Path Traversal
- Zip Slip Vulnerability
- Rust zip crate security advisories
