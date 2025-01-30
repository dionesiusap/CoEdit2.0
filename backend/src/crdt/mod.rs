/*
 * File: crdt/mod.rs
 * Purpose: Module organization for CRDT implementation
 * 
 * Responsibilities:
 * - Define module structure
 * - Export public types and functions
 * - Organize CRDT-related components
 * 
 * This file defines the module hierarchy for the CRDT implementation,
 * making the necessary types and functions available to other parts
 * of the codebase.
 */

pub mod position;
pub mod document;

pub use position::Position;
pub use document::{Document, Operation};
