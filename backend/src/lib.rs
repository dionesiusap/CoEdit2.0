/*
 * File: src/lib.rs
 * Purpose: Library root and module organization
 * 
 * This is the root of the backend library, organizing and
 * re-exporting the main components:
 * - CRDT implementation
 * - WebSocket server
 */

pub mod crdt;
pub mod websocket;

// Re-export commonly used types
pub use crdt::{Document, Operation, Position, Timestamp};
pub use websocket::{Message, MessageType};
