//! Request multiple permissions concurrently

use kodegen_native_permissions::{PermissionManager, PermissionType, PermissionStatus};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = PermissionManager::new();

    let permissions = vec![
        PermissionType::Camera,
        PermissionType::Microphone,
        PermissionType::Location,
    ];

    println!("Requesting {} permissions concurrently...", permissions.len());
    let results = manager.request_permissions(&permissions).await;

    println!("\nResults:");
    for (perm_type, result) in results {
        match result {
            Ok(PermissionStatus::Authorized) => {
                println!("✅ {:?}: Granted", perm_type);
            }
            Ok(status) => {
                println!("❌ {:?}: {:?}", perm_type, status);
            }
            Err(e) => {
                println!("❌ {:?}: Error - {}", perm_type, e);
            }
        }
    }

    Ok(())
}
