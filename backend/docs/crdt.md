# CRDT Implementation Documentation

This document describes the implementation of the Conflict-free Replicated Data Type (CRDT) used in our collaborative text editor.

## Overview

The CRDT implementation consists of several key components:

1. Position Management (`position.rs`)
2. Document Operations (`document.rs`)
3. State Management (`document.rs`)

## Position Management

The `Position` type represents a location in the document using a path-based approach. Each position is a sequence of integers that defines its total ordering.

### Key Features

- Total ordering between positions
- Generation of positions between existing positions
- Support for boundary positions (start/end)
- Dense sequence handling for frequent insertions

### Example Usage

```rust
use crdt_editor_backend::Position;

// Create positions
let start = Position::start();
let end = Position::end();
let pos = Position::new(vec![1, 2, 3]);

// Generate a position between two others
let between = Position::between(&start, &end);
```

## Document Operations

The `Operation` enum defines the possible operations that can be performed on the document:

1. Insert: Add a character at a specific position
2. Delete: Remove a character at a specific position

### Example Usage

```rust
use crdt_editor_backend::{Document, Operation, Position};

let mut doc = Document::new("doc1".to_string());

// Insert a character
let op = Operation::insert(
    "client1".to_string(),
    'H',
    Position::start(),
);
doc.apply(op);

// Delete a character
let op = Operation::delete(
    "client1".to_string(),
    Position::new(vec![1]),
);
doc.apply(op);
```

## State Management

The `Document` struct manages the state of the text content and handles operations:

- Maintains a list of characters with their positions
- Tracks operation history
- Ensures eventual consistency across clients
- Handles concurrent operations

### Consistency Guarantees

1. **Convergence**: All replicas will eventually reach the same state when they have applied the same set of operations.
2. **Causality Preservation**: If operation A happened before operation B, all replicas will apply them in that order.
3. **Intention Preservation**: The effect of concurrent operations is preserved according to their positions.

## Implementation Details

### Character Storage

Characters are stored in a sorted vector, each with:
- The actual character value
- Its position in the document
- A deletion flag

### Operation Application

When applying operations:
1. For inserts:
   - Find the correct index based on position ordering
   - Insert the character at that index
2. For deletes:
   - Find the character with the matching position
   - Mark it as deleted

### Concurrent Operations

The system handles concurrent operations by:
1. Using unique positions for each character
2. Maintaining a total ordering of positions
3. Preserving operation intentions through position-based ordering

## Garbage Collection

The document implementation includes a garbage collection mechanism to remove deleted characters and optimize memory usage.

### Features

1. **Manual Garbage Collection**
   - Call `collect_garbage()` to remove all deleted characters
   - Preserves operation history while reducing memory usage
   - Safe to use at any time

2. **Automatic Garbage Collection**
   - Set a threshold for automatic collection
   - Triggers when deleted character count reaches threshold
   - Configurable via `set_garbage_collection_threshold()`
   - Can be disabled by setting threshold to `None`

### Example Usage

```rust
use crdt_editor_backend::{Document, Operation, Position};

let mut doc = Document::new("doc1".to_string());

// Enable automatic garbage collection after 5 deletions
doc.set_garbage_collection_threshold(5);

// Or manually collect garbage
doc.collect_garbage();
```

### Implementation Details

The garbage collection process:
1. Retains only non-deleted characters
2. Preserves character ordering
3. Maintains operation history
4. Resets the deleted character counter

### Memory Management

Garbage collection helps manage memory by:
1. Removing unnecessary character storage
2. Maintaining a compact character list
3. Preserving document consistency
4. Allowing for efficient memory usage in long-lived documents

### Best Practices

1. **Threshold Selection**
   - Set based on expected document size
   - Consider update frequency
   - Balance memory usage vs. CPU overhead

2. **Manual Collection**
   - Use during low activity periods
   - Consider after bulk operations
   - Run before document persistence

3. **Automatic Collection**
   - Enable for long-running sessions
   - Set threshold based on usage patterns
   - Monitor performance impact

## Testing

The implementation includes comprehensive tests covering:
1. Basic operations (insert/delete)
2. Concurrent operations
3. Boundary cases
4. Position ordering
5. Document state consistency

## Future Improvements

Potential areas for enhancement:
1. Operation compression
2. Improved position allocation strategy
3. Undo/redo support
