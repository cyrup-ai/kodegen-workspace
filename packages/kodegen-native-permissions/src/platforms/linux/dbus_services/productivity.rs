//! Productivity-related D-Bus service permissions (Calendar, Contacts)

use tokio::sync::oneshot;

#[cfg(target_os = "linux")]
use {
    dbus::{Message, blocking::Connection},
    std::time::Duration,
};

use crate::types::{PermissionError, PermissionStatus};

pub fn check_calendar() -> Result<PermissionStatus, PermissionError> {
    #[cfg(target_os = "linux")]
    {
        match Connection::new_session() {
            Ok(conn) => {
                let msg = match Message::new_method_call(
                    "org.gnome.evolution.dataserver.Calendar7",
                    "/org/gnome/evolution/dataserver/Calendar",
                    "org.gnome.evolution.dataserver.Calendar",
                    "GetCalendarList",
                ) {
                    Ok(msg) => msg,
                    Err(e) => {
                        return Err(PermissionError::SystemError(format!(
                            "D-Bus message creation failed for Evolution Calendar \
                             GetCalendarList: {}",
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

pub fn check_contacts() -> Result<PermissionStatus, PermissionError> {
    #[cfg(target_os = "linux")]
    {
        match Connection::new_session() {
            Ok(conn) => {
                let msg = match Message::new_method_call(
                    "org.gnome.evolution.dataserver.AddressBook10",
                    "/org/gnome/evolution/dataserver/AddressBook",
                    "org.gnome.evolution.dataserver.AddressBook",
                    "GetBookList",
                ) {
                    Ok(msg) => msg,
                    Err(e) => {
                        return Err(PermissionError::SystemError(format!(
                            "D-Bus message creation failed for Evolution AddressBook GetBookList: \
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

pub fn request_calendar(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    #[cfg(target_os = "linux")]
    {
        std::thread::spawn(move || {
            let result = match Connection::new_session() {
                Ok(conn) => {
                    let msg = match Message::new_method_call(
                        "org.gnome.evolution.dataserver.Calendar7",
                        "/org/gnome/evolution/dataserver/Calendar",
                        "org.gnome.evolution.dataserver.Calendar",
                        "GetCalendarList",
                    ) {
                        Ok(msg) => msg,
                        Err(e) => {
                            return Err(PermissionError::SystemError(format!(
                                "D-Bus message creation failed for Evolution Calendar \
                                 GetCalendarList: {}",
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
                            "Evolution Calendar request method call failed: {}",
                            e
                        ))),
                    }
                },
                Err(e) => Err(PermissionError::SystemError(format!(
                    "Evolution Calendar request connection failed: {}",
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

pub fn request_contacts(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    #[cfg(target_os = "linux")]
    {
        std::thread::spawn(move || {
            let result = match Connection::new_session() {
                Ok(conn) => {
                    let msg = match Message::new_method_call(
                        "org.gnome.evolution.dataserver.AddressBook10",
                        "/org/gnome/evolution/dataserver/AddressBook",
                        "org.gnome.evolution.dataserver.AddressBook",
                        "GetBookList",
                    ) {
                        Ok(msg) => msg,
                        Err(e) => {
                            return Err(PermissionError::SystemError(format!(
                                "D-Bus message creation failed for Evolution AddressBook \
                                 GetBookList: {}",
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
                            "Evolution AddressBook request method call failed: {}",
                            e
                        ))),
                    }
                },
                Err(e) => Err(PermissionError::SystemError(format!(
                    "Evolution AddressBook request connection failed: {}",
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
