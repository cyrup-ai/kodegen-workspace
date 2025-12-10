# Task: Implement Per-User Installation Scope

## Priority: P2 (Installation Polish)

## Related Error
- `install/installer/windows/paths.rs:28` - variant `User` never constructed

## Problem Statement

The `InstallScope` enum defines two installation modes:
```rust
pub enum InstallScope {
    /// System-wide installation (C:\Program Files, requires admin)
    System,
    /// Per-user installation (C:\Users\{user}\AppData\Local\Programs)
    User,
}
```

Currently, only `InstallScope::System` is used. The `User` variant is never constructed.

## Why Per-User Installation Matters

1. **No Admin Required**: Users without admin privileges can still install
2. **Enterprise Environments**: Some IT policies block system-wide installs
3. **Development/Testing**: Developers can install without affecting system
4. **Multi-User Machines**: Each user gets their own installation

## Installation Paths

### System Scope (Current)
- Binary: `C:\Program Files\Kodegen\kodegend.exe`
- Config: `C:\ProgramData\kodegend\`
- Service: Windows Service (runs as SYSTEM)

### User Scope (To Implement)
- Binary: `%LOCALAPPDATA%\Programs\Kodegen\kodegend.exe`
- Config: `%APPDATA%\kodegen\kodegend\`
- Service: Scheduled Task or User Service (runs as current user)

## Required Implementation

### 1. Add CLI Flag for Scope Selection

In `src/main.rs` or CLI definition:
```rust
#[arg(long, default_value = "system")]
scope: InstallScope,
```

Or interactive prompt:
```
Install for:
  [1] All users (requires Administrator)
  [2] Current user only
```

### 2. Thread Scope Through Installation

Pass scope to all path functions:
```rust
let install_path = paths::install_dir(scope);
let config_path = paths::installer_config_dir(scope);
```

### 3. Handle Service Registration Differently

**System scope**: Windows Service via SCM
**User scope**: Options include:
- Windows Task Scheduler (runs at login)
- User-mode Windows Service (Windows 10+)
- Startup folder shortcut

Recommended: Task Scheduler with:
```xml
<Task>
  <Triggers>
    <LogonTrigger><UserId>%USERNAME%</UserId></LogonTrigger>
  </Triggers>
  <Actions>
    <Exec>
      <Command>%LOCALAPPDATA%\Programs\Kodegen\kodegend.exe</Command>
      <Arguments>--user-mode</Arguments>
    </Exec>
  </Actions>
</Task>
```

### 4. Update Path Functions

The path functions already exist and handle both scopes:
```rust
pub fn install_dir(scope: InstallScope) -> PathBuf {
    match scope {
        InstallScope::System => program_files_dir().join("Kodegen"),
        InstallScope::User => {
            dirs::data_local_dir()
                .unwrap_or_else(|| PathBuf::from(r"C:\Users\Default\AppData\Local"))
                .join("Programs")
                .join("Kodegen")
        }
    }
}
```

These need to be called with appropriate scope instead of hardcoded `System`.

### 5. Handle Mixed Installations

If both system and user installations exist:
- System installation takes precedence
- Warn user about conflict
- Or support both running simultaneously on different ports

## Files to Modify

- `src/main.rs` - Add scope CLI argument
- `src/install/mod.rs` - Thread scope through installation
- `src/install/privilege.rs` - Skip UAC for user scope
- `src/install/installer/windows/service_creation.rs` - Add Task Scheduler option
- Various files that call path functions

## Testing

1. User scope install without admin - should succeed
2. User scope install with existing system install - appropriate warning
3. User scope service starts at login
4. User scope uninstall cleans up completely
5. System scope still works as before

## Acceptance Criteria

- [ ] `InstallScope::User` variant is used
- [ ] Per-user installation works without admin
- [ ] Service/daemon runs correctly in user mode
- [ ] Paths are correctly set for user scope
- [ ] Documentation updated for both installation modes
