//! Windows connectivity permissions (Bluetooth, WiFi, Location)

use tokio::sync::oneshot;

#[cfg(target_os = "windows")]
use {
    windows::Devices::Bluetooth::BluetoothAdapter,
    windows::Devices::Geolocation::{GeolocationAccessStatus, Geolocator},
    windows::Devices::WiFi::{WiFiAccessStatus, WiFiAdapter},
    windows::core::Result as WinResult,
};

use crate::types::{PermissionError, PermissionStatus, PermissionType};

pub fn check_location() -> Result<PermissionStatus, PermissionError> {
    #[cfg(target_os = "windows")]
    {
        // Use tokio runtime for WinRT async operations
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| PermissionError::SystemError(format!("Failed to create tokio runtime: {}", e)))?;

        let status = match rt.block_on(async {
            match Geolocator::RequestAccessAsync() {
                Ok(future) => future.await,
                Err(e) => Err(e),
            }
        }) {
            Ok(GeolocationAccessStatus::Allowed) => PermissionStatus::Authorized,
            Ok(GeolocationAccessStatus::Denied) => PermissionStatus::Denied,
            Ok(GeolocationAccessStatus::Unspecified) => PermissionStatus::NotDetermined,
            Ok(_) => PermissionStatus::NotDetermined,
            Err(_) => PermissionStatus::NotDetermined,
        };
        Ok(status)
    }
    #[cfg(not(target_os = "windows"))]
    Ok(PermissionStatus::Authorized)
}

pub fn check_bluetooth() -> Result<PermissionStatus, PermissionError> {
    #[cfg(target_os = "windows")]
    {
        // Use tokio runtime for WinRT async operations
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| PermissionError::SystemError(format!("Failed to create tokio runtime: {}", e)))?;

        let status = match rt.block_on(async {
            match BluetoothAdapter::GetDefaultAsync() {
                Ok(future) => future.await,
                Err(e) => Err(e),
            }
        }) {
            Ok(adapter) => {
                // Check if adapter exists and has capability
                match adapter {
                    Some(adapter) => {
                        // Check if Bluetooth Low Energy is supported as indicator of working
                        // Bluetooth
                        match adapter.IsLowEnergySupported() {
                            Ok(supported) => {
                                if supported {
                                    PermissionStatus::Authorized
                                } else {
                                    // Classic Bluetooth fallback
                                    match adapter.IsClassicSupported() {
                                        Ok(classic_supported) => {
                                            if classic_supported {
                                                PermissionStatus::Authorized
                                            } else {
                                                PermissionStatus::Denied
                                            }
                                        },
                                        Err(_) => PermissionStatus::Denied,
                                    }
                                }
                            },
                            Err(_) => PermissionStatus::Denied,
                        }
                    },
                    None => PermissionStatus::Denied,
                }
            },
            Err(_) => PermissionStatus::Denied,
        };
        Ok(status)
    }
    #[cfg(not(target_os = "windows"))]
    Ok(PermissionStatus::Authorized)
}

pub fn check_wifi() -> Result<PermissionStatus, PermissionError> {
    #[cfg(target_os = "windows")]
    {
        // Use tokio runtime for WinRT async operations
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| PermissionError::SystemError(format!("Failed to create tokio runtime: {}", e)))?;

        let status = match rt.block_on(async {
            match WiFiAdapter::RequestAccessAsync() {
                Ok(future) => future.await,
                Err(e) => Err(e),
            }
        }) {
            Ok(WiFiAccessStatus::Allowed) => PermissionStatus::Authorized,
            Ok(WiFiAccessStatus::DeniedBySystem) => PermissionStatus::Denied,
            Ok(WiFiAccessStatus::DeniedByUser) => PermissionStatus::Denied,
            Ok(_) => PermissionStatus::NotDetermined,
            Err(_) => PermissionStatus::NotDetermined,
        };
        Ok(status)
    }
    #[cfg(not(target_os = "windows"))]
    Ok(PermissionStatus::Authorized)
}

pub fn request_location(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    #[cfg(target_os = "windows")]
    {
        tokio::task::spawn_blocking(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Failed to create tokio runtime for WinRT async operations");

            let result = rt.block_on(async {
                let status = Geolocator::RequestAccessAsync()
                    .map_err(|e| {
                        PermissionError::SystemError(format!("Failed to request location: {}", e))
                    })?
                    .await
                    .map_err(|e| {
                        PermissionError::SystemError(format!("Location request failed: {}", e))
                    })?;

                Ok(convert_geolocation_status(status))
            });

            tx.send(result).ok();
        });
    }
    #[cfg(not(target_os = "windows"))]
    {
        tx.send(Ok(PermissionStatus::Authorized)).ok();
    }
}

pub fn request_bluetooth(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    #[cfg(target_os = "windows")]
    {
        tokio::task::spawn_blocking(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Failed to create tokio runtime for WinRT async operations");

            let result = rt.block_on(async {
                match BluetoothAdapter::GetDefaultAsync() {
                    Ok(future) => {
                        match future.await {
                            Ok(adapter) => {
                                // Check if adapter exists and has capability (same as
                                // check_permission)
                                match adapter {
                                    Some(adapter) => {
                                        // Check if Bluetooth Low Energy is supported as indicator
                                        // of working Bluetooth
                                        match adapter.IsLowEnergySupported() {
                                            Ok(supported) => {
                                                if supported {
                                                    Ok(PermissionStatus::Authorized)
                                                } else {
                                                    // Classic Bluetooth fallback
                                                    match adapter.IsClassicSupported() {
                                                        Ok(classic_supported) => {
                                                            if classic_supported {
                                                                Ok(PermissionStatus::Authorized)
                                                            } else {
                                                                Ok(PermissionStatus::Denied)
                                                            }
                                                        },
                                                        Err(e) => Err(
                                                            PermissionError::SystemError(format!(
                                                                "Failed to check classic \
                                                                 Bluetooth support: {}",
                                                                e
                                                            )),
                                                        ),
                                                    }
                                                }
                                            },
                                            Err(e) => Err(PermissionError::SystemError(format!(
                                                "Failed to check Bluetooth LE support: {}",
                                                e
                                            ))),
                                        }
                                    },
                                    None => Ok(PermissionStatus::Denied),
                                }
                            },
                            Err(e) => Err(PermissionError::SystemError(format!(
                                "Bluetooth adapter request failed: {}",
                                e
                            ))),
                        }
                    },
                    Err(e) => Err(PermissionError::SystemError(format!(
                        "Failed to request Bluetooth adapter: {}",
                        e
                    ))),
                }
            });

            tx.send(result).ok();
        });
    }
    #[cfg(not(target_os = "windows"))]
    {
        tx.send(Ok(PermissionStatus::Authorized)).ok();
    }
}

pub fn request_wifi(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    #[cfg(target_os = "windows")]
    {
        tokio::task::spawn_blocking(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Failed to create tokio runtime for WinRT async operations");

            let result = rt.block_on(async {
                match WiFiAdapter::RequestAccessAsync() {
                    Ok(future) => match future.await {
                        Ok(WiFiAccessStatus::Allowed) => Ok(PermissionStatus::Authorized),
                        Ok(WiFiAccessStatus::DeniedBySystem) => Ok(PermissionStatus::Denied),
                        Ok(WiFiAccessStatus::DeniedByUser) => Ok(PermissionStatus::Denied),
                        Ok(_) => Ok(PermissionStatus::NotDetermined),
                        Err(e) => Err(PermissionError::SystemError(format!(
                            "WiFi access request failed: {}",
                            e
                        ))),
                    },
                    Err(e) => Err(PermissionError::SystemError(format!(
                        "Failed to request WiFi access: {}",
                        e
                    ))),
                }
            });
            tx.send(result).ok();
        });
    }
    #[cfg(not(target_os = "windows"))]
    {
        tx.send(Ok(PermissionStatus::Authorized)).ok();
    }
}

#[cfg(target_os = "windows")]
fn convert_geolocation_status(status: GeolocationAccessStatus) -> PermissionStatus {
    match status {
        GeolocationAccessStatus::Allowed => PermissionStatus::Authorized,
        GeolocationAccessStatus::Denied => PermissionStatus::Denied,
        _ => PermissionStatus::NotDetermined,
    }
}
