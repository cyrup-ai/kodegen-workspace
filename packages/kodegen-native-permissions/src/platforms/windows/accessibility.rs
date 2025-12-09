//! Windows accessibility permissions

use tokio::sync::oneshot;

// Windows provides broad accessibility API access by default
// No imports needed - authorization is always granted

use crate::types::{PermissionError, PermissionStatus, PermissionType};

pub fn check_accessibility() -> Result<PermissionStatus, PermissionError> {
    #[cfg(target_os = "windows")]
    {
        // Windows provides broad accessibility API access by default
        // No runtime permission required for UI Automation APIs
        Ok(PermissionStatus::Authorized)
    }
    #[cfg(not(target_os = "windows"))]
    Ok(PermissionStatus::Authorized)
}

pub fn request_accessibility(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    #[cfg(target_os = "windows")]
    {
        // Windows provides broad accessibility API access by default
        // No runtime permission required for UI Automation APIs
        tx.send(Ok(PermissionStatus::Authorized)).ok();
    }
    #[cfg(not(target_os = "windows"))]
    {
        tx.send(Ok(PermissionStatus::Authorized)).ok();
    }
}
