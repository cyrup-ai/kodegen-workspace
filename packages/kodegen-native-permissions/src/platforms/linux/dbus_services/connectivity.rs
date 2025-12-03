//! Network and connectivity D-Bus service permissions

use tokio::sync::oneshot;

#[cfg(target_os = "linux")]
use {
    dbus::{Message, blocking::Connection},
    std::time::Duration,
};

use crate::types::{PermissionError, PermissionStatus};

pub fn check_bluetooth() -> Result<PermissionStatus, PermissionError> {
    #[cfg(target_os = "linux")]
    {
        match Connection::new_session() {
            Ok(conn) => {
                let msg = match Message::new_method_call(
                    "org.bluez",
                    "/",
                    "org.bluez.Manager",
                    "GetDefaultAdapter",
                ) {
                    Ok(msg) => msg,
                    Err(e) => {
                        return Err(PermissionError::SystemError(format!(
                            "D-Bus message creation failed for BlueZ GetDefaultAdapter: {}",
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

pub fn check_wifi() -> Result<PermissionStatus, PermissionError> {
    #[cfg(target_os = "linux")]
    {
        match Connection::new_session() {
            Ok(conn) => {
                let msg = match Message::new_method_call(
                    "org.freedesktop.NetworkManager",
                    "/org/freedesktop/NetworkManager",
                    "org.freedesktop.NetworkManager",
                    "GetDevices",
                ) {
                    Ok(msg) => msg,
                    Err(e) => {
                        return Err(PermissionError::SystemError(format!(
                            "D-Bus message creation failed for NetworkManager GetDevices: {}",
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

pub fn request_bluetooth(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    #[cfg(target_os = "linux")]
    {
        std::thread::spawn(move || {
            let result = match Connection::new_session() {
                Ok(conn) => {
                    let msg = match Message::new_method_call(
                        "org.bluez",
                        "/",
                        "org.bluez.Manager",
                        "GetDefaultAdapter",
                    ) {
                        Ok(msg) => msg,
                        Err(e) => {
                            return Err(PermissionError::SystemError(format!(
                                "D-Bus message creation failed: {}",
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
                        Err(e) => Err(PermissionError::SystemError(e.to_string())),
                    }
                },
                Err(e) => Err(PermissionError::SystemError(e.to_string())),
            };
            tx.send(result).ok();
        });
    }
    #[cfg(not(target_os = "linux"))]
    {
        tx.send(Ok(PermissionStatus::Authorized)).ok();
    }
}

pub fn request_wifi(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    #[cfg(target_os = "linux")]
    {
        std::thread::spawn(move || {
            let result = match Connection::new_session() {
                Ok(conn) => {
                    let msg = match Message::new_method_call(
                        "org.freedesktop.NetworkManager",
                        "/",
                        "org.freedesktop.NetworkManager",
                        "GetDevices",
                    ) {
                        Ok(msg) => msg,
                        Err(e) => {
                            return Err(PermissionError::SystemError(format!(
                                "D-Bus message creation failed: {}",
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
                        Err(e) => Err(PermissionError::SystemError(e.to_string())),
                    }
                },
                Err(e) => Err(PermissionError::SystemError(e.to_string())),
            };
            tx.send(result).ok();
        });
    }
    #[cfg(not(target_os = "linux"))]
    {
        tx.send(Ok(PermissionStatus::Authorized)).ok();
    }
}
