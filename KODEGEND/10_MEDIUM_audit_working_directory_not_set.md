# MEDIUM: cargo audit Runs Without Working Directory Set

## Priority: MEDIUM
## Category: Functional Correctness
## File: `packages/kodegend/src/security/audit.rs`

## Issue Description

The `run_cargo_audit()` method spawns `cargo audit` without setting a working directory. This means it runs in whatever directory the daemon was started from, making it non-functional for scanning specific projects.

## Location

- **File**: `packages/kodegend/src/security/audit.rs`
- **Lines**: 388-407

## Problematic Code

```rust
async fn run_cargo_audit(&self) -> Result<AuditResult, AuditError> {
    let command = Command::new("cargo")
        .args(["audit", "--format", "json", "--color", "never"])
        .output();  // No .current_dir() set!

    // ... rest
}
```

## Impact

`cargo audit` requires a `Cargo.lock` file in the current directory (or parent directories). Without setting the working directory:

1. **Scans wrong project**: Uses daemon's startup directory
2. **Fails to find Cargo.lock**: Returns error if daemon wasn't started in a Rust project
3. **Non-functional for multi-project**: Can't scan different projects
4. **Confusing errors**: "Could not find Cargo.lock" even though target project has one

## Real-World Scenario

```bash
# Daemon started from /opt/kodegen
$ kodegend start

# User wants to scan /home/user/my-rust-project
# But cargo audit runs in /opt/kodegen (no Cargo.lock)
# Scan fails!
```

## Correct Implementation

### Option 1: Add project_path Parameter

```rust
pub struct VulnerabilityScanner {
    cache: Arc<DashMap<ArrayString<MAX_IDENTIFIER_SIZE>, VulnerabilityStatus>>,
    // ... other fields
    /// Path to project root (containing Cargo.lock)
    project_path: PathBuf,
}

impl VulnerabilityScanner {
    pub fn new(thresholds: AuditThresholds, project_path: PathBuf) -> Self {
        Self {
            // ... other fields
            project_path,
        }
    }

    async fn run_cargo_audit(&self) -> Result<AuditResult, AuditError> {
        let command = Command::new("cargo")
            .args(["audit", "--format", "json", "--color", "never"])
            .current_dir(&self.project_path)  // Set working directory!
            .output();

        // ... rest
    }
}
```

### Option 2: Pass Path to scan_dependencies

```rust
impl VulnerabilityScanner {
    pub async fn scan_dependencies(&self, project_path: &Path) -> Result<AuditResult, AuditError> {
        let start_time = std::time::Instant::now();
        self.total_scans.fetch_add(1, Ordering::Relaxed);

        let result = self.run_cargo_audit(project_path).await;

        // ... rest
    }

    async fn run_cargo_audit(&self, project_path: &Path) -> Result<AuditResult, AuditError> {
        let command = Command::new("cargo")
            .args(["audit", "--format", "json", "--color", "never"])
            .current_dir(project_path)
            .output();

        // ... rest
    }
}
```

### Option 3: Use Explicit --manifest-path

```rust
async fn run_cargo_audit(&self, manifest_path: &Path) -> Result<AuditResult, AuditError> {
    let command = Command::new("cargo")
        .args([
            "audit",
            "--manifest-path",
            manifest_path.to_str().ok_or_else(|| {
                AuditError::InvalidVulnerabilityData("Invalid manifest path".to_string())
            })?,
            "--format",
            "json",
            "--color",
            "never",
        ])
        .output();

    // ... rest
}
```

## Recommendation

Use **Option 2** (pass path to scan_dependencies) because:
- Allows scanning multiple projects with one scanner instance
- Clearer API (explicit path parameter)
- No need to create new scanner per project
- Works well with daemon architecture

## Example Usage

```rust
let scanner = VulnerabilityScanner::new(thresholds);

// Scan different projects
let result1 = scanner.scan_dependencies(Path::new("/home/user/project1")).await?;
let result2 = scanner.scan_dependencies(Path::new("/home/user/project2")).await?;
```

## Validation

Add path validation:

```rust
pub async fn scan_dependencies(&self, project_path: &Path) -> Result<AuditResult, AuditError> {
    // Validate path exists
    if !project_path.exists() {
        return Err(AuditError::InvalidVulnerabilityData(
            format!("Project path does not exist: {:?}", project_path)
        ));
    }

    // Check for Cargo.lock
    let cargo_lock = project_path.join("Cargo.lock");
    if !cargo_lock.exists() {
        return Err(AuditError::CargoAuditFailed(
            format!("Cargo.lock not found in {:?}", project_path)
        ));
    }

    // ... proceed with scan
}
```

## Testing

```rust
#[tokio::test]
async fn test_scan_specific_project() {
    let scanner = VulnerabilityScanner::new(/* ... */);
    
    // Create temp project with Cargo.lock
    let temp_dir = tempfile::tempdir().unwrap();
    std::fs::write(
        temp_dir.path().join("Cargo.lock"),
        "# Minimal Cargo.lock for testing"
    ).unwrap();
    
    let result = scanner.scan_dependencies(temp_dir.path()).await;
    
    assert!(result.is_ok(), "Should scan specified directory");
}

#[tokio::test]
async fn test_scan_missing_cargo_lock() {
    let scanner = VulnerabilityScanner::new(/* ... */);
    
    let temp_dir = tempfile::tempdir().unwrap();
    // No Cargo.lock
    
    let result = scanner.scan_dependencies(temp_dir.path()).await;
    
    assert!(result.is_err(), "Should fail without Cargo.lock");
}
```

## Related Issues

- None directly, but this is a fundamental functional issue

## Priority Justification

**Medium** because:
- Blocks multi-project scanning
- Makes scanner only work in specific circumstances
- Easy to fix
- Not a security issue, just functional limitation
