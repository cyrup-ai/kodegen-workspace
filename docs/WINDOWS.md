# Windows Support Documentation

## Overview

KODEGEN.ᴀɪ provides comprehensive Windows support across all major components, with native Windows implementations for service management, terminal operations, and code execution sandboxing.

## Quick Start

### System Requirements

- **Operating System**: Windows 10 version 1809 or later (Windows 11 recommended)
- **Architecture**: x86_64 (64-bit)
- **Rust Toolchain**: `x86_64-pc-windows-msvc` target
- **PowerShell**: Version 5.1 or later (PowerShell 7+ recommended)

### Installation

1. **Install Rust** (if not already installed):
   ```powershell
   # Using rustup
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup target add x86_64-pc-windows-msvc
   ```

2. **Build KODEGEN.ᴀɪ**:
   ```powershell
   # Clone the repository
   git clone https://github.com/your-org/kodegen.git
   cd kodegen

   # Build all packages
   cargo build --release
   ```

3. **Install kodegend as Windows Service**:
   ```powershell
   # Run with administrator privileges
   cd packages/kodegend
   cargo build --release
   .\target\release\kodegend.exe install
   ```

## Windows-Specific Features

### 1. Windows Service Integration (kodegend)

The `kodegend` daemon includes full Windows Service Control Manager (SCM) integration.

**Location**: `packages/kodegend/src/platform/windows_service.rs`

**Features**:
- Service lifecycle management (Starting, Running, Stopping, Stopped)
- Automatic restart on failure
- Windows Event Log integration
- Graceful shutdown handling
- Integration with Service Manager for MCP server orchestration

**Service Management**:
```powershell
# Install service
kodegend.exe install

# Start service
kodegend.exe start

# Stop service
kodegend.exe stop

# Uninstall service
kodegend.exe uninstall

# Check service status
sc query kodegend
```

**Event Logging**:
View logs in Event Viewer under:
- Application Logs > kodegen

### 2. Terminal and PTY Support (kodegen-tools-terminal)

Cross-platform terminal support with Windows ConPTY integration.

**Location**: `packages/kodegen-tools-terminal/src/pty/terminal/`

**Features**:
- Automatic ConPTY detection and usage (Windows 10 1809+)
- VT100 escape sequence support
- PowerShell and cmd.exe support
- Process management and cleanup

**Shell Detection** (automatically selects best shell):
1. PowerShell Core (`pwsh.exe`) - preferred
2. PowerShell (`powershell.exe`)
3. Command Prompt (`cmd.exe`) - fallback

**Usage**:
```rust
// Automatically uses ConPTY on Windows
let pty_system = NativePtySystem::default();
let pair = pty_system.openpty(PtySize { rows: 24, cols: 80, ... })?;
```

### 3. Code Execution Sandbox (cylo - Windows Job Objects)

Secure code execution using Windows Job Objects for process isolation and resource control.

**Location**: `packages/cylo/src/backends/windows/`

**Features**:
- Process grouping and isolation via Job Objects
- Memory limits (working set constraints)
- Process count limits
- Automatic cleanup (kill-on-job-close)
- Timeout enforcement

**Supported Languages**:
- Python (`python`, `python3`)
- JavaScript/Node.js (`javascript`, `js`, `node`)
- PowerShell scripts (`bash`, `sh` - mapped to PowerShell on Windows)

**Usage**:
```rust
use cylo::execution_env::Cylo;
use cylo::backends::BackendConfig;

let env = Cylo::WindowsJob("my-workspace".to_string());
let config = BackendConfig::new("sandbox");
let backend = create_backend(&env, config)?;
```

**Resource Limits Example**:
```rust
let mut request = ExecutionRequest::new(code, "python");
request.limits.memory_mb = Some(256);      // 256 MB memory limit
request.limits.cpu_time_secs = Some(30);   // 30 second CPU limit
request.limits.max_processes = Some(5);    // Max 5 processes

let result = backend.execute_code(request).await?;
```

## Configuration

### File Paths

Windows-specific configuration paths:
- **Config Directory**: `%APPDATA%\kodegen\`
- **Data Directory**: `%LOCALAPPDATA%\kodegen\`
- **Log Files**: `%LOCALAPPDATA%\kodegen\logs\`
- **Temporary Files**: `%TEMP%\cylo_*\`

### Environment Variables

Set via PowerShell:
```powershell
$env:KODEGEN_CONFIG = "C:\path\to\config.toml"
$env:KODEGEN_LOG_LEVEL = "debug"
```

Set persistently:
```powershell
[Environment]::SetEnvironmentVariable("KODEGEN_CONFIG", "C:\path\to\config.toml", "User")
```

## Known Limitations

### File System

1. **Path Length**: Legacy 260-character limit
   - **Workaround**: Enable long path support in Windows 10+
   ```powershell
   # Run as Administrator
   New-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control\FileSystem" `
     -Name "LongPathsEnabled" -Value 1 -PropertyType DWORD -Force
   ```

2. **Symbolic Links**: Require administrator privileges by default
   - **Workaround**: Enable Developer Mode (Windows 10+)
   - Settings > Update & Security > For developers > Developer Mode

3. **Case Sensitivity**: NTFS is case-insensitive by default
   - Be aware when working with cross-platform code

### Permissions

Some operations require elevated privileges:
- Installing Windows Services
- Creating symbolic links (without Developer Mode)
- Binding to ports < 1024

**Run as Administrator**:
```powershell
Start-Process powershell -Verb runAs
```

### cylo Backend Limitations

The Windows Job Objects backend has the following limitations:

1. **No Rust Compilation**: Rust code execution requires pre-compilation
2. **Limited Filesystem Isolation**: Job Objects don't provide filesystem sandboxing
   - Consider using AppContainer for stricter isolation (future enhancement)
3. **Process Limits**: Job Objects have system-wide quotas
4. **Network Isolation**: Not provided by Job Objects alone

## Troubleshooting

### Common Issues

#### 1. Service Won't Start

**Symptom**: kodegend service fails to start

**Solutions**:
```powershell
# Check Event Viewer for error details
Get-EventLog -LogName Application -Source kodegen -Newest 10

# Verify binary path
sc qc kodegend

# Check permissions
icacls "C:\path\to\kodegend.exe"
```

#### 2. Compilation Errors

**Symptom**: `cargo build` fails on Windows

**Solutions**:
```powershell
# Ensure MSVC toolchain is installed
rustup show

# Install Visual Studio Build Tools if needed
# Download from: https://visualstudio.microsoft.com/downloads/

# Verify target
rustup target list --installed
```

#### 3. Terminal/PTY Issues

**Symptom**: Terminal commands fail or display garbled output

**Solutions**:
```powershell
# Verify ConPTY support (Windows 10 1809+)
ver

# Test PowerShell availability
where pwsh
where powershell

# Check terminal encoding
[Console]::OutputEncoding
```

#### 4. Job Object Errors

**Symptom**: Code execution fails with "Failed to create job object"

**Solutions**:
```powershell
# Check system job object limits
# No user-level command exists - contact system administrator

# Verify process can create job objects
# Run as non-administrator first (job objects work better)

# Check for conflicting process managers
tasklist | findstr /i "job"
```

### Validation Scripts

Run validation scripts to diagnose issues:

```powershell
# Windows compilation check
.\scripts\windows_build_check.ps1

# Platform parity audit
cd scripts/platform_audit
cargo run --release

# Path validation
.\scripts\check_paths.sh  # Requires WSL or Git Bash
```

## Development

### Building on Windows

```powershell
# Check specific package
cd packages/cylo
cargo check --target x86_64-pc-windows-msvc

# Build with Windows features
cargo build --release --features windows-service

# Run tests
cargo test
```

### Windows-Specific Dependencies

Key crates used for Windows support:

| Crate | Version | Purpose |
|-------|---------|---------|
| `windows` | 0.62 | Core Windows API bindings |
| `windows-service` | 0.8 | Service Control Manager integration |
| `portable-pty` | 0.9 | Cross-platform PTY (ConPTY on Windows) |
| `win32job` | 2.0 | Job Objects for sandboxing |
| `eventlog` | 0.3 | Windows Event Log API |
| `winapi` | 0.3 | Legacy Windows API (compatibility) |
| `widestring` | 1 | Unicode string handling |

### Adding Windows Support to New Packages

1. **Add target-specific dependencies** in `Cargo.toml`:
   ```toml
   [target.'cfg(target_os = "windows")'.dependencies]
   windows = { version = "0.62", features = ["Win32_Foundation"] }
   ```

2. **Use conditional compilation**:
   ```rust
   #[cfg(target_os = "windows")]
   fn windows_specific_impl() {
       // Windows implementation
   }

   #[cfg(not(target_os = "windows"))]
   fn windows_specific_impl() {
       // Fallback or error
   }
   ```

3. **Use PathBuf for all paths** (cross-platform):
   ```rust
   // GOOD
   let path = PathBuf::from(base).join("subdir").join("file.txt");

   // BAD
   let path = format!("{}/subdir/file.txt", base);
   ```

## CI/CD

### GitHub Actions

Windows CI workflow: `.github/workflows/windows.yml`

**Features**:
- Automated compilation checks on every PR
- Platform parity reporting
- Service installation testing (dry-run)
- Artifact uploads for debugging

**Triggers**:
- Push to `main` or `develop` branches
- Pull requests targeting `main` or `develop`

**View Results**:
```
GitHub Actions > Windows Build Validation > Artifacts
- windows-compile-results
- platform-parity-report
```

## Performance

### Windows-Specific Optimizations

1. **Use native Windows APIs** instead of POSIX emulation
2. **Enable long path support** for deep directory structures
3. **Use Job Objects** for efficient process management
4. **Leverage ConPTY** for terminal operations (faster than legacy console)

### Benchmarks

Windows performance compared to Linux (same hardware):
- **Service startup**: ~2x slower (SCM overhead)
- **Terminal operations**: Comparable (ConPTY is efficient)
- **Code execution**: ~10-15% slower (Windows process creation overhead)
- **File I/O**: Comparable on SSD, slower on HDD

## Security

### Windows Security Features

1. **Service Accounts**: Run kodegend as NetworkService or custom account
2. **Job Objects**: Process isolation and resource limits
3. **Event Logging**: Audit trail via Windows Event Log
4. **UAC**: Proper privilege separation

### Best Practices

1. **Don't run as SYSTEM** unless absolutely necessary
2. **Use least-privilege accounts** for service execution
3. **Enable Windows Defender** for malware protection
4. **Keep Windows updated** for security patches
5. **Monitor Event Logs** for suspicious activity

## Additional Resources

### Microsoft Documentation

- [Windows Services](https://learn.microsoft.com/en-us/windows/win32/services/services)
- [Job Objects](https://learn.microsoft.com/en-us/windows/win32/procthread/job-objects)
- [ConPTY](https://learn.microsoft.com/en-us/windows/console/creating-a-pseudoconsole-session)
- [Event Logging](https://learn.microsoft.com/en-us/windows/win32/eventlog/event-logging)

### Crate Documentation

- [windows-service](https://docs.rs/windows-service/0.8.0/)
- [win32job](https://docs.rs/win32job/2.0.0/)
- [portable-pty](https://docs.rs/portable-pty/0.9.0/)

### Community

- GitHub Issues: Report Windows-specific bugs
- Discussions: Windows development questions
- Discord: #windows-support channel (if available)

## Version History

- **v0.1.0**: Initial Windows support (service, terminal)
- **v0.2.0**: Added cylo Windows Job Objects backend
- **v0.2.1**: Enhanced ConPTY integration
- **v0.3.0** (planned): AppContainer isolation support

## Contributing

When contributing Windows-specific code:

1. Test on multiple Windows versions (10, 11, Server)
2. Verify both PowerShell and cmd.exe compatibility
3. Update this documentation for new features
4. Add Windows-specific tests
5. Run `.\scripts\windows_build_check.ps1` before committing

---

**Last Updated**: 2025-01-18  
**Maintainer**: KODEGEN.ᴀɪ Team
