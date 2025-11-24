# LOW: Repeated Environment Variable Lookups Without Caching

## Severity
**LOW - PERFORMANCE OPTIMIZATION**

## Location
- `packages/kodegend/src/platform/unix.rs:86` (XDG_RUNTIME_DIR)
- `packages/kodegend/src/platform/windows.rs:132` (ProgramData)

## Issue Description
Path functions repeatedly call `std::env::var()` every time they're invoked, without caching results. While environment variable lookups are relatively fast, they involve system calls and could be optimized for frequently-called functions.

### Current Code - Unix
```rust
pub(super) fn platform_runtime_dir(is_elevated: bool) -> PathBuf {
    if is_elevated {
        PathBuf::from("/var/run/kodegend")
    } else {
        std::env::var("XDG_RUNTIME_DIR")  // ← Lookup every call
            .ok()
            .map(PathBuf::from)
            .or_else(dirs::runtime_dir)
            .unwrap_or_else(|| {
                PathBuf::from(format!("/tmp/kodegend-{}", geteuid()))
            })
            .join("kodegend")
    }
}
```

### Current Code - Windows
```rust
pub(super) fn platform_system_config_dir() -> PathBuf {
    std::env::var("ProgramData")  // ← Lookup every call
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("C:\\ProgramData"))
        .join("kodegend")
}
```

### Performance Impact

**Environment variable lookup cost:**
- System call overhead: ~100-500ns per lookup
- String allocation: ~50-200ns
- Path construction: ~100ns

**Total per call:** ~250-800ns

**Frequency:**
- `runtime_dir()`: Called for PID file access, socket paths - possibly hundreds of times
- `log_dir()`: Called per log message if not cached by logger
- `system_config_dir()`: Called for config reads

**Estimated savings:** If called 1000 times during daemon lifetime, saves ~0.25-0.8ms total. Negligible.

## When This Matters
- **Embedded systems**: Every microsecond counts
- **Hot path**: If called in tight loop (unlikely)
- **Container startup**: Repeated daemon restarts
- **Benchmarking**: Visible in microbenchmarks

## Recommended Fix (If Optimizing)

### Option 1: Lazy static cache
```rust
use once_cell::sync::Lazy;

static XDG_RUNTIME_DIR: Lazy<Option<PathBuf>> = Lazy::new(|| {
    std::env::var("XDG_RUNTIME_DIR")
        .ok()
        .map(PathBuf::from)
});

pub(super) fn platform_runtime_dir(is_elevated: bool) -> PathBuf {
    if is_elevated {
        PathBuf::from("/var/run/kodegend")
    } else {
        XDG_RUNTIME_DIR
            .clone()
            .or_else(|| dirs::runtime_dir())
            .unwrap_or_else(|| {
                PathBuf::from(format!("/tmp/kodegend-{}", geteuid()))
            })
            .join("kodegend")
    }
}
```

### Option 2: Cache at first call
```rust
use std::sync::OnceLock;

static RUNTIME_DIR: OnceLock<PathBuf> = OnceLock::new();

pub(super) fn platform_runtime_dir(is_elevated: bool) -> PathBuf {
    RUNTIME_DIR.get_or_init(|| {
        if is_elevated {
            PathBuf::from("/var/run/kodegend")
        } else {
            std::env::var("XDG_RUNTIME_DIR")
                .ok()
                .map(PathBuf::from)
                .or_else(dirs::runtime_dir)
                .unwrap_or_else(|| {
                    PathBuf::from(format!("/tmp/kodegend-{}", geteuid()))
                })
                .join("kodegend")
        }
    }).clone()
}
```

### Option 3: Compute once in config struct
```rust
pub struct PlatformPaths {
    runtime_dir: PathBuf,
    log_dir: PathBuf,
    config_dir: PathBuf,
}

impl PlatformPaths {
    pub fn new(is_elevated: bool) -> Self {
        Self {
            runtime_dir: compute_runtime_dir(is_elevated),
            log_dir: compute_log_dir(is_elevated),
            config_dir: compute_config_dir(is_elevated),
        }
    }
}

// Then cache PlatformPaths instance instead of calling functions repeatedly
```

## Recommendation

**Do NOT optimize unless:**
1. Profiling shows this is actually a bottleneck
2. These functions are called in hot path
3. Targeting resource-constrained environment

**Current implementation is fine:**
- Environment variables rarely change during process lifetime
- Lookup cost is negligible (< 1 microsecond)
- Premature optimization adds complexity
- Code is clearer without caching

## If You Must Optimize

Benchmark first:
```rust
#[bench]
fn bench_runtime_dir_lookup(b: &mut Bencher) {
    b.iter(|| {
        platform_runtime_dir(false)
    });
}
```

## Impact
- **Severity**: LOW - Micro-optimization
- **Savings**: < 1 microsecond per call
- **Complexity**: Adds caching logic
- **Priority**: VERY LOW - Don't do this unless profiling shows need

## Files to Modify
- `packages/kodegend/src/platform/unix.rs`
- `packages/kodegend/src/platform/windows.rs`

## Conclusion
This is NOT a real issue. Current code is fine. Only optimize if profiling shows actual bottleneck.

**Priority: WONTFIX** (premature optimization)
