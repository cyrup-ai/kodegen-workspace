//! Windows helper functions for permission conversion

#[cfg(target_os = "windows")]
use windows::Security::Authorization::AppCapabilityAccess::AppCapabilityAccessStatus;

use crate::types::PermissionStatus;

#[cfg(target_os = "windows")]
pub fn convert_app_capability_status(status: AppCapabilityAccessStatus) -> PermissionStatus {
    match status {
        AppCapabilityAccessStatus::Allowed => PermissionStatus::Authorized,
        AppCapabilityAccessStatus::DeniedByUser => PermissionStatus::Denied,
        AppCapabilityAccessStatus::DeniedBySystem => PermissionStatus::Restricted,
        _ => PermissionStatus::NotDetermined,
    }
}

#[cfg(not(target_os = "windows"))]
pub fn convert_app_capability_status(_status: i32) -> PermissionStatus {
    PermissionStatus::Authorized
}
