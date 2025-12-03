//! Windows accessibility permissions

use tokio::sync::oneshot;

#[cfg(target_os = "windows")]
use {
    windows::UI::UIAutomation::{UIAutomation, UIAutomationType},
    windows::core::Result as WinResult,
};

use crate::types::{PermissionError, PermissionStatus, PermissionType};

pub fn check_accessibility() -> Result<PermissionStatus, PermissionError> {
    #[cfg(target_os = "windows")]
    {
        let status = match UIAutomation::GetForCurrentThread(UIAutomationType::CoreWindow) {
            Ok(_) => PermissionStatus::Authorized,
            Err(_) => PermissionStatus::Denied,
        };
        Ok(status)
    }
    #[cfg(not(target_os = "windows"))]
    Ok(PermissionStatus::Authorized)
}

pub fn request_accessibility(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    #[cfg(target_os = "windows")]
    {
        let result = match UIAutomation::GetForCurrentThread(UIAutomationType::CoreWindow) {
            Ok(_) => Ok(PermissionStatus::Authorized),
            Err(e) => Err(PermissionError::SystemError(format!(
                "Windows Runtime operation failed: {}",
                e
            ))),
        };
        tx.send(result).ok();
    }
    #[cfg(not(target_os = "windows"))]
    {
        tx.send(Ok(PermissionStatus::Authorized)).ok();
    }
}
