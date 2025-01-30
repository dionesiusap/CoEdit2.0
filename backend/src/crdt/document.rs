/*
 * File: crdt/document.rs
 * Purpose: Implementation of CRDT document operations and state management
 * 
 * Responsibilities:
 * - Define document structure and operations
 * - Manage document state and content
 * - Handle concurrent operations
 * - Ensure consistency across replicas
 * - Manage garbage collection of deleted characters
 * 
 * This file implements the core document logic for the CRDT,
 * managing the state of text content and handling operations
 * in a way that ensures eventual consistency across all clients.
 */

use serde::{Deserialize, Serialize};
use crate::crdt::{Position, Timestamp};

/// A character in the CRDT document
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct Character {
    /// The actual character value
    value: char,
    /// Position identifier in the document
    position: Position,
    /// Whether this character has been deleted
    deleted: bool,
}

/// An operation that can be applied to the document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operation {
    /// Insert a character at a position
    Insert {
        /// ID of the client that created this operation
        client_id: String,
        /// Character to insert
        character: char,
        /// Position to insert at
        position: Position,
        /// Timestamp of the operation
        timestamp: Timestamp,
    },
    /// Delete a character at a position
    Delete {
        /// ID of the client that created this operation
        client_id: String,
        /// Position of the character to delete
        position: Position,
        /// Timestamp of the operation
        timestamp: Timestamp,
    },
}

impl Operation {
    /// Create a new insert operation
    pub fn insert(client_id: String, character: char, position: Position) -> Self {
        let timestamp = Timestamp::new(client_id.clone());
        Self::Insert {
            client_id,
            character,
            position,
            timestamp,
        }
    }

    /// Create a new delete operation
    pub fn delete(client_id: String, position: Position) -> Self {
        let timestamp = Timestamp::new(client_id.clone());
        Self::Delete {
            client_id,
            position,
            timestamp,
        }
    }

    /// Returns the client ID associated with this operation
    pub fn client_id(&self) -> &str {
        match self {
            Operation::Insert { client_id, .. } => client_id,
            Operation::Delete { client_id, .. } => client_id,
        }
    }

    /// Returns the position associated with this operation
    pub fn position(&self) -> &Position {
        match self {
            Operation::Insert { position, .. } => position,
            Operation::Delete { position, .. } => position,
        }
    }

    /// Returns the timestamp associated with this operation
    pub fn timestamp(&self) -> &Timestamp {
        match self {
            Operation::Insert { timestamp, .. } => timestamp,
            Operation::Delete { timestamp, .. } => timestamp,
        }
    }
}

/// A CRDT document that supports concurrent editing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    /// Unique identifier for this document
    id: String,
    /// List of characters in the document
    characters: Vec<Character>,
    /// List of operations that have been applied
    operations: Vec<Operation>,
    /// Number of deleted characters before triggering garbage collection
    garbage_collection_threshold: Option<usize>,
    /// Count of deleted characters since last garbage collection
    deleted_count: usize,
}

impl Document {
    /// Create a new empty document
    pub fn new(id: String) -> Self {
        Self {
            id,
            characters: Vec::new(),
            operations: Vec::new(),
            garbage_collection_threshold: None,
            deleted_count: 0,
        }
    }

    /// Get the document's unique identifier
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get the current content of the document as a string
    pub fn content(&self) -> String {
        self.characters
            .iter()
            .filter(|c| !c.deleted)
            .map(|c| c.value)
            .collect()
    }

    /// Get a reference to the list of operations
    pub fn operations(&self) -> &[Operation] {
        &self.operations
    }

    /// Get the total number of characters (including deleted ones)
    pub fn character_count(&self) -> usize {
        self.characters.len()
    }

    /// Set the threshold for automatic garbage collection
    /// When the number of deleted characters reaches this threshold,
    /// garbage collection will be triggered automatically.
    /// Set to None to disable automatic garbage collection.
    pub fn set_garbage_collection_threshold(&mut self, threshold: usize) {
        self.garbage_collection_threshold = Some(threshold);
    }

    /// Remove all deleted characters from the document
    pub fn collect_garbage(&mut self) {
        self.characters.retain(|c| !c.deleted);
        self.deleted_count = 0;
    }

    /// Apply an operation to the document
    pub fn apply(&mut self, operation: Operation) {
        match &operation {
            Operation::Insert { character, position, .. } => {
                // Find the insertion index
                let index = self.find_insert_index(position);
                
                // Insert the character
                self.characters.insert(index, Character {
                    value: *character,
                    position: position.clone(),
                    deleted: false,
                });
            }
            Operation::Delete { position, .. } => {
                // Find and mark the character as deleted
                if let Some(index) = self.find_character_index(position) {
                    self.characters[index].deleted = true;
                    self.deleted_count += 1;

                    // Check if we need to run garbage collection
                    if let Some(threshold) = self.garbage_collection_threshold {
                        if self.deleted_count >= threshold {
                            self.collect_garbage();
                        }
                    }
                }
            }
        }
        
        // Record the operation
        self.operations.push(operation);
    }

    /// Find the index where a character should be inserted
    fn find_insert_index(&self, position: &Position) -> usize {
        self.characters
            .iter()
            .position(|c| c.position > *position)
            .unwrap_or(self.characters.len())
    }

    /// Find the index of a character with the given position
    fn find_character_index(&self, position: &Position) -> Option<usize> {
        self.characters
            .iter()
            .position(|c| c.position == *position)
    }
}
