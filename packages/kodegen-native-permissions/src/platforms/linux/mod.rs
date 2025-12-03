//! Linux platform-specific permission implementations - Complete reference implementation
//!
//! This module is decomposed into logical sub-modules for maintainability:
//! - `portal`: Portal-based permissions (Camera, Microphone, Location)
//! - `dbus_services`: D-Bus service permissions (Bluetooth, WiFi, Calendar, etc.)
//! - `filesystem`: Filesystem-based permissions (Photos, Documents, etc.)
//! - `system`: System-level permissions (Admin, Screen capture, etc.)
//! - `platform_specific`: Platform-specific permission mappings

use tokio::sync::oneshot;

use crate::types::{PermissionError, PermissionStatus, PermissionType};

pub mod dbus_services;
pub mod filesystem;
pub mod notification_permissions;
pub mod platform_specific;
pub mod portal;
pub mod system;

pub fn check_permission(typ: PermissionType) -> Result<PermissionStatus, PermissionError> {
    #[cfg(target_os = "linux")]
    {
        match typ {
            // Portal-based permissions
            PermissionType::Camera => portal::check_camera(),
            PermissionType::Microphone => portal::check_microphone(),
            PermissionType::Location => portal::check_location(),

            // D-Bus service permissions
            PermissionType::Bluetooth => dbus_services::check_bluetooth(),
            PermissionType::WiFi => dbus_services::check_wifi(),
            PermissionType::Calendar | PermissionType::Reminders => dbus_services::check_calendar(),
            PermissionType::Contacts | PermissionType::AddressBook => {
                dbus_services::check_contacts()
            },
            PermissionType::SpeechRecognition => dbus_services::check_speech_recognition(),
            PermissionType::Accessibility | PermissionType::AccessibilityMouse => {
                dbus_services::check_accessibility()
            },
            PermissionType::NearbyInteraction => dbus_services::check_nearby_interaction(),

            // Filesystem-based permissions
            PermissionType::Photos | PermissionType::PhotosAdd => filesystem::check_photos(),
            PermissionType::MediaLibrary => filesystem::check_media_library(),
            PermissionType::DesktopFolder => filesystem::check_desktop_folder(),
            PermissionType::DocumentsFolder => filesystem::check_documents_folder(),
            PermissionType::DownloadsFolder => filesystem::check_downloads_folder(),

            // System-level permissions
            PermissionType::FullDiskAccess | PermissionType::AdminFiles => {
                system::check_admin_files()
            },
            PermissionType::ScreenCapture | PermissionType::RemoteDesktop => {
                system::check_screen_capture()
            },
            PermissionType::InputMonitoring => system::check_input_monitoring(),
            PermissionType::NetworkVolumes => system::check_network_volumes(),
            PermissionType::RemovableVolumes => system::check_removable_volumes(),
            PermissionType::Motion => system::check_motion(),

            // Notification permission
            PermissionType::Notification => notification_permissions::check_permission(),

            // Platform-specific permissions
            PermissionType::AppleEvents | PermissionType::PostEvent => {
                platform_specific::check_apple_events()
            },
            PermissionType::All
            | PermissionType::DeveloperTools
            | PermissionType::FileProviderDomain
            | PermissionType::FileProviderPresence
            | PermissionType::UbiquitousFileProvider
            | PermissionType::WillfulWrite => {
                platform_specific::handle_general_linux_permission(typ)
            },

            // iOS-specific permissions not available on Linux
            PermissionType::Calls
            | PermissionType::FaceID
            | PermissionType::FocusStatus
            | PermissionType::Siri => platform_specific::handle_ios_specific_permission(typ),
        }
    }
    #[cfg(not(target_os = "linux"))]
    Ok(PermissionStatus::Authorized)
}

pub fn request_permission(
    typ: PermissionType,
    tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>,
) {
    #[cfg(target_os = "linux")]
    {
        match typ {
            // Portal-based permissions
            PermissionType::Camera => portal::request_camera(tx),
            PermissionType::Microphone => portal::request_microphone(tx),
            PermissionType::Location => portal::request_location(tx),

            // D-Bus service permissions
            PermissionType::Bluetooth => dbus_services::request_bluetooth(tx),
            PermissionType::WiFi => dbus_services::request_wifi(tx),
            PermissionType::Accessibility | PermissionType::AccessibilityMouse => {
                dbus_services::request_accessibility(tx)
            },
            PermissionType::Calendar | PermissionType::Reminders => {
                dbus_services::request_calendar(tx)
            },
            PermissionType::Contacts | PermissionType::AddressBook => {
                dbus_services::request_contacts(tx)
            },
            PermissionType::SpeechRecognition => dbus_services::request_speech_recognition(tx),
            PermissionType::NearbyInteraction => dbus_services::request_nearby_interaction(tx),

            // Filesystem-based permissions
            PermissionType::Photos | PermissionType::PhotosAdd => filesystem::request_photos(tx),
            PermissionType::MediaLibrary => filesystem::request_media_library(tx),
            PermissionType::DesktopFolder => filesystem::request_desktop_folder(tx),
            PermissionType::DocumentsFolder => filesystem::request_documents_folder(tx),
            PermissionType::DownloadsFolder => filesystem::request_downloads_folder(tx),

            // System-level permissions
            PermissionType::FullDiskAccess | PermissionType::AdminFiles => {
                system::request_admin_files(tx)
            },
            PermissionType::ScreenCapture | PermissionType::RemoteDesktop => {
                system::request_screen_capture(tx)
            },
            PermissionType::InputMonitoring => system::request_input_monitoring(tx),
            PermissionType::NetworkVolumes => system::request_network_volumes(tx),
            PermissionType::RemovableVolumes => system::request_removable_volumes(tx),
            PermissionType::Motion => system::request_motion(tx),

            // Notification permission
            PermissionType::Notification => notification_permissions::request_permission(tx),

            // Platform-specific permissions
            _ => platform_specific::request_general_linux_permission(typ, tx),
        }
    }
    #[cfg(not(target_os = "linux"))]
    {
        tx.send(Ok(PermissionStatus::Authorized)).ok();
    }
}
