//! Permission manager with caching and async support

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use tokio::sync::oneshot;

use crate::types::{PermissionError, PermissionStatus, PermissionType};

#[cfg(target_os = "macos")]
use crate::platforms::macos::handler::MacOSHandler;

/// Thread-safe permission manager with caching and async support
pub struct PermissionManager {
    cache: Arc<RwLock<HashMap<PermissionType, PermissionStatus>>>,
}

impl PermissionManager {
    /// Create a new permission manager instance
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Synchronously check permission status (uses cache if available)
    pub fn check_permission(
        &self,
        typ: PermissionType,
    ) -> Result<PermissionStatus, PermissionError> {
        // Try cache first
        if let Ok(cache) = self.cache.read()
            && let Some(status) = cache.get(&typ)
        {
            return Ok(*status);
        }

        // Platform-specific check logic
        let status = match typ {
            #[cfg(target_os = "macos")]
            PermissionType::Camera | PermissionType::Microphone => {
                crate::platforms::macos::av_permissions::check_permission(typ)
            },
            #[cfg(target_os = "macos")]
            PermissionType::Location => {
                crate::platforms::macos::location_permissions::check_permission()
            },
            #[cfg(target_os = "macos")]
            PermissionType::Calendar | PermissionType::Reminders => {
                crate::platforms::macos::event_kit_permissions::check_permission(typ)
            },
            #[cfg(target_os = "macos")]
            PermissionType::Contacts => {
                crate::platforms::macos::contacts_permissions::check_permission()
            },
            #[cfg(target_os = "macos")]
            PermissionType::Bluetooth => {
                crate::platforms::macos::bluetooth_permissions::check_permission()
            },
            #[cfg(target_os = "macos")]
            PermissionType::Accessibility | PermissionType::AccessibilityMouse => {
                MacOSHandler::new().check_accessibility()
            },
            #[cfg(target_os = "macos")]
            PermissionType::WiFi => {
                MacOSHandler::new().check_wifi()
            },
            #[cfg(target_os = "macos")]
            PermissionType::ScreenCapture => {
                MacOSHandler::new().check_screen_recording()
            },
            #[cfg(target_os = "macos")]
            PermissionType::InputMonitoring => {
                MacOSHandler::new().check_input_monitoring()
            },
            #[cfg(target_os = "macos")]
            PermissionType::Notification => {
                crate::platforms::macos::notification_permissions::check_permission()
            },
            #[cfg(target_os = "macos")]
            _ => crate::platforms::macos::tcc_permissions::check_permission(typ),

            #[cfg(target_os = "windows")]
            _ => crate::platforms::windows::check_permission(typ),

            #[cfg(target_os = "linux")]
            _ => crate::platforms::linux::check_permission(typ),

            #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
            _ => Err(PermissionError::Unknown),
        };

        // Update cache on success
        if let Ok(s) = &status
            && let Ok(mut cache) = self.cache.write()
        {
            cache.insert(typ, *s);
        }
        
        status
    }

    /// Asynchronously request permission (shows native OS dialog)
    /// 
    /// This method triggers the native OS permission dialog and awaits the result
    /// using a tokio oneshot channel. The OS handles user interaction asynchronously,
    /// and results are delivered via OS thread callbacks.
    pub async fn request_permission(
        &self,
        typ: PermissionType,
    ) -> Result<PermissionStatus, PermissionError> {
        let cache = self.cache.clone();
        
        // ✅ Step 1: Create tokio oneshot channel
        // The sender works from any thread (including OS callbacks)
        // The receiver integrates with tokio's async runtime
        let (tx, rx) = oneshot::channel();
        
        // ✅ Step 2: Call platform code directly - NO spawn_blocking!
        // Platform functions trigger OS dialogs and return immediately
        // Results arrive later via OS thread callbacks that call tx.send()
        #[cfg(target_os = "macos")]
        match typ {
            PermissionType::Camera | PermissionType::Microphone => {
                crate::platforms::macos::av_permissions::request_permission(typ, tx);
            },
            PermissionType::Location => {
                crate::platforms::macos::location_permissions::request_permission(tx);
            },
            PermissionType::Calendar | PermissionType::Reminders => {
                crate::platforms::macos::event_kit_permissions::request_permission(typ, tx);
            },
            PermissionType::Contacts => {
                crate::platforms::macos::contacts_permissions::request_permission(tx);
            },
            PermissionType::Bluetooth => {
                crate::platforms::macos::bluetooth_permissions::request_permission(tx);
            },
            PermissionType::Accessibility | PermissionType::AccessibilityMouse => {
                MacOSHandler::new().request_accessibility(tx);
            },
            PermissionType::WiFi => {
                // WiFi doesn't have a request API, just return current status
                return MacOSHandler::new().check_wifi();
            },
            PermissionType::ScreenCapture => {
                MacOSHandler::new().request_screen_recording(tx);
            },
            PermissionType::InputMonitoring => {
                MacOSHandler::new().request_input_monitoring(tx);
            },
            PermissionType::Notification => {
                crate::platforms::macos::notification_permissions::request_permission(tx);
            },
            _ => {
                // TCC permissions have synchronous request methods
                return crate::platforms::macos::tcc_permissions::request_permission(typ);
            }
        }
        
        #[cfg(target_os = "windows")]
        crate::platforms::windows::request_permission(typ, tx);
        
        #[cfg(target_os = "linux")]
        crate::platforms::linux::request_permission(typ, tx);
        
        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
        {
            let _ = tx; // Consume tx to avoid unused variable warning
            return Err(PermissionError::Unknown);
        }
        
        // ✅ Step 3: Directly await the tokio receiver - clean async!
        // rx.await returns Result<Result<PermissionStatus, PermissionError>, RecvError>
        // Outer Result is for channel errors (sender dropped)
        // Inner Result is for permission errors
        let result = rx.await
            .map_err(|_| PermissionError::SystemError("Permission channel closed".into()))??;
        
        // ✅ Step 4: Update cache on success
        if let Ok(mut cache_guard) = cache.write() {
            cache_guard.insert(typ, result);
        }
        
        Ok(result)
    }

    /// Request multiple permissions concurrently
    /// 
    /// Returns a map of permission types to their results. Permissions are requested
    /// in parallel using tokio tasks for maximum efficiency.
    pub async fn request_permissions(
        &self,
        types: &[PermissionType],
    ) -> HashMap<PermissionType, Result<PermissionStatus, PermissionError>> {
        let mut tasks = Vec::new();
        
        for &typ in types {
            let manager = self.clone();
            let task = tokio::spawn(async move {
                (typ, manager.request_permission(typ).await)
            });
            tasks.push(task);
        }

        let mut results = HashMap::new();
        for task in tasks {
            if let Ok((typ, result)) = task.await {
                results.insert(typ, result);
            }
        }

        results
    }

    /// Manually refresh the cache for a specific permission
    pub fn refresh_cache(&self, typ: PermissionType) {
        let _ = self.check_permission(typ);
    }

    /// Clear all cached permission statuses
    pub fn clear_cache(&self) {
        if let Ok(mut cache) = self.cache.write() {
            cache.clear();
        }
    }
}

impl Default for PermissionManager {
    fn default() -> Self {
        Self::new()
    }
}

// Make PermissionManager cloneable (Arc-based)
impl Clone for PermissionManager {
    fn clone(&self) -> Self {
        Self {
            cache: Arc::clone(&self.cache),
        }
    }
}
