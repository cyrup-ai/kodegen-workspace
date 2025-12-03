//! Windows platform-specific and cross-platform permission handling

use tokio::sync::oneshot;

use crate::types::{PermissionError, PermissionStatus, PermissionType};

pub fn check_platform_specific(typ: PermissionType) -> Result<PermissionStatus, PermissionError> {
    #[cfg(target_os = "windows")]
    {
        let status = match typ {
            // Platform-specific permissions
            PermissionType::All => {
                // "All" permissions is not a real Windows permission concept
                // Return NotDetermined as this should be checked per individual permission
                PermissionStatus::NotDetermined
            },
            PermissionType::AppleEvents | PermissionType::PostEvent => {
                // Windows equivalent would be COM automation - generally available but not
                // guaranteed
                PermissionStatus::NotDetermined
            },
            PermissionType::DeveloperTools => {
                // Windows doesn't have system-level developer tools restrictions
                PermissionStatus::Authorized
            },
            PermissionType::FileProviderDomain | PermissionType::FileProviderPresence => {
                // Windows file system access is different from iOS file providers
                PermissionStatus::NotDetermined
            },
            PermissionType::UbiquitousFileProvider => {
                // This is iOS-specific for iCloud - not applicable to Windows
                PermissionStatus::Denied
            },
            PermissionType::WillfulWrite => {
                // Windows doesn't have this iOS-specific permission concept
                PermissionStatus::NotDetermined
            },
            // iOS-specific permissions that don't apply to Windows
            PermissionType::Calls
            | PermissionType::FaceID
            | PermissionType::FocusStatus
            | PermissionType::Siri => {
                PermissionStatus::Denied // Not available on Windows
            },
            _ => PermissionStatus::NotDetermined,
        };
        Ok(status)
    }
    #[cfg(not(target_os = "windows"))]
    Ok(PermissionStatus::Authorized)
}

pub fn request_platform_specific(
    typ: PermissionType,
    tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>,
) {
    #[cfg(target_os = "windows")]
    {
        let status = match typ {
            // Platform-specific permissions
            PermissionType::All => {
                // "All" permissions is not a real Windows permission concept
                // Return NotDetermined as this should be checked per individual permission
                Ok(PermissionStatus::NotDetermined)
            },

            PermissionType::AppleEvents | PermissionType::PostEvent => {
                // Windows equivalent would be COM automation - generally available but not
                // guaranteed
                Ok(PermissionStatus::NotDetermined)
            },

            PermissionType::DeveloperTools => {
                // Windows doesn't have system-level developer tools restrictions
                Ok(PermissionStatus::Authorized)
            },

            PermissionType::FileProviderDomain | PermissionType::FileProviderPresence => {
                // Windows file system access is different from iOS file providers
                Ok(PermissionStatus::NotDetermined)
            },

            PermissionType::UbiquitousFileProvider => {
                // This is iOS-specific for iCloud - not applicable to Windows
                Ok(PermissionStatus::Denied)
            },

            PermissionType::WillfulWrite => {
                // Windows doesn't have this iOS-specific permission concept
                Ok(PermissionStatus::NotDetermined)
            },

            // iOS-specific permissions not available on Windows
            PermissionType::Calls
            | PermissionType::FaceID
            | PermissionType::FocusStatus
            | PermissionType::Siri => Ok(PermissionStatus::Denied),

            _ => Ok(PermissionStatus::NotDetermined),
        };

        tx.send(status).ok();
    }
    #[cfg(not(target_os = "windows"))]
    {
        tx.send(Ok(PermissionStatus::Authorized)).ok();
    }
}
