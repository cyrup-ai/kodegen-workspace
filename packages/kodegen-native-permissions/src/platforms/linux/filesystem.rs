//! Filesystem-based permission implementations

use tokio::sync::oneshot;

use crate::types::{PermissionError, PermissionStatus, PermissionType};

pub fn check_photos() -> Result<PermissionStatus, PermissionError> {
    let pictures_path = format!("{}/Pictures", std::env::var("HOME").unwrap_or_default());
    match std::fs::read_dir(&pictures_path) {
        Ok(_) => Ok(PermissionStatus::Authorized),
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => Ok(PermissionStatus::Denied),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(PermissionStatus::Denied),
        Err(e) => Err(PermissionError::SystemError(format!(
            "System operation failed: {}",
            e
        ))),
    }
}

pub fn check_media_library() -> Result<PermissionStatus, PermissionError> {
    let music_path = format!("{}/Music", std::env::var("HOME").unwrap_or_default());
    match std::fs::read_dir(&music_path) {
        Ok(_) => Ok(PermissionStatus::Authorized),
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => Ok(PermissionStatus::Denied),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(PermissionStatus::Denied),
        Err(e) => Err(PermissionError::SystemError(format!(
            "System operation failed: {}",
            e
        ))),
    }
}

pub fn check_desktop_folder() -> Result<PermissionStatus, PermissionError> {
    let desktop_path = format!("{}/Desktop", std::env::var("HOME").unwrap_or_default());
    match std::fs::read_dir(&desktop_path) {
        Ok(_) => Ok(PermissionStatus::Authorized),
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => Ok(PermissionStatus::Denied),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(PermissionStatus::Denied),
        Err(e) => Err(PermissionError::SystemError(format!(
            "System operation failed: {}",
            e
        ))),
    }
}

pub fn check_documents_folder() -> Result<PermissionStatus, PermissionError> {
    let documents_path = format!("{}/Documents", std::env::var("HOME").unwrap_or_default());
    match std::fs::read_dir(&documents_path) {
        Ok(_) => Ok(PermissionStatus::Authorized),
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => Ok(PermissionStatus::Denied),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(PermissionStatus::Denied),
        Err(e) => Err(PermissionError::SystemError(format!(
            "System operation failed: {}",
            e
        ))),
    }
}

pub fn check_downloads_folder() -> Result<PermissionStatus, PermissionError> {
    let downloads_path = format!("{}/Downloads", std::env::var("HOME").unwrap_or_default());
    match std::fs::read_dir(&downloads_path) {
        Ok(_) => Ok(PermissionStatus::Authorized),
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => Ok(PermissionStatus::Denied),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(PermissionStatus::Denied),
        Err(e) => Err(PermissionError::SystemError(format!(
            "System operation failed: {}",
            e
        ))),
    }
}

pub fn request_photos(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    std::thread::spawn(move || {
        let result = check_photos();
        tx.send(result).ok();
    });
}

pub fn request_media_library(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    std::thread::spawn(move || {
        let result = check_media_library();
        tx.send(result).ok();
    });
}

pub fn request_desktop_folder(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    std::thread::spawn(move || {
        let result = check_desktop_folder();
        tx.send(result).ok();
    });
}

pub fn request_documents_folder(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    std::thread::spawn(move || {
        let result = check_documents_folder();
        tx.send(result).ok();
    });
}

pub fn request_downloads_folder(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    std::thread::spawn(move || {
        let result = check_downloads_folder();
        tx.send(result).ok();
    });
}
