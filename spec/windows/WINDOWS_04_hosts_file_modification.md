# Task: Unify Hosts File Modification Approach

## Priority: P1 (Core Functionality)

## Related Errors
- `install/installer/config/hosts.rs:15` - function `check_hosts_entry` never used
- `install/installer/config/hosts.rs:149` - function `add_kodegen_host_entries` never used

## Problem Statement

There are TWO implementations for modifying the Windows hosts file:

### Implementation A: Rust Function (Unused)
`src/install/installer/config/hosts.rs` lines 149-181:
```rust
#[cfg(windows)]
pub fn add_kodegen_host_entries() -> Result<()> {
    // Full Rust implementation with atomic file writes
    // Uses check_hosts_entry() helper
    // Properly handles existing entries
}
```

### Implementation B: Structured Command Protocol (Used)
`src/install/privilege.rs` lines 263-268:
```rust
// Uses APPEND_HOSTS command to helper process
commands.push(format!("APPEND_HOSTS|127.0.0.1 mcp.kodegen.ai"));
```

The structured command approach was introduced to handle UAC elevation securely, but it bypasses the more sophisticated Rust implementation.

## Analysis

### Implementation A Advantages
- Atomic file writes (temp file + rename)
- Idempotent (checks for existing entries)
- Structured Kodegen block with markers
- Proper error handling

### Implementation B Advantages
- Works with UAC elevation
- Simple append operation
- Handled by elevated helper process

### Problem with Implementation B
- No idempotency check (may add duplicate entries)
- No atomic write (corruption on crash)
- No structured block (harder to remove on uninstall)

## Required Implementation

### Option 1: Use Rust Implementation from Elevated Helper

Modify the elevated helper to call `add_kodegen_host_entries()` instead of raw append:

In the helper process command handler:
```rust
"ADD_HOST_ENTRIES" => {
    crate::install::installer::config::hosts::add_kodegen_host_entries()?;
}
```

And in privilege.rs:
```rust
commands.push("ADD_HOST_ENTRIES".to_string());
```

### Option 2: Enhance Structured Command

If keeping the command protocol, enhance it:
```rust
// In helper process
"APPEND_HOSTS" => {
    let entry = parts[1]; // "127.0.0.1 mcp.kodegen.ai"
    let (ip, hostname) = entry.split_once(' ').unwrap();

    // Check if already exists
    if !check_hosts_entry_exists(ip, hostname)? {
        append_to_hosts_atomic(entry)?;
    }
}
```

### Option 3: Remove Unused Implementation

If the structured command approach is preferred and works correctly:
1. Remove `add_kodegen_host_entries()` from hosts.rs
2. Remove `check_hosts_entry()` helper
3. Document why the simpler approach is sufficient

## Recommendation

**Option 1** is recommended because:
- Reuses tested code
- Provides proper atomic writes
- Maintains idempotency
- Easier uninstall (structured block)

## Files to Modify

- `src/install/privilege.rs` - Change command from `APPEND_HOSTS` to `ADD_HOST_ENTRIES`
- Helper process code - Add handler for `ADD_HOST_ENTRIES`
- Or remove unused code if Option 3

## Testing

1. Fresh install - verify hosts entry added
2. Reinstall - verify no duplicate entries
3. Manually corrupt hosts file - verify recovery
4. Uninstall - verify clean removal of Kodegen block

## Acceptance Criteria

- [ ] Hosts file modification is idempotent
- [ ] No duplicate entries on reinstall
- [ ] Atomic writes prevent corruption
- [ ] No dead code warnings for hosts.rs functions
