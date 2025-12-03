//! COM initialization and cleanup helpers for Windows WinRT APIs
//!
//! This module provides RAII-based COM initialization using the ComGuard pattern.
//! COM (Component Object Model) requires per-thread initialization before creating
//! COM objects like Toast notifications, MediaCapture, Geolocator, etc.
//!
//! ## Why COM Initialization is Required
//!
//! Windows Runtime (WinRT) APIs are built on top of COM. According to Microsoft:
//!
//! > "Any Windows program that uses COM must initialize the COM library by calling
//! > the CoInitializeEx function. Each thread that uses a COM interface must make
//! > a separate call to this function."
//!
//! Without proper COM initialization, WinRT APIs will fail with error codes or crash.
//!
//! ## Thread Safety
//!
//! COM initialization is **per-thread**. Each thread that creates COM objects must
//! initialize COM separately. The ComGuard automatically handles cleanup via Drop,
//! ensuring resources are freed even during panics or early returns.
//!
//! ## Usage Example
//!
//! ```rust
//! use super::com_init::with_com;
//!
//! pub fn check_camera() -> Result<PermissionStatus, PermissionError> {
//!     with_com(|| {
//!         AppCapability::CreateForCapabilityName(&"webcam".into())
//!             .map(|cap| convert_status(cap.AccessStatus()?))
//!             .map_err(|e| PermissionError::PlatformError(format!("Camera check failed: {:?}", e)))
//!     })
//! }
//! ```
//!
//! ## References
//!
//! - [Microsoft: Initializing the COM Library](https://learn.microsoft.com/en-us/windows/win32/learnwin32/initializing-the-com-library)
//! - [Microsoft: CoInitializeEx](https://learn.microsoft.com/en-us/windows/win32/api/combaseapi/nf-combaseapi-coinitializeex)
//! - [Microsoft: Threading and COM](https://learn.microsoft.com/en-us/windows/win32/com/processes--threads--and-apartments)

use windows::Win32::Foundation::{S_FALSE, S_OK};
use windows::Win32::System::Com::{
    CoInitializeEx, CoUninitialize, COINIT_APARTMENTTHREADED, COINIT_DISABLE_OLE1DDE,
};

/// RAII guard for COM initialization
///
/// Automatically calls CoUninitialize when dropped, ensuring COM resources
/// are properly cleaned up even in the face of panics or early returns.
///
/// ## Example
///
/// ```rust
/// let _guard = ComGuard::new()?;
/// // COM is now initialized for this thread
/// let toast = Toast::new(Toast::POWERSHELL_APP_ID)?;
/// // COM automatically cleaned up when _guard goes out of scope
/// ```
///
/// ## Success Codes
///
/// - `S_OK` (0x00000000): First COM initialization on this thread
/// - `S_FALSE` (0x00000001): COM already initialized on this thread (safe)
///
/// Both are treated as success cases. Only actual errors fail initialization.
pub struct ComGuard {
    initialized: bool,
}

impl ComGuard {
    /// Initialize COM for the current thread
    ///
    /// Uses `COINIT_APARTMENTTHREADED` for single-threaded apartment model,
    /// which is required for most WinRT objects like Toast notifications,
    /// MediaCapture, Geolocator, etc.
    ///
    /// Also uses `COINIT_DISABLE_OLE1DDE` to disable legacy OLE1 DDE support
    /// for improved performance and security.
    ///
    /// ## Return Values
    ///
    /// - `Ok(ComGuard)`: COM initialized successfully (S_OK or S_FALSE)
    ///   - S_OK: First initialization on this thread
    ///   - S_FALSE: Already initialized on this thread (safe to continue)
    /// - `Err(windows::core::Error)`: Actual initialization failure
    ///
    /// ## Thread Safety
    ///
    /// Each thread must call this separately. Multiple ComGuards on the same
    /// thread will return S_FALSE after the first initialization.
    ///
    /// ## References
    ///
    /// - [COINIT_APARTMENTTHREADED](https://learn.microsoft.com/en-us/windows/win32/api/objidl/ne-objidl-coinit)
    pub fn new() -> windows::core::Result<Self> {
        unsafe {
            let hr = CoInitializeEx(
                None, // Reserved parameter, must be NULL
                COINIT_APARTMENTTHREADED | COINIT_DISABLE_OLE1DDE,
            );

            // Both S_OK and S_FALSE are success cases
            // S_OK = first COM init on this thread
            // S_FALSE = COM already initialized on this thread
            if hr == S_OK || hr == S_FALSE {
                Ok(ComGuard { initialized: true })
            } else {
                // Actual error - convert HRESULT to windows::core::Error
                Err(windows::core::Error::from(hr))
            }
        }
    }
}

impl Drop for ComGuard {
    /// Automatically uninitialize COM when guard goes out of scope
    ///
    /// This ensures COM resources are always cleaned up, even if:
    /// - Panics occur within the guarded scope
    /// - Early returns happen (e.g., via `?` operator)
    /// - Errors are propagated up the call stack
    ///
    /// The Drop guarantee is critical for preventing COM resource leaks.
    fn drop(&mut self) {
        if self.initialized {
            unsafe {
                CoUninitialize();
            }
        }
    }
}

/// Execute a closure with COM initialized
///
/// Convenience helper that initializes COM, executes the closure,
/// and automatically cleans up COM afterwards via RAII Drop.
///
/// ## Example
///
/// ```rust
/// use super::com_init::with_com;
///
/// pub fn check_location() -> Result<PermissionStatus, PermissionError> {
///     with_com(|| {
///         match Geolocator::RequestAccessAsync() {
///             Ok(future) => {
///                 // Process async future...
///                 Ok(PermissionStatus::Authorized)
///             }
///             Err(e) => Err(PermissionError::PlatformError(format!("Location check failed: {:?}", e)))
///         }
///     })
/// }
/// ```
///
/// ## Error Handling
///
/// Returns `Err` if COM initialization fails. If initialization succeeds,
/// returns the result of the closure.
///
/// ## Performance Note
///
/// This function creates a new ComGuard each time. If calling multiple
/// WinRT APIs in sequence, consider creating a single ComGuard manually
/// to avoid redundant initialization checks:
///
/// ```rust
/// let _guard = ComGuard::new()?;
/// let camera_status = check_camera_internal()?;
/// let mic_status = check_microphone_internal()?;
/// // Both calls use the same COM initialization
/// ```
pub fn with_com<F, R>(f: F) -> windows::core::Result<R>
where
    F: FnOnce() -> R,
{
    let _guard = ComGuard::new()?;
    Ok(f())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_com_guard_drop() {
        // ComGuard should clean up on drop
        {
            let _guard = ComGuard::new().expect("COM init failed");
            // Guard is active here
        }
        // Guard dropped, COM should be uninitialized

        // Should be able to initialize again
        let _guard2 = ComGuard::new().expect("COM re-init failed");
    }

    #[test]
    fn test_with_com() {
        let result = with_com(|| 42).expect("with_com failed");
        assert_eq!(result, 42);
    }

    #[test]
    fn test_multiple_guards_same_thread() {
        // First guard initializes (S_OK)
        let _guard1 = ComGuard::new().expect("First COM init failed");

        // Second guard should succeed with S_FALSE (already initialized)
        let _guard2 = ComGuard::new().expect("Second COM init failed");

        // Both guards valid simultaneously
    }
}
