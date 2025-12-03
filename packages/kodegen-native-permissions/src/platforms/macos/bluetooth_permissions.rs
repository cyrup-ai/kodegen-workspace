//! Core Bluetooth permissions - Complete reference implementation

use tokio::sync::oneshot;
use std::sync::{Arc, Mutex, OnceLock};

use objc2::rc::Retained;
use objc2::runtime::{NSObject, NSObjectProtocol, ProtocolObject};
use objc2::{MainThreadMarker, MainThreadOnly, define_class, msg_send};
use objc2_core_bluetooth::{
    CBCentralManager, CBCentralManagerDelegate, CBManager, CBManagerAuthorization,
};

use crate::types::{PermissionError, PermissionStatus};

type BluetoothTxType = Arc<Mutex<Option<oneshot::Sender<Result<PermissionStatus, PermissionError>>>>>;

static BLUETOOTH_TX: OnceLock<BluetoothTxType> = OnceLock::new();

define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    struct BluetoothDelegate;

    unsafe impl NSObjectProtocol for BluetoothDelegate {}

    unsafe impl CBCentralManagerDelegate for BluetoothDelegate {
        #[unsafe(method(centralManagerDidUpdateState:))]
        #[allow(non_snake_case)]
        fn centralManagerDidUpdateState(&self, _central: &CBCentralManager) {
            if let Some(tx_arc) = BLUETOOTH_TX.get()
                && let Ok(mut guard) = tx_arc.lock()
                && let Some(tx) = guard.take()
            {
                let auth = unsafe { CBManager::authorization_class() };
                let status = match auth {
                    CBManagerAuthorization::AllowedAlways => PermissionStatus::Authorized,
                    CBManagerAuthorization::Denied => PermissionStatus::Denied,
                    CBManagerAuthorization::Restricted => PermissionStatus::Restricted,
                    _ => PermissionStatus::NotDetermined,
                };
                tx.send(Ok(status)).ok();
            }
        }
    }
);

impl From<CBManagerAuthorization> for PermissionStatus {
    fn from(auth: CBManagerAuthorization) -> Self {
        match auth {
            CBManagerAuthorization::AllowedAlways => Self::Authorized,
            CBManagerAuthorization::Denied => Self::Denied,
            CBManagerAuthorization::Restricted => Self::Restricted,
            _ => Self::NotDetermined,
        }
    }
}

pub fn check_permission() -> Result<PermissionStatus, PermissionError> {
    let auth = unsafe { CBManager::authorization_class() };
    Ok(auth.into())
}

pub fn request_permission(tx: oneshot::Sender<Result<PermissionStatus, PermissionError>>) {
    if let Some(mtm) = MainThreadMarker::new() {
        let _ = BLUETOOTH_TX.set(Arc::new(Mutex::new(Some(tx))));
        let delegate = BluetoothDelegate::alloc(mtm);
        let delegate: Retained<BluetoothDelegate> = unsafe { msg_send![delegate, init] };
        let manager = unsafe { CBCentralManager::new() };
        unsafe {
            manager.setDelegate(Some(ProtocolObject::from_ref(&*delegate)));
        }
    }
}
