/*
 * File: crdt/mod.rs
 * Purpose: Module organization for CRDT implementation
 * 
 * This module contains:
 * - Document: Main CRDT document implementation
 * - Position: Fractional indexing for character positions
 * - Operation: Document operations (insert/delete)
 * - Timestamp: Lamport timestamps for causality tracking
 */

pub mod document;
pub mod position;
pub mod timestamp;

pub use document::{Document, Operation};
pub use position::{Position, PositionBounds};
pub use timestamp::Timestamp;
