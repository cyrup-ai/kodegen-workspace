# Linux: String Allocation in Hot Path

## Location
`packages/kodegend/src/control/linux_control.rs`

## Issue Type
Performance Bottleneck

## Severity
Medium

## Description
The `service_name` variable is allocated via `format!("{}.service", SERVICE_NAME)` on every call to:
- `check_status()` (line 12)
- `start_daemon()` (line 33)
- `stop_daemon()` (line 57)
- `restart_daemon()` (line 81)

This creates unnecessary heap allocations in the hot path. Each daemon control operation allocates a new String even though the service name is constant.

## Impact
- Unnecessary heap allocations on every daemon control operation
- Increased memory fragmentation
- Slower performance for high-frequency status checks
- Goes against kodegen's "zero-allocation hot paths" principle from CLAUDE.md

## Recommendation
Use `const` or `lazy_static!`/`once_cell::sync::Lazy` to create the formatted service name once at module initialization:

```rust
use once_cell::sync::Lazy;

static SERVICE_NAME_FULL: Lazy<String> = Lazy::new(|| {
    format!("{}.service", SERVICE_NAME)
});

// Then use &SERVICE_NAME_FULL in all functions
```

Or use a const concat if possible:
```rust
const SERVICE_NAME_FULL: &str = const_format::concatcp!(SERVICE_NAME, ".service");
```

## Related Issues
- Similar issue in Windows implementation (UTF-16 encoding)
