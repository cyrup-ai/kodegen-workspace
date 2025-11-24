# MEDIUM: Non-Atomic Log Compression Leaves Orphaned Files

**Priority:** MEDIUM  
**Component:** `packages/kodegend/src/service.rs`  
**Lines:** 469-492  
**Impact:** Robustness - Crash during compression leaves inconsistent state

## Problem

The log rotation compression process is not atomic - it performs multiple steps (rename, compress, delete) that can be interrupted, leaving the filesystem in an inconsistent state with orphaned uncompressed files.

### Problematic Code

**Lines 469-492:**
```rust
// Rename current log to rotated name
// The service will automatically create a new file on next write
fs::rename(path, &rotated_name)?;  // ← STEP 1

// Compress if requested
if compress {
    // ... setup ...
    
    // Read the rotated file
    let input = fs::read(&rotated_name)?;  // ← STEP 2
    
    // Write compressed version
    let output_path = format!("{}.gz", rotated_name);
    let output_file = fs::File::create(&output_path)?;
    let mut encoder = GzEncoder::new(output_file, Compression::default());
    encoder.write_all(&input)?;
    encoder.finish()?;  // ← STEP 3 (flush and finalize gzip stream)
    
    // Remove uncompressed file (only keep .gz)
    fs::remove_file(&rotated_name)?;  // ← STEP 4
}
```

### Race Conditions and Failure Modes

#### Scenario 1: Crash After Rename, Before Compression

1. `fs::rename(path, &rotated_name)` succeeds (line 471)
2. Daemon crashes (OOM, SIGKILL, power loss, etc.)
3. **Result:** `service.log.1` exists (uncompressed), `service.log.1.gz` does not exist

**Impact:**
- Rotated file exists but is not compressed
- Wastes disk space (should have been compressed)
- Next rotation sees uncompressed file and shifts it (lines 456-463)
- File never gets compressed automatically

#### Scenario 2: Crash During Compression Write

1. Rename succeeds
2. Compression starts writing to `.gz` file
3. Daemon crashes mid-write
4. **Result:** 
   - `service.log.1` exists (uncompressed, original)
   - `service.log.1.gz` exists (incomplete, corrupted)

**Impact:**
- Disk space wasted (both files exist)
- Compressed file is corrupt, can't be decompressed
- Next rotation might try to shift both files

#### Scenario 3: Crash After Compression, Before Delete

1. Rename succeeds
2. Compression completes successfully
3. Daemon crashes before `fs::remove_file(&rotated_name)` (line 491)
4. **Result:**
   - `service.log.1` exists (uncompressed, original)
   - `service.log.1.gz` exists (compressed, valid)

**Impact:**
- Both files exist, wasting disk space
- Which one should rotation use on next run?
- Lines 456-463 check for both `.gz` and uncompressed - might shift both

#### Scenario 4: I/O Error During Compression

1. Rename succeeds
2. Compression fails (disk full, I/O error, permission denied)
3. `.gz` file partially written or not created
4. Code returns error, but uncompressed file already renamed

**Impact:**
- Uncompressed file exists at rotated location
- Should have been compressed but wasn't
- No recovery mechanism

### Current "Recovery" Mechanism

**Lines 456-463:** The shifting logic checks for both compressed and uncompressed:

```rust
if Path::new(&old).exists() {
    fs::rename(&old, &new).ok();
}
let old_gz = format!("{}.gz", old);
let new_gz = format!("{}.gz", new);
if Path::new(&old_gz).exists() {
    fs::rename(&old_gz, &new_gz).ok();
}
```

This handles orphaned files by shifting them, but:
- Doesn't compress orphaned uncompressed files
- Doesn't validate compressed files
- Doesn't deduplicate (both files shifted)

## Solutions

### Option 1: Atomic Rename Pattern (Recommended)

Compress to a temporary file, then atomically rename:

```rust
if compress {
    use std::io::{BufReader, BufWriter, copy};
    use flate2::Compression;
    use flate2::write::GzEncoder;
    
    // Compress to temporary file
    let temp_compressed = format!("{}.gz.tmp", rotated_name);
    
    // Stream-compress to temp file (also fixes memory issue)
    {
        let input_file = fs::File::open(&rotated_name)?;
        let input = BufReader::new(input_file);
        
        let output_file = fs::File::create(&temp_compressed)?;
        let output = BufWriter::new(output_file);
        let mut encoder = GzEncoder::new(output, Compression::default());
        
        copy(&mut input, &mut encoder)
            .context("compress log file")?;
        
        encoder.finish()?;
    }  // Files closed here
    
    // Atomic rename: temp.gz.tmp → temp.gz
    let final_compressed = format!("{}.gz", rotated_name);
    fs::rename(&temp_compressed, &final_compressed)
        .context("finalize compressed log")?;
    
    // Now safe to delete original (compressed version is guaranteed complete)
    fs::remove_file(&rotated_name)
        .context("remove uncompressed rotated log")?;
}
```

**Benefits:**
- Compressed file only visible after it's complete
- If crash occurs during compression, `.tmp` file is orphaned (cleanup later)
- Original uncompressed file remains until compression confirmed successful
- Next rotation can recognize `.tmp` files and clean them up

**Crash recovery:**
```rust
// At start of rotate_single_log(), clean up orphaned temp files
let temp_pattern = format!("{}.*.tmp", log_path);
for temp_file in glob::glob(&temp_pattern)? {
    if let Ok(path) = temp_file {
        warn!("Cleaning up orphaned temp file: {:?}", path);
        fs::remove_file(path).ok();
    }
}
```

### Option 2: Idempotent Compression Check

Make compression idempotent by checking if compressed version already exists:

```rust
if compress {
    let compressed_path = format!("{}.gz", rotated_name);
    
    // Check if already compressed (recovery from previous crash)
    if Path::new(&compressed_path).exists() {
        // Compressed version exists, just delete uncompressed
        warn!("Compressed log already exists, removing uncompressed: {}", rotated_name);
        fs::remove_file(&rotated_name)?;
        return Ok(());
    }
    
    // ... normal compression logic ...
}
```

**Benefits:**
- Handles Scenario 3 (crash after compression, before delete)
- Simple check

**Cons:**
- Doesn't handle Scenario 2 (corrupt .gz file)
- Doesn't validate .gz file integrity

### Option 3: Checksum Validation

Validate compressed file before deleting original:

```rust
if compress {
    // ... compress to .gz file ...
    
    // Verify compressed file is valid
    let compressed_path = format!("{}.gz", rotated_name);
    let decompressed_size = verify_gzip_file(&compressed_path)?;
    
    let original_size = fs::metadata(&rotated_name)?.len();
    if decompressed_size != original_size {
        return Err(anyhow::anyhow!(
            "Compressed file size mismatch: {} bytes original, {} bytes decompressed",
            original_size, decompressed_size
        ));
    }
    
    // Compressed file verified, safe to delete original
    fs::remove_file(&rotated_name)?;
}

fn verify_gzip_file(path: &str) -> Result<u64> {
    use flate2::read::GzDecoder;
    use std::io::Read;
    
    let file = fs::File::open(path)?;
    let mut decoder = GzDecoder::new(file);
    let mut count = 0u64;
    let mut buf = [0u8; 8192];
    
    loop {
        let n = decoder.read(&mut buf)?;
        if n == 0 { break; }
        count += n as u64;
    }
    
    Ok(count)
}
```

**Benefits:**
- Guarantees compressed file integrity
- Detects corruption

**Cons:**
- Doubles I/O (compress then decompress to verify)
- Slower rotation
- Defeats the purpose of fast compression

## Recommended Solution

**Option 1** (Atomic rename with .tmp files) because:
- Guarantees atomic operation visibility
- No duplicate I/O for verification
- Natural crash recovery (cleanup .tmp files)
- Also fixes the memory issue (can switch to streaming)
- Standard pattern used by many Unix tools

## Implementation Steps

1. Add temp file cleanup at start of `rotate_single_log()`
2. Change compression to write to `.gz.tmp` file
3. Atomically rename `.gz.tmp` → `.gz` after compression completes
4. Only delete original after rename succeeds
5. Add glob dependency if not already present (for cleanup)

## Testing

### Crash Simulation Tests

**Test 1: Crash during compression**
```rust
// Modify code to inject crash
if random::<bool>() {
    panic!("Simulated crash during compression");
}
```

Run rotation many times, verify:
- No corrupt .gz files
- Orphaned .tmp files are cleaned up on next rotation
- No data loss

**Test 2: Disk full during compression**
```bash
# Create small filesystem
truncate -s 100M /tmp/test.img
mkfs.ext4 /tmp/test.img
mount /tmp/test.img /mnt/test

# Configure service to log to /mnt/test
# Fill disk to trigger failure
```

Verify:
- Graceful error handling
- Original file remains intact
- No corrupt .gz files

**Test 3: Permission denied after compression**
```bash
# After compression completes, make file immutable
chattr +i service.log.1

# Try to delete - should fail
```

Verify:
- Error is logged
- Both files remain (better than losing data)

### Idempotency Test

1. Manually create scenario with both files:
   ```bash
   touch service.log.1
   touch service.log.1.gz
   ```

2. Trigger rotation

3. Verify:
   - Correct file is kept
   - Duplicate is handled appropriately

## Impact

- **Prevents data loss** from crashes during rotation
- **Clearer error handling** for I/O failures
- **Automatic recovery** from previous crashes
- **Better disk space management** (no duplicate files)

## References

- Line 469-492: Compression implementation in `rotate_single_log()`
- Line 471: Rename operation
- Line 481-491: Compression and deletion
- Line 456-463: File shifting that handles both compressed and uncompressed
