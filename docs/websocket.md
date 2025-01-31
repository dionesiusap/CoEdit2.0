# WebSocket Module Documentation

## Overview
The WebSocket module provides real-time communication for collaborative text editing. It handles client connections, message routing, and document synchronization.

## Architecture

### Message Module (`message.rs`)
Handles WebSocket message types and serialization.

#### Types
- `Message`: Base message structure containing type, client ID, and payload
- `MessageType`: Enum defining different message types (Connect, Operation, etc.)
- `OperationMessage`: Specialized message for CRDT operations
- `StatusMessage`: Connection status updates
- `DocumentStateMessage`: Document synchronization state

#### Features
- Serde serialization/deserialization
- Message validation
- Type-safe message creation
- Error handling

### Connection Module (`connection.rs`)
Manages client connections and their states.

#### Types
- `ConnectionManager`: Central connection management
- `ConnectionStatus`: Connection state enumeration
- `ClientInfo`: Client metadata and statistics
- `ConnectionError`: Connection-specific errors

#### Features
- Client registration and tracking
- Connection state management
- Heartbeat mechanism
- Connection timeout detection
- Connection recovery
- Concurrent connection handling

### Server Module (`server.rs`)
Implements the WebSocket server functionality.

#### Types
- `EditorServer`: Main server implementation
- `ServerConfig`: Server configuration
- `DocumentManager`: Document state management
- `BroadcastManager`: Message broadcasting

#### Features
- WebSocket endpoint handling
- Document state synchronization
- Operation broadcasting
- Error handling and recovery
- Server statistics

## Message Flow
1. Client connects via WebSocket
2. Server authenticates and registers client
3. Client requests document state
4. Server sends current document state
5. Client sends operations
6. Server broadcasts operations to other clients
7. Clients apply operations locally

## Error Handling
- Connection timeouts
- Invalid operations
- Client disconnections
- Message validation failures
- Server errors

## Performance Considerations
- Asynchronous operation handling
- Efficient broadcasting
- Connection pooling
- Resource cleanup
