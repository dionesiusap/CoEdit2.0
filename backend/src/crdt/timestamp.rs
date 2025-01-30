/*
 * File: src/crdt/timestamp.rs
 * Purpose: Implementation of Lamport timestamps for causality tracking
 * 
 * The Timestamp struct provides:
 * - Logical clock for tracking causal relationships
 * - Client identification for distributed operations
 * - Total ordering of operations across the system
 */

use serde::{Serialize, Deserialize};
use std::cmp::Ordering;

/// A Lamport timestamp implementation for tracking causality in distributed operations.
/// 
/// Each timestamp consists of:
/// - A logical clock value that increases monotonically
/// - A client ID to break ties between operations with the same logical clock
/// 
/// # Example
/// ```
/// use crdt_editor_backend::crdt::Timestamp;
/// 
/// let mut ts = Timestamp::new("client1".to_string());
/// ts.increment(); // Increment before performing an operation
/// assert_eq!(ts.logical_clock(), 1);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timestamp {
    logical_clock: u64,
    client_id: String,
}

impl Timestamp {
    /// Creates a new Timestamp with logical clock initialized to 0.
    pub fn new(client_id: String) -> Self {
        Self {
            logical_clock: 0,
            client_id,
        }
    }
    
    /// Returns the current logical clock value.
    pub fn logical_clock(&self) -> u64 {
        self.logical_clock
    }
    
    /// Returns a reference to the client ID.
    pub fn client_id(&self) -> &str {
        &self.client_id
    }
    
    /// Increments the logical clock.
    /// Call this before performing an operation.
    pub fn increment(&mut self) {
        self.logical_clock += 1;
    }
    
    /// Updates this timestamp's logical clock based on another timestamp.
    /// The new value will be max(self.logical_clock, other.logical_clock).
    pub fn update(&mut self, other: &Timestamp) {
        self.logical_clock = std::cmp::max(self.logical_clock, other.logical_clock);
    }
}

impl PartialEq for Timestamp {
    fn eq(&self, other: &Self) -> bool {
        self.logical_clock == other.logical_clock && self.client_id == other.client_id
    }
}

impl Eq for Timestamp {}

impl PartialOrd for Timestamp {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Timestamp {
    fn cmp(&self, other: &Self) -> Ordering {
        // First compare logical clocks
        match self.logical_clock.cmp(&other.logical_clock) {
            Ordering::Equal => self.client_id.cmp(&other.client_id),
            ord => ord,
        }
    }
}
