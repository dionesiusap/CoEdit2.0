/*
 * File: lib.rs
 * Purpose: Root library file for the CRDT text editor backend
 * 
 * Responsibilities:
 * - Export public modules and types
 * - Define common types and traits
 * - Provide high-level documentation for the library
 * 
 * This file serves as the main entry point for the library crate,
 * organizing and exposing the various components needed for the
 * CRDT-based collaborative text editor.
 */

pub mod crdt;

// Re-export commonly used types
pub use crdt::position::Position;
