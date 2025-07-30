/*
 * File: src/websocket/server.rs
 * Purpose: WebSocket server implementation for real-time collaboration
 * 
 * This module implements the WebSocket server that handles:
 * - Client connections and disconnections
 * - Message routing between clients
 * - Document state management
 * - Heartbeat mechanism for connection health
 */

use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};

use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use tokio::sync::RwLock;
use serde_json::json;
use tokio::sync::mpsc;
use warp::{
    ws::{Message as WsMessage, WebSocket},
    Filter,
};
use uuid::Uuid;

/// Represents a connected client


/// Tracks all connected clients
struct ClientManager {
    clients: RwLock<HashMap<String, mpsc::Sender<WsMessage>>>,
    client_count: AtomicUsize,
}

impl ClientManager {
    /// Create a new client manager
    fn new() -> Self {
        Self {
            clients: RwLock::new(HashMap::new()),
            client_count: AtomicUsize::new(0),
        }
    }

    /// Add a new client
    async fn add_client(&self, id: String, sender: mpsc::Sender<WsMessage>) {
        self.clients.write().await.insert(id, sender);
        self.client_count.fetch_add(1, Ordering::SeqCst);
    }

    /// Remove a client
    async fn remove_client(&self, id: &str) -> Option<mpsc::Sender<WsMessage>> {
        let mut clients = self.clients.write().await;
        let sender = clients.remove(id);
        if sender.is_some() {
            self.client_count.fetch_sub(1, Ordering::SeqCst);
        }
        sender
    }

    /// Get the number of connected clients


    /// Broadcast a message to all clients except the specified one
    async fn broadcast(&self, message: &Message, exclude_id: Option<&str>) {
        let message = match serde_json::to_string(message) {
            Ok(msg) => msg,
            Err(e) => {
                log::error!("Failed to serialize message: {}", e);
                return;
            }
        };

        let clients = self.clients.read().await;
        for (client_id, sender) in clients.iter() {
            if let Some(exclude) = exclude_id {
                if client_id == exclude {
                    continue;
                }
            }

            if let Err(e) = sender.send(WsMessage::text(message.clone())).await {
                log::error!("Failed to send message to client {}: {}", client_id, e);
            }
        }
    }
}

use crate::{
    crdt::Document,
    websocket::{
        connection::ConnectionManager,
        message::{Message, MessageType, OperationMessage},
    },
};

/// Configuration for the WebSocket server
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// Port to bind the server to
    pub port: u16,
    /// Host address to bind to
    pub host: String,
    /// Interval for sending heartbeat pings to clients
    pub heartbeat_interval: Duration,
    /// Time before considering a connection as timed out
    pub connection_timeout: Duration,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: 8080,
            host: "127.0.0.1".to_string(),
            heartbeat_interval: Duration::from_secs(30),
            connection_timeout: Duration::from_secs(60),
        }
    }
}

/// Main WebSocket server implementation
pub struct EditorServer {
    config: ServerConfig,
    connections: Arc<RwLock<ConnectionManager>>,
    documents: Arc<RwLock<HashMap<String, Document>>>,
    clients: Arc<ClientManager>,
}

impl EditorServer {
    /// Create a new WebSocket server with the given configuration
    pub fn new(config: ServerConfig) -> Self {
        Self {
            config,
            connections: Arc::new(RwLock::new(ConnectionManager::new())),
            documents: Arc::new(RwLock::new(HashMap::new())),
            clients: Arc::new(ClientManager::new()),
        }
    }

    /// Start the WebSocket server
    pub async fn run(&self) -> Result<()> {
        let connections = self.connections.clone();
        let documents = self.documents.clone();
        let clients = self.clients.clone();
        
        // WebSocket route
        let ws_route = warp::path("ws")
            .and(warp::ws())
            .map(move |ws: warp::ws::Ws| {
                let connections = connections.clone();
                let documents = documents.clone();
                let clients = clients.clone();
                
                ws.on_upgrade(move |socket| {
                    Self::handle_connection(
                        socket,
                        connections,
                        documents,
                        clients,
                    )
                })
            });

        // Start the server
        let addr = std::net::SocketAddr::new(
            self.config.host.parse()?,
            self.config.port,
        );
        
        log::info!("Starting WebSocket server on ws://{}", addr);
        warp::serve(ws_route)
            .run(addr)
            .await;

        Ok(())
    }

    /// Handle a new WebSocket connection
    async fn handle_connection(
        socket: WebSocket,
        connections: Arc<RwLock<ConnectionManager>>,
        documents: Arc<RwLock<HashMap<String, Document>>>,
        clients: Arc<ClientManager>,
    ) {
        // Generate a unique client ID
        let client_id = Uuid::new_v4().to_string();
        
        // Split the WebSocket into sender and receiver
        let (mut ws_sender, mut ws_receiver) = socket.split();
        
        // Create a channel for sending messages to this client
        let (tx, mut rx) = mpsc::channel(32);
        
        // Add client to client manager before registering with connection manager
        clients.add_client(client_id.clone(), tx.clone()).await;
        
        // Add the client to the connection manager
        {
            let mut manager = connections.write().await;
            if let Err(e) = manager.register_client(client_id.clone()).await {
                log::error!("Failed to register client: {}", e);
                clients.remove_client(&client_id).await;
                return;
            }
        }
        
        log::info!("Client connected: {}", client_id);
        
        // Send welcome message
        let welcome_msg = Message::new(
            MessageType::Status,
            client_id.clone(),
            json!({ "status": "connected", "client_id": &client_id }),
        );
        
        if let Err(e) = tx.send(WsMessage::text(serde_json::to_string(&welcome_msg).unwrap())).await {
            log::error!("Failed to send welcome message: {}", e);
            clients.remove_client(&client_id).await;
            return;
        }
        
        // Spawn a task to handle outgoing messages
        let send_task = tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                if let Err(e) = ws_sender.send(message).await {
                    log::error!("Failed to send WebSocket message: {}", e);
                    break;
                }
            }
        });
        
        // Spawn a task to handle incoming messages
        let receive_task = tokio::spawn({
            let connections = connections.clone();
            let documents = documents.clone();
            let clients = clients.clone();
            let client_id = client_id.clone();
            
            async move {
                while let Some(result) = ws_receiver.next().await {
                    match result {
                        Ok(msg) => {
                            if let Ok(text) = msg.to_str() {
                                if let Ok(message) = serde_json::from_str::<Message>(text) {
                                    // Create a new task to handle the message asynchronously
                                    let connections = connections.clone();
                                    let documents = documents.clone();
                                    let clients = clients.clone();
                                    let client_id = client_id.clone();
                                    
                                    tokio::spawn(async move {
                                        Self::handle_message(
                                            message,
                                            &client_id,
                                            &connections,
                                            &documents,
                                            &*clients
                                        ).await;
                                    });
                                }
                            }
                        }
                        Err(e) => {
                            log::error!("WebSocket error: {}", e);
                            break;
                        }
                    }
                }
            }
        });
        
        // Wait for either task to complete
        tokio::select! {
            _ = send_task => {}
            _ = receive_task => {}
        }
        
        // Clean up on disconnect
        log::info!("Client disconnected: {}", client_id);
        clients.remove_client(&client_id).await;
        if let Err(e) = connections.write().await.disconnect_client(&client_id).await {
            log::error!("Failed to remove connection: {}", e);
        }
    }
    
    /// Handle incoming WebSocket messages
    async fn handle_message(
        message: Message,
        client_id: &str,
        _connections: &Arc<RwLock<ConnectionManager>>,
        documents: &Arc<RwLock<HashMap<String, Document>>>,
        clients: &ClientManager,
    ) {
        match message.message_type() {
            MessageType::Operation => {
                if let Ok(op_msg) = serde_json::from_value::<OperationMessage>(message.payload().clone()) {
                    // Handle document operation
                    let mut docs = documents.write().await;
                    if let Some(doc) = docs.get_mut(&op_msg.document_id) {
                        // Apply the operation to the document
                        if let Err(e) = doc.apply_operation(op_msg.operation.clone()) {
                            log::error!("Failed to apply operation: {}", e);
                        } else {
                            log::info!("Applied operation to document {}", op_msg.document_id);
                        }
                    } else {
                        log::warn!("Document not found: {}", op_msg.document_id);
                    }
                }

                // Broadcast the operation to other clients
                clients.broadcast(&message, Some(client_id)).await;
            }
            _ => {
                log::debug!("Unhandled message type: {:?}", message.message_type());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    
    
    
    #[tokio::test]
    async fn test_server_initialization() {
        // Create a test server configuration
        let config = ServerConfig {
            port: 0, // Let OS assign a port
            ..Default::default()
        };
        
        // Create a new server instance
        let server = EditorServer::new(config);
        
        // Test connection statistics
        let stats = server.connections.read().await.get_statistics().await;
        assert_eq!(stats.total_clients, 0);
        
        // Test documents map is empty
        assert!(server.documents.read().await.is_empty());
    }
    
    #[tokio::test]
    async fn test_websocket_connection() {
        // This would test the WebSocket connection flow
        // Implementation would involve starting a test server and connecting to it
    }
}
