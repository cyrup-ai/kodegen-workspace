//! Windows notification permission handling using Toast API
//!
//! Supports configurable Application User Model IDs (AUMIDs) to ensure
//! notifications display the correct application name instead of "Windows PowerShell".

use crate::types::{PermissionError, PermissionStatus};
use std::sync::OnceLock;
use tokio::sync::oneshot;

#[cfg(target_os = "windows")]
use tauri_winrt_notification::Toast;

#[cfg(target_os = "windows")]
use windows_version::OsVersion;

/// Execute a closure with COM initialized, ensuring cleanup
/// 
/// This is a convenience wrapper around the shared `com_init::with_com` that
/// maintains backward compatibility by converting `windows::core::Error` to
/// `PermissionError`.
/// 
/// This helper ensures COM is properly initialized before calling Toast APIs
/// and cleaned up afterward, preventing resource leaks.
#[cfg(target_os = "windows")]
fn with_com<F, T>(f: F) -> Result<T, PermissionError>
where
    F: FnOnce() -> Result<T, PermissionError>,
{
    super::com_init::with_com(|| f())
        .map_err(|e| PermissionError::PlatformError(format!("COM initialization failed: {:?}", e)))?
}

/// Default fallback AUMID when no custom ID is configured
/// 
/// This is PowerShell's AUMID and is used as a last resort to ensure
/// notifications work even in development/uninstalled scenarios.
/// 
/// **Note**: Notifications will show "Windows PowerShell" as the source
/// when using this fallback. Applications should call `set_windows_app_id()`
/// at startup to use a proper custom identifier.
#[cfg(target_os = "windows")]
const FALLBACK_APP_ID: &str = Toast::POWERSHELL_APP_ID;

/// Get the app ID to use for Toast notifications
/// 
/// Priority order:
/// 1. Custom AUMID set via `set_windows_app_id()`
/// 2. Environment variable `KODEGEN_WINDOWS_APP_ID`
/// 3. Fallback to `POWERSHELL_APP_ID` (shows as "Windows PowerShell")
/// 
/// This three-tier approach ensures:
/// - Production: Uses configured AUMID from installer or application code
/// - Development: Can override via environment variable for testing
/// - Fallback: Always works even when not configured
#[cfg(target_os = "windows")]
fn get_app_id() -> String {
    // Priority 1: Check for configured AUMID
    if let Some(app_id) = crate::config::get_windows_app_id() {
        return app_id;
    }
    
    // Priority 2: Check environment variable
    if let Ok(app_id) = std::env::var("KODEGEN_WINDOWS_APP_ID") {
        if !app_id.is_empty() {
            return app_id;
        }
    }
    
    // Priority 3: Fallback to PowerShell AUMID
    FALLBACK_APP_ID.to_string()
}

/// Cached result of Toast notification availability check.
///
/// Initialized once on first call to check_permission().
/// Windows notification availability is a static system capability that doesn't
/// change at runtime. Caching eliminates wasteful Toast creation on every check.
///
/// Thread safety: OnceLock guarantees only one thread executes initialization,
/// all others block and receive the same cached result.
#[cfg(target_os = "windows")]
static TOAST_AVAILABLE: OnceLock<PermissionStatus> = OnceLock::new();

/// Check if Windows Toast notifications are available
///
/// First call: Performs expensive Toast availability test (20-500ms)
/// Subsequent calls: Returns cached result instantly (0ms, zero allocations)
pub fn check_permission() -> Result<PermissionStatus, PermissionError> {
    #[cfg(target_os = "windows")]
    {
        // Check Windows version compatibility (Toast requires Windows 10+)
        if OsVersion::current() < OsVersion::new(10, 0, 0, 0) {
            return Err(PermissionError::SystemError(
                "Toast notifications require Windows 10 or later. Current OS version does not support this feature.".to_string()
            ));
        }

        // Cache the availability check - this closure runs ONLY ONCE
        // All subsequent calls return the cached status instantly
        let status = TOAST_AVAILABLE.get_or_init(|| {
            let app_id = get_app_id();
            
            // Try to create Toast with configured AUMID
            let result = with_com(|| {
                match Toast::new(&app_id) {
                    Ok(_) => Ok(PermissionStatus::Authorized),
                    Err(e) => Err(PermissionError::PlatformError(format!("Toast creation failed: {:?}", e)))
                }
            });
            
            match result {
                Ok(status) => status,
                Err(_) => {
                    // Fallback: Try PowerShell AUMID if custom one failed
                    // This ensures functionality even if the configured AUMID is invalid
                    if app_id != FALLBACK_APP_ID {
                        with_com(|| {
                            match Toast::new(FALLBACK_APP_ID) {
                                Ok(_) => Ok(PermissionStatus::Authorized),
                                Err(_) => Err(PermissionError::PlatformError("Fallback Toast creation failed".to_string()))
                            }
                        }).unwrap_or(PermissionStatus::Denied)
                    } else {
                        PermissionStatus::Denied
                    }
                }
            }
        });
        
        // All subsequent calls: Return cached value (zero allocations)
        Ok(*status)
    }

    #[cfg(not(target_os = "windows"))]
    Ok(PermissionStatus::Authorized)
}

/// Request notification permission (Windows doesn't require user permission)
pub fn request_permission(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    #[cfg(target_os = "windows")]
    {
        // ✅ Wrap blocking Toast::new() call in spawn_blocking to avoid blocking tokio runtime
        tokio::task::spawn_blocking(move || {
            // Toast::new() performs blocking COM initialization and registry lookups
            // This can take 20-500ms depending on system load and COM cold start
            let result = check_permission();
            tx.send(result).ok();
        });
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        tx.send(Ok(PermissionStatus::Authorized)).ok();
    }
}
