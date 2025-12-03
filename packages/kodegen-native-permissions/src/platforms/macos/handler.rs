//! macOS permissions implementation using proven patterns from tauri-plugin-macos-permissions

use crate::types::{PermissionError, PermissionStatus, PermissionType};
use crate::traits::PermissionHandler;
use super::{av_permissions, location_permissions, event_kit_permissions, contacts_permissions, bluetooth_permissions, notification_permissions, tcc_permissions};
use tokio::sync::oneshot;

#[cfg(target_os = "macos")]
use {
    macos_accessibility_client::accessibility::{
        application_is_trusted, application_is_trusted_with_prompt,
    },
};

#[cfg(target_os = "macos")]
#[link(name = "CoreGraphics", kind = "framework")]
unsafe extern "C" {
    fn CGPreflightScreenCaptureAccess() -> bool;
    fn CGRequestScreenCaptureAccess() -> bool;
}

#[cfg(target_os = "macos")]
#[link(name = "IOKit", kind = "framework")]
unsafe extern "C" {
    fn IOHIDCheckAccess(request: u32) -> u32;
}

/// macOS-specific permission handler
pub struct MacOSHandler;

impl Default for MacOSHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl MacOSHandler {
    pub fn new() -> Self {
        Self
    }
}

impl PermissionHandler for MacOSHandler {
    fn check_permission(&self, typ: PermissionType) -> Result<PermissionStatus, PermissionError> {
        match typ {
            PermissionType::Camera | PermissionType::Microphone => av_permissions::check_permission(typ),
            PermissionType::Location => location_permissions::check_permission(),
            PermissionType::Calendar | PermissionType::Reminders => event_kit_permissions::check_permission(typ),
            PermissionType::Contacts => contacts_permissions::check_permission(),
            PermissionType::Bluetooth => bluetooth_permissions::check_permission(),
            PermissionType::Accessibility | PermissionType::AccessibilityMouse => self.check_accessibility(),
            PermissionType::WiFi => self.check_wifi(),
            PermissionType::ScreenCapture => self.check_screen_recording(),
            PermissionType::FullDiskAccess => tcc_permissions::check_permission(typ),
            PermissionType::InputMonitoring => self.check_input_monitoring(),
            PermissionType::Photos => tcc_permissions::check_permission(typ),
            PermissionType::SpeechRecognition => tcc_permissions::check_permission(typ),
            PermissionType::DesktopFolder | PermissionType::DocumentsFolder | PermissionType::DownloadsFolder => tcc_permissions::check_permission(typ),
            PermissionType::AppleEvents => tcc_permissions::check_permission(typ),
            PermissionType::DeveloperTools => tcc_permissions::check_permission(typ),
            PermissionType::AdminFiles => tcc_permissions::check_permission(typ),
            PermissionType::NetworkVolumes => tcc_permissions::check_permission(typ),
            PermissionType::RemovableVolumes => tcc_permissions::check_permission(typ),
            PermissionType::Notification => notification_permissions::check_permission(),
            // All remaining permission types handled through TCC permissions
            PermissionType::AddressBook |
            PermissionType::All |
            PermissionType::Calls |
            PermissionType::FaceID |
            PermissionType::FileProviderDomain |
            PermissionType::FileProviderPresence |
            PermissionType::FocusStatus |
            PermissionType::MediaLibrary |
            PermissionType::Motion |
            PermissionType::NearbyInteraction |
            PermissionType::PhotosAdd |
            PermissionType::PostEvent |
            PermissionType::RemoteDesktop |
            PermissionType::Siri |
            PermissionType::UbiquitousFileProvider |
            PermissionType::WillfulWrite => tcc_permissions::check_permission(typ),
        }
    }

    fn request_permission(&self, typ: PermissionType, tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
        match typ {
            PermissionType::Camera | PermissionType::Microphone => av_permissions::request_permission(typ, tx),
            PermissionType::Location => location_permissions::request_permission(tx),
            PermissionType::Calendar | PermissionType::Reminders => event_kit_permissions::request_permission(typ, tx),
            PermissionType::Contacts => contacts_permissions::request_permission(tx),
            PermissionType::Bluetooth => bluetooth_permissions::request_permission(tx),
            PermissionType::Accessibility | PermissionType::AccessibilityMouse => self.request_accessibility(tx),
            PermissionType::WiFi => {
                let result = self.check_wifi();
                tx.send(result).ok();
            },
            PermissionType::ScreenCapture => self.request_screen_recording(tx),
            PermissionType::FullDiskAccess => self.request_tcc_permission(typ, tx),
            PermissionType::InputMonitoring => self.request_input_monitoring(tx),
            PermissionType::Photos => self.request_tcc_permission(typ, tx),
            PermissionType::SpeechRecognition => self.request_tcc_permission(typ, tx),
            PermissionType::DesktopFolder | PermissionType::DocumentsFolder | PermissionType::DownloadsFolder => self.request_tcc_permission(typ, tx),
            PermissionType::AppleEvents => self.request_tcc_permission(typ, tx),
            PermissionType::DeveloperTools => self.request_tcc_permission(typ, tx),
            PermissionType::AdminFiles => self.request_tcc_permission(typ, tx),
            PermissionType::NetworkVolumes => self.request_tcc_permission(typ, tx),
            PermissionType::RemovableVolumes => self.request_tcc_permission(typ, tx),
            PermissionType::Notification => notification_permissions::request_permission(tx),
            // All remaining permission types handled through TCC permissions
            PermissionType::AddressBook |
            PermissionType::All |
            PermissionType::Calls |
            PermissionType::FaceID |
            PermissionType::FileProviderDomain |
            PermissionType::FileProviderPresence |
            PermissionType::FocusStatus |
            PermissionType::MediaLibrary |
            PermissionType::Motion |
            PermissionType::NearbyInteraction |
            PermissionType::PhotosAdd |
            PermissionType::PostEvent |
            PermissionType::RemoteDesktop |
            PermissionType::Siri |
            PermissionType::UbiquitousFileProvider |
            PermissionType::WillfulWrite => self.request_tcc_permission(typ, tx),
        }
    }
}

impl MacOSHandler {
    pub fn check_accessibility(&self) -> Result<PermissionStatus, PermissionError> {
        #[cfg(target_os = "macos")]
        {
            let is_trusted = application_is_trusted();
            Ok(if is_trusted {
                PermissionStatus::Authorized
            } else {
                PermissionStatus::Denied
            })
        }
        
        #[cfg(not(target_os = "macos"))]
        Ok(PermissionStatus::Authorized)
    }

    pub fn request_accessibility(&self, tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
        #[cfg(target_os = "macos")]
        {
            application_is_trusted_with_prompt();
            // Check again after prompt
            let is_trusted = application_is_trusted();
            let result = if is_trusted {
                Ok(PermissionStatus::Authorized)
            } else {
                Ok(PermissionStatus::Denied)
            };
            tx.send(result).ok();
        }
        
        #[cfg(not(target_os = "macos"))]
        {
            tx.send(Ok(PermissionStatus::Authorized)).ok();
        }
    }

    pub fn check_wifi(&self) -> Result<PermissionStatus, PermissionError> {
        #[cfg(target_os = "macos")]
        {
            use objc2_core_wlan::CWWiFiClient;
            
            unsafe {
                let client = CWWiFiClient::sharedWiFiClient();
                if let Some(_interface) = client.interface() {
                    // If we can get an interface, WiFi is available
                    Ok(PermissionStatus::Authorized)
                } else {
                    // No interface available - WiFi might be disabled or permission denied
                    Ok(PermissionStatus::Denied)
                }
            }
        }
        
        #[cfg(not(target_os = "macos"))]
        Ok(PermissionStatus::Authorized)
    }

    pub fn check_screen_recording(&self) -> Result<PermissionStatus, PermissionError> {
        #[cfg(target_os = "macos")]
        unsafe {
            let has_access = CGPreflightScreenCaptureAccess();
            Ok(if has_access {
                PermissionStatus::Authorized
            } else {
                PermissionStatus::Denied
            })
        }
        
        #[cfg(not(target_os = "macos"))]
        Ok(PermissionStatus::Authorized)
    }

    pub fn request_screen_recording(&self, tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
        #[cfg(target_os = "macos")]
        unsafe {
            let granted = CGRequestScreenCaptureAccess();
            let result = if granted {
                Ok(PermissionStatus::Authorized)
            } else {
                Ok(PermissionStatus::Denied)
            };
            tx.send(result).ok();
        }
        
        #[cfg(not(target_os = "macos"))]
        {
            tx.send(Ok(PermissionStatus::Authorized)).ok();
        }
    }

    pub fn check_input_monitoring(&self) -> Result<PermissionStatus, PermissionError> {
        #[cfg(target_os = "macos")]
        unsafe {
            let status = IOHIDCheckAccess(1);
            Ok(if status == 0 {
                PermissionStatus::Authorized
            } else {
                PermissionStatus::Denied
            })
        }
        
        #[cfg(not(target_os = "macos"))]
        Ok(PermissionStatus::Authorized)
    }

    pub fn request_input_monitoring(&self, tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
        #[cfg(target_os = "macos")]
        {
            std::process::Command::new("open")
                .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_ListenEvent")
                .output()
                .ok();
            
            // Return current status (user needs to manually grant)
            let result = self.check_input_monitoring();
            tx.send(result).ok();
        }
        
        #[cfg(not(target_os = "macos"))]
        {
            tx.send(Ok(PermissionStatus::Authorized)).ok();
        }
    }

    fn request_tcc_permission(&self, typ: PermissionType, tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
        #[cfg(target_os = "macos")]
        {
            let result = tcc_permissions::request_permission(typ);
            tx.send(result).ok();
        }
        
        #[cfg(not(target_os = "macos"))]
        {
            tx.send(Ok(PermissionStatus::NotDetermined)).ok();
        }
    }
}