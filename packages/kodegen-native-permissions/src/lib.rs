//! kodegen-native-permissions
//!
//! Pure Tokio-based cross-platform system permissions management library.
//! Provides async APIs to check and request native OS permissions (Camera, Microphone, 
//! Location, etc.) with zero UI framework dependencies.
//!
//! # Example
//! ```rust
//! use kodegen_native_permissions::{PermissionManager, PermissionType};
//!
//! #[tokio::main]
//! async fn main() {
//!     let manager = PermissionManager::new();
//!     
//!     // Check current status
//!     if let Ok(status) = manager.check_permission(PermissionType::Camera) {
//!         println!("Camera permission: {:?}", status);
//!     }
//!     
//!     // Request permission (shows native OS dialog)
//!     match manager.request_permission(PermissionType::Camera).await {
//!         Ok(status) => println!("Permission granted: {:?}", status),
//!         Err(e) => eprintln!("Permission denied: {}", e),
//!     }
//! }
//! ```

#![recursion_limit = "256"]

pub mod manager;
pub mod traits;
pub mod types;

// Windows-specific configuration module
#[cfg(target_os = "windows")]
pub mod config;

#[cfg(target_os = "macos")]
pub mod platforms;

#[cfg(target_os = "windows")]
pub mod platforms;

#[cfg(target_os = "linux")]
pub mod platforms;

// Clean re-exports
pub use manager::PermissionManager;
pub use traits::PermissionHandler;
pub use types::{PermissionError, PermissionStatus, PermissionType};

// Re-export Windows config functions
#[cfg(target_os = "windows")]
pub use config::{get_windows_app_id, set_windows_app_id};
