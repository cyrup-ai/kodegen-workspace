# Task: Add cfg(unix) Gate to Constants Import in daemon.rs

## Priority: P3 (Style/Quality)

## Related Error
- `daemon.rs:83` - unused import `crate::constants::*`

## Problem Statement

The `daemon.rs` file imports all constants:
```rust
use crate::constants::*;
```

However, most constants in `constants.rs` are Unix-specific:
- `PID_FILE_MODE` - Unix file permissions
- `PID_DIR_MODE` - Unix directory permissions
- `READINESS_SIGNAL` - Unix signal for systemd readiness
- etc.

Windows uses SCM (Service Control Manager) instead of these Unix mechanisms.

## Analysis

Looking at `src/constants.rs`, the constants are already marked with `#[cfg(unix)]`:
```rust
#[cfg(unix)]
pub const PID_FILE_MODE: u32 = 0o644;

#[cfg(unix)]
pub const PID_DIR_MODE: u32 = 0o755;

#[cfg(unix)]
pub const READINESS_SIGNAL: i32 = libc::SIGUSR1;
```

The import statement in `daemon.rs` pulls in ALL constants, but on Windows there's nothing to import (or only a few Windows-applicable constants).

## Required Implementation

### Option A: Conditional Import (Recommended)

```rust
#[cfg(unix)]
use crate::constants::*;
```

This completely skips the import on Windows.

### Option B: Specific Imports

If some constants ARE needed on Windows, import them specifically:
```rust
#[cfg(unix)]
use crate::constants::{PID_FILE_MODE, PID_DIR_MODE, READINESS_SIGNAL, ...};

// Any Windows-specific constants would be imported unconditionally
use crate::constants::{SOME_CROSS_PLATFORM_CONSTANT};
```

### Option C: Add Windows Constants

If Windows needs equivalent constants (e.g., for ACL permissions), add them to `constants.rs`:
```rust
#[cfg(windows)]
pub const PID_FILE_ACL: &str = "D:P(A;;FA;;;SY)(A;;FA;;;BA)"; // SDDL string
```

## Files to Modify

- `src/daemon.rs:83` - Add `#[cfg(unix)]` to the import

## Verification

After the change, run:
```bash
cargo check --target x86_64-pc-windows-msvc
```

The "unused import" warning should be resolved.

## Acceptance Criteria

- [ ] No "unused import: crate::constants::*" warning on Windows
- [ ] Unix build still works correctly
- [ ] If Windows constants are needed in future, there's a clear pattern to follow
