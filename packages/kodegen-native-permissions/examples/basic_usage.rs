//! Basic permission checking and requesting with Tokio

use kodegen_native_permissions::{PermissionManager, PermissionType, PermissionStatus};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = PermissionManager::new();

    // Check camera permission status
    println!("Checking camera permission...");
    match manager.check_permission(PermissionType::Camera) {
        Ok(status) => println!("Camera status: {:?}", status),
        Err(e) => eprintln!("Error checking camera: {}", e),
    }

    // Request camera permission (shows native OS dialog)
    println!("\nRequesting camera permission...");
    match manager.request_permission(PermissionType::Camera).await {
        Ok(PermissionStatus::Authorized) => {
            println!("✅ Camera permission granted!");
        }
        Ok(status) => {
            println!("❌ Camera permission not granted: {:?}", status);
        }
        Err(e) => {
            eprintln!("❌ Error requesting camera: {}", e);
        }
    }

    Ok(())
}
