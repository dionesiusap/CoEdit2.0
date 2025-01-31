/*
 * File: src/websocket/message.rs
 * Purpose: WebSocket message types and serialization
 * 
 * This module defines the message types used in WebSocket communication:
 * - Message: Base message structure
 * - MessageType: Enumeration of message types
 * - Specialized message types (Operation, Status, etc.)
 * 
 * Messages are serialized using serde for WebSocket transmission
 */

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::crdt::{Operation, Document};

/// Represents the type of WebSocket message
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MessageType {
    Connect,
    Connected,
    Disconnect,
    CreateDocument,
    DocumentCreated,
    GetDocument,
    DocumentState,
    Operation,
    Error,
    Status,
}

/// Base message structure for WebSocket communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "type")]
    message_type: MessageType,
    client_id: String,
    payload: serde_json::Value,
}

/// Message for document operations (insert, delete)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationMessage {
    pub operation: Operation,
    pub document_id: String,
}

/// Message for connection status updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusMessage {
    pub client_id: String,
    pub status: String,
    pub timestamp: DateTime<Utc>,
}

/// Message for document state synchronization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentStateMessage {
    pub document_id: String,
    pub content: String,
    pub version: u64,
}

impl Message {
    /// Create a new message with specified type, client ID, and payload
    pub fn new(
        message_type: MessageType,
        client_id: String,
        payload: serde_json::Value,
    ) -> Self {
        Self {
            message_type,
            client_id,
            payload,
        }
    }

    /// Create an error message
    pub fn error(client_id: String, error: String) -> Self {
        Self::new(
            MessageType::Error,
            client_id,
            serde_json::Value::String(error),
        )
    }

    /// Get the message type
    pub fn message_type(&self) -> &MessageType {
        &self.message_type
    }

    /// Get the client ID
    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    /// Get the message payload
    pub fn payload(&self) -> &serde_json::Value {
        &self.payload
    }
}

impl OperationMessage {
    /// Create a new operation message
    pub fn new(operation: Operation, document_id: String) -> Self {
        Self {
            operation,
            document_id,
        }
    }

    /// Validate the operation message
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.document_id.is_empty() {
            return Err("Document ID cannot be empty");
        }
        Ok(())
    }
}

impl StatusMessage {
    /// Create a new status message
    pub fn new(client_id: String, status: String) -> Self {
        Self {
            client_id,
            status,
            timestamp: Utc::now(),
        }
    }
}

impl DocumentStateMessage {
    /// Create a new document state message
    pub fn new(document_id: String, document: &Document) -> Self {
        Self {
            document_id,
            content: document.content().to_string(),
            version: document.version(),
        }
    }
}
