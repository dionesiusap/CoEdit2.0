/*
 * File: src/lib.rs
 * Purpose: Library root and module organization
 * 
 * This file:
 * - Defines the library's module structure
 * - Re-exports public types and functions
 * - Provides high-level documentation
 */

pub mod crdt;

// Re-export commonly used types for convenience
pub use crdt::{Document, Operation, Position, Timestamp};
