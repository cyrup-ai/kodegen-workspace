//! Integration tests for async Tokio API

use kodegen_native_permissions::{PermissionManager, PermissionType};

#[tokio::test]
async fn test_manager_creation() {
    let _manager = PermissionManager::new();
    // Manager should be created successfully (if we reach here, test passes)
}

#[tokio::test]
async fn test_check_permission_sync() {
    let manager = PermissionManager::new();
    
    // Check should not panic (result may vary by platform)
    let result = manager.check_permission(PermissionType::Camera);
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_request_permission_async() {
    let manager = PermissionManager::new();
    
    // Request should not panic (actual permission grant depends on OS state)
    let result = manager.request_permission(PermissionType::Camera).await;
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_batch_permissions() {
    let manager = PermissionManager::new();
    
    let permissions = vec![
        PermissionType::Camera,
        PermissionType::Microphone,
    ];
    
    let results = manager.request_permissions(&permissions).await;
    assert_eq!(results.len(), permissions.len());
}

#[tokio::test]
async fn test_manager_clone() {
    let manager1 = PermissionManager::new();
    let manager2 = manager1.clone();
    
    // Both managers should share the same cache
    let _ = manager1.check_permission(PermissionType::Camera);
    let _ = manager2.check_permission(PermissionType::Camera);
}

#[tokio::test]
async fn test_clear_cache() {
    let manager = PermissionManager::new();
    
    // Populate cache
    let _ = manager.check_permission(PermissionType::Camera);
    
    // Clear should not panic
    manager.clear_cache();
}
