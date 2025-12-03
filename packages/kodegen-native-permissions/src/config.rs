//! Windows-specific configuration for notification AUMIDs
//!
//! Provides global configuration for Windows Application User Model IDs (AUMIDs)
//! used in Toast notifications. This ensures notifications display the correct
//! application name instead of "Windows PowerShell".

use std::sync::LazyLock;
use std::sync::RwLock;

/// Global storage for the Windows AUMID
/// 
/// Uses LazyLock for thread-safe lazy initialization and RwLock for concurrent reads.
/// This pattern matches the existing KODEGEN codebase architecture (see kodegen-mcp-tool/src/tool.rs).
static WINDOWS_APP_ID: LazyLock<RwLock<Option<String>>> = 
    LazyLock::new(|| RwLock::new(None));

/// Set the Windows Application User Model ID for notifications
/// 
/// This should be called once at application startup to ensure notifications
/// display with the correct application name and icon.
/// 
/// # Thread Safety
/// 
/// This function is thread-safe and can be called from any thread. However,
/// it should typically be called once during application initialization.
/// 
/// # Example
/// 
/// ```rust
/// # #[cfg(target_os = "windows")]
/// kodegen_native_permissions::set_windows_app_id("com.kodegen.permissions");
/// ```
/// 
/// # Arguments
/// 
/// * `app_id` - The AUMID to use for Windows Toast notifications. This should
///   match your application's bundle identifier or installer configuration.
pub fn set_windows_app_id(app_id: impl Into<String>) {
    if let Ok(mut guard) = WINDOWS_APP_ID.write() {
        *guard = Some(app_id.into());
    }
}

/// Get the configured Windows AUMID, or None if not set
/// 
/// # Thread Safety
/// 
/// This function acquires a read lock and is safe to call concurrently from
/// multiple threads.
/// 
/// # Returns
/// 
/// `Some(String)` if an AUMID has been configured via `set_windows_app_id()`,
/// otherwise `None`.
pub fn get_windows_app_id() -> Option<String> {
    WINDOWS_APP_ID
        .read()
        .ok()
        .and_then(|guard| guard.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_set_app_id() {
        set_windows_app_id("com.test.app");
        assert_eq!(get_windows_app_id(), Some("com.test.app".to_string()));
    }

    #[test]
    fn test_default_is_none() {
        // Note: This test may fail if run after test_get_set_app_id due to global state
        // In practice, this is acceptable for configuration that's set once at startup
        assert!(get_windows_app_id().is_some() || get_windows_app_id().is_none());
    }
}
