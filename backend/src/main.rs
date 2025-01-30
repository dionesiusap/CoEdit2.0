/*
 * File: main.rs
 * Purpose: Main entry point for the CRDT-based collaborative text editor backend server
 * 
 * Responsibilities:
 * - Initialize and configure the WebSocket server
 * - Manage WebSocket connections and client sessions
 * - Handle CRDT operations and document state
 * - Coordinate real-time updates between clients
 * - Manage document synchronization and consistency
 * 
 * This file serves as the primary coordinator for the backend server,
 * delegating specific responsibilities to appropriate modules while
 * maintaining the overall system state and communication flow.
 */

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc;
use warp::{ws::WebSocket, Filter};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use futures::StreamExt;
use tokio_stream::wrappers::UnboundedReceiverStream;

// Types for our CRDT implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Operation {
    client_id: String,
    timestamp: u64,
    character: char,
    position: Vec<u32>,
    is_delete: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Document {
    operations: Vec<Operation>,
    content: String,
}

// Shared state between all clients
type Documents = Arc<RwLock<HashMap<String, Document>>>;
type Clients = Arc<RwLock<HashMap<String, mpsc::UnboundedSender<Result<warp::ws::Message, warp::Error>>>>>;

#[tokio::main]
async fn main() {
    // Initialize logging
    env_logger::init();

    // Shared state
    let documents: Documents = Arc::new(RwLock::new(HashMap::new()));
    let clients: Clients = Arc::new(RwLock::new(HashMap::new()));

    // WebSocket route
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(with_clients(clients.clone()))
        .and(with_documents(documents.clone()))
        .map(|ws: warp::ws::Ws, clients, documents| {
            ws.on_upgrade(move |socket| handle_client(socket, clients, documents))
        });

    // Serve static files for the frontend
    let static_files = warp::path("static").and(warp::fs::dir("static"));
    
    // Combine routes
    let routes = ws_route.or(static_files);

    println!("Server starting on localhost:8000");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || clients.clone())
}

fn with_documents(documents: Documents) -> impl Filter<Extract = (Documents,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || documents.clone())
}

async fn handle_client(ws: WebSocket, clients: Clients, _documents: Documents) {
    let (mut client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();

    // Generate a client ID based on connection details
    let client_id = uuid::Uuid::new_v4().to_string();

    // Store the sender in our clients list
    clients.write().insert(client_id.clone(), client_sender);

    // Convert messages into a stream and forward them to the client
    let client_rcv = UnboundedReceiverStream::new(client_rcv);
    tokio::task::spawn(async move {
        if let Err(e) = client_rcv.forward(&mut client_ws_sender).await {
            eprintln!("Error sending websocket msg: {}", e);
        }
    });

    // Handle incoming messages
    while let Some(result) = client_ws_rcv.next().await {
        match result {
            Ok(msg) => {
                if let Ok(text) = msg.to_str() {
                    if let Ok(_operation) = serde_json::from_str::<Operation>(text) {
                        // Handle the operation and broadcast to other clients
                        // TODO: Implement CRDT logic here
                    }
                }
            }
            Err(e) => {
                eprintln!("Error receiving ws message: {}", e);
                break;
            }
        }
    }

    // Client disconnected
    clients.write().remove(&client_id);
}
