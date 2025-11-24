# MEDIUM: Missing Validation for Log Rotation Configuration Parameters

**Priority:** MEDIUM  
**Component:** `packages/kodegend/src/service.rs`  
**Lines:** 419-425  
**Impact:** Robustness - Invalid configs cause undefined behavior

## Problem

The `rotate_single_log()` function accepts log rotation parameters but performs no validation, allowing invalid configurations to cause unexpected behavior, silent failures, or incorrect rotation.

### Problematic Code

**Lines 419-425:**
```rust
fn rotate_single_log(
    log_path: &str,
    max_size_mb: u64,
    max_files: u32,
    compress: bool,
    timestamp: bool,
) -> Result<()> {
    // NO VALIDATION - parameters used directly
```

### Validation Issues

#### Issue 1: max_size_mb = 0

**Line 439:**
```rust
if size_mb < max_size_mb {
    return Ok(());  // Not large enough to rotate yet
}
```

If `max_size_mb` is 0:
- `size_mb < 0` is always false (size_mb is u64, can't be negative)
- Actually, `size_mb < 0` would never be true because `0 < 0` is false
- Wait, the check is `size_mb < max_size_mb`
- If `max_size_mb = 0`, then `size_mb < 0` is always false (any size >= 0)
- **Result: Rotation attempts on EVERY file, even 1 byte files**
- Massive overhead, logs rotated constantly

#### Issue 2: max_files = 0

**Line 451:**
```rust
for i in (1..max_files).rev() {
    // Shift files
}
```

If `max_files = 0`:
- Range `1..0` is empty (reversed or not)
- No shifting happens
- Jump to line 466: `format!("{}.1", log_path)`
- Current log renamed to `.1`

**Line 497:**
```rust
for i in (max_files + 1).. {
    // This becomes: for i in 1.. {}
```

If `max_files = 0`:
- Cleanup starts at position 1 (max_files + 1 = 1)
- **Immediately deletes the rotated file just created**
- Every rotation creates `.1` then deletes it
- **All rotated logs are lost**

#### Issue 3: max_files = 1

**Line 451:**
```rust
for i in (1..1).rev() {  // Empty range
```

- No shifting (expected for max_files=1)
- Rotation creates `.1`

**Line 497:**
```rust
for i in 2.. {  // Starts at max_files + 1 = 2
```

- Tries to delete `.2`, `.3`, etc.
- None exist
- Works correctly by accident

#### Issue 4: Empty or Invalid log_path

**Line 429:**
```rust
let path = Path::new(log_path);
```

No validation that `log_path` is:
- Non-empty
- Valid UTF-8
- Not a directory
- Contains valid path characters

If `log_path = ""`:
- `Path::new("")` creates path to current directory
- `path.exists()` returns true (line 432)
- `fs::metadata("")` might fail or return directory metadata
- Undefined behavior

If `log_path` contains null bytes or invalid characters:
- Platform-dependent behavior
- Potential security issue on some systems

### Real-World Consequences

**Scenario 1: Typo in config (max_size_mb = 0)**
```yaml
services:
  - name: my-service
    log_rotation:
      max_size_mb: 0  # Typo, meant 100
      max_files: 10
```

Result:
- Rotation runs on every 1-byte file
- Massive I/O overhead
- Logs constantly rotated
- System performance degraded

**Scenario 2: Misconfigured max_files**
```yaml
services:
  - name: my-service
    log_rotation:
      max_size_mb: 100
      max_files: 0  # Mistake, user doesn't understand config
```

Result:
- Rotated logs immediately deleted
- No log history preserved
- Compliance/debugging issues

## Solution

### Validate at Config Load Time

Add validation when loading `ServiceDefinition`:

```rust
// In config loading (crate::config)
impl ServiceDefinition {
    pub fn validate(&self) -> Result<()> {
        // Validate log paths
        if let Some(ref path) = self.log_stdout {
            validate_log_path(path)?;
        }
        if let Some(ref path) = self.log_stderr {
            validate_log_path(path)?;
        }
        
        // Validate log rotation config
        if let Some(ref rotation) = self.log_rotation {
            if rotation.max_size_mb == 0 {
                return Err(anyhow::anyhow!(
                    "Service '{}': log_rotation.max_size_mb must be > 0, got 0",
                    self.name
                ));
            }
            
            if rotation.max_size_mb > 10_000 {  // 10 GB seems excessive
                warn!(
                    "Service '{}': log_rotation.max_size_mb is very large ({}), \
                     this may cause memory issues during compression",
                    self.name, rotation.max_size_mb
                );
            }
            
            if rotation.max_files == 0 {
                return Err(anyhow::anyhow!(
                    "Service '{}': log_rotation.max_files must be > 0, got 0",
                    self.name
                ));
            }
            
            if rotation.max_files > 1000 {
                warn!(
                    "Service '{}': log_rotation.max_files is very large ({})",
                    self.name, rotation.max_files
                );
            }
        }
        
        Ok(())
    }
}

fn validate_log_path(path: &str) -> Result<()> {
    if path.is_empty() {
        return Err(anyhow::anyhow!("Log path cannot be empty"));
    }
    
    if path.contains('\0') {
        return Err(anyhow::anyhow!("Log path contains null byte"));
    }
    
    let path_obj = Path::new(path);
    if path_obj.is_dir() {
        return Err(anyhow::anyhow!(
            "Log path '{}' is a directory, must be a file path",
            path
        ));
    }
    
    Ok(())
}
```

### Defensive Checks in rotate_single_log

Add runtime assertions as a safety net:

```rust
fn rotate_single_log(
    log_path: &str,
    max_size_mb: u64,
    max_files: u32,
    compress: bool,
    timestamp: bool,
) -> Result<()> {
    // Defensive checks (should have been validated earlier)
    if max_size_mb == 0 {
        warn!("rotate_single_log: max_size_mb is 0, skipping rotation");
        return Ok(());
    }
    
    if max_files == 0 {
        warn!("rotate_single_log: max_files is 0, skipping rotation");
        return Ok(());
    }
    
    if log_path.is_empty() {
        return Err(anyhow::anyhow!("Empty log path"));
    }
    
    // ... rest of function
}
```

## Recommended Approach

1. **Primary validation:** At config load time (fail fast on invalid config)
2. **Defensive checks:** Runtime checks in `rotate_single_log` (safety net)
3. **Clear error messages:** Help users fix their config

This follows the principle of "make invalid states unrepresentable" by validating early.

## Testing

### Valid Config Tests

```yaml
# Should pass
max_size_mb: 100
max_files: 10
```

### Invalid Config Tests

```yaml
# Should fail with clear error
max_size_mb: 0
max_files: 10

# Should fail
max_size_mb: 100
max_files: 0

# Should warn
max_size_mb: 50000  # 50 GB - very large
max_files: 10
```

### Edge Cases

1. `max_size_mb: 1` (minimum valid) - should work
2. `max_files: 1` (minimum valid) - should work
3. Empty log_path - should fail
4. Log path with null bytes - should fail
5. Log path pointing to directory - should fail

## Impact

- **Prevents silent failures** when users misconfigure rotation
- **Fails fast** at startup instead of at runtime
- **Better UX** with clear error messages
- **Defensive programming** with runtime checks as safety net

## References

- Line 419-425: `rotate_single_log()` function signature
- Line 439: max_size_mb usage
- Line 451: max_files usage in shifting loop
- Line 497: max_files usage in cleanup loop
- `crate::config::ServiceDefinition`: Config structure (needs investigation)
