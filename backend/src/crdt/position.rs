/*
 * File: crdt/position.rs
 * Purpose: Implementation of CRDT position identifiers
 * 
 * Responsibilities:
 * - Define the position identifier structure
 * - Implement position comparison and ordering
 * - Generate new positions between existing positions
 * - Ensure unique, totally-ordered position identifiers
 * 
 * This file implements the core position identifier logic for the CRDT,
 * enabling consistent ordering of characters across all clients.
 */

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// Trait for types that can represent position boundaries
pub trait PositionBounds {
    /// Check if this position is the start boundary
    fn is_start(&self) -> bool;
    /// Check if this position is the end boundary
    fn is_end(&self) -> bool;
}

/// A position identifier in the CRDT document
/// Represents a location in the document using a path-based approach.
/// Each position is a sequence of integers that defines its total ordering.
/// 
/// # Examples
/// ```
/// use crdt_editor_backend::Position;
/// 
/// let start = Position::start();
/// let end = Position::end();
/// let pos1 = Position::new(vec![1, 2, 3]);
/// let pos2 = Position::new(vec![1, 2, 4]);
/// 
/// assert!(start < pos1);
/// assert!(pos1 < pos2);
/// assert!(pos2 < end);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Position {
    /// Path components representing position
    path: Vec<u32>,
    /// Special flag for end position
    is_end: bool,
}

impl Position {
    /// Create a new position with the given path
    pub fn new(path: Vec<u32>) -> Self {
        Self { 
            path,
            is_end: false,
        }
    }

    /// Get a reference to the position's path
    pub fn path(&self) -> &Vec<u32> {
        &self.path
    }

    /// Create a new position that sorts between two existing positions.
    /// This is the core operation for inserting new characters in the document.
    /// 
    /// The generated position is guaranteed to be:
    /// 1. Greater than the left position
    /// 2. Less than the right position
    /// 3. Unique from both positions
    /// 
    /// # Panics
    /// Panics if either position is an end position.
    /// 
    /// # Examples
    /// ```
    /// use crdt_editor_backend::Position;
    /// 
    /// let pos1 = Position::new(vec![1, 2]);
    /// let pos2 = Position::new(vec![1, 4]);
    /// let between = Position::between(&pos1, &pos2);
    /// 
    /// assert!(pos1 < between);
    /// assert!(between < pos2);
    /// ```
    pub fn between(left: &Position, right: &Position) -> Self {
        // Handle special cases involving end positions
        if left.is_end || right.is_end {
            panic!("Cannot generate position involving end position");
        }

        // Ensure left is less than right
        if left >= right {
            return Self::between(right, left);
        }

        // Find the common prefix length
        let common_len = left.path.iter()
            .zip(right.path.iter())
            .take_while(|(a, b)| a == b)
            .count();

        // Get the differing components or next available components
        let left_next = left.path.get(common_len).copied();
        let right_next = right.path.get(common_len).copied();

        let mut new_path = left.path[..common_len].to_vec();

        match (left_next, right_next) {
            // Case 1: Both positions have a differing component
            (Some(l), Some(r)) => {
                // If the numbers are too close, extend the path
                if r - l <= 1 {
                    new_path.extend_from_slice(&left.path[common_len..]);
                    new_path.push(1);
                } else {
                    // Generate a number between l and r
                    new_path.push(l + ((r - l) / 2));
                }
            },
            // Case 2: Left position is a prefix of right
            (None, Some(r)) => {
                // Generate a number before r
                new_path.push(r / 2);
            },
            // Case 3: Right position is a prefix of left
            (Some(l), None) => {
                // Generate a number after l
                new_path.push(l + 1);
            },
            // Case 4: Both positions are identical
            (None, None) => {
                // Append a new component
                new_path.push(1);
            },
        }

        Self::new(new_path)
    }

    /// Create a position representing the start of the document.
    /// This position is guaranteed to be less than any other non-start position.
    pub fn start() -> Self {
        Self {
            path: Vec::new(),
            is_end: false,
        }
    }

    /// Create a position representing the end of the document.
    /// This position is guaranteed to be greater than any other non-end position.
    pub fn end() -> Self {
        Self {
            path: Vec::new(),
            is_end: true,
        }
    }

    /// Compare two paths lexicographically
    fn compare_paths(a: &[u32], b: &[u32]) -> Ordering {
        // Compare elements until we find a difference or reach the end of one path
        for (x, y) in a.iter().zip(b.iter()) {
            match x.cmp(y) {
                Ordering::Equal => continue,
                other => return other,
            }
        }
        
        // If we've exhausted one or both paths, compare lengths
        a.len().cmp(&b.len())
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        // Handle end position cases first
        match (self.is_end, other.is_end) {
            (true, true) => Ordering::Equal,
            (true, false) => Ordering::Greater,
            (false, true) => Ordering::Less,
            (false, false) => Position::compare_paths(&self.path, &other.path),
        }
    }
}

impl PositionBounds for Position {
    fn is_start(&self) -> bool {
        !self.is_end && self.path.is_empty()
    }

    fn is_end(&self) -> bool {
        self.is_end
    }
}
