# Collaborative Text Editor with Math Support

A real-time collaborative text editor built with Rust and Vue.js that supports mathematical expressions. The editor uses Conflict-free Replicated Data Types (CRDT) to handle concurrent edits from multiple users.

## Features

- Real-time collaboration using WebSocket
- Mathematical expressions support
- CRDT-based conflict resolution
- Multiple concurrent users support
- User identification via IP/hostname and port
- Vue.js-based responsive UI
- Rust backend for performance and reliability

## Project Structure

```
crdt_editor/
├── backend/           # Rust backend server
│   ├── src/          # Rust source files
│   └── Cargo.toml    # Rust dependencies
├── frontend/         # Vue.js frontend
│   ├── src/         # Vue source files
│   └── package.json # Node.js dependencies
└── docs/            # Documentation
```

## Prerequisites

- Rust (latest stable version)
- Node.js (v18 or later)
- npm or yarn
- WebSocket-compatible browser

## Development Setup

1. Backend Setup:
```bash
cd backend
cargo build
cargo run
```

2. Frontend Setup:
```bash
cd frontend
npm install
npm run dev
```

## Architecture

The application uses a client-server architecture where:
- Backend (Rust):
  - Handles WebSocket connections
  - Implements CRDT logic
  - Manages document state
  - Broadcasts changes to connected clients

- Frontend (Vue.js):
  - Provides text editor interface
  - Renders mathematical expressions
  - Manages WebSocket connection
  - Implements CRDT operations client-side

## License

MIT
