//! Linux notification permission handling using D-Bus

use crate::types::{PermissionError, PermissionStatus};
use tokio::sync::oneshot;
use std::sync::OnceLock;

// Global cached runtime - created once on first use, reused forever
// Eliminates expensive thread creation/destruction overhead (100ms + 150ms per call)
static RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();

/// Get or create the cached tokio runtime for D-Bus operations
/// 
/// The runtime is created once on first call and reused for all subsequent calls.
/// It lives for the entire program lifetime to avoid the expensive overhead of
/// spawning and joining worker threads (8 threads × ~20ms join time = 160ms overhead).
/// 
/// # Panics
/// 
/// Panics if runtime creation fails, which indicates a fundamental system problem
/// that would prevent all permission checking from working anyway.
fn get_or_create_runtime() -> &'static tokio::runtime::Runtime {
    RUNTIME.get_or_init(|| {
        tokio::runtime::Runtime::new()
            .expect("Failed to create tokio runtime for Linux notification permissions")
    })
}

/// Check if D-Bus notification service is available
pub fn check_permission() -> Result<PermissionStatus, PermissionError> {
    #[cfg(target_os = "linux")]
    {
        // Check if we can connect to D-Bus session bus and if notification service exists
        let rt = tokio::runtime::Handle::try_current();

        let result = match rt {
            Ok(handle) => tokio::task::block_in_place(|| {
                handle.block_on(async { check_dbus_notification_service().await })
            }),
            Err(_) => {
                // No tokio runtime available - use cached runtime
                // First call: creates runtime (~100ms), caches it
                // Subsequent calls: reuses cached runtime (zero overhead)
                get_or_create_runtime()
                    .block_on(async { check_dbus_notification_service().await })
            }
        };

        result
    }

    #[cfg(not(target_os = "linux"))]
    Ok(PermissionStatus::Authorized)
}

#[cfg(target_os = "linux")]
async fn check_dbus_notification_service() -> Result<PermissionStatus, PermissionError> {
    // Try to connect to D-Bus session bus
    let connection = zbus::Connection::session()
        .await
        .map_err(|e| PermissionError::SystemError(format!("D-Bus connection failed: {}", e)))?;

    // Check if org.freedesktop.Notifications service is available
    let reply = connection
        .call_method(
            Some("org.freedesktop.DBus"),
            "/org/freedesktop/DBus",
            Some("org.freedesktop.DBus"),
            "ListNames",
            &(),
        )
        .await;

    match reply {
        Ok(msg) => {
            let names: Vec<String> = msg
                .body()
                .deserialize()
                .map_err(|e| {
                    PermissionError::SystemError(format!(
                        "Failed to deserialize D-Bus ListNames response: {}",
                        e
                    ))
                })?;

            if names
                .iter()
                .any(|n| n == "org.freedesktop.Notifications")
            {
                Ok(PermissionStatus::Authorized)
            } else {
                // Service not running but might be activatable - check activatable names
                let activatable_reply = connection
                    .call_method(
                        Some("org.freedesktop.DBus"),
                        "/org/freedesktop/DBus",
                        Some("org.freedesktop.DBus"),
                        "ListActivatableNames",
                        &(),
                    )
                    .await;

                match activatable_reply {
                    Ok(msg) => {
                        let activatable: Vec<String> = msg
                            .body()
                            .deserialize()
                            .map_err(|e| {
                                PermissionError::SystemError(format!(
                                    "Failed to deserialize D-Bus ListActivatableNames response: {}",
                                    e
                                ))
                            })?;
                        if activatable
                            .iter()
                            .any(|n| n == "org.freedesktop.Notifications")
                        {
                            Ok(PermissionStatus::Authorized)
                        } else {
                            Ok(PermissionStatus::Denied)
                        }
                    }
                    Err(e) => Err(PermissionError::SystemError(format!(
                        "D-Bus ListActivatableNames failed: {}. Cannot verify notification service availability.",
                        e
                    ))),
                }
            }
        }
        Err(_) => Ok(PermissionStatus::Denied),
    }
}

/// Request notification permission (Linux doesn't require user permission)
pub fn request_permission(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    #[cfg(target_os = "linux")]
    {
        // Spawn async task in existing runtime (zero overhead)
        // This avoids creating a new runtime or using block_in_place
        tokio::spawn(async move {
            // Directly call the async D-Bus check function
            let result = check_dbus_notification_service().await;
            
            // Send result back through the oneshot channel
            tx.send(result).ok();
        });
    }
    
    #[cfg(not(target_os = "linux"))]
    {
        // Non-Linux platforms: notifications always authorized
        tx.send(Ok(PermissionStatus::Authorized)).ok();
    }
}
