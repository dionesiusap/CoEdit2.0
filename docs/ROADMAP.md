# Feature Roadmap

## Phase 1: Basic Real-time Text Editor
### Core Features
- [ ] Basic text editor interface using CodeMirror
- [ ] WebSocket connection setup
  - [ ] Backend WebSocket server
  - [ ] Frontend WebSocket client
  - [ ] Connection status indicator
- [x] Basic CRDT implementation
  - [x] Character-wise operations (insert/delete)
  - [x] Unique position identifiers
  - [x] Lamport timestamps for causality
- [ ] Simple user identification by IP/port
- [ ] Single document editing
- [ ] Real-time updates across clients

### Technical Requirements
- [ ] WebSocket server in Rust using warp
- [ ] Vue.js frontend with CodeMirror integration
- [x] Basic CRDT data structures
- [ ] In-memory document storage
- [ ] Basic error handling and reconnection logic

## Phase 2: Mathematical Expressions
### Core Features
- [ ] Math expression syntax detection
- [ ] LaTeX-style math input support
- [ ] Real-time math rendering
  - [ ] Inline math expressions
  - [ ] Block math expressions
- [ ] Math-aware CRDT operations
  - [ ] Treating math expressions as atomic units
  - [ ] Preserving math expression integrity during concurrent edits
- [ ] Math syntax highlighting
- [ ] Preview panel for math expressions

### Technical Requirements
- [ ] Integration with KaTeX/MathJax
- [ ] Custom CodeMirror extensions for math syntax
- [ ] Enhanced CRDT operations for math blocks
- [ ] Math expression validation
- [ ] Optimized rendering performance

## Phase 3: Document Management
### Core Features
- [ ] Multiple document support
  - [ ] Document creation
  - [ ] Document listing
  - [ ] Document switching
- [ ] Document persistence
  - [ ] Save/load documents
  - [ ] Auto-save functionality
- [ ] Document metadata
  - [ ] Title
  - [ ] Last modified
  - [ ] Active users
- [ ] Document sharing
  - [ ] Shareable links
  - [ ] Read/write permissions

### Technical Requirements
- [ ] Database integration (e.g., PostgreSQL)
- [ ] Document versioning system
- [ ] API endpoints for document management
- [ ] Efficient document storage format
- [ ] Concurrent document access handling

## Phase 4: User Management
### Core Features
- [ ] User accounts
  - [ ] Registration
  - [ ] Authentication
  - [ ] Profile management
- [ ] User presence
  - [ ] Online status
  - [ ] Cursor positions
  - [ ] User colors
- [ ] Collaboration features
  - [ ] User mentions
  - [ ] Comments/annotations
  - [ ] Edit history by user
- [ ] Permissions system
  - [ ] Document ownership
  - [ ] Access control
  - [ ] Invitation system

### Technical Requirements
- [ ] Authentication system (e.g., JWT)
- [ ] User database schema
- [ ] WebSocket authentication
- [ ] Real-time presence tracking
- [ ] Permission validation middleware

## Phase 5: Advanced Features
### Core Features
- [ ] Offline support
  - [ ] Local changes queue
  - [ ] Sync on reconnection
- [ ] Rich text formatting
  - [ ] Text styles
  - [ ] Lists
  - [ ] Tables
- [ ] Import/Export
  - [ ] LaTeX export
  - [ ] PDF export
  - [ ] Markdown import/export
- [ ] Collaboration tools
  - [ ] Change suggestions
  - [ ] Review mode
  - [ ] Version control

### Technical Requirements
- [ ] IndexedDB for offline storage
- [ ] Complex CRDT operations for rich text
- [ ] Export rendering engine
- [ ] Conflict resolution for offline changes
- [ ] Version control system integration

## Phase 6: Performance and Polish
### Core Features
- [ ] Performance optimizations
  - [ ] CRDT operation batching
  - [ ] Efficient math rendering
  - [ ] Document chunking
- [ ] UI/UX improvements
  - [ ] Responsive design
  - [ ] Keyboard shortcuts
  - [ ] Custom themes
- [ ] Error handling
  - [ ] Graceful degradation
  - [ ] Error recovery
  - [ ] User feedback
- [ ] Analytics and monitoring
  - [ ] Usage metrics
  - [ ] Performance monitoring
  - [ ] Error tracking

### Technical Requirements
- [ ] Performance profiling tools
- [ ] UI component optimization
- [ ] Error tracking system
- [ ] Analytics integration
- [ ] Load testing infrastructure

## Notes
- Each phase builds upon the previous ones
- Early phases focus on core functionality
- Later phases add polish and advanced features
- Phases can be adjusted based on user feedback
- Security considerations should be addressed in each phase
