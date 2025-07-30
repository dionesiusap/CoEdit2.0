/*
 * File: tests/websocket/connection_tests.rs
 * Purpose: Test suite for WebSocket connection handling
 * 
 * Test Categories:
 * - Connection establishment and closure
 * - Client tracking and identification
 * - Connection status updates
 * - Connection error handling
 * - Connection recovery
 */

use std::time::Duration;
use tokio::time::timeout;
use crdt_editor_backend::websocket::connection::{
    ConnectionManager,
    ConnectionStatus,
    ClientInfo,
    ConnectionError,
};

#[tokio::test]
async fn test_connection_establishment() {
    let mut manager = ConnectionManager::new();
    let client_id = "client1".to_string();
    
    let result = manager.register_client(client_id.clone()).await;
    assert!(result.is_ok());
    
    let status = manager.get_client_status(&client_id).await;
    assert_eq!(status, Some(ConnectionStatus::Connected));
}

#[tokio::test]
async fn test_client_info_tracking() {
    let mut manager = ConnectionManager::new();
    let client_id = "client1".to_string();
    let client_info = ClientInfo {
        id: client_id.clone(),
        ip: "127.0.0.1".to_string(),
        connected_at: chrono::Utc::now(),
        last_activity: None,
    };
    
    manager.register_client_with_info(client_info.clone()).await.unwrap();
    
    let stored_info = manager.get_client_info(&client_id).await;
    assert_eq!(stored_info.as_ref().map(|i| &i.id), Some(&client_id));
    assert_eq!(stored_info.map(|i| i.ip), Some("127.0.0.1".to_string()));
}

#[tokio::test]
async fn test_connection_closure() {
    let mut manager = ConnectionManager::new();
    let client_id = "client1".to_string();
    
    manager.register_client(client_id.clone()).await.unwrap();
    manager.disconnect_client(&client_id).await.unwrap();
    
    let status = manager.get_client_status(&client_id).await;
    assert_eq!(status, Some(ConnectionStatus::Disconnected));
}

#[tokio::test]
async fn test_connection_timeout() {
    let mut manager = ConnectionManager::new();
    let client_id = "client1".to_string();
    
    manager.register_client(client_id.clone()).await.unwrap();
    
    // Simulate no activity for longer than heartbeat interval
    tokio::time::sleep(Duration::from_secs(5)).await;
    
    // Check if the connection has timed out
    let status = manager.get_client_status(&client_id).await;
    assert_eq!(status, Some(ConnectionStatus::TimedOut));
}

#[tokio::test]
async fn test_connection_recovery() {
    let mut manager = ConnectionManager::new();
    let client_id = "client1".to_string();
    
    // Initial connection
    manager.register_client(client_id.clone()).await.unwrap();
    
    // Simulate disconnection
    manager.disconnect_client(&client_id).await.unwrap();
    assert_eq!(
        manager.get_client_status(&client_id).await,
        Some(ConnectionStatus::Disconnected)
    );
    
    // Attempt recovery
    let result = manager.recover_connection(&client_id).await;
    assert!(result.is_ok());
    assert_eq!(
        manager.get_client_status(&client_id).await,
        Some(ConnectionStatus::Connected)
    );
}

#[tokio::test]
async fn test_concurrent_connections() {
    let manager = ConnectionManager::new();
    let mut manager1 = manager.clone();
    let mut manager2 = manager.clone();
    let client1 = "client1".to_string();
    let client2 = "client2".to_string();
    
    // Register multiple clients concurrently
    let (result1, result2) = tokio::join!(
        manager1.register_client(client1.clone()),
        manager2.register_client(client2.clone())
    );
    
    assert!(result1.is_ok());
    assert!(result2.is_ok());
    
    // Verify both clients are connected
    assert_eq!(
        manager.get_client_status(&client1).await,
        Some(ConnectionStatus::Connected)
    );
    assert_eq!(
        manager.get_client_status(&client2).await,
        Some(ConnectionStatus::Connected)
    );
}

#[tokio::test]
async fn test_connection_error_handling() {
    let mut manager = ConnectionManager::new();
    let client_id = "client1".to_string();
    
    // Try to disconnect a non-existent client
    let result = manager.disconnect_client(&client_id).await;
    assert!(matches!(
        result.unwrap_err(),
        ConnectionError::ClientNotFound(_)
    ));
    
    // Try to recover a non-existent connection
    let result = manager.recover_connection(&client_id).await;
    assert!(matches!(
        result.unwrap_err(),
        ConnectionError::ClientNotFound(_)
    ));
}

#[tokio::test]
async fn test_connection_heartbeat() {
    let mut manager = ConnectionManager::new();
    let client_id = "client1".to_string();
    
    manager.register_client(client_id.clone()).await.unwrap();
    
    // Send heartbeat
    let result = manager.update_heartbeat(&client_id).await;
    assert!(result.is_ok());
    
    // Verify last activity is updated
    let info = manager.get_client_info(&client_id).await.unwrap();
    assert!(info.last_activity.unwrap() > info.connected_at);
}

#[tokio::test]
async fn test_connection_statistics() {
    let mut manager = ConnectionManager::new();
    
    // Register multiple clients
    for i in 1..=3 {
        manager.register_client(format!("client{}", i)).await.unwrap();
    }
    
    // Disconnect one client
    manager.disconnect_client("client2").await.unwrap();
    
    let stats = manager.get_statistics().await;
    assert_eq!(stats.total_clients, 3);
    assert_eq!(stats.connected_clients, 2);
    assert_eq!(stats.disconnected_clients, 1);
}
