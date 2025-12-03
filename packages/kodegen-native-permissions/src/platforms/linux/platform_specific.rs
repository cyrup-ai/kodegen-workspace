//! Platform-specific permission mappings for Linux

use tokio::sync::oneshot;

use crate::types::{PermissionError, PermissionStatus, PermissionType};

pub fn check_apple_events() -> Result<PermissionStatus, PermissionError> {
    #[cfg(target_os = "linux")]
    {
        use std::time::Duration;

        use dbus::Message;
        use dbus::blocking::Connection;

        match Connection::new_session() {
            Ok(conn) => {
                let msg = match Message::new_method_call(
                    "org.freedesktop.DBus",
                    "/org/freedesktop/DBus",
                    "org.freedesktop.DBus",
                    "ListNames",
                ) {
                    Ok(msg) => msg,
                    Err(e) => {
                        return Err(PermissionError::SystemError(format!(
                            "D-Bus message creation failed for D-Bus ListNames: {}",
                            e
                        )));
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
                        "System operation failed: {}",
                        e
                    ))),
                }
            },
            Err(e) => Err(PermissionError::SystemError(format!(
                "System operation failed: {}",
                e
            ))),
        }
    }
    #[cfg(not(target_os = "linux"))]
    Ok(PermissionStatus::Authorized)
}

pub fn handle_ios_specific_permission(
    typ: PermissionType,
) -> Result<PermissionStatus, PermissionError> {
    match typ {
        PermissionType::Calls
        | PermissionType::FaceID
        | PermissionType::FocusStatus
        | PermissionType::Siri => Ok(PermissionStatus::Denied),
        _ => Ok(PermissionStatus::NotDetermined),
    }
}

pub fn handle_general_linux_permission(
    typ: PermissionType,
) -> Result<PermissionStatus, PermissionError> {
    match typ {
        PermissionType::All => Ok(PermissionStatus::NotDetermined), // Not a real Linux permission
        PermissionType::AppleEvents | PermissionType::PostEvent => check_apple_events(),
        PermissionType::DeveloperTools => Ok(PermissionStatus::Authorized), /* No Linux restrictions */
        PermissionType::FileProviderDomain | PermissionType::FileProviderPresence => {
            Ok(PermissionStatus::NotDetermined) // Linux file system access is different
        },
        PermissionType::UbiquitousFileProvider => Ok(PermissionStatus::Denied), // iOS iCloud
        // specific
        PermissionType::WillfulWrite => Ok(PermissionStatus::NotDetermined), // iOS concept
        _ => Ok(PermissionStatus::NotDetermined),
    }
}

pub fn request_apple_events(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    #[cfg(target_os = "linux")]
    {
        std::thread::spawn(move || {
            let result = match dbus::blocking::Connection::new_session() {
                Ok(conn) => {
                    let msg = match dbus::Message::new_method_call(
                        "org.freedesktop.DBus",
                        "/org/freedesktop/DBus",
                        "org.freedesktop.DBus",
                        "ListNames",
                    ) {
                        Ok(msg) => msg,
                        Err(e) => {
                            return Err(PermissionError::SystemError(format!(
                                "D-Bus message creation failed: {}",
                                e
                            )));
                        },
                    };
                    match conn.send_with_reply_and_block(msg, std::time::Duration::from_secs(2)) {
                        Ok(reply) => {
                            if reply.msg_type() == dbus::MessageType::MethodReturn {
                                Ok(PermissionStatus::Authorized)
                            } else {
                                Ok(PermissionStatus::Denied)
                            }
                        },
                        Err(e) => Err(PermissionError::SystemError(format!(
                            "System request method call failed: {}",
                            e
                        ))),
                    }
                },
                Err(e) => Err(PermissionError::SystemError(format!(
                    "System request connection failed: {}",
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

pub fn request_general_linux_permission(
    typ: PermissionType,
    tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>,
) {
    match typ {
        PermissionType::All => {
            tx.send(Ok(PermissionStatus::Authorized)).ok();
        },
        PermissionType::AppleEvents | PermissionType::PostEvent => {
            request_apple_events(tx);
        },
        PermissionType::DeveloperTools => {
            tx.send(Ok(PermissionStatus::Authorized)).ok();
        },
        PermissionType::FileProviderDomain | PermissionType::FileProviderPresence => {
            // Linux file system access is generally available
            tx.send(Ok(PermissionStatus::Authorized)).ok();
        },
        PermissionType::UbiquitousFileProvider => {
            // Cloud storage integration available
            tx.send(Ok(PermissionStatus::Authorized)).ok();
        },
        PermissionType::WillfulWrite => {
            tx.send(Ok(PermissionStatus::Authorized)).ok();
        },
        // iOS-specific permissions not available on Linux
        PermissionType::Calls
        | PermissionType::FaceID
        | PermissionType::FocusStatus
        | PermissionType::Siri => {
            tx.send(Ok(PermissionStatus::Denied)).ok();
        },
        _ => {
            tx.send(Ok(PermissionStatus::NotDetermined)).ok();
        },
    }
}
