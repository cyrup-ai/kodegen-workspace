//! System-level permission implementations

use tokio::sync::oneshot;

use crate::types::{PermissionError, PermissionStatus, PermissionType};

pub fn check_admin_files() -> Result<PermissionStatus, PermissionError> {
    match std::process::Command::new("id").arg("-u").output() {
        Ok(output) => {
            let uid = String::from_utf8_lossy(&output.stdout)
                .trim()
                .parse::<u32>()
                .unwrap_or(1000);
            if uid == 0 {
                Ok(PermissionStatus::Authorized)
            } else {
                Ok(PermissionStatus::Denied)
            }
        },
        Err(e) => Err(PermissionError::SystemError(format!(
            "System operation failed: {}",
            e
        ))),
    }
}

pub fn check_screen_capture() -> Result<PermissionStatus, PermissionError> {
    // Test actual screen capture device access
    match std::fs::File::open("/dev/fb0") {
        Ok(_) => Ok(PermissionStatus::Authorized),
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => Ok(PermissionStatus::Denied),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(PermissionStatus::Denied),
        Err(e) => Err(PermissionError::SystemError(format!(
            "System operation failed: {}",
            e
        ))),
    }
}

pub fn check_input_monitoring() -> Result<PermissionStatus, PermissionError> {
    match std::fs::File::open("/dev/input/event0") {
        Ok(_) => Ok(PermissionStatus::Authorized),
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => Ok(PermissionStatus::Denied),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(PermissionStatus::Denied),
        Err(e) => Err(PermissionError::SystemError(format!(
            "System operation failed: {}",
            e
        ))),
    }
}

pub fn check_network_volumes() -> Result<PermissionStatus, PermissionError> {
    match std::fs::metadata("/mnt") {
        Ok(_) => Ok(PermissionStatus::Authorized),
        Err(e) => Err(PermissionError::SystemError(format!(
            "System operation failed: {}",
            e
        ))),
    }
}

pub fn check_removable_volumes() -> Result<PermissionStatus, PermissionError> {
    match std::fs::metadata("/media") {
        Ok(_) => Ok(PermissionStatus::Authorized),
        Err(e) => Err(PermissionError::SystemError(format!(
            "System operation failed: {}",
            e
        ))),
    }
}

pub fn check_motion() -> Result<PermissionStatus, PermissionError> {
    match std::fs::metadata("/sys/class/input") {
        Ok(_) => Ok(PermissionStatus::Authorized),
        Err(e) => Err(PermissionError::SystemError(format!(
            "System operation failed: {}",
            e
        ))),
    }
}

pub fn request_admin_files(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    std::thread::spawn(move || {
        let result = check_admin_files();
        tx.send(result).ok();
    });
}

pub fn request_screen_capture(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    #[cfg(target_os = "linux")]
    {
        std::thread::spawn(move || {
            use std::time::Duration;

            use dbus::Message;
            use dbus::blocking::Connection;

            // Use XDG Desktop Portal for screen capture
            let result = match Connection::new_session() {
                Ok(conn) => {
                    let msg = match Message::new_method_call(
                        "org.freedesktop.portal.Desktop",
                        "/org/freedesktop/portal/desktop",
                        "org.freedesktop.portal.ScreenCast",
                        "CreateSession",
                    ) {
                        Ok(msg) => msg,
                        Err(e) => {
                            tx.send(Err(PermissionError::SystemError(format!(
                                "D-Bus message creation failed for Portal ScreenCast \
                                 CreateSession: {}",
                                e
                            )))).ok();
                            return;
                        },
                    };
                    match conn.send_with_reply_and_block(msg, Duration::from_secs(2)) {
                        Ok(reply) => {
                            if reply.msg_type() == dbus::MessageType::MethodReturn {
                                Ok(PermissionStatus::Authorized)
                            } else {
                                Ok(PermissionStatus::Denied)
                            }
                        },
                        Err(e) => Err(PermissionError::SystemError(format!(
                            "Portal ScreenCast request method call failed: {}",
                            e
                        ))),
                    }
                },
                Err(e) => Err(PermissionError::SystemError(format!(
                    "Portal ScreenCast request connection failed: {}",
                    e
                ))),
            };
            tx.send(result).ok();
        });
    }

    #[cfg(not(target_os = "linux"))]
    {
        tx.send(Ok(PermissionStatus::Authorized)).ok();
    }
}

pub fn request_input_monitoring(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    std::thread::spawn(move || {
        let result = check_input_monitoring();
        tx.send(result).ok();
    });
}

pub fn request_network_volumes(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    std::thread::spawn(move || {
        let result = check_network_volumes();
        tx.send(result).ok();
    });
}

pub fn request_removable_volumes(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    std::thread::spawn(move || {
        let result = check_removable_volumes();
        tx.send(result).ok();
    });
}

pub fn request_motion(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    std::thread::spawn(move || {
        let result = check_motion();
        tx.send(result).ok();
    });
}
