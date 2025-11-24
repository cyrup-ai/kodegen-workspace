# LOW: packages_scanned Field Never Set

## Priority: LOW
## Category: Incomplete Implementation
## File: `packages/kodegend/src/security/audit.rs`

## Issue Description

The `AuditResult::packages_scanned` field is initialized to 0 and never updated, making it useless for monitoring.

## Location

- **File**: `packages/kodegend/src/security/audit.rs`  
- **Lines**: 198 (initialization), 690-698 (output formatting uses it)

## Code

```rust
impl AuditResult {
    pub fn new() -> Self {
        Self {
            vulnerabilities: ArrayVec::new(),
            scan_duration_ms: 0,
            packages_scanned: 0,  // Always 0!
            scan_timestamp: /* ... */,
            success: false,
        }
    }
}

// In ci_cd module (line 690-698)
pub fn format_scan_results(result: &AuditResult) -> ArrayString<1024> {
    let mut output = ArrayString::new();
    let _ = output.try_push_str(&format!(
        // ...
        "- Packages scanned: {}\n",  // Always prints 0!
        result.packages_scanned,
        // ...
    ));
    output
}
```

## Impact

- **Metrics incomplete**: CI/CD output shows "Packages scanned: 0" even after successful scan
- **Monitoring misleading**: Can't track scan coverage
- **Dead field**: Serves no purpose in current state

## Correct Implementation

### Option 1: Parse from cargo-audit Output

cargo-audit JSON includes package count:

```json
{
  "database": {
    "advisory-count": 500,
    "last-commit": "...",
    "last-updated": "..."
  },
  "lockfile": {
    "dependency-count": 123  // <-- This is what we want
  },
  "vulnerabilities": { ... }
}
```

Update parsing:

```rust
#[derive(Deserialize)]
struct CargoAuditOutput {
    lockfile: LockfileInfo,
    vulnerabilities: VulnerabilitiesSection,
}

#[derive(Deserialize)]
struct LockfileInfo {
    #[serde(rename = "dependency-count")]
    dependency_count: u32,
}

async fn parse_audit_output(&self, output: &str) -> Result<AuditResult, AuditError> {
    let audit_output: CargoAuditOutput = serde_json::from_str(output)?;
    
    let mut result = AuditResult::new();
    result.packages_scanned = audit_output.lockfile.dependency_count;  // Set it!
    
    // ... parse vulnerabilities
    
    Ok(result)
}
```

### Option 2: Remove the Field

If package count isn't needed:

```rust
pub struct AuditResult {
    pub vulnerabilities: ArrayVec<Vulnerability, MAX_VULNERABILITIES>,
    pub scan_duration_ms: u64,
    // Remove: pub packages_scanned: u32,
    pub scan_timestamp: u64,
    pub success: bool,
}

// Update format_scan_results to not print it
```

## Recommendation

**Option 1** - it's useful information and cargo-audit provides it.

## Testing

```rust
#[test]
fn test_packages_scanned_set() {
    let mock_output = r#"{
        "lockfile": {"dependency-count": 42},
        "vulnerabilities": {"list": []}
    }"#;
    
    let scanner = VulnerabilityScanner::new(/* ... */);
    let result = scanner.parse_audit_output(mock_output).await.unwrap();
    
    assert_eq!(result.packages_scanned, 42, "Should parse package count");
}
```

## Related Issues

- Issue #7: Manual JSON parsing (using serde would make this trivial)

## Priority

**Low** because:
- Doesn't affect functionality
- Just incomplete metrics
- Easy to fix once JSON parsing is corrected
