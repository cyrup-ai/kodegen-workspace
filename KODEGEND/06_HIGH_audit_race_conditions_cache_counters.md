# HIGH: Race Conditions in Cache and Counter Updates

## Priority: HIGH
## Category: Concurrency / Race Condition
## File: `packages/kodegend/src/security/audit.rs`

## Issue Description

The vulnerability scanner updates atomic counters and cache separately without synchronization, leading to race conditions and inconsistent state when multiple scans run concurrently.

## Location

- **File**: `packages/kodegend/src/security/audit.rs`
- **Lines**: 553-576 (update_counters, update_cache)
- **Lines**: 369-386 (scan_dependencies)

## Race Conditions Identified

### Race #1: Counter/Cache Inconsistency

```rust
// In scan_dependencies()
if let Ok(audit_result) = &result {
    if audit_result.success {
        self.successful_scans.fetch_add(1, Ordering::Relaxed);
        self.update_counters(audit_result);  // Line 378
        self.update_cache(audit_result);     // Line 379
    }
}
```

**Problem**: Between update_counters() and update_cache(), another thread could read:
- Updated counters but stale cache
- Or call get_metrics() and see inconsistent data

### Race #2: Counter Stores Instead of Accumulation

```rust
fn update_counters(&self, result: &AuditResult) {
    let critical = result.count_by_severity(VulnerabilitySeverity::Critical) as u32;
    let high = result.count_by_severity(VulnerabilitySeverity::High) as u32;
    let medium = result.count_by_severity(VulnerabilitySeverity::Medium) as u32;
    let low = result.count_by_severity(VulnerabilitySeverity::Low) as u32;

    // BUG: Uses store() instead of fetch_add()
    self.critical_count.store(critical, Ordering::Relaxed);  // Line 559
    self.high_count.store(high, Ordering::Relaxed);
    self.medium_count.store(medium, Ordering::Relaxed);
    self.low_count.store(low, Ordering::Relaxed);
}
```

**Problem**: If two scans run concurrently:
1. Scan A finds 5 critical vulnerabilities, stores 5
2. Scan B finds 3 critical vulnerabilities, stores 3  
3. Final count is 3 (last writer wins), not 8!

**Semantics Issue**: Unclear if counters should represent:
- Last scan only (current behavior with race)
- Cumulative across all scans (broken)
- Peak values seen (not implemented)

### Race #3: Cache Status Transitions

```rust
fn update_cache(&self, result: &AuditResult) {
    for vulnerability in &result.vulnerabilities {
        let key = vulnerability.id;
        let status = if vulnerability.patched.is_some() {
            VulnerabilityStatus::Patched
        } else {
            VulnerabilityStatus::Active
        };
        self.cache.insert(key, status);  // Line 574
    }
}
```

**Problem**: No atomic read-modify-write for status transitions:
- Scan A marks vulnerability as Active
- Scan B (slightly newer data) marks same vulnerability as Patched
- But if Scan A runs slower, it could overwrite with stale Active status
- Last writer wins, but may have older data

## Impact

1. **Metrics Unreliable**: get_metrics() returns inconsistent snapshots
2. **Lost Data**: Concurrent scans overwrite each other's counters
3. **Stale Cache**: Vulnerability status can revert to older state
4. **Monitoring Broken**: CI/CD systems see wrong counts

## Recommended Solutions

### Solution 1: Document Eventual Consistency

If the design intent is "counters reflect last scan only":

```rust
/// VulnerabilityScanner metrics represent the MOST RECENT scan results only.
/// When multiple scans run concurrently, metrics reflect whichever scan
/// completes last (non-deterministic in concurrent scenarios).
/// 
/// For aggregate metrics across scans, maintain a separate history.
pub struct VulnerabilityScanner {
    // ... fields ...
    
    /// Counters represent LAST SCAN ONLY (not cumulative)
    /// Updated with store(), not fetch_add()
    critical_count: AtomicU32,
    // ...
}

fn update_counters(&self, result: &AuditResult) {
    // Add comment explaining behavior
    // Store values from this scan (overwrites previous scan)
    self.critical_count.store(
        result.count_by_severity(VulnerabilitySeverity::Critical) as u32,
        Ordering::Release  // Use Release for proper sync
    );
    // ... rest with Release ordering
}
```

### Solution 2: Use Mutex for Atomic Update

If counters and cache must be consistent:

```rust
use std::sync::Mutex;

pub struct VulnerabilityScanner {
    // Group related state
    state: Mutex<ScannerState>,
    cache: Arc<DashMap<ArrayString<MAX_IDENTIFIER_SIZE>, VulnerabilityStatus>>,
    // ... other fields
}

struct ScannerState {
    critical_count: u32,
    high_count: u32,
    medium_count: u32,
    low_count: u32,
}

fn update_metrics(&self, result: &AuditResult) {
    let mut state = self.state.lock().unwrap();
    
    // Atomic update of all counters
    state.critical_count = result.count_by_severity(VulnerabilitySeverity::Critical) as u32;
    state.high_count = result.count_by_severity(VulnerabilitySeverity::High) as u32;
    state.medium_count = result.count_by_severity(VulnerabilitySeverity::Medium) as u32;
    state.low_count = result.count_by_severity(VulnerabilitySeverity::Low) as u32;
    
    // Update cache while holding lock
    for vulnerability in &result.vulnerabilities {
        let key = vulnerability.id;
        let status = if vulnerability.patched.is_some() {
            VulnerabilityStatus::Patched
        } else {
            VulnerabilityStatus::Active
        };
        self.cache.insert(key, status);
    }
}
```

### Solution 3: Add Timestamp-Based Versioning

Prevent stale updates:

```rust
#[derive(Clone, Copy)]
struct CachedVulnerability {
    status: VulnerabilityStatus,
    timestamp: u64,  // When this status was determined
}

fn update_cache(&self, result: &AuditResult) {
    let scan_time = result.scan_timestamp;
    
    for vulnerability in &result.vulnerabilities {
        let key = vulnerability.id;
        let new_status = if vulnerability.patched.is_some() {
            VulnerabilityStatus::Patched
        } else {
            VulnerabilityStatus::Active
        };
        
        // Only update if this scan is newer
        self.cache.entry(key)
            .and_modify(|cached| {
                if scan_time > cached.timestamp {
                    cached.status = new_status;
                    cached.timestamp = scan_time;
                }
            })
            .or_insert(CachedVulnerability {
                status: new_status,
                timestamp: scan_time,
            });
    }
}
```

### Solution 4: Use SeqCst Ordering

At minimum, fix memory ordering:

```rust
fn update_counters(&self, result: &AuditResult) {
    // Use SeqCst for stronger guarantees
    self.critical_count.store(
        result.count_by_severity(VulnerabilitySeverity::Critical) as u32,
        Ordering::SeqCst  // Not just Relaxed
    );
    // ... same for others
}
```

## Testing

```rust
#[tokio::test]
async fn test_concurrent_scans() {
    let scanner = Arc::new(VulnerabilityScanner::new(/* ... */));
    
    let mut handles = vec![];
    
    // Launch 10 concurrent scans
    for i in 0..10 {
        let scanner_clone = Arc::clone(&scanner);
        let handle = tokio::spawn(async move {
            scanner_clone.scan_dependencies().await
        });
        handles.push(handle);
    }
    
    // Wait for all scans
    for handle in handles {
        handle.await.unwrap();
    }
    
    let metrics = scanner.get_metrics();
    
    // Metrics should be consistent (one scan's results)
    // But which scan? Non-deterministic!
    assert!(metrics.total_scans == 10);
}
```

## Related Issues

- Issue #14: Counter semantics unclear (store vs accumulate)
- Issue #22: set_timeout requires &mut self but scanner is shared

## Recommended Action

1. **Immediate**: Document that counters represent "last scan only"
2. **Short-term**: Add timestamp-based cache versioning  
3. **Long-term**: Use Mutex for atomic state updates if consistency is required
4. **Testing**: Add concurrent scan tests to verify behavior
