//! Windows media permissions (Camera, Microphone, Speech Recognition)

use tokio::sync::oneshot;

#[cfg(target_os = "windows")]
use {
    windows::Media::Capture::{MediaCapture, MediaCaptureInitializationSettings},
    windows::Security::Authorization::AppCapabilityAccess::{
        AppCapability, AppCapabilityAccessStatus,
    },
    windows::core::Result as WinResult,
};

use super::helpers::convert_app_capability_status;
use crate::types::{PermissionError, PermissionStatus, PermissionType};

pub fn check_camera() -> Result<PermissionStatus, PermissionError> {
    #[cfg(target_os = "windows")]
    {
        match AppCapability::CreateForCapabilityName(&"webcam".into()) {
            Ok(capability) => match capability.AccessStatus() {
                Ok(status) => Ok(convert_app_capability_status(status)),
                Err(_) => Err(PermissionError::SystemError(
                    "Failed to get camera status".to_string(),
                )),
            },
            Err(_) => Err(PermissionError::SystemError(
                "Failed to create camera capability".to_string(),
            )),
        }
    }
    #[cfg(not(target_os = "windows"))]
    Ok(PermissionStatus::Authorized)
}

pub fn check_microphone() -> Result<PermissionStatus, PermissionError> {
    #[cfg(target_os = "windows")]
    {
        match AppCapability::CreateForCapabilityName(&"microphone".into()) {
            Ok(capability) => match capability.AccessStatus() {
                Ok(status) => Ok(convert_app_capability_status(status)),
                Err(_) => Err(PermissionError::SystemError(
                    "Failed to get microphone status".to_string(),
                )),
            },
            Err(_) => Err(PermissionError::SystemError(
                "Failed to create microphone capability".to_string(),
            )),
        }
    }
    #[cfg(not(target_os = "windows"))]
    Ok(PermissionStatus::Authorized)
}

pub fn check_speech_recognition() -> Result<PermissionStatus, PermissionError> {
    #[cfg(target_os = "windows")]
    {
        match AppCapability::CreateForCapabilityName(&"microphone".into()) {
            Ok(capability) => match capability.AccessStatus() {
                Ok(status) => Ok(convert_app_capability_status(status)),
                Err(_) => Err(PermissionError::SystemError(
                    "Failed to get speech recognition status".to_string(),
                )),
            },
            Err(_) => Err(PermissionError::SystemError(
                "Failed to create speech recognition capability".to_string(),
            )),
        }
    }
    #[cfg(not(target_os = "windows"))]
    Ok(PermissionStatus::Authorized)
}

pub fn request_camera(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    request_media_capture(tx, "camera");
}

pub fn request_microphone(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    request_media_capture(tx, "microphone");
}

fn request_media_capture(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>, media_type: &str) {
    #[cfg(target_os = "windows")]
    {
        tokio::task::spawn_blocking(move || {
            // Create single-threaded tokio runtime for !Send WinRT futures
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Failed to create tokio runtime for WinRT async operations");
            
            let result = rt.block_on(async {
                let settings = MediaCaptureInitializationSettings::new().map_err(|e| {
                    PermissionError::SystemError(format!(
                        "Failed to create {media_type} settings: {e}"
                    ))
                })?;

                let capture = MediaCapture::new().map_err(|e| {
                    PermissionError::SystemError(format!(
                        "Failed to create {media_type} MediaCapture: {e}"
                    ))
                })?;

                capture
                    .InitializeAsync(&settings)
                    .map_err(|e| {
                        PermissionError::SystemError(format!(
                            "Failed to initialize {media_type}: {e}"
                        ))
                    })?
                    .await
                    .map_err(|e| {
                        PermissionError::SystemError(format!(
                            "{media_type} initialization failed: {e}"
                        ))
                    })?;

                Ok(PermissionStatus::Authorized)
            });

            tx.send(result).ok();
        });
    }
    #[cfg(not(target_os = "windows"))]
    {
        tx.send(Ok(PermissionStatus::Authorized)).ok();
    }
}

pub fn request_speech_recognition(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    #[cfg(target_os = "windows")]
    {
        tokio::task::spawn_blocking(move || {
            // Create single-threaded tokio runtime for !Send WinRT futures
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Failed to create tokio runtime for WinRT async operations");
            
            let result = rt.block_on(async {
                let settings = MediaCaptureInitializationSettings::new().map_err(|e| {
                    PermissionError::SystemError(format!("Failed to create speech settings: {}", e))
                })?;

                // Set audio-only capture mode for speech
                settings
                    .SetStreamingCaptureMode(windows::Media::Capture::StreamingCaptureMode::Audio)
                    .map_err(|e| {
                        PermissionError::SystemError(format!(
                            "Failed to set speech capture mode: {}",
                            e
                        ))
                    })?;

                // Set MediaCategory to Speech for speech recognition permissions
                settings
                    .SetMediaCategory(windows::Media::Capture::MediaCategory::Speech)
                    .map_err(|e| {
                        PermissionError::SystemError(format!(
                            "Failed to set speech category: {}",
                            e
                        ))
                    })?;

                let capture = MediaCapture::new().map_err(|e| {
                    PermissionError::SystemError(format!(
                        "Failed to create speech MediaCapture: {}",
                        e
                    ))
                })?;

                capture
                    .InitializeAsync(&settings)
                    .map_err(|e| {
                        PermissionError::SystemError(format!(
                            "Failed to initialize speech capture: {}",
                            e
                        ))
                    })?
                    .await
                    .map_err(|e| {
                        PermissionError::SystemError(format!(
                            "Speech capture initialization failed: {}",
                            e
                        ))
                    })?;

                Ok(PermissionStatus::Authorized)
            });

            tx.send(result).ok();
        });
    }
    #[cfg(not(target_os = "windows"))]
    {
        tx.send(Ok(PermissionStatus::Authorized)).ok();
    }
}
