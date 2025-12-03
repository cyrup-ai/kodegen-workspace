//! TCC (Transparency, Consent, and Control) permissions - Complete reference implementation

use std::fs::File;
use std::io::ErrorKind;
use std::path::PathBuf;

use home::home_dir;

use crate::types::{PermissionError, PermissionStatus, PermissionType};

pub fn check_permission(typ: PermissionType) -> Result<PermissionStatus, PermissionError> {
    let path = get_protected_path(typ);
    if let Some(p) = path {
        match File::open(&p) {
            Ok(_) => Ok(PermissionStatus::Authorized),
            Err(e) if e.kind() == ErrorKind::PermissionDenied => Ok(PermissionStatus::Denied),
            Err(e) => Err(PermissionError::SystemError(e.to_string())),
        }
    } else {
        // If no path is defined, assume permission is generally available
        Ok(PermissionStatus::NotDetermined)
    }
}

pub fn request_permission(typ: PermissionType) -> Result<PermissionStatus, PermissionError> {
    // TCC permissions cannot be requested programmatically - they require manual user grant
    // Open System Preferences to the appropriate privacy section
    let privacy_pane = match typ {
        // Specialized permissions handled by other modules - open to general Privacy pane
        PermissionType::Camera => "Privacy_Camera",
        PermissionType::Microphone => "Privacy_Microphone",
        PermissionType::Location => "Privacy_LocationServices",
        PermissionType::Calendar => "Privacy_Calendars",
        PermissionType::Reminders => "Privacy_Reminders",
        PermissionType::Contacts => "Privacy_Contacts",
        PermissionType::Bluetooth => "Privacy_Bluetooth",
        PermissionType::Accessibility => "Privacy_Accessibility",
        PermissionType::AccessibilityMouse => "Privacy_Accessibility",
        PermissionType::WiFi => "Privacy_WiFi",
        // TCC-specific permissions
        PermissionType::FullDiskAccess => "Privacy_AllFiles",
        PermissionType::DesktopFolder => "Privacy_DesktopFolder",
        PermissionType::DocumentsFolder => "Privacy_DocumentsFolder",
        PermissionType::DownloadsFolder => "Privacy_DownloadsFolder",
        PermissionType::Photos => "Privacy_Photos",
        PermissionType::SpeechRecognition => "Privacy_SpeechRecognition",
        PermissionType::ScreenCapture => "Privacy_ScreenCapture",
        PermissionType::InputMonitoring => "Privacy_ListenEvent",
        PermissionType::AppleEvents => "Privacy_Automation",
        PermissionType::DeveloperTools => "Privacy_DeveloperTool",
        PermissionType::AdminFiles => "Privacy_SystemPolicyAllFiles",
        PermissionType::NetworkVolumes => "Privacy_AllFiles",
        PermissionType::RemovableVolumes => "Privacy_AllFiles",
        // iOS/Modern permissions - open to general Privacy pane
        PermissionType::AddressBook => "Privacy_Contacts",
        PermissionType::All => "Privacy",
        PermissionType::Calls => "Privacy_CallHistory",
        PermissionType::FaceID => "Privacy_FaceID",
        PermissionType::FileProviderDomain => "Privacy_AllFiles",
        PermissionType::FileProviderPresence => "Privacy_AllFiles",
        PermissionType::FocusStatus => "Privacy_Focus",
        PermissionType::MediaLibrary => "Privacy_MediaLibrary",
        PermissionType::Motion => "Privacy_Motion",
        PermissionType::NearbyInteraction => "Privacy_NearbyInteraction",
        PermissionType::PhotosAdd => "Privacy_Photos",
        PermissionType::PostEvent => "Privacy_Automation",
        PermissionType::RemoteDesktop => "Privacy_RemoteDesktop",
        PermissionType::Siri => "Privacy_Siri",
        PermissionType::UbiquitousFileProvider => "Privacy_AllFiles",
        PermissionType::WillfulWrite => "Privacy_AllFiles",
        // Notification is handled by notification_permissions module, not TCC
        PermissionType::Notification => "Privacy_Notifications",
    };

    // Open System Preferences to the specific privacy pane
    std::process::Command::new("open")
        .arg(format!(
            "x-apple.systempreferences:com.apple.preference.security?{}",
            privacy_pane
        ))
        .output()
        .map_err(|e| PermissionError::SystemError(e.to_string()))?;

    // Return current status - user must manually grant permission
    check_permission(typ)
}

fn get_protected_path(typ: PermissionType) -> Option<PathBuf> {
    let home = home_dir()?;
    match typ {
        // Specialized permissions handled by other modules - return None to fall back to
        // NotDetermined
        PermissionType::Camera => None,
        PermissionType::Microphone => None,
        PermissionType::Location => None,
        PermissionType::Calendar => None,
        PermissionType::Reminders => None,
        PermissionType::Contacts => None,
        PermissionType::Bluetooth => None,
        PermissionType::Accessibility => None,
        PermissionType::AccessibilityMouse => None,
        PermissionType::WiFi => None,
        // TCC-specific file paths
        PermissionType::FullDiskAccess => Some(home.join("Library/Safari/Bookmarks.plist")),
        PermissionType::DesktopFolder => Some(home.join("Desktop")),
        PermissionType::DocumentsFolder => Some(home.join("Documents")),
        PermissionType::DownloadsFolder => Some(home.join("Downloads")),
        PermissionType::AdminFiles => Some(PathBuf::from(
            "/Library/Preferences/com.apple.TimeMachine.plist",
        )),
        PermissionType::Photos => Some(home.join("Pictures/Photos Library.photoslibrary")),
        PermissionType::SpeechRecognition => {
            Some(home.join(
                "Library/Preferences/com.apple.speech.recognition.AppleSpeechRecognition.prefs",
            ))
        },
        PermissionType::AppleEvents => {
            Some(home.join("Library/Preferences/com.apple.systemevents.plist"))
        },
        PermissionType::ScreenCapture => {
            Some(home.join("Library/Preferences/com.apple.screencapture.plist"))
        },
        PermissionType::InputMonitoring => {
            Some(home.join("Library/Preferences/com.apple.HIToolbox.plist"))
        },
        PermissionType::DeveloperTools => {
            Some(home.join("Library/Preferences/com.apple.dt.Xcode.plist"))
        },
        PermissionType::NetworkVolumes => Some(PathBuf::from("/Volumes")),
        PermissionType::RemovableVolumes => Some(PathBuf::from("/Volumes")),
        // iOS/Modern permissions - check via plist files or directories where applicable
        PermissionType::AddressBook => Some(home.join("Library/Application Support/AddressBook")),
        PermissionType::All => Some(home.join("Library")), // General library access
        PermissionType::Calls => Some(home.join("Library/CallHistoryDB")),
        PermissionType::FaceID => {
            Some(home.join("Library/Preferences/com.apple.LocalAuthentication.plist"))
        },
        PermissionType::FileProviderDomain => Some(home.join("Library/FileProvider")),
        PermissionType::FileProviderPresence => Some(home.join("Library/FileProvider")),
        PermissionType::FocusStatus => Some(home.join("Library/Preferences/com.apple.focus.plist")),
        PermissionType::MediaLibrary => {
            Some(home.join("Library/Application Support/com.apple.medialibraryd"))
        },
        PermissionType::Motion => Some(home.join("Library/Preferences/com.apple.CoreMotion.plist")),
        PermissionType::NearbyInteraction => {
            Some(home.join("Library/Preferences/com.apple.nearbyinteraction.plist"))
        },
        PermissionType::PhotosAdd => Some(home.join("Pictures/Photos Library.photoslibrary")),
        PermissionType::PostEvent => {
            Some(home.join("Library/Preferences/com.apple.systemevents.plist"))
        },
        PermissionType::RemoteDesktop => {
            Some(home.join("Library/Preferences/com.apple.RemoteDesktop.plist"))
        },
        PermissionType::Siri => Some(home.join("Library/Preferences/com.apple.Siri.plist")),
        PermissionType::UbiquitousFileProvider => Some(home.join("Library/Mobile Documents")),
        PermissionType::WillfulWrite => Some(home.join("Library/Preferences")),
        // Notification is handled by notification_permissions module, not TCC
        PermissionType::Notification => None,
    }
}
