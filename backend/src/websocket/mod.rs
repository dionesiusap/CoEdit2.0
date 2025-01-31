/*
 * File: src/websocket/mod.rs
 * Purpose: WebSocket module organization and public exports
 * 
 * This module provides WebSocket functionality for real-time collaboration:
 * - message: Message types and serialization
 * - connection: Client connection management
 * - server: WebSocket server implementation
 */

pub mod message;
pub mod connection;
pub mod server;

// Re-export commonly used types
pub use message::{Message, MessageType};
pub use connection::{ConnectionManager, ConnectionStatus};
pub use server::{EditorServer, ServerConfig};
