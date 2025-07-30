/*
 * File: tests/websocket/message_tests.rs
 * Purpose: Test suite for WebSocket message handling
 * 
 * Test Categories:
 * - Message creation and validation
 * - Message serialization/deserialization
 * - Operation message handling
 * - Connection status messages
 * - Error message handling
 */

use crdt_editor_backend::websocket::message::{Message, MessageType, OperationMessage, StatusMessage};
use crdt_editor_backend::crdt::{Operation, Position};

#[test]
fn test_message_creation() {
    let client_id = "client1".to_string();
    let message = Message::new(
        MessageType::Operation,
        client_id.clone(),
        serde_json::to_value(OperationMessage {
            operation: Operation::insert(
                client_id,
                'A',
                Position::start(),
            ),
            document_id: "doc1".to_string(),
        }).unwrap(),
    );
    
    assert_eq!(message.message_type(), &MessageType::Operation);
}

#[test]
fn test_operation_message_serialization() {
    let client_id = "client1".to_string();
    let operation = Operation::insert(
        client_id.clone(),
        'A',
        Position::start(),
    );
    
    let msg = OperationMessage {
        operation,
        document_id: "doc1".to_string(),
    };
    
    let serialized = serde_json::to_string(&msg).unwrap();
    let deserialized: OperationMessage = serde_json::from_str(&serialized).unwrap();
    
    assert_eq!(deserialized.document_id, "doc1");
}

#[test]
fn test_status_message_serialization() {
    let msg = StatusMessage {
        client_id: "client1".to_string(),
        status: "connected".to_string(),
        timestamp: chrono::Utc::now(),
    };
    
    let serialized = serde_json::to_string(&msg).unwrap();
    let deserialized: StatusMessage = serde_json::from_str(&serialized).unwrap();
    
    assert_eq!(msg.client_id, deserialized.client_id);
    assert_eq!(msg.status, deserialized.status);
}

#[test]
fn test_error_message_handling() {
    let error_msg = Message::error(
        "client1".to_string(),
        "Invalid operation".to_string(),
    );
    
    assert_eq!(error_msg.message_type(), &MessageType::Error);
    
    if let serde_json::Value::String(error) = error_msg.payload() {
        assert_eq!(error, "Invalid operation");
    } else {
        panic!("Expected error message to be a string");
    }
}

#[test]
fn test_message_validation() {
    let client_id = "client1".to_string();
    let operation = Operation::insert(
        client_id.clone(),
        'A',
        Position::start(),
    );
    
    let msg = OperationMessage {
        operation,
        document_id: "".to_string(), // Invalid empty document ID
    };
    
    assert!(msg.validate().is_err());
}
