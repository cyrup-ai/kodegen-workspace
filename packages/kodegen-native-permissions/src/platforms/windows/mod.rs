//! Windows platform-specific permission implementations - Complete reference implementation
//!
//! This module is decomposed into logical sub-modules for maintainability:
//! - `media`: Media permissions (Camera, Microphone, Speech Recognition)
//! - `connectivity`: Connectivity permissions (Bluetooth, WiFi, Location)
//! - `app_capabilities`: App capability permissions (Calendar, Contacts, Photos, etc.)
//! - `filesystem`: Filesystem permissions (Documents, Network/Removable volumes)
//! - `system`: System-level permissions (Screen capture, Input monitoring, Admin)
//! - `accessibility`: Accessibility permissions
//! - `platform_specific`: Platform-specific and iOS permission handling
//! - `helpers`: Helper functions for permission conversion

use tokio::sync::oneshot;

use crate::types::{PermissionError, PermissionStatus, PermissionType};

pub mod accessibility;
pub mod app_capabilities;
pub mod com_init;
pub mod connectivity;
pub mod filesystem;
pub mod helpers;
pub mod media;
pub mod notification_permissions;
pub mod platform_specific;
pub mod system;

pub fn check_permission(typ: PermissionType) -> Result<PermissionStatus, PermissionError> {
    #[cfg(target_os = "windows")]
    {
        match typ {
            // Media permissions
            PermissionType::Camera => media::check_camera(),
            PermissionType::Microphone => media::check_microphone(),
            PermissionType::SpeechRecognition => media::check_speech_recognition(),

            // Connectivity permissions
            PermissionType::Location => connectivity::check_location(),
            PermissionType::Bluetooth => connectivity::check_bluetooth(),
            PermissionType::WiFi => connectivity::check_wifi(),

            // Accessibility permissions
            PermissionType::Accessibility | PermissionType::AccessibilityMouse => {
                accessibility::check_accessibility()
            },

            // App capability permissions
            PermissionType::Calendar | PermissionType::Reminders => {
                app_capabilities::check_calendar()
            },
            PermissionType::Contacts | PermissionType::AddressBook => {
                app_capabilities::check_contacts()
            },
            PermissionType::Photos | PermissionType::PhotosAdd => app_capabilities::check_photos(),
            PermissionType::MediaLibrary => app_capabilities::check_media_library(),
            PermissionType::Motion => app_capabilities::check_motion(),
            PermissionType::NearbyInteraction => app_capabilities::check_nearby_interaction(),

            // Filesystem permissions
            PermissionType::DesktopFolder
            | PermissionType::DocumentsFolder
            | PermissionType::DownloadsFolder => filesystem::check_documents(),
            PermissionType::NetworkVolumes => filesystem::check_network_volumes(),
            PermissionType::RemovableVolumes => filesystem::check_removable_volumes(),

            // System permissions
            PermissionType::ScreenCapture | PermissionType::RemoteDesktop => {
                system::check_screen_capture()
            },
            PermissionType::InputMonitoring => system::check_input_monitoring(),
            PermissionType::FullDiskAccess | PermissionType::AdminFiles => {
                system::check_admin_access()
            },

            // Notification permission
            PermissionType::Notification => notification_permissions::check_permission(),

            // Platform-specific permissions
            _ => platform_specific::check_platform_specific(typ),
        }
    }
    #[cfg(not(target_os = "windows"))]
    Ok(PermissionStatus::Authorized)
}

pub fn request_permission(
    typ: PermissionType,
    tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>,
) {
    #[cfg(target_os = "windows")]
    {
        match typ {
            // Media permissions
            PermissionType::Camera => media::request_camera(tx),
            PermissionType::Microphone => media::request_microphone(tx),
            PermissionType::SpeechRecognition => media::request_speech_recognition(tx),

            // Connectivity permissions
            PermissionType::Location => connectivity::request_location(tx),
            PermissionType::Bluetooth => connectivity::request_bluetooth(tx),
            PermissionType::WiFi => connectivity::request_wifi(tx),

            // Accessibility permissions
            PermissionType::Accessibility | PermissionType::AccessibilityMouse => {
                accessibility::request_accessibility(tx)
            },

            // App capability permissions
            PermissionType::Calendar | PermissionType::Reminders => {
                app_capabilities::request_calendar(tx)
            },
            PermissionType::Contacts | PermissionType::AddressBook => {
                app_capabilities::request_contacts(tx)
            },
            PermissionType::Photos | PermissionType::PhotosAdd => {
                app_capabilities::request_photos(tx)
            },
            PermissionType::MediaLibrary => app_capabilities::request_media_library(tx),
            PermissionType::Motion => app_capabilities::request_motion(tx),
            PermissionType::NearbyInteraction => app_capabilities::request_nearby_interaction(tx),

            // Filesystem permissions
            PermissionType::DesktopFolder
            | PermissionType::DocumentsFolder
            | PermissionType::DownloadsFolder => filesystem::request_documents(tx),
            PermissionType::NetworkVolumes => filesystem::request_network_volumes(tx),
            PermissionType::RemovableVolumes => filesystem::request_removable_volumes(tx),

            // System permissions
            PermissionType::ScreenCapture | PermissionType::RemoteDesktop => {
                system::request_screen_capture(tx)
            },
            PermissionType::InputMonitoring => system::request_input_monitoring(tx),
            PermissionType::FullDiskAccess | PermissionType::AdminFiles => {
                system::request_admin_access(tx)
            },

            // Notification permission
            PermissionType::Notification => notification_permissions::request_permission(tx),

            // Platform-specific permissions
            _ => platform_specific::request_platform_specific(typ, tx),
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        tx.send(Ok(PermissionStatus::Authorized)).ok();
    }
}
