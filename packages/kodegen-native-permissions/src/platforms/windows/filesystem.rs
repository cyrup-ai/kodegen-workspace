//! Windows filesystem permissions (Documents, Network/Removable volumes)

use tokio::sync::oneshot;

#[cfg(target_os = "windows")]
use {
    windows::Security::Authorization::AppCapabilityAccess::{
        AppCapability, AppCapabilityAccessStatus,
    },
    windows::Win32::Storage::FileSystem::{GetDriveTypeW, GetLogicalDrives},
    windows::Win32::System::WindowsProgramming::{DRIVE_REMOTE, DRIVE_REMOVABLE},
    windows::core::Result as WinResult,
};

use super::helpers::convert_app_capability_status;
use crate::types::{PermissionError, PermissionStatus, PermissionType};

pub fn check_documents() -> Result<PermissionStatus, PermissionError> {
    #[cfg(target_os = "windows")]
    {
        match AppCapability::Create(&windows::core::HSTRING::from("documentsLibrary")) {
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

pub fn check_network_volumes() -> Result<PermissionStatus, PermissionError> {
    #[cfg(target_os = "windows")]
    {
        unsafe {
            let drives = GetLogicalDrives();
            let mut has_network_drive = false;

            for i in 0..26 {
                if (drives & (1 << i)) != 0 {
                    let drive_letter = format!("{}:\\", (b'A' + i as u8) as char);
                    let drive_path: Vec<u16> = drive_letter.encode_utf16().chain(Some(0)).collect();

                    if GetDriveTypeW(windows::core::PCWSTR(drive_path.as_ptr())) == DRIVE_REMOTE {
                        has_network_drive = true;
                        break;
                    }
                }
            }

            let status = if has_network_drive {
                PermissionStatus::Authorized
            } else {
                PermissionStatus::Denied
            };
            Ok(status)
        }
    }
    #[cfg(not(target_os = "windows"))]
    Ok(PermissionStatus::Authorized)
}

pub fn check_removable_volumes() -> Result<PermissionStatus, PermissionError> {
    #[cfg(target_os = "windows")]
    {
        unsafe {
            let drives = GetLogicalDrives();
            let mut has_removable_drive = false;

            for i in 0..26 {
                if (drives & (1 << i)) != 0 {
                    let drive_letter = format!("{}:\\", (b'A' + i as u8) as char);
                    let drive_path: Vec<u16> = drive_letter.encode_utf16().chain(Some(0)).collect();

                    if GetDriveTypeW(windows::core::PCWSTR(drive_path.as_ptr()))
                        == DRIVE_REMOVABLE
                    {
                        has_removable_drive = true;
                        break;
                    }
                }
            }

            let status = if has_removable_drive {
                PermissionStatus::Authorized
            } else {
                PermissionStatus::Denied
            };
            Ok(status)
        }
    }
    #[cfg(not(target_os = "windows"))]
    Ok(PermissionStatus::Authorized)
}

pub fn request_documents(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    #[cfg(target_os = "windows")]
    {
        let result = match AppCapability::Create(&windows::core::HSTRING::from("documentsLibrary")) {
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

pub fn request_network_volumes(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    #[cfg(target_os = "windows")]
    {
        tokio::task::spawn_blocking(move || {
            let result = unsafe {
                let drives = GetLogicalDrives();
                let mut has_network_drive = false;

                for i in 0..26 {
                    if (drives & (1 << i)) != 0 {
                        let drive_letter = format!("{}:\\", (b'A' + i as u8) as char);
                        let drive_path: Vec<u16> =
                            drive_letter.encode_utf16().chain(Some(0)).collect();

                        if GetDriveTypeW(windows::core::PCWSTR(drive_path.as_ptr()))
                            == DRIVE_REMOTE
                        {
                            has_network_drive = true;
                            break;
                        }
                    }
                }

                if has_network_drive {
                    Ok(PermissionStatus::Authorized)
                } else {
                    Ok(PermissionStatus::Denied)
                }
            };
            tx.send(result).ok();
        });
    }
    #[cfg(not(target_os = "windows"))]
    {
        tx.send(Ok(PermissionStatus::Authorized)).ok();
    }
}

pub fn request_removable_volumes(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    #[cfg(target_os = "windows")]
    {
        tokio::task::spawn_blocking(move || {
            let result = unsafe {
                let drives = GetLogicalDrives();
                let mut has_removable_drive = false;

                for i in 0..26 {
                    if (drives & (1 << i)) != 0 {
                        let drive_letter = format!("{}:\\", (b'A' + i as u8) as char);
                        let drive_path: Vec<u16> =
                            drive_letter.encode_utf16().chain(Some(0)).collect();

                        if GetDriveTypeW(windows::core::PCWSTR(drive_path.as_ptr()))
                            == DRIVE_REMOVABLE
                        {
                            has_removable_drive = true;
                            break;
                        }
                    }
                }

                if has_removable_drive {
                    Ok(PermissionStatus::Authorized)
                } else {
                    Ok(PermissionStatus::Denied)
                }
            };
            tx.send(result).ok();
        });
    }
    #[cfg(not(target_os = "windows"))]
    {
        tx.send(Ok(PermissionStatus::Authorized)).ok();
    }
}
