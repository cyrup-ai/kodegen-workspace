//! Permission types and error definitions

use std::fmt;

/// System permission types supported across platforms
#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum PermissionType {
    Camera,
    Microphone,
    Location,
    Calendar,
    Reminders,
    Contacts,
    Bluetooth,
    FullDiskAccess,
    ScreenCapture,
    Accessibility,
    AccessibilityMouse,
    InputMonitoring,
    Photos,
    SpeechRecognition,
    DesktopFolder,
    DocumentsFolder,
    DownloadsFolder,
    AppleEvents,
    DeveloperTools,
    AdminFiles,
    AddressBook,
    All,
    Calls,
    FaceID,
    FileProviderDomain,
    FileProviderPresence,
    FocusStatus,
    MediaLibrary,
    Motion,
    NearbyInteraction,
    PhotosAdd,
    PostEvent,
    RemoteDesktop,
    Siri,
    NetworkVolumes,
    RemovableVolumes,
    UbiquitousFileProvider,
    WillfulWrite,
    WiFi,
    Notification,
}

/// Permission status returned by the system
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PermissionStatus {
    /// Permission has not been requested yet
    NotDetermined,
    /// Permission has been granted
    Authorized,
    /// Permission has been denied by user
    Denied,
    /// Permission is restricted by system policy
    Restricted,
    /// User prompt required for elevation (UAC on Windows, sudo on Unix)
    PromptRequired,
    /// Permission status is unknown
    Unknown,
}

/// Errors that can occur during permission operations
#[derive(Debug)]
pub enum PermissionError {
    Denied,
    Restricted,
    SystemError(String),
    PlatformError(String),
    Unknown,
    Cancelled,
}

impl fmt::Display for PermissionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Denied => write!(f, "Permission denied"),
            Self::Restricted => write!(f, "Permission restricted"),
            Self::SystemError(s) => write!(f, "System error: {}", s),
            Self::PlatformError(s) => write!(f, "Platform error: {}", s),
            Self::Unknown => write!(f, "Unknown error"),
            Self::Cancelled => write!(f, "Operation cancelled"),
        }
    }
}

impl fmt::Display for PermissionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Camera => write!(f, "Camera"),
            Self::Microphone => write!(f, "Microphone"),
            Self::Location => write!(f, "Location"),
            Self::Calendar => write!(f, "Calendar"),
            Self::Reminders => write!(f, "Reminders"),
            Self::Contacts => write!(f, "Contacts"),
            Self::Bluetooth => write!(f, "Bluetooth"),
            Self::FullDiskAccess => write!(f, "Full Disk Access"),
            Self::ScreenCapture => write!(f, "Screen Capture"),
            Self::Accessibility => write!(f, "Accessibility"),
            Self::AccessibilityMouse => write!(f, "Accessibility Mouse"),
            Self::InputMonitoring => write!(f, "Input Monitoring"),
            Self::Photos => write!(f, "Photos"),
            Self::SpeechRecognition => write!(f, "Speech Recognition"),
            Self::DesktopFolder => write!(f, "Desktop Folder"),
            Self::DocumentsFolder => write!(f, "Documents Folder"),
            Self::DownloadsFolder => write!(f, "Downloads Folder"),
            Self::AppleEvents => write!(f, "Apple Events"),
            Self::DeveloperTools => write!(f, "Developer Tools"),
            Self::AdminFiles => write!(f, "Admin Files"),
            Self::AddressBook => write!(f, "Address Book"),
            Self::All => write!(f, "All Permissions"),
            Self::Calls => write!(f, "Calls"),
            Self::FaceID => write!(f, "Face ID"),
            Self::FileProviderDomain => write!(f, "File Provider Domain"),
            Self::FileProviderPresence => write!(f, "File Provider Presence"),
            Self::FocusStatus => write!(f, "Focus Status"),
            Self::MediaLibrary => write!(f, "Media Library"),
            Self::Motion => write!(f, "Motion"),
            Self::NearbyInteraction => write!(f, "Nearby Interaction"),
            Self::PhotosAdd => write!(f, "Photos Add"),
            Self::PostEvent => write!(f, "Post Event"),
            Self::RemoteDesktop => write!(f, "Remote Desktop"),
            Self::Siri => write!(f, "Siri"),
            Self::NetworkVolumes => write!(f, "Network Volumes"),
            Self::RemovableVolumes => write!(f, "Removable Volumes"),
            Self::UbiquitousFileProvider => write!(f, "Ubiquitous File Provider"),
            Self::WillfulWrite => write!(f, "Willful Write"),
            Self::WiFi => write!(f, "WiFi"),
            Self::Notification => write!(f, "Notification"),
        }
    }
}

impl fmt::Display for PermissionStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NotDetermined => write!(f, "Not Determined"),
            Self::Authorized => write!(f, "Authorized"),
            Self::Denied => write!(f, "Denied"),
            Self::Restricted => write!(f, "Restricted"),
            Self::PromptRequired => write!(f, "Prompt Required"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}
