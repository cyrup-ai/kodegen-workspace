//! Accessibility and interaction D-Bus service permissions

use tokio::sync::oneshot;

#[cfg(target_os = "linux")]
use {
    dbus::{Message, blocking::Connection},
    std::time::Duration,
};

use crate::types::{PermissionError, PermissionStatus};

pub fn check_accessibility() -> Result<PermissionStatus, PermissionError> {
    #[cfg(target_os = "linux")]
    {
        match Connection::new_session() {
            Ok(conn) => {
                let msg = match Message::new_method_call(
                    "org.a11y.Bus",
                    "/org/a11y/bus",
                    "org.a11y.Status",
                    "GetStatus",
                ) {
                    Ok(msg) => msg,
                    Err(e) => {
                        return Err(PermissionError::SystemError(format!(
                            "D-Bus message creation failed for A11y Bus GetStatus: {}",
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

pub fn check_speech_recognition() -> Result<PermissionStatus, PermissionError> {
    #[cfg(target_os = "linux")]
    {
        match Connection::new_session() {
            Ok(conn) => {
                let msg = match Message::new_method_call(
                    "org.freedesktop.speech-dispatcher",
                    "/org/freedesktop/speech/dispatcher",
                    "org.freedesktop.speech.dispatcher",
                    "GetDefaultVoice",
                ) {
                    Ok(msg) => msg,
                    Err(e) => {
                        return Err(PermissionError::SystemError(format!(
                            "D-Bus message creation failed for Speech Dispatcher GetDefaultVoice: \
                             {}",
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

pub fn request_accessibility(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    #[cfg(target_os = "linux")]
    {
        std::thread::spawn(move || {
            let result = match Connection::new_session() {
                Ok(conn) => {
                    let msg = match Message::new_method_call(
                        "org.a11y.Bus",
                        "/org/a11y/bus",
                        "org.a11y.Status",
                        "GetStatus",
                    ) {
                        Ok(msg) => msg,
                        Err(e) => {
                            return Err(PermissionError::SystemError(format!(
                                "D-Bus message creation failed for A11y Bus GetStatus: {}",
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
            };
            tx.send(result).ok();
        });
    }
    #[cfg(not(target_os = "linux"))]
    {
        tx.send(Ok(PermissionStatus::Authorized)).ok();
    }
}

pub fn request_speech_recognition(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    #[cfg(target_os = "linux")]
    {
        std::thread::spawn(move || {
            let result = match Connection::new_session() {
                Ok(conn) => {
                    let msg = match Message::new_method_call(
                        "org.freedesktop.speech-dispatcher",
                        "/org/freedesktop/speech/dispatcher",
                        "org.freedesktop.speech.dispatcher",
                        "GetDefaultVoice",
                    ) {
                        Ok(msg) => msg,
                        Err(e) => {
                            return Err(PermissionError::SystemError(format!(
                                "D-Bus message creation failed for Speech Dispatcher \
                                 GetDefaultVoice: {}",
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
                            "Speech Dispatcher request method call failed: {}",
                            e
                        ))),
                    }
                },
                Err(e) => Err(PermissionError::SystemError(format!(
                    "Speech Dispatcher request connection failed: {}",
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

pub fn check_nearby_interaction() -> Result<PermissionStatus, PermissionError> {
    // Check Bluetooth availability as proxy for nearby interaction
    super::connectivity::check_bluetooth()
}

pub fn request_nearby_interaction(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    // Use Bluetooth as proxy for nearby interaction
    super::connectivity::request_bluetooth(tx);
}
