# HIGH: Unbounded Loop in Numbered Log Rotation Cleanup

**Priority:** HIGH  
**Component:** `packages/kodegend/src/service.rs`  
**Lines:** 497-509  
**Impact:** Performance - Potential long delays during log rotation  
**Frequency:** Every 3600 seconds (1 hour) per service - see line 74

---

## Executive Summary

The numbered log rotation cleanup in `kodegend` uses an unbounded loop (`for i in (max_files + 1)..`) that iterates indefinitely until finding a missing file. This creates performance and safety risks, especially on systems with many rotated logs or gaps in numbering. The fix is to add a bounded iteration limit using a constant `MAX_LOG_CLEANUP_ITERATIONS`.

**Industry Context:** Standard log rotation tools like `logrotate` typically keep 4-7 rotated files by default [(source)](https://blog.victormendonca.com/2024/09/09/logrotate-simplifying-log-management/). A safety limit of 100-200 iterations is **extremely conservative** (25-50x higher than typical usage) and will not affect normal operations.

---

## Problem Analysis

### Problematic Code Location

**File:** [`packages/kodegend/src/service.rs`](../packages/kodegend/src/service.rs)  
**Function:** `rotate_single_log()` (lines 419-538)  
**Specific Issue:** Lines 497-509

```rust
// Clean up old rotated files beyond max_files limit
if !timestamp {
    // For numbered rotation, delete files beyond max_files
    for i in (max_files + 1).. {  // ← UNBOUNDED RANGE - PROBLEM HERE
        let old_file = format!("{}.{}", log_path, i);
        let old_gz = format!("{}.gz", old_file);
        
        // Stop when no more files exist
        if !Path::new(&old_file).exists() && !Path::new(&old_gz).exists() {
            break;
        }
        
        // Remove both compressed and uncompressed versions
        fs::remove_file(&old_file).ok();
        fs::remove_file(&old_gz).ok();
    }
}
```

### Root Cause

The range `(max_files + 1)..` is **open-ended** and has no upper bound. The loop only terminates when it encounters a number with no corresponding file (neither `.N` nor `.N.gz` exists). This creates several problematic scenarios:

1. **Gaps in numbering:** If files exist at positions 5, 6, 8, 9 (missing 7), the loop checks every integer from `max_files+1` through infinity until hitting the first gap after the highest numbered file.

2. **Filesystem I/O overhead:** Each iteration performs:
   - 2× `format!()` allocations (lines 498-499)
   - 2× `Path::new()` allocations (line 502)
   - 2× `.exists()` filesystem syscalls (stat operations)
   - 2× `fs::remove_file()` attempts (lines 507-508)

3. **Execution frequency:** This runs on **every rotation tick** (3600 seconds = 1 hour, see line 74) for **EVERY managed service**.

### Performance Impact Calculation

**Example scenario:**
- 10 managed services
- Each service has 100 old rotated logs beyond `max_files`
- Rotation tick: every hour

**Total operations per hour:**
- 1000 loop iterations
- 2000 string allocations
- 2000 Path allocations
- 2000 filesystem stat syscalls
- 2000 file removal attempts

On systems with slow filesystems (network mounts, encrypted drives), this compounds significantly.

---

## Solution: Bounded Search with Safety Limit (Recommended)

### Implementation Strategy

Add a constant `MAX_LOG_CLEANUP_ITERATIONS` to bound the cleanup loop. This prevents unbounded iteration while being generous enough to handle any realistic log rotation scenario.

### Why This Approach

**Advantages:**
- ✅ Simple, minimal code change (3 lines)
- ✅ Prevents runaway loops completely
- ✅ 100-200 iterations is 25-50x higher than industry standard usage
- ✅ Aligns with Rust safety best practices for bounded operations
- ✅ No risk to normal operations

**Trade-offs:**
- ⚠️ Files beyond the limit won't be cleaned up automatically
- ✅ This is acceptable: if >100 old logs exist beyond `max_files`, it indicates misconfiguration or manual intervention is needed anyway

---

## Exact Implementation Instructions

### Step 1: Add Constant Definition

**Location:** Near the top of [`packages/kodegend/src/service.rs`](../packages/kodegend/src/service.rs), after the imports and before the `ServiceError` enum (around line 15-18).

**Code to add:**
```rust
/// Maximum iterations when cleaning up old numbered log files.
/// This prevents unbounded loops while being generous enough for any realistic scenario.
/// Industry standard tools like logrotate typically keep 4-7 rotated files.
const MAX_LOG_CLEANUP_ITERATIONS: u32 = 100;
```

**Justification for value 100:**
- Industry standard: 4-7 rotated files
- Conservative multiplier: 25x industry standard
- Handles edge cases: systems with aggressive rotation policies
- Safety margin: prevents pathological cases

### Step 2: Replace Unbounded Loop

**Location:** [`packages/kodegend/src/service.rs`](../packages/kodegend/src/service.rs), line 497

**Current code:**
```rust
for i in (max_files + 1).. {
```

**Replace with:**
```rust
for i in (max_files + 1)..(max_files + 1 + MAX_LOG_CLEANUP_ITERATIONS) {
```

**Full context of modified section:**
```rust
// Clean up old rotated files beyond max_files limit
if !timestamp {
    // For numbered rotation, delete files beyond max_files
    for i in (max_files + 1)..(max_files + 1 + MAX_LOG_CLEANUP_ITERATIONS) {
        let old_file = format!("{}.{}", log_path, i);
        let old_gz = format!("{}.gz", old_file);
        
        // Stop when no more files exist
        if !Path::new(&old_file).exists() && !Path::new(&old_gz).exists() {
            break;
        }
        
        // Remove both compressed and uncompressed versions
        fs::remove_file(&old_file).ok();
        fs::remove_file(&old_gz).ok();
    }
}
```

### Step 3: Verify Context

Ensure the modification is within the `rotate_single_log()` function and specifically in the numbered rotation cleanup section (the `if !timestamp` branch, not the `else` branch for timestamped rotation).

**Function signature for reference (line 419):**
```rust
fn rotate_single_log(
    log_path: &str,
    max_size_mb: u64,
    max_files: u32,
    compress: bool,
    timestamp: bool,
) -> Result<()> {
```

---

## Alternative Approaches (For Reference)

### Option 2: Directory Scan Approach

The **timestamped rotation** already uses this superior approach at lines 510-535. It scans the directory once, collects all matching files, sorts them, and deletes the oldest beyond `max_files`.

**Reference implementation in same file:**
```rust
// For timestamped rotation, count existing archives and delete oldest
let parent = path.parent().unwrap_or_else(|| Path::new("."));
let Some(file_name_os) = path.file_name() else {
    return Ok(());
};
let filename = file_name_os.to_string_lossy();

// Find all rotated versions (both .gz and non-.gz)
let mut archives: Vec<_> = fs::read_dir(parent)?
    .filter_map(|e| e.ok())
    .filter(|e| {
        let name = e.file_name().to_string_lossy().to_string();
        name.starts_with(filename.as_ref()) && name != filename
    })
    .collect();

// Sort by modification time (oldest first)
archives.sort_by_key(|e| e.metadata().ok().and_then(|m| m.modified().ok()));

// Delete oldest archives beyond max_files
let to_delete = archives.len().saturating_sub(max_files as usize);
for entry in archives.iter().take(to_delete) {
    fs::remove_file(entry.path()).ok();
}
```

**Why not use this for numbered rotation?**
- More complex to implement (requires parsing numbers from filenames)
- Higher implementation risk
- Numbered rotation is legacy; timestamped is preferred
- The bounded approach is simpler and sufficient

### Option 3: Consecutive Misses Threshold

Stop after N consecutive missing files instead of just one:

```rust
let mut consecutive_misses = 0;
const MAX_CONSECUTIVE_MISSES: u32 = 3;

for i in (max_files + 1)..(max_files + 1 + MAX_LOG_CLEANUP_ITERATIONS) {
    let old_file = format!("{}.{}", log_path, i);
    let old_gz = format!("{}.gz", old_file);
    
    if !Path::new(&old_file).exists() && !Path::new(&old_gz).exists() {
        consecutive_misses += 1;
        if consecutive_misses >= MAX_CONSECUTIVE_MISSES {
            break;
        }
        continue;
    }
    
    consecutive_misses = 0;
    fs::remove_file(&old_file).ok();
    fs::remove_file(&old_gz).ok();
}
```

**Why not recommended:**
- More complex logic
- Still needs the bounded limit anyway
- Minimal benefit over simple bounded approach
- Harder to reason about behavior

---

## Files to Modify

### Primary Change

**File:** [`packages/kodegend/src/service.rs`](../packages/kodegend/src/service.rs)

**Changes:**
1. Add `MAX_LOG_CLEANUP_ITERATIONS` constant (after imports, before `ServiceError` enum)
2. Replace unbounded range at line 497 with bounded range

**Total lines changed:** 1 line modified, 4 lines added (constant + documentation)

---

## Definition of Done

The task is complete when:

1. ✅ The constant `MAX_LOG_CLEANUP_ITERATIONS` is defined in [`service.rs`](../packages/kodegend/src/service.rs)
2. ✅ The loop at line 497 uses the bounded range: `(max_files + 1)..(max_files + 1 + MAX_LOG_CLEANUP_ITERATIONS)`
3. ✅ The code compiles without warnings: `cargo check` in `packages/kodegend/`
4. ✅ The code passes clippy: `cargo clippy` in `packages/kodegend/`
5. ✅ Manual verification: Create a service with `max_files=5`, manually create log files `.1` through `.150`, trigger rotation, verify cleanup completes quickly and only iterates up to the limit

**Success criteria:**
- Cleanup loop has a guaranteed upper bound
- No performance regression for normal cases (early `break` on missing file still works)
- Pathological cases (hundreds of old logs) are handled gracefully with bounded iteration

---

## Context and Background

### How Log Rotation Works in kodegend

1. **Rotation trigger:** Every 3600 seconds (1 hour), the `rotate_tick` fires (line 74)
2. **Rotation execution:** `rotate_logs()` is called, which calls `rotate_single_log()` for each service (line 88)
3. **Rotation strategies:**
   - **Numbered:** `service.log` → `service.log.1`, `service.log.1` → `service.log.2`, etc.
   - **Timestamped:** `service.log` → `service.log.20250118_143022`

4. **Cleanup phase:**
   - **Numbered:** Delete files beyond `max_files` by iterating upward from `max_files+1`
   - **Timestamped:** Scan directory, sort by time, delete oldest beyond `max_files`

### Configuration Structure

**Type:** [`LogRotationConfig`](../packages/kodegend/src/config.rs) (lines 420-426)

```rust
pub struct LogRotationConfig {
    pub max_size_mb: u64,      // Size threshold for rotation
    pub max_files: u32,        // Number of rotated files to keep
    pub interval_days: u32,    // Not currently used in rotation logic
    pub compress: bool,        // Whether to gzip rotated files
    pub timestamp: bool,       // Use timestamp vs numbered strategy
}
```

**Typical values:** Based on industry standards, `max_files` is typically 4-7 for production systems.

---

## Research Citations

### Industry Standards

1. **Logrotate default behavior:** "rotate 4 - Keeps 4 old versions of the log"  
   Source: [Logrotate: Simplifying Log Management](https://blog.victormendonca.com/2024/09/09/logrotate-simplifying-log-management/)

2. **Log rotation best practices:** Weekly rotation with 4-7 files retained  
   Source: [Mastering Log Rotation in Linux with Logrotate](https://www.dash0.com/guides/log-rotation-linux-logrotate)

### Rust Safety Patterns

3. **Bounded operations:** Rust safety best practices recommend bounded loops for filesystem operations  
   Source: [Rust Security Features and Best Practices in 2025](https://andrewodendaal.com/rust-security-features-best-practices/)

4. **Memory safety in cleanup operations:** Bounded model checking is standard for safe Rust code  
   Source: [UnsafeCop: Towards Memory Safety for Real-World Unsafe Rust Code](https://dl.acm.org/doi/10.1007/978-3-031-71177-0_19)

---

## Related Code References

### Key Files
- Main implementation: [`packages/kodegend/src/service.rs`](../packages/kodegend/src/service.rs)
- Configuration types: [`packages/kodegend/src/config.rs`](../packages/kodegend/src/config.rs)
- Service worker spawn: [`packages/kodegend/src/service.rs`](../packages/kodegend/src/service.rs) lines 44-70

### Key Functions
- `ServiceWorker::run()` - Main service loop (line 72)
- `rotate_single_log()` - Log rotation implementation (line 419)
- Rotation tick setup: line 74 - `let rotate_tick = tick(Duration::from_secs(3600));`
- Rotation execution: line 88 - `recv(rotate_tick) -> _ => self.rotate_logs()?`

### Existing Patterns
- **Good example of bounded cleanup:** Timestamped rotation at lines 510-535 uses directory scan
- **Pattern to avoid:** Unbounded iteration relying only on filesystem state

---

## Implementation Checklist

- [ ] Add `MAX_LOG_CLEANUP_ITERATIONS` constant to [`service.rs`](../packages/kodegend/src/service.rs)
- [ ] Update line 497 to use bounded range
- [ ] Run `cargo check` in `packages/kodegend/`
- [ ] Run `cargo clippy` in `packages/kodegend/`
- [ ] Verify the change compiles cleanly
- [ ] Review the diff to ensure only the intended lines changed
- [ ] Commit the change with message: "Fix unbounded loop in numbered log rotation cleanup"

---

## Notes

- **No tests required:** This is a safety/performance fix with clear mechanical changes
- **No benchmarks required:** The performance improvement is self-evident (bounded vs unbounded)
- **No documentation required:** The code change is self-documenting with the constant name and comment
- **Scope:** Only modify numbered rotation cleanup; leave timestamped rotation unchanged
- **Backward compatibility:** Full backward compatibility; normal operations unchanged
- **Risk level:** Very low - conservative bound, early exit still works, no behavior change for normal cases
