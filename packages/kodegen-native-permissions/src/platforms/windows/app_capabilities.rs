//! Windows app capability permissions (Calendar, Contacts, Photos, MediaLibrary)

use tokio::sync::oneshot;

#[cfg(target_os = "windows")]
use {
    windows::Security::Authorization::AppCapabilityAccess::{
        AppCapability, AppCapabilityAccessStatus,
    },
    windows::core::Result as WinResult,
};

use super::helpers::convert_app_capability_status;
use crate::types::{PermissionError, PermissionStatus, PermissionType};

pub fn check_calendar() -> Result<PermissionStatus, PermissionError> {
    check_app_capability("appointments")
}

pub fn check_contacts() -> Result<PermissionStatus, PermissionError> {
    check_app_capability("contacts")
}

pub fn check_photos() -> Result<PermissionStatus, PermissionError> {
    check_app_capability("picturesLibrary")
}

pub fn check_media_library() -> Result<PermissionStatus, PermissionError> {
    check_app_capability("musicLibrary")
}

pub fn check_motion() -> Result<PermissionStatus, PermissionError> {
    check_app_capability("activity")
}

pub fn check_nearby_interaction() -> Result<PermissionStatus, PermissionError> {
    check_app_capability("radios")
}

fn check_app_capability(capability_name: &str) -> Result<PermissionStatus, PermissionError> {
    #[cfg(target_os = "windows")]
    {
        match AppCapability::Create(&windows::core::HSTRING::from(capability_name)) {
            Ok(capability) => match capability.CheckAccess() {
                Ok(status) => Ok(convert_app_capability_status(status)),
                Err(_) => Ok(PermissionStatus::Denied),
            },
            Err(_) => Ok(PermissionStatus::Denied),
        }
    }
    #[cfg(not(target_os = "windows"))]
    Ok(PermissionStatus::Authorized)
}

pub fn request_calendar(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    request_app_capability(tx, "appointments");
}

pub fn request_contacts(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    request_app_capability(tx, "contacts");
}

pub fn request_photos(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    request_app_capability(tx, "picturesLibrary");
}

pub fn request_media_library(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    request_app_capability(tx, "musicLibrary");
}

pub fn request_motion(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    request_app_capability(tx, "activity");
}

pub fn request_nearby_interaction(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    request_app_capability(tx, "radios");
}

fn request_app_capability(
    tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>,
    capability_name: &str,
) {
    #[cfg(target_os = "windows")]
    {
        let capability_name = capability_name.to_string();
        let result = match AppCapability::Create(&windows::core::HSTRING::from(capability_name)) {
            Ok(capability) => match capability.CheckAccess() {
                Ok(status) => Ok(convert_app_capability_status(status)),
                Err(e) => Err(PermissionError::SystemError(format!(
                    "Windows Runtime operation failed: {}",
                    e
                ))),
            },
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
