/*
 * File: tests/crdt/timestamp_tests.rs
 * Purpose: Test suite for Lamport timestamp implementation
 * 
 * Test Categories:
 * - Timestamp creation and initialization
 * - Timestamp comparison and ordering
 * - Timestamp increment and update
 * - Timestamp synchronization between replicas
 */

use crdt_editor_backend::crdt::Timestamp;

#[test]
fn test_timestamp_creation() {
    let ts = Timestamp::new("client1".to_string());
    assert_eq!(ts.logical_clock(), 0);
    assert_eq!(ts.client_id(), "client1");
}

#[test]
fn test_timestamp_increment() {
    let mut ts = Timestamp::new("client1".to_string());
    ts.increment();
    assert_eq!(ts.logical_clock(), 1);
    ts.increment();
    assert_eq!(ts.logical_clock(), 2);
}

#[test]
fn test_timestamp_ordering() {
    let mut ts1 = Timestamp::new("client1".to_string());
    let mut ts2 = Timestamp::new("client2".to_string());
    
    // Same logical clock, different clients
    assert!(ts1 != ts2);
    
    // Different logical clocks
    ts1.increment();
    assert!(ts1 > ts2);
    
    ts2.increment();
    ts2.increment();
    assert!(ts2 > ts1);
}

#[test]
fn test_timestamp_update() {
    let mut ts1 = Timestamp::new("client1".to_string());
    let mut ts2 = Timestamp::new("client2".to_string());
    
    ts1.increment(); // ts1 = 1
    ts2.increment();
    ts2.increment(); // ts2 = 2
    
    // Update ts1 with ts2's value
    ts1.update(&ts2);
    
    // ts1 should now be max(1, 2) = 2
    assert_eq!(ts1.logical_clock(), 2);
}

#[test]
fn test_timestamp_clone() {
    let mut ts1 = Timestamp::new("client1".to_string());
    ts1.increment();
    
    let ts2 = ts1.clone();
    assert_eq!(ts1.logical_clock(), ts2.logical_clock());
    assert_eq!(ts1.client_id(), ts2.client_id());
}

#[test]
fn test_timestamp_serialization() {
    let mut ts = Timestamp::new("client1".to_string());
    ts.increment();
    
    let serialized = serde_json::to_string(&ts).unwrap();
    let deserialized: Timestamp = serde_json::from_str(&serialized).unwrap();
    
    assert_eq!(ts.logical_clock(), deserialized.logical_clock());
    assert_eq!(ts.client_id(), deserialized.client_id());
}
