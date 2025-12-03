//! macOS notification permission handling using UNUserNotificationCenter

use crate::types::{PermissionError, PermissionStatus};
use tokio::sync::oneshot;

#[cfg(target_os = "macos")]
use {
    block2::RcBlock,
    objc2::rc::autoreleasepool,
    objc2::runtime::Bool,
    objc2_foundation::NSError,
    objc2_user_notifications::{
        UNAuthorizationOptions, UNAuthorizationStatus, UNNotificationSettings,
        UNUserNotificationCenter,
    },
    parking_lot::{Condvar, Mutex},
    std::sync::Arc,
    std::sync::atomic::{AtomicBool, Ordering},
    std::time::Duration,
    tokio_util::sync::CancellationToken,
};

#[cfg(not(target_os = "macos"))]
use std::time::Duration;

/// Timeout for waiting on macOS notification settings query callback.
///
/// This timeout applies to `getNotificationSettingsWithCompletionHandler` which
/// queries the current authorization status. This is a pure system API call
/// with no user interaction - the callback fires immediately when macOS
/// returns the status.
///
/// 5 seconds is generous for what should be a sub-second operation, providing
/// headroom for slow or loaded systems while not blocking callers indefinitely.
#[cfg(target_os = "macos")]
const SETTINGS_CALLBACK_TIMEOUT: Duration = Duration::from_secs(5);

/// Synchronously check current notification permission status.
///
/// **Note**: This function blocks the calling thread for up to 5 seconds while
/// waiting for the macOS notification settings callback. For async contexts,
/// prefer `check_permission_async()`.
pub fn check_permission() -> Result<PermissionStatus, PermissionError> {
    check_permission_with_cancel(None)
}

/// Check current notification permission status with cancellation support
///
/// # Arguments
/// * `cancel` - Optional cancellation token. When cancelled, returns `Err(PermissionError::Cancelled)`
///
/// # Cancellation Behavior
/// - Cancellation is checked every 100ms during the wait
/// - If cancelled, immediately returns `Err(PermissionError::Cancelled)`
/// - If not cancelled within 5 seconds, returns `Ok(PermissionStatus::Unknown)`
#[cfg(target_os = "macos")]
pub fn check_permission_with_cancel(
    cancel: Option<&CancellationToken>,
) -> Result<PermissionStatus, PermissionError> {
    autoreleasepool(|_| {
        let notification_center = UNUserNotificationCenter::currentNotificationCenter();

        let status_holder = Arc::new((
            Mutex::new(None::<UNAuthorizationStatus>),
            Condvar::new(),
        ));
        let status_clone = Arc::clone(&status_holder);

        // Cancellation flag to prevent callback work after timeout/cancellation
        let cancelled = Arc::new(AtomicBool::new(false));
        let cancelled_clone = Arc::clone(&cancelled);

        let settings_block =
            RcBlock::new(move |settings: std::ptr::NonNull<UNNotificationSettings>| {
                // Early exit: Check cancellation BEFORE any work
                if cancelled_clone.load(Ordering::Acquire) {
                    return;  // Caller already timed out or was cancelled
                }

                let status = unsafe { settings.as_ref() }.authorizationStatus();
                let (lock, cvar) = &*status_clone;
                
                let mut guard = lock.lock();
                *guard = Some(status);
                cvar.notify_all();
            });

        notification_center.getNotificationSettingsWithCompletionHandler(&settings_block);

        // Wait with cancellation checking
        let (lock, cvar) = &*status_holder;
        let mut guard = lock.lock();

        let check_interval = Duration::from_millis(100);
        let max_wait = SETTINGS_CALLBACK_TIMEOUT;
        let mut elapsed = Duration::ZERO;

        loop {
            // Check cancellation first
            if let Some(token) = cancel
                && token.is_cancelled()
            {
                // Signal callback to skip work if it fires later
                cancelled.store(true, Ordering::Release);
                return Err(PermissionError::Cancelled);
            }

            // Check if result already available
            if guard.is_some() {
                break;
            }

            // Wait for small interval
            let timed_out = cvar.wait_for(&mut guard, check_interval).timed_out();

            if !timed_out {
                break; // Got result
            }

            elapsed += check_interval;
            if elapsed >= max_wait {
                // Signal callback to skip work if it fires later
                cancelled.store(true, Ordering::Release);
                return Ok(PermissionStatus::Unknown);
            }
        }

        match *guard {
            Some(UNAuthorizationStatus::Authorized)
            | Some(UNAuthorizationStatus::Provisional)
            | Some(UNAuthorizationStatus::Ephemeral) => Ok(PermissionStatus::Authorized),
            Some(UNAuthorizationStatus::Denied) => Ok(PermissionStatus::Denied),
            Some(UNAuthorizationStatus::NotDetermined) => Ok(PermissionStatus::NotDetermined),
            _ => Ok(PermissionStatus::Unknown),
        }
    })
}

/// Check current notification permission status with cancellation support (non-macOS stub)
#[cfg(not(target_os = "macos"))]
pub fn check_permission_with_cancel(
    _cancel: Option<&tokio_util::sync::CancellationToken>,
) -> Result<PermissionStatus, PermissionError> {
    Ok(PermissionStatus::Authorized)
}

/// Asynchronously check current notification permission status.
///
/// This is the preferred method for async contexts. Uses a callback-driven
/// architecture (no thread spawning) and returns the result via a tokio oneshot channel.
pub async fn check_permission_async() -> Result<PermissionStatus, PermissionError> {
    let (tx, rx) = oneshot::channel();
    check_permission_internal(tx);
    match rx.await {
        Ok(result) => result,
        Err(_) => Ok(PermissionStatus::Unknown),
    }
}

/// Internal helper that performs async permission check without blocking
fn check_permission_internal(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    #[cfg(target_os = "macos")]
    {
        let notification_center = UNUserNotificationCenter::currentNotificationCenter();

        // Wrap sender in Arc<Mutex<Option<_>>> for interior mutability
        let tx = Arc::new(Mutex::new(Some(tx)));

        let settings_block =
            RcBlock::new(move |settings: std::ptr::NonNull<UNNotificationSettings>| {
                let status = unsafe { settings.as_ref() }.authorizationStatus();
                
                let result = match status {
                    UNAuthorizationStatus::Authorized
                    | UNAuthorizationStatus::Provisional
                    | UNAuthorizationStatus::Ephemeral => Ok(PermissionStatus::Authorized),
                    UNAuthorizationStatus::Denied => Ok(PermissionStatus::Denied),
                    UNAuthorizationStatus::NotDetermined => Ok(PermissionStatus::NotDetermined),
                    _ => Ok(PermissionStatus::Unknown),
                };

                // Take ownership of sender to call send()
                let mut guard = tx.lock();
                if let Some(sender) = guard.take() {
                    let _ = sender.send(result);
                }
            });

        notification_center.getNotificationSettingsWithCompletionHandler(&settings_block);
        // Returns immediately - NO thread spawn, NO blocking
        // macOS calls settings_block when status is available
    }

    #[cfg(not(target_os = "macos"))]
    {
        tx.send(Ok(PermissionStatus::Authorized)).ok();
    }
}

/// Request notification permission from user
pub fn request_permission(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    request_permission_with_cancel(tx, None)
}

/// Request notification permission from user with cancellation support
///
/// # Arguments
/// * `tx` - Oneshot sender for the result
/// * `cancel` - Optional cancellation token. Checked in callback before sending result.
///
/// # Cancellation Behavior
/// - If cancelled before callback fires, result is not sent (receiver gets Err)
/// - Dialog may still be shown to user (macOS controls display)
/// - No threads are spawned, callback-driven architecture
#[cfg(target_os = "macos")]
pub fn request_permission_with_cancel(
    tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>,
    cancel: Option<CancellationToken>,
) {
    // If already cancelled, send error immediately and return
    if let Some(ref token) = cancel
        && token.is_cancelled()
    {
        tx.send(Err(PermissionError::Cancelled)).ok();
        return;
    }

    let notification_center = UNUserNotificationCenter::currentNotificationCenter();

    // Wrap sender and cancellation token in Arc<Mutex<Option<_>>> for interior mutability
    let state = Arc::new(Mutex::new(Some((tx, cancel.clone()))));
    
    // Cancellation flag to prevent callback work after cancellation
    let cancelled = Arc::new(AtomicBool::new(false));
    let cancelled_clone = Arc::clone(&cancelled);

    let request_block = RcBlock::new(move |granted: Bool, error: *mut NSError| {
        // Early exit: Check cancellation BEFORE any work
        if cancelled_clone.load(Ordering::Acquire) {
            return;  // Already cancelled, skip all work
        }

        // Now do the actual work
        let result = if !error.is_null() {
            let error_description = unsafe {
                let err_ref = &*error;
                err_ref.localizedDescription().to_string()
            };
            Err(PermissionError::SystemError(error_description))
        } else if granted.as_bool() {
            Ok(PermissionStatus::Authorized)
        } else {
            Ok(PermissionStatus::Denied)
        };

        // Take ownership of state to send result
        let mut guard = state.lock();
        if let Some((sender, cancel_token)) = guard.take() {
            // Check cancellation again before sending
            if let Some(ref token) = cancel_token
                && token.is_cancelled()
            {
                // Set atomic flag for any subsequent callback invocations
                cancelled_clone.store(true, Ordering::Release);
                return;  // Drop sender without sending - receiver gets Err
            }
            let _ = sender.send(result);
        }
    });

    // Request authorization with Alert, Sound, and Badge options
    let options = UNAuthorizationOptions::Alert
        | UNAuthorizationOptions::Sound
        | UNAuthorizationOptions::Badge;

    notification_center
        .requestAuthorizationWithOptions_completionHandler(options, &request_block);
    // Returns immediately - NO thread spawn, NO blocking
    // macOS calls request_block when user responds
}

/// Request notification permission from user with cancellation support (non-macOS stub)
#[cfg(not(target_os = "macos"))]
pub fn request_permission_with_cancel(
    tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>,
    _cancel: Option<tokio_util::sync::CancellationToken>,
) {
    tx.send(Ok(PermissionStatus::Authorized)).ok();
}
