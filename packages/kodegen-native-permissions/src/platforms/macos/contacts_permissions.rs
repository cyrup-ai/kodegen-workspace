//! Contacts framework permissions - Complete reference implementation

use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;

use block2::RcBlock;
use objc2::runtime::Bool;
use objc2_contacts::{CNAuthorizationStatus, CNContactStore, CNEntityType};
use objc2_foundation::NSError;

use crate::types::{PermissionError, PermissionStatus};

pub fn check_permission() -> Result<PermissionStatus, PermissionError> {
    let status =
        unsafe { CNContactStore::authorizationStatusForEntityType(CNEntityType::Contacts) };
    let mapped = match status {
        CNAuthorizationStatus::Authorized => PermissionStatus::Authorized,
        CNAuthorizationStatus::Denied => PermissionStatus::Denied,
        CNAuthorizationStatus::Restricted => PermissionStatus::Restricted,
        _ => PermissionStatus::NotDetermined,
    };
    Ok(mapped)
}

pub fn request_permission(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    let store = unsafe { CNContactStore::new() };
    // Wrap sender in Arc<Mutex<Option<_>>> to allow interior mutability in Fn closure
    let tx = Arc::new(Mutex::new(Some(tx)));
    let handler = RcBlock::new(move |granted: Bool, error: *mut NSError| {
        // Take ownership of sender to call send()
        if let Ok(mut guard) = tx.lock()
            && let Some(sender) = guard.take()
        {
            if !error.is_null() {
                let _ = sender.send(Err(PermissionError::SystemError(
                    "Error in request".to_string(),
                )));
                return;
            }
            let status = if granted.as_bool() {
                PermissionStatus::Authorized
            } else {
                PermissionStatus::Denied
            };
            let _ = sender.send(Ok(status));
        }
    });

    unsafe {
        store.requestAccessForEntityType_completionHandler(CNEntityType::Contacts, &handler);
    }
}
