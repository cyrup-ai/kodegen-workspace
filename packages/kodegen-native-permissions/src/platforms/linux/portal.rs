//! Portal-based permission implementations for Camera, Microphone, and Location

use tokio::sync::oneshot;

#[cfg(target_os = "linux")]
use {
    ashpd::Request, ashpd::desktop::camera::Camera, ashpd::desktop::location::Location,
    ashpd::desktop::microphone::Microphone,
};

use crate::types::{PermissionError, PermissionStatus, PermissionType};

pub fn check_camera() -> Result<PermissionStatus, PermissionError> {
    #[cfg(target_os = "linux")]
    {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                match Camera::request().await {
                    Ok(request) => {
                        match request.response().await {
                            Ok(granted) => {
                                if granted {
                                    Ok(PermissionStatus::Authorized)
                                } else {
                                    Ok(PermissionStatus::Denied)
                                }
                            },
                            Err(_) => {
                                // Fallback to device file access if portal fails
                                match std::fs::File::open("/dev/video0") {
                                    Ok(_) => Ok(PermissionStatus::Authorized),
                                    Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
                                        Ok(PermissionStatus::Denied)
                                    },
                                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                                        Ok(PermissionStatus::Denied)
                                    },
                                    Err(e) => Err(PermissionError::SystemError(format!(
                                        "Portal method call failed: {}",
                                        e
                                    ))),
                                }
                            },
                        }
                    },
                    Err(_) => {
                        // Fallback to device file access if portal unavailable
                        match std::fs::File::open("/dev/video0") {
                            Ok(_) => Ok(PermissionStatus::Authorized),
                            Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
                                Ok(PermissionStatus::Denied)
                            },
                            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                                Ok(PermissionStatus::Denied)
                            },
                            Err(e) => Err(PermissionError::SystemError(format!(
                                "Video device access failed: {}",
                                e
                            ))),
                        }
                    },
                }
            })
        })
    }
    #[cfg(not(target_os = "linux"))]
    Ok(PermissionStatus::Authorized)
}

pub fn check_microphone() -> Result<PermissionStatus, PermissionError> {
    #[cfg(target_os = "linux")]
    {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                match Microphone::request().await {
                    Ok(request) => {
                        match request.response().await {
                            Ok(granted) => {
                                if granted {
                                    Ok(PermissionStatus::Authorized)
                                } else {
                                    Ok(PermissionStatus::Denied)
                                }
                            },
                            Err(_) => {
                                // Fallback to device file access if portal fails
                                match std::fs::File::open("/dev/snd/controlC0") {
                                    Ok(_) => Ok(PermissionStatus::Authorized),
                                    Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
                                        Ok(PermissionStatus::Denied)
                                    },
                                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                                        Ok(PermissionStatus::Denied)
                                    },
                                    Err(e) => Err(PermissionError::SystemError(format!(
                                        "System operation failed: {}",
                                        e
                                    ))),
                                }
                            },
                        }
                    },
                    Err(_) => {
                        // Fallback to device file access if portal unavailable
                        match std::fs::File::open("/dev/snd/controlC0") {
                            Ok(_) => Ok(PermissionStatus::Authorized),
                            Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
                                Ok(PermissionStatus::Denied)
                            },
                            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                                Ok(PermissionStatus::Denied)
                            },
                            Err(e) => Err(PermissionError::SystemError(format!(
                                "System operation failed: {}",
                                e
                            ))),
                        }
                    },
                }
            })
        })
    }
    #[cfg(not(target_os = "linux"))]
    Ok(PermissionStatus::Authorized)
}

pub fn check_location() -> Result<PermissionStatus, PermissionError> {
    #[cfg(target_os = "linux")]
    {
        use std::time::Duration;

        use dbus::Message;
        use dbus::blocking::Connection;

        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
            match Location::request().await {
                Ok(request) => {
                    match request.response().await {
                        Ok(granted) => {
                            if granted {
                                Ok(PermissionStatus::Authorized)
                            } else {
                                Ok(PermissionStatus::Denied)
                            }
                        },
                        Err(_) => {
                            // Fallback to D-Bus GeoClue2 check if portal fails
                            match Connection::new_session() {
                                Ok(conn) => {
                                    let msg = match Message::new_method_call(
                                        "org.freedesktop.GeoClue2",
                                        "/org/freedesktop/GeoClue2/Manager",
                                        "org.freedesktop.GeoClue2.Manager",
                                        "GetClient",
                                    ) {
                                        Ok(msg) => msg,
                                        Err(e) => {
                                            return Err(PermissionError::SystemError(format!(
                                                "D-Bus message creation failed for GeoClue2 \
                                                 GetClient: {}",
                                                e
                                            )));
                                        },
                                    };
                                    match conn
                                        .send_with_reply_and_block(msg, Duration::from_secs(2))
                                    {
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
                        },
                    }
                },
                Err(_) => {
                    // Fallback to D-Bus GeoClue2 check if portal unavailable
                    match Connection::new_session() {
                        Ok(conn) => {
                            let msg = match Message::new_method_call(
                                "org.freedesktop.GeoClue2",
                                "/org/freedesktop/GeoClue2/Manager",
                                "org.freedesktop.GeoClue2.Manager",
                                "GetClient",
                            ) {
                                Ok(msg) => msg,
                                Err(e) => {
                                    return Err(PermissionError::SystemError(format!(
                                        "D-Bus message creation failed for GeoClue2 GetClient: {}",
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
                },
            }
            })
        })
    }
    #[cfg(not(target_os = "linux"))]
    Ok(PermissionStatus::Authorized)
}

pub fn request_camera(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    #[cfg(target_os = "linux")]
    {
        tokio::spawn(async move {
            let result = match Camera::request().await {
                Ok(request) => match request.response().await {
                    Ok(granted) => {
                        if granted {
                            Ok(PermissionStatus::Authorized)
                        } else {
                            Ok(PermissionStatus::Denied)
                        }
                    },
                    Err(e) => Err(PermissionError::SystemError(e.to_string())),
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

pub fn request_microphone(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    #[cfg(target_os = "linux")]
    {
        tokio::spawn(async move {
            let result = match Microphone::request().await {
                Ok(request) => match request.response().await {
                    Ok(granted) => {
                        if granted {
                            Ok(PermissionStatus::Authorized)
                        } else {
                            Ok(PermissionStatus::Denied)
                        }
                    },
                    Err(e) => Err(PermissionError::SystemError(e.to_string())),
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

pub fn request_location(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    #[cfg(target_os = "linux")]
    {
        tokio::spawn(async move {
            let result = match Location::request().await {
                Ok(request) => match request.response().await {
                    Ok(granted) => {
                        if granted {
                            Ok(PermissionStatus::Authorized)
                        } else {
                            Ok(PermissionStatus::Denied)
                        }
                    },
                    Err(e) => Err(PermissionError::SystemError(e.to_string())),
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
