//! D-Bus service-based permission implementations
//!
//! This module contains D-Bus service integrations organized by functional area:
//! - `connectivity`: Network-related services (Bluetooth, WiFi)
//! - `productivity`: Productivity services (Calendar, Contacts via Evolution)
//! - `accessibility`: Accessibility and interaction services (A11y, Speech)

pub mod accessibility;
pub mod connectivity;
pub mod productivity;

use tokio::sync::oneshot;

pub use accessibility::{
    check_accessibility, check_nearby_interaction, check_speech_recognition, request_accessibility,
    request_nearby_interaction, request_speech_recognition,
};
// Re-export functions for compatibility
pub use connectivity::{check_bluetooth, check_wifi, request_bluetooth, request_wifi};
pub use productivity::{check_calendar, check_contacts, request_calendar, request_contacts};

use crate::types::{PermissionError, PermissionStatus};
