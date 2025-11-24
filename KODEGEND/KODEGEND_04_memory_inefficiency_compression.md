# HIGH: Memory Inefficiency in Log Compression - Entire File Loaded Into RAM

**Priority:** HIGH  
**Component:** `packages/kodegend/src/service.rs`  
**Lines:** 473-492  
**Impact:** Performance/Resource - Memory spikes and potential OOM

## Problem

The log compression implementation in `kodegend` loads entire log files into memory during rotation, which can cause massive memory spikes and potential out-of-memory (OOM) conditions when rotating large log files. This is a critical issue for a daemon process that should maintain a minimal and predictable memory footprint.

### Problematic Code

**File:** `/packages/kodegend/src/service.rs`  
**Lines:** 473-492

```rust
if compress {
    // Use flate2 for gzip compression (already in Cargo.toml)
    use std::io::Write;
    use flate2::Compression;
    use flate2::write::GzEncoder;
    
    // Read the rotated file
    let input = fs::read(&rotated_name)?;  // ← ENTIRE FILE IN MEMORY
    
    // Write compressed version
    let output_path = format!("{}.gz", rotated_name);
    let output_file = fs::File::create(&output_path)?;
    let mut encoder = GzEncoder::new(output_file, Compression::default());
    encoder.write_all(&input)?;  // ← WRITE ENTIRE BUFFER AT ONCE
    encoder.finish()?;  // Flush and finalize gzip stream
    
    // Remove uncompressed file (only keep .gz)
    fs::remove_file(&rotated_name)?;
}
```

### Root Cause

`fs::read(&rotated_name)` allocates a **single contiguous `Vec<u8>`** containing the entire log file contents. This happens during log rotation, which is triggered when the file reaches `max_size_mb` (configured per service).

**Memory Allocation Behavior:**
- `fs::read()` performs a single `read_to_end()` operation
- Allocates memory equal to file size (e.g., 500 MB file = 500 MB allocation)
- Additional memory overhead from `GzEncoder` internal buffers (compression state, output buffering)
- Total memory spike can be **1.5-2x** the log file size during compression

### Memory Impact Analysis

**Single Service Rotation:**

| max_size_mb | File Size | Memory Allocated | Peak Usage (with compression) | Impact |
|-------------|-----------|------------------|------------------------------|---------|
| 10 MB | 10 MB | 10 MB | ~15 MB | Minor |
| 100 MB | 100 MB | 100 MB | ~150 MB | Noticeable spike |
| 500 MB | 500 MB | 500 MB | ~750 MB | Significant spike |
| 1 GB | 1 GB | 1 GB | ~1.5 GB | High risk of OOM |

**Multi-Service Concurrent Rotation:**

The daemon manages multiple services that all check for rotation on the same tick (hourly by default, see line 74). If multiple services hit `max_size_mb` around the same time, their memory spikes compound:

| Scenario | Services | max_size_mb Each | Simultaneous Allocation | System Risk |
|----------|----------|------------------|------------------------|-------------|
| Small deployment | 3 services | 50 MB | 150 MB | Low |
| Medium deployment | 5 services | 100 MB | 500 MB | Medium |
| Large deployment | 10 services | 200 MB | 2 GB | High (OOM likely) |
| Production scale | 20 services | 500 MB | 10 GB | Critical (system crash) |

### When This Occurs

- **Frequency:** Every hour when `rotate_tick` fires (configured at line 74: 3600 seconds)
- **Trigger:** When log file size exceeds `max_size_mb` (checked at line 439)
- **Concurrency:** All managed services check rotation on the same tick, leading to synchronized allocations
- **Worst Case:** All services started simultaneously, all writing logs at similar rates, all hitting rotation threshold at the same hour

### Real-World Failure Scenario

```
Production System: 4GB RAM, 8 services with max_size_mb=500
Normal daemon memory: 200 MB
Available RAM: ~2.5 GB (after OS and other processes)

Hour 72 - Multiple services hit 500 MB log size simultaneously
12:00:00 - rotate_tick fires for all 8 services
12:00:01 - Service 1: allocates 500 MB (total daemon: 700 MB)
12:00:01 - Service 2: allocates 500 MB (total daemon: 1.2 GB)
12:00:02 - Service 3: allocates 500 MB (total daemon: 1.7 GB)
12:00:02 - Service 4: allocates 500 MB (total daemon: 2.2 GB)
12:00:03 - Service 5: attempts 500 MB allocation
12:00:03 - System OOM killer activates
12:00:04 - Random process terminated (could be database, web server, or kodegend itself)
```

### Additional Technical Issues

1. **Allocation Latency:** Large allocations (>100 MB) can cause page faults and memory pressure, slowing down the entire system
2. **Fragmentation:** Repeated large allocations and deallocations can fragment the heap, degrading performance over time
3. **GC Pressure:** On systems with swap, large allocations can trigger excessive swapping, causing severe performance degradation
4. **Daemon Philosophy Violation:** A well-behaved daemon should have **constant, predictable memory usage** - this violates that principle

## Solution: Stream-Based Compression

### Core Approach

Replace the memory-loading approach with **streaming I/O** that processes the file in small chunks, maintaining constant memory usage regardless of log file size.

**Key Principle:** Use `BufReader` + `BufWriter` + `io::copy()` to stream data from disk → through compressor → back to disk in fixed-size chunks (typically 64 KB).

### Recommended Implementation (Option 1: Streaming with flate2)

This is the recommended approach as it:
- Uses existing `flate2` dependency (already in Cargo.toml)
- Requires minimal code changes
- Maintains identical `.gz` output format
- Provides constant memory usage

**Replace lines 473-492 with:**

```rust
if compress {
    use std::io::{BufReader, BufWriter, copy};
    use flate2::Compression;
    use flate2::write::GzEncoder;
    
    // Buffer size: 64 KB is optimal for most filesystems
    // - Large enough to amortize syscall overhead
    // - Small enough to avoid memory pressure
    // - Matches typical filesystem block size (4-64 KB)
    const BUFFER_SIZE: usize = 64 * 1024;
    
    // Open input file for reading
    let input_file = fs::File::open(&rotated_name)
        .context("open rotated log for compression")?;
    let mut input = BufReader::with_capacity(BUFFER_SIZE, input_file);
    
    // Open output file for writing
    let output_path = format!("{}.gz", rotated_name);
    let output_file = fs::File::create(&output_path)
        .context("create compressed output file")?;
    let output = BufWriter::with_capacity(BUFFER_SIZE, output_file);
    let mut encoder = GzEncoder::new(output, Compression::default());
    
    // Stream copy: reads and compresses in 64 KB chunks
    // This maintains constant memory usage regardless of file size
    copy(&mut input, &mut encoder)
        .context("stream compress log file")?;
    
    // Flush and finalize gzip stream
    encoder.finish()
        .context("finalize gzip compression")?;
    
    // Remove uncompressed file (only keep .gz)
    fs::remove_file(&rotated_name)
        .context("remove uncompressed log after compression")?;
}
```

**Memory Usage:**
- Input buffer: 64 KB
- Output buffer: 64 KB
- GzEncoder internal state: ~32 KB (compression dictionary and state)
- **Total peak memory: ~128 KB** (constant, regardless of file size)

### Alternative Implementation (Option 2: External gzip)

If minimizing Rust code complexity is preferred, you can delegate to the system `gzip` command:

```rust
if compress {
    use std::process::Command;
    
    let output = Command::new("gzip")
        .arg("--best")          // Maximum compression
        .arg("--force")         // Overwrite if exists
        .arg(&rotated_name)     // Input file
        .output()
        .context("spawn gzip process")?;
    
    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "gzip compression failed: {}", 
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    
    // Note: gzip automatically creates .gz and removes original
    // No need to manually remove rotated_name
}
```

**Pros:**
- Zero daemon memory overhead (compression runs in separate process)
- Leverages highly optimized system gzip implementation
- Minimal Rust code

**Cons:**
- External dependency (requires `gzip` in PATH)
- Less control over compression level
- Harder to handle errors gracefully
- Platform-specific behavior differences

**Recommendation:** Use **Option 1 (streaming with flate2)** for production because:
- No external dependencies
- Predictable cross-platform behavior
- Better error handling and context
- Minimal memory overhead (128 KB vs. 500 MB+)
- Already using flate2 elsewhere in the codebase

## Existing Patterns in Codebase

The codebase **already uses streaming I/O patterns** in multiple places. The log compression code is an **outlier** that doesn't follow these established patterns.

### Example 1: Streaming Decompression (extract.rs)

**File:** [`/packages/kodegend/src/install/download/extract.rs`](../packages/kodegend/src/install/download/extract.rs)  
**Lines:** 45-50

```rust
let tar_gz_file = std::fs::File::open(&data_tar_gz_clone)?;
let tar = GzDecoder::new(tar_gz_file);  // ← Streaming decompression
let mut archive = Archive::new(tar);
archive.unpack(&extract_dir_clone)?;
```

This code uses `GzDecoder` with a file handle directly - it **does not** load the entire `.tar.gz` into memory first. This is the **inverse operation** of what we need to fix (decompress vs compress), but demonstrates the same streaming pattern.

### Example 2: Streaming Copy Between Processes (extract.rs)

**File:** [`/packages/kodegend/src/install/download/extract.rs`](../packages/kodegend/src/install/download/extract.rs)  
**Lines:** 96-99

```rust
// Spawn task to copy data between processes
tokio::spawn(async move {
    let _ = tokio::io::copy(&mut rpm2cpio_stdout, &mut cpio_stdin).await;
});
```

Uses `tokio::io::copy()` to stream data between process pipes without buffering entire output in memory.

### Example 3: Streaming ZIP Extraction (extract.rs)

**File:** [`/packages/kodegend/src/install/download/extract.rs`](../packages/kodegend/src/install/download/extract.rs)  
**Lines:** 283-284

```rust
std::io::copy(&mut file, &mut outfile)
    .context("Failed to extract binary from ZIP")?;
```

Uses `std::io::copy()` to stream file extraction from ZIP archive directly to disk.

### Pattern Consistency

All three examples demonstrate that the codebase **already follows streaming I/O best practices**:
- No calls to `fs::read()` for large files
- Use of `BufReader`, `BufWriter`, and `io::copy()`
- Constant memory usage regardless of file size

The log compression code at lines 473-492 should be updated to **match these established patterns**.

## Benefits of Streaming Approach

### 1. Constant Memory Usage

| Approach | Memory (10 MB file) | Memory (100 MB file) | Memory (1 GB file) | Memory (10 GB file) |
|----------|-------------------|---------------------|-------------------|---------------------|
| Current (`fs::read`) | 10 MB | 100 MB | 1 GB | 10 GB |
| Streaming | **128 KB** | **128 KB** | **128 KB** | **128 KB** |

Memory usage becomes **O(1)** instead of **O(n)** relative to file size.

### 2. Scalability

- Can compress files larger than available RAM
- No risk of OOM conditions
- Predictable performance characteristics

### 3. Concurrent Safety

Multiple services can rotate and compress simultaneously without memory pressure:

```
10 services × 128 KB each = 1.28 MB total
vs.
10 services × 500 MB each = 5 GB total (current)
```

### 4. System Stability

- No memory spikes that could trigger OOM killer
- No swap pressure from large allocations
- Better cache utilization (data stays in filesystem cache)

### 5. Same Functionality

- Produces **identical** gzip output (bit-for-bit compatible)
- Same `.gz` file format
- Same compression ratio
- Same decompression compatibility

## Implementation Steps

### Step 1: Locate the Code

Open `/packages/kodegend/src/service.rs` and navigate to the `rotate_single_log()` function around line 473.

### Step 2: Replace the Compression Block

Replace lines 473-492 (the entire `if compress { ... }` block) with the streaming implementation from **Option 1** above.

### Step 3: Verify Imports

Ensure the following imports are available at the top of `service.rs`:
- `use std::fs;` (already present at line 426)
- `use std::io::{BufReader, BufWriter, copy};` (add if needed)
- `use flate2::Compression;` (already present in block)
- `use flate2::write::GzEncoder;` (already present in block)

The `use` statements inside the `if compress` block are fine - they're scoped appropriately.

### Step 4: Preserve Error Context

The new implementation uses `.context()` from `anyhow` crate for better error messages. Ensure `anyhow::Context` is imported (should already be imported at the top of the file from line 9).

### Step 5: Verify Cleanup Logic

The code following the compression block (lines 494-540) handles cleanup of old rotated files. **Do not modify** this section - it correctly handles both `.gz` and uncompressed files.

## Technical References

### flate2 Documentation
- **flate2 crate**: <https://docs.rs/flate2/latest/flate2/>
- **I/O Integration**: Explains how flate2 integrates with Rust's `Read` and `Write` traits for streaming compression
- **GzEncoder**: <https://docs.rs/flate2/latest/flate2/write/struct.GzEncoder.html>

### Streaming I/O Best Practices
- **BufReader/BufWriter**: Standard library types that reduce syscall overhead by batching I/O operations
- **io::copy()**: Standard library function that efficiently copies data between `Read` and `Write` in chunks
- **Buffer Size**: 64 KB is optimal for most filesystems (matches typical block sizes and amortizes syscall overhead)

### Existing Dependencies

From [`/packages/kodegend/Cargo.toml`](../packages/kodegend/Cargo.toml):

**Line 67-69:**
```toml
flate2 = { version = "1", default-features = false, features = [
  "rust_backend",
] }
```

- `rust_backend` feature uses `miniz_oxide` (pure Rust implementation)
- No external C dependencies required
- Performance comparable to native zlib

## Definition of Done

The task is complete when:

1. **Code Changed:** Lines 473-492 in `/packages/kodegend/src/service.rs` replaced with streaming implementation
2. **Memory Usage:** Compression uses constant ~128 KB memory regardless of log file size
3. **Functionality Preserved:** 
   - Still produces `.gz` compressed files
   - Cleanup logic still works (lines 494-540 unchanged)
   - Error handling still provides useful context
4. **No Regressions:**
   - Rotation still triggers at correct file size
   - Numbered and timestamped rotation strategies both work
   - Compressed and uncompressed files still cleaned up correctly
5. **Code Compiles:** `cargo check` passes for `packages/kodegend`
6. **No Breaking Changes:** 
   - Same `.gz` output format
   - Same configuration options
   - Same behavior from external perspective

## Additional Context

### Configuration Reference

Log rotation configuration is defined in service definitions (see `/packages/kodegend/src/config.rs` line 424):

```rust
pub struct LogRotation {
    pub max_size_mb: u64,     // File size threshold for rotation
    pub max_files: u32,       // Number of old logs to keep
    pub compress: bool,       // Whether to gzip rotated logs
    pub timestamp: bool,      // Use timestamp vs numbered naming
}
```

### Rotation Tick

Rotation checks happen on a regular tick defined at line 74:
```rust
let rotate_tick = tick(Duration::from_secs(3600));  // Every hour
```

This means all services check for rotation simultaneously every hour, which is why concurrent memory spikes are a real concern.

### Related Code Locations

- **Line 385-396:** `rotate_logs()` function that calls `rotate_single_log()` for each service
- **Line 417:** `rotate_single_log()` function signature
- **Line 439:** Size check that determines if rotation is needed
- **Line 455-467:** Filename generation for rotated logs (numbered or timestamped)
- **Line 494-540:** Cleanup of old rotated logs (handles both `.gz` and uncompressed)

## Notes

- This fix addresses **memory efficiency only** - compression ratio and speed remain unchanged
- The streaming approach has **negligible performance impact** (often faster due to better cache utilization)
- After this fix, log rotation will be safe even for GB-sized log files on constrained systems
- The fix aligns with Rust best practices and matches existing patterns in the codebase
