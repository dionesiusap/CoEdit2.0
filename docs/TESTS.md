# Test Documentation

## WebSocket Tests

### Message Tests (`tests/websocket/message_tests.rs`)
- `test_message_creation`: Verifies creation of WebSocket messages with proper type and payload
- `test_operation_message_serialization`: Tests serialization/deserialization of CRDT operation messages
- `test_status_message_serialization`: Ensures proper handling of connection status messages
- `test_error_message_handling`: Validates error message creation and formatting
- `test_message_validation`: Checks message validation rules (e.g., non-empty document IDs)

### Connection Tests (`tests/websocket/connection_tests.rs`)
- `test_connection_establishment`: Verifies new client connections
- `test_client_info_tracking`: Tests client metadata tracking
- `test_connection_closure`: Validates proper connection termination
- `test_connection_timeout`: Ensures inactive connections are detected
- `test_connection_recovery`: Tests reconnection after disconnection
- `test_concurrent_connections`: Validates handling of multiple simultaneous clients
- `test_connection_error_handling`: Verifies proper error handling for invalid operations
- `test_connection_heartbeat`: Tests connection keep-alive mechanism
- `test_connection_statistics`: Validates connection statistics tracking

### Server Tests (`tests/websocket/server_tests.rs`)
- `test_server_initialization`: Verifies server startup with configuration
- `test_client_connection`: Tests WebSocket handshake and client registration
- `test_document_creation`: Validates document creation and initial state
- `test_operation_broadcast`: Ensures operations are broadcast to all clients
- `test_document_state_sync`: Tests document state synchronization
- `test_error_handling`: Validates server-side error handling
- `test_server_shutdown`: Ensures clean server shutdown
- `test_concurrent_operations`: Tests handling of simultaneous operations

## CRDT Tests

### Document Tests (`tests/crdt/document_tests.rs`)
- `test_document_creation`: Verifies document initialization
- `test_single_character_insertion`: Tests basic character insertion
- `test_multiple_character_insertion`: Tests multiple sequential insertions
- `test_character_deletion`: Validates character deletion
- `test_concurrent_insertions`: Tests concurrent operation handling
- `test_garbage_collection`: Verifies deletion cleanup
- `test_automatic_garbage_collection`: Tests automatic cleanup triggering
- `test_garbage_collection_with_concurrent_operations`: Validates GC with ongoing operations

### Position Tests (`tests/crdt/position_tests.rs`)
- `test_position_creation`: Verifies position identifier creation
- `test_position_between`: Tests position generation between existing positions
- `test_position_ordering`: Validates total ordering of positions
- `test_position_bounds`: Tests boundary position handling
- `test_position_dense_sequence`: Verifies handling of dense insertions
- `test_position_serialization`: Tests position serialization/deserialization

### Timestamp Tests (`tests/crdt/timestamp_tests.rs`)
- `test_timestamp_creation`: Verifies Lamport timestamp initialization
- `test_timestamp_increment`: Tests logical clock increments
- `test_timestamp_ordering`: Validates timestamp comparison
- `test_timestamp_update`: Tests timestamp synchronization
- `test_timestamp_clone`: Verifies timestamp cloning
- `test_timestamp_serialization`: Tests timestamp serialization/deserialization
