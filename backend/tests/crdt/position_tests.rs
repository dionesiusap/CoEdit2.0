/*
 * File: tests/crdt/position_tests.rs
 * Purpose: Test suite for CRDT position identifier implementation
 * 
 * Test Categories:
 * 1. Position Creation - Test basic position creation and validation
 * 2. Position Comparison - Test ordering and comparison operations
 * 3. Position Generation - Test creating positions between existing ones
 * 4. Edge Cases - Test boundary conditions and special cases
 * 
 * These tests ensure the correctness of the CRDT position identifier
 * implementation, which is crucial for maintaining document consistency.
 */

use crdt_editor_backend::crdt::position::{Position, PositionBounds};

#[test]
fn test_position_creation() {
    // Test creating a position with valid path
    let path = vec![10, 20, 30];
    let pos = Position::new(path.clone());
    assert_eq!(pos.path(), &path);
    
    // Test creating an empty position
    let empty_pos = Position::new(vec![]);
    assert!(empty_pos.path().is_empty());
}

#[test]
fn test_position_ordering() {
    // Test positions with different lengths
    let pos1 = Position::new(vec![1, 2]);
    let pos2 = Position::new(vec![1, 2, 3]);
    assert!(pos1 < pos2);
    
    // Test positions with same length but different values
    let pos3 = Position::new(vec![1, 2, 3]);
    let pos4 = Position::new(vec![1, 2, 4]);
    assert!(pos3 < pos4);
    
    // Test equal positions
    let pos5 = Position::new(vec![1, 2, 3]);
    let pos6 = Position::new(vec![1, 2, 3]);
    assert_eq!(pos5, pos6);
}

#[test]
fn test_position_between() {
    let pos1 = Position::new(vec![10]);
    let pos2 = Position::new(vec![20]);
    
    // Test generating a position between two others
    let between = Position::between(&pos1, &pos2);
    assert!(pos1 < between);
    assert!(between < pos2);
}

#[test]
fn test_position_bounds() {
    // Test start position
    let start = Position::start();
    assert!(start.is_start());
    
    // Test end position
    let end = Position::end();
    assert!(end.is_end());
    
    // Test that start is less than end
    assert!(start < end);
    
    // Test that any position is between start and end
    let pos = Position::new(vec![10, 20]);
    assert!(start < pos);
    assert!(pos < end);
}

#[test]
fn test_position_dense_sequence() {
    let mut positions = Vec::new();
    let start = Position::new(vec![10]);
    let end = Position::new(vec![20]);
    
    // Generate 10 positions between start and end
    let mut prev = start.clone();
    for _ in 0..10 {
        let next = Position::between(&prev, &end);
        assert!(prev < next);
        positions.push(next.clone());
        prev = next;
    }
    
    // Verify all positions are unique and ordered
    for i in 0..positions.len()-1 {
        assert!(positions[i] < positions[i+1]);
    }
}

#[test]
fn test_position_serialization() {
    let pos = Position::new(vec![10, 20, 30]);
    
    // Test JSON serialization
    let serialized = serde_json::to_string(&pos).unwrap();
    let deserialized: Position = serde_json::from_str(&serialized).unwrap();
    
    assert_eq!(pos, deserialized);
}
