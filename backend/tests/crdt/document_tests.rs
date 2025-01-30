/*
 * File: tests/crdt/document_tests.rs
 * Purpose: Test suite for CRDT document operations
 * 
 * Test Categories:
 * - Document creation and initialization
 * - Character insertion at various positions
 * - Character deletion
 * - Concurrent operations and conflict resolution
 * - Document state consistency
 * - Garbage collection
 */

use crdt_editor_backend::crdt::{Document, Operation, Position};

#[test]
fn test_document_creation() {
    let doc = Document::new("test_doc".to_string());
    assert_eq!(doc.content(), "");
    assert_eq!(doc.id(), "test_doc");
    assert!(doc.operations().is_empty());
}

#[test]
fn test_single_character_insertion() {
    let mut doc = Document::new("test_doc".to_string());
    
    // Insert 'H' at the start
    let op = Operation::insert(
        "client1".to_string(),
        'H',
        Position::start(),
    );
    doc.apply(op);
    assert_eq!(doc.content(), "H");
}

#[test]
fn test_multiple_character_insertion() {
    let mut doc = Document::new("test_doc".to_string());
    
    // Insert "Hello"
    let mut last_pos = Position::start();
    let chars = vec!['H', 'e', 'l', 'l', 'o'];
    
    for c in chars {
        let pos = Position::between(&last_pos, &Position::new(vec![u32::MAX]));
        let op = Operation::insert("client1".to_string(), c, pos.clone());
        doc.apply(op);
        last_pos = pos;
    }
    
    assert_eq!(doc.content(), "Hello");
}

#[test]
fn test_character_deletion() {
    let mut doc = Document::new("test_doc".to_string());
    
    // Insert "Hello"
    let mut last_pos = Position::start();
    let chars = vec!['H', 'e', 'l', 'l', 'o'];
    let mut positions = Vec::new();
    
    for c in chars {
        let pos = Position::between(&last_pos, &Position::new(vec![u32::MAX]));
        positions.push(pos.clone());
        let op = Operation::insert("client1".to_string(), c, pos.clone());
        doc.apply(op);
        last_pos = pos;
    }
    
    // Delete 'l' characters
    let delete_ops = vec![
        Operation::delete("client1".to_string(), positions[2].clone()),
        Operation::delete("client1".to_string(), positions[3].clone()),
    ];
    
    for op in delete_ops {
        doc.apply(op);
    }
    
    assert_eq!(doc.content(), "Heo");
}

#[test]
fn test_concurrent_insertions() {
    let mut doc = Document::new("test_doc".to_string());
    
    // Client 1 inserts 'A' at start
    let op1 = Operation::insert(
        "client1".to_string(),
        'A',
        Position::new(vec![1]),
    );
    
    // Client 2 concurrently inserts 'B' at start with higher position
    let op2 = Operation::insert(
        "client2".to_string(),
        'B',
        Position::new(vec![2]),
    );
    
    // Apply operations in different orders
    let mut doc1 = doc.clone();
    doc1.apply(op1.clone());
    doc1.apply(op2.clone());
    
    let mut doc2 = doc;
    doc2.apply(op2);
    doc2.apply(op1);
    
    // Both documents should have the same content
    assert_eq!(doc1.content(), doc2.content());
    assert_eq!(doc1.content(), "AB"); // Now deterministic due to position ordering
}

#[test]
fn test_garbage_collection() {
    let mut doc = Document::new("test_doc".to_string());
    
    // Insert "Hello"
    let mut last_pos = Position::start();
    let chars = vec!['H', 'e', 'l', 'l', 'o'];
    let mut positions = Vec::new();
    
    for c in chars {
        let pos = Position::between(&last_pos, &Position::new(vec![u32::MAX]));
        positions.push(pos.clone());
        let op = Operation::insert("client1".to_string(), c, pos.clone());
        doc.apply(op);
        last_pos = pos;
    }
    
    // Delete some characters
    let delete_ops = vec![
        Operation::delete("client1".to_string(), positions[2].clone()), // Delete first 'l'
        Operation::delete("client1".to_string(), positions[3].clone()), // Delete second 'l'
    ];
    
    for op in delete_ops {
        doc.apply(op);
    }
    
    // Before garbage collection
    assert_eq!(doc.content(), "Heo");
    assert_eq!(doc.character_count(), 5); // Still storing deleted characters
    
    // Run garbage collection
    doc.collect_garbage();
    
    // After garbage collection
    assert_eq!(doc.content(), "Heo");
    assert_eq!(doc.character_count(), 3); // Deleted characters removed
    
    // Ensure operations are preserved
    assert_eq!(doc.operations().len(), 7); // 5 inserts + 2 deletes
}

#[test]
fn test_garbage_collection_with_concurrent_operations() {
    let mut doc = Document::new("test_doc".to_string());
    
    // Insert "Hello"
    let mut last_pos = Position::start();
    let chars = vec!['H', 'e', 'l'];
    let mut positions = Vec::new();
    
    for c in chars {
        let pos = Position::between(&last_pos, &Position::new(vec![u32::MAX]));
        positions.push(pos.clone());
        let op = Operation::insert("client1".to_string(), c, pos.clone());
        doc.apply(op);
        last_pos = pos;
    }
    
    // Delete 'l'
    doc.apply(Operation::delete(
        "client1".to_string(),
        positions[2].clone(),
    ));
    
    // Run garbage collection
    doc.collect_garbage();
    
    // Insert at a position after 'e'
    doc.apply(Operation::insert(
        "client2".to_string(),
        'x',
        Position::between(&positions[1], &Position::new(vec![u32::MAX])),
    ));
    
    // Verify final state
    assert_eq!(doc.content(), "Hex");
    assert_eq!(doc.character_count(), 3);
}

#[test]
fn test_automatic_garbage_collection() {
    let mut doc = Document::new("test_doc".to_string());
    doc.set_garbage_collection_threshold(3); // Collect after 3 deleted characters
    
    // Insert "Hello"
    let mut last_pos = Position::start();
    let chars = vec!['H', 'e', 'l', 'l', 'o'];
    let mut positions = Vec::new();
    
    for c in chars {
        let pos = Position::between(&last_pos, &Position::new(vec![u32::MAX]));
        positions.push(pos.clone());
        let op = Operation::insert("client1".to_string(), c, pos.clone());
        doc.apply(op);
        last_pos = pos;
    }
    
    assert_eq!(doc.character_count(), 5);
    
    // Delete characters one by one
    doc.apply(Operation::delete("client1".to_string(), positions[0].clone())); // Delete 'H'
    assert_eq!(doc.character_count(), 5); // No GC yet
    
    doc.apply(Operation::delete("client1".to_string(), positions[1].clone())); // Delete 'e'
    assert_eq!(doc.character_count(), 5); // No GC yet
    
    doc.apply(Operation::delete("client1".to_string(), positions[2].clone())); // Delete 'l'
    assert_eq!(doc.character_count(), 2); // GC triggered, only 'l' and 'o' remain
    
    assert_eq!(doc.content(), "lo");
}
