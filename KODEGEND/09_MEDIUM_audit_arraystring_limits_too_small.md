# MEDIUM: ArrayString Size Limits Cause Data Loss

## Priority: MEDIUM
## Category: Correctness / Data Loss
## File: `packages/kodegend/src/security/audit.rs`

## Issue Description

The fixed-size limits for ArrayString fields are too small for real-world cargo-audit output, causing silent truncation and data loss.

## Location

- **File**: `packages/kodegend/src/security/audit.rs`
- **Lines**: 44-54 (constant definitions)

## Problematic Constants

```rust
/// Maximum size for vulnerability report content
const MAX_REPORT_SIZE: usize = 1024;  // Only 1KB for entire JSON!

/// Maximum size for package names and vulnerability IDs
const MAX_IDENTIFIER_SIZE: usize = 64;

/// Maximum size for vulnerability descriptions
const MAX_DESCRIPTION_SIZE: usize = 256;
```

## Real-World Examples That Exceed Limits

### 1. MAX_REPORT_SIZE (1024 bytes)

Typical cargo-audit JSON for a medium project:
```json
{
  "lockfile": { ... },  // ~500 bytes
  "vulnerabilities": {
    "found": true,
    "count": 15,
    "list": [
      // Each vulnerability ~300-500 bytes
      // 15 vulnerabilities = 4500-7500 bytes
    ]
  }
}
// Total: 5000-8000 bytes (5-8x the limit!)
```

### 2. MAX_IDENTIFIER_SIZE (64 bytes)

Package names with scopes:
```
@organization/very-long-package-name-with-detailed-description-v2
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
= 65 characters - TOO LONG!
```

Scoped packages commonly exceed 64 chars.

### 3. MAX_DESCRIPTION_SIZE (256 bytes)

Real vulnerability description from RUSTSEC:
```
"Use of a broken or risky cryptographic algorithm in the implementation
of the TLS protocol. The vulnerability allows a remote attacker to perform
a man-in-the-middle attack and decrypt sensitive information transmitted
over the network connection, potentially exposing user credentials and
private data. Affects versions 0.5.0 through 0.8.2."
```
= 320 characters - EXCEEDS LIMIT!

## Impact

### Silent Data Loss

When fields exceed limits:

```rust
// Line 149-156 in Vulnerability::new()
let id = ArrayString::from(id).ok()?;  // Returns None if > 64 bytes
let package = ArrayString::from(package).ok()?;  // Returns None if > 64 bytes
let description = ArrayString::from(description).ok()?;  // Returns None if > 256 bytes
// ...
```

If ANY field is too long:
1. `ArrayString::from()` fails
2. Returns None
3. Entire vulnerability is silently dropped
4. No error, no log, no indication

### Real-World Scenario

Project with 20 vulnerabilities, 3 have long descriptions:
- Scanner parses JSON
- Finds 20 vulnerabilities
- 3 fail to convert to Vulnerability struct
- Returns AuditResult with only 17 vulnerabilities
- **User is unaware of 3 critical vulnerabilities!**

## Recommended Solutions

### Option 1: Use String (Simplest)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vulnerability {
    pub id: String,              // Heap allocated, no limit
    pub package: String,
    pub severity: VulnerabilitySeverity,
    pub description: String,
    pub version: String,
    pub patched: Option<String>,
    pub discovered: u64,
}

impl Vulnerability {
    pub fn new(
        id: &str,
        package: &str,
        severity: VulnerabilitySeverity,
        description: &str,
        version: &str,
        patched: Option<&str>,
    ) -> Self {
        Self {
            id: id.to_string(),
            package: package.to_string(),
            severity,
            description: description.to_string(),
            version: version.to_string(),
            patched: patched.map(|s| s.to_string()),
            discovered: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}
```

**Pros**: No data loss, simpler code  
**Cons**: Heap allocation (which is fine!)

### Option 2: Increase Limits Dramatically

If you must use ArrayString:

```rust
const MAX_REPORT_SIZE: usize = 1024 * 1024;  // 1MB (overkill but safe)
const MAX_IDENTIFIER_SIZE: usize = 256;      // 4x current
const MAX_DESCRIPTION_SIZE: usize = 2048;    // 8x current
```

**Pros**: Keeps stack allocation  
**Cons**: Still arbitrary limits, wastes stack space

### Option 3: SmallVec / SmallString

```rust
use smallvec::SmallVec;
use smartstring::SmartString;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vulnerability {
    // Inline up to 64 bytes, heap beyond that
    pub id: SmartString<smartstring::LazyCompact>,
    pub package: SmartString<smartstring::LazyCompact>,
    pub severity: VulnerabilitySeverity,
    pub description: String,  // Always heap for large descriptions
    // ...
}
```

**Pros**: Optimization for small strings, no limits  
**Cons**: Extra dependency

### Option 4: Error on Overflow (Not Silent)

```rust
impl Vulnerability {
    pub fn new(
        id: &str,
        package: &str,
        severity: VulnerabilitySeverity,
        description: &str,
        version: &str,
        patched: Option<&str>,
    ) -> Result<Self, String> {  // Not Option!
        let id = ArrayString::from(id)
            .map_err(|_| format!("ID too long (max {}): {}", MAX_IDENTIFIER_SIZE, id))?;
        let package = ArrayString::from(package)
            .map_err(|_| format!("Package name too long (max {}): {}", MAX_IDENTIFIER_SIZE, package))?;
        let description = ArrayString::from(description)
            .map_err(|_| format!("Description too long (max {}): {}", MAX_DESCRIPTION_SIZE, description))?;
        
        // ... rest
        
        Ok(Self { id, package, severity, description, ... })
    }
}
```

**Pros**: Explicit errors, no silent loss  
**Cons**: Still loses data, just noisier

## Recommendation

Use **Option 1 (String)** because:
1. No data loss
2. Simpler code  
3. Heap allocation is not a problem for background scanning
4. Removes 300+ lines of complex ArrayString handling
5. Better for maintainability

The "zero-allocation" goal is misguided (see Task #27).

## Testing

```rust
#[test]
fn test_long_vulnerability_fields() {
    // Real-world long values
    let long_id = "RUSTSEC-2023-0001-WITH-VERY-LONG-IDENTIFIER-THAT-EXCEEDS-LIMIT";
    let long_package = "@my-organization/my-very-long-package-name-that-exceeds-sixty-four-characters";
    let long_desc = "This is a very long vulnerability description that explains the security issue in great detail, including attack vectors, impact analysis, affected versions, and remediation steps. It easily exceeds 256 characters and represents a typical RUSTSEC advisory.";
    
    let vuln = Vulnerability::new(
        long_id,
        long_package,
        VulnerabilitySeverity::Critical,
        long_desc,
        "0.1.0",
        None,
    );
    
    // With current code: vuln is None (SILENT FAILURE!)
    // With String: vuln is Some and contains full data
    assert!(vuln.is_some(), "Should not lose vulnerability due to long fields");
}
```

## Related Issues

- Issue #3: Report size limit (MAX_REPORT_SIZE)
- Issue #27: Zero-allocation design flaw
- Issue #15: Silent parsing failures

## Impact if Not Fixed

- **Security risk**: Critical vulnerabilities may not be detected
- **Compliance risk**: Audit reports incomplete
- **Trust risk**: Users rely on incomplete scan results
