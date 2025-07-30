/*
 * File: src/websocket/connection.rs
 * Purpose: WebSocket connection management
 * 
 * This module handles client connections and their states:
 * - Connection lifecycle (connect, disconnect, recover)
 * - Client tracking and identification
 * - Connection status monitoring
 * - Heartbeat mechanism
 */

use std::{
    collections::HashMap,
    sync::Arc,
};
use chrono::{DateTime, Utc};
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use thiserror::Error;
use tracing::{debug, error, info, warn};

/// Connection-specific errors
#[derive(Error, Debug)]
pub enum ConnectionError {
    #[error("Client {0} not found")]
    ClientNotFound(String),
    #[error("Client {0} already exists")]
    ClientExists(String),
    #[error("Invalid connection state: {0}")]
    InvalidState(String),
    #[error("Connection timeout")]
    Timeout,
}

/// Connection status states
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConnectionStatus {
    Connected,
    Disconnected,
    TimedOut,
}

/// Client information and metadata
#[derive(Debug, Clone)]
pub struct ClientInfo {
    pub id: String,
    pub ip: String,
    pub connected_at: DateTime<Utc>,
    pub last_activity: Option<DateTime<Utc>>,
}

/// Connection statistics
#[derive(Debug, Clone)]
pub struct ConnectionStats {
    pub total_clients: usize,
    pub connected_clients: usize,
    pub disconnected_clients: usize,
}

/// Manages WebSocket client connections
#[derive(Clone)]
pub struct ConnectionManager {
    clients: Arc<RwLock<HashMap<String, ClientInfo>>>,
    statuses: Arc<RwLock<HashMap<String, ConnectionStatus>>>,
}

impl ConnectionManager {
    /// Create a new connection manager
    pub fn new() -> Self {
        Self {
            clients: Arc::new(RwLock::new(HashMap::new())),
            statuses: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a new client with the given ID
    pub async fn register_client(&mut self, client_id: String) -> Result<(), ConnectionError> {
        let client_info = ClientInfo {
            id: client_id.clone(),
            ip: "127.0.0.1".to_string(), // Default IP for now
            connected_at: chrono::Utc::now(),
            last_activity: Some(chrono::Utc::now()),
        };
        self.register_client_with_info(client_info).await
    }

    /// Register a new client with the given client info
    pub async fn register_client_with_info(&mut self, client_info: ClientInfo) -> Result<(), ConnectionError> {
        let client_id = client_info.id.clone();
        
        // Update client info
        let mut clients = self.clients.write().await;
        clients.insert(client_id.clone(), client_info);
        
        // Update connection status
        let mut statuses = self.statuses.write().await;
        statuses.insert(client_id, ConnectionStatus::Connected);
        
        Ok(())
    }

    /// Get the current status of a client
    pub async fn get_client_status(&self, client_id: &str) -> Option<ConnectionStatus> {
        // First check if the client has timed out
        if let Some(info) = self.get_client_info(client_id).await {
            if let Some(last_activity) = info.last_activity {
                let now = chrono::Utc::now();
                let duration = now.signed_duration_since(last_activity);
                
                // If last activity was more than 3 seconds ago, mark as timed out
                if duration.num_seconds() > 3 {
                    let mut statuses = self.statuses.write().await;
                    statuses.insert(client_id.to_string(), ConnectionStatus::TimedOut);
                }
            }
        }
        
        let statuses = self.statuses.read().await;
        statuses.get(client_id).cloned()
    }

    /// Get client information
    pub async fn get_client_info(&self, client_id: &str) -> Option<ClientInfo> {
        let clients = self.clients.read().await;
        clients.get(client_id).cloned()
    }

    /// Update client heartbeat
    pub async fn update_heartbeat(&mut self, client_id: &str) -> Result<(), ConnectionError> {
        let mut clients = self.clients.write().await;
        
        if let Some(client_info) = clients.get_mut(client_id) {
            client_info.last_activity = Some(chrono::Utc::now());
            Ok(())
        } else {
            Err(ConnectionError::ClientNotFound(client_id.to_string()))
        }
    }

    /// Disconnect a client
    pub async fn disconnect_client(&mut self, client_id: &str) -> Result<(), ConnectionError> {
        let clients = self.clients.read().await;
        if !clients.contains_key(client_id) {
            return Err(ConnectionError::ClientNotFound(client_id.to_string()));
        }
        
        let mut statuses = self.statuses.write().await;
        statuses.insert(client_id.to_string(), ConnectionStatus::Disconnected);
        info!("Client disconnected: {}", client_id);
        Ok(())
    }

    /// Check if client connection has timed out
    pub async fn check_connection_timeout(&mut self, client_id: &str) -> Result<bool, ConnectionError> {
        let clients = self.clients.read().await;
        let mut statuses = self.statuses.write().await;

        let client = clients.get(client_id)
            .ok_or_else(|| ConnectionError::ClientNotFound(client_id.to_string()))?;

        if let Some(last_activity) = client.last_activity {
            let timeout = Utc::now()
                .signed_duration_since(last_activity)
                .num_seconds() > 30; // 30 seconds timeout

            if timeout {
                statuses.insert(client_id.to_string(), ConnectionStatus::TimedOut);
                warn!("Client connection timed out: {}", client_id);
            }

            Ok(timeout)
        } else {
            Ok(false)
        }
    }

    /// Attempt to recover a disconnected client
    pub async fn recover_connection(&mut self, client_id: &str) -> Result<(), ConnectionError> {
        let clients = self.clients.read().await;
        let mut statuses = self.statuses.write().await;

        if !clients.contains_key(client_id) {
            return Err(ConnectionError::ClientNotFound(client_id.to_string()));
        }

        match statuses.get(client_id) {
            Some(ConnectionStatus::Disconnected) | Some(ConnectionStatus::TimedOut) => {
                statuses.insert(client_id.to_string(), ConnectionStatus::Connected);
                info!("Client connection recovered: {}", client_id);
                Ok(())
            }
            Some(ConnectionStatus::Connected) => {
                Err(ConnectionError::InvalidState("Client is already connected".to_string()))
            }
            None => Err(ConnectionError::ClientNotFound(client_id.to_string())),
        }
    }

    /// Get connection statistics
    pub async fn get_statistics(&self) -> ConnectionStats {
        let clients = self.clients.read().await;
        let statuses = self.statuses.read().await;
        
        let total = clients.len();
        let connected = statuses.values()
            .filter(|&status| *status == ConnectionStatus::Connected)
            .count();
        let disconnected = total - connected;

        ConnectionStats {
            total_clients: total,
            connected_clients: connected,
            disconnected_clients: disconnected,
        }
    }
}

impl Default for ConnectionManager {
    fn default() -> Self {
        Self::new()
    }
}
