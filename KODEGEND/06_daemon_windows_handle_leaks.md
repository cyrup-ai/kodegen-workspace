# Windows Handle Leaks in Process Management

## Severity: HIGH

## Location
`packages/kodegend/src/daemon.rs:157-168` (is_process_running)
`packages/kodegend/src/daemon.rs:203-210` (stop_service)

## Issue Description
The Windows implementations of process management functions can leak handles if errors occur or panics happen between `OpenProcess` and `CloseHandle`.

## Current Vulnerable Code

### is_process_running()
```rust
#[cfg(windows)]
{
    use windows_sys::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION};
    use windows_sys::Win32::Foundation::{CloseHandle, HANDLE};
    
    unsafe {
        let handle = OpenProcess(PROCESS_QUERY_INFORMATION, 0, pid as u32);
        if handle.is_null() {
            return Ok(false);
        }
        CloseHandle(handle);  // If panic occurs before this, handle leaks!
        Ok(true)
    }
}
```

### stop_service()
```rust
#[cfg(windows)]
{
    unsafe {
        let handle = OpenProcess(PROCESS_TERMINATE, 0, pid as u32);
        if handle.is_null() {
            return Err(anyhow!("Failed to open process"));
        }
        TerminateProcess(handle, 1);
        CloseHandle(handle);  // Not reached if TerminateProcess panics!
    }
}
```

## Problems

### 1. Panic Safety
If any code panics between `OpenProcess` and `CloseHandle`:
- Handle is never closed
- Handle leaks permanently
- Repeated leaks exhaust handle table
- Eventually causes handle exhaustion

### 2. Early Return Paths
In `is_process_running`, if we add any code that returns early:
```rust
let handle = OpenProcess(...);
if some_condition {
    return Err(...);  // LEAK! Forgot to close handle
}
CloseHandle(handle);
```

### 3. Accumulation Over Time
On Windows, each process has a handle limit (typically 10,000):
- Every leak reduces available handles
- High-frequency operations (health checks, status polls) accumulate leaks
- Eventually reaches limit and fails with ERROR_TOO_MANY_OPEN_FILES

## Real-World Impact
```powershell
PS> kodegend status
PS> kodegend status
... (after 10,000 status checks)
PS> kodegend status
Error: Failed to open process (error code: 1454 - ERROR_WORKING_SET_QUOTA)
PS> kodegend stop
Error: Failed to open process (error code: 1454)
# Service is now unmanageable!
```

## Recommended Solution

### Option 1: RAII Wrapper (Recommended)
```rust
#[cfg(windows)]
struct ProcessHandle(HANDLE);

#[cfg(windows)]
impl ProcessHandle {
    fn open(access: u32, pid: u32) -> Result<Self> {
        unsafe {
            let handle = OpenProcess(access, 0, pid);
            if handle.is_null() {
                Err(anyhow!("Failed to open process {}: {}", pid, 
                    std::io::Error::last_os_error()))
            } else {
                Ok(ProcessHandle(handle))
            }
        }
    }
    
    fn as_raw(&self) -> HANDLE {
        self.0
    }
}

#[cfg(windows)]
impl Drop for ProcessHandle {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.0);
        }
    }
}

// Now use it safely:
pub fn is_process_running(pid: i32) -> Result<bool> {
    #[cfg(windows)]
    {
        match ProcessHandle::open(PROCESS_QUERY_INFORMATION, pid as u32) {
            Ok(_handle) => Ok(true),  // Automatically closed when _handle drops
            Err(_) => Ok(false),
        }
    }
    // ... unix implementation
}

pub fn stop_service(pid_file: &Path) -> Result<()> {
    // ... 
    #[cfg(windows)]
    {
        let handle = ProcessHandle::open(PROCESS_TERMINATE, pid as u32)?;
        unsafe {
            TerminateProcess(handle.as_raw(), 1);
        }
        // handle automatically closed on drop - panic safe!
    }
}
```

### Option 2: Explicit Scope Guards
Using the `scopeguard` crate:
```rust
#[cfg(windows)]
{
    use scopeguard::defer;
    
    unsafe {
        let handle = OpenProcess(PROCESS_QUERY_INFORMATION, 0, pid as u32);
        if handle.is_null() {
            return Ok(false);
        }
        
        defer! {
            CloseHandle(handle);
        }
        
        // Handle is guaranteed to be closed, even on panic
        Ok(true)
    }
}
```

### Option 3: Use windows-rs Instead of windows-sys
The `windows` crate has proper RAII wrappers built-in:
```rust
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION};

pub fn is_process_running(pid: i32) -> Result<bool> {
    #[cfg(windows)]
    {
        unsafe {
            match OpenProcess(PROCESS_QUERY_INFORMATION, false, pid as u32) {
                Ok(handle) => {
                    // handle is OwnedHandle, automatically closed
                    Ok(true)
                }
                Err(_) => Ok(false),
            }
        }
    }
}
```

## Recommended Approach
Use Option 1 (RAII wrapper) because:
1. ✅ No external dependencies
2. ✅ Zero runtime overhead
3. ✅ Compile-time guarantee of cleanup
4. ✅ Idiomatic Rust
5. ✅ Panic-safe by design

## Additional Safety Improvements

### Add Debug Assertions
```rust
#[cfg(windows)]
impl Drop for ProcessHandle {
    fn drop(&mut self) {
        unsafe {
            let result = CloseHandle(self.0);
            debug_assert!(result != 0, "CloseHandle failed");
        }
    }
}
```

### Add Handle Tracking (Debug Mode)
```rust
#[cfg(all(windows, debug_assertions))]
static OPEN_HANDLES: AtomicUsize = AtomicUsize::new(0);

#[cfg(windows)]
impl ProcessHandle {
    fn open(access: u32, pid: u32) -> Result<Self> {
        let handle = unsafe { OpenProcess(access, 0, pid) };
        if handle.is_null() {
            return Err(anyhow!("Failed to open process"));
        }
        
        #[cfg(debug_assertions)]
        OPEN_HANDLES.fetch_add(1, Ordering::Relaxed);
        
        Ok(ProcessHandle(handle))
    }
}

#[cfg(windows)]
impl Drop for ProcessHandle {
    fn drop(&mut self) {
        #[cfg(debug_assertions)]
        OPEN_HANDLES.fetch_sub(1, Ordering::Relaxed);
        
        unsafe { CloseHandle(self.0); }
    }
}
```

## Testing Strategy
1. **Leak detection test**: Open/close handles in loop, monitor handle count
2. **Panic test**: Force panic after OpenProcess, verify cleanup
3. **Stress test**: Rapid status checks, verify no accumulation
4. **Resource limits**: Test behavior at handle limit

### Test Tool
Use Windows Sysinternals Handle.exe:
```powershell
# Before
PS> handle.exe -p kodegend | measure

# Run operations
PS> for($i=0; $i -lt 10000; $i++) { kodegend status }

# After
PS> handle.exe -p kodegend | measure
# Should show no increase
```

## Dependencies
```toml
# For Option 2
[dependencies]
scopeguard = "1.2"

# For Option 3
[target.'cfg(windows)'.dependencies]
windows = { version = "0.52", features = ["Win32_System_Threading"] }
```

## Performance Impact
- RAII wrapper: Zero overhead (optimized away)
- Scope guard: Minimal (few instructions)
- Memory: One extra pointer per handle

## References
- "Windows Internals" - Handle tables and quotas
- Raymond Chen's blog - Handle leaks and debugging
- Windows SDK - Best practices for handle management
