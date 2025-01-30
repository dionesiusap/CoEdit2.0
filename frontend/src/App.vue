<!--
File: App.vue
Purpose: Main application component for the collaborative text editor frontend

Responsibilities:
- Provide the main editor interface and layout
- Manage WebSocket connection to the backend
- Handle real-time collaborative editing
- Render mathematical expressions
- Display connection status and connected users
- Coordinate editor state with CRDT operations

This component serves as the primary user interface for the collaborative
text editor, integrating CodeMirror for text editing and managing
real-time synchronization with other users.
-->

<template>
  <div class="editor-container">
    <header>
      <h1>Collaborative Text Editor</h1>
      <div class="connection-status" :class="{ connected: isConnected }">
        {{ isConnected ? 'Connected' : 'Disconnected' }}
      </div>
    </header>
    
    <main>
      <div class="editor" ref="editorElement"></div>
      <div class="sidebar">
        <div class="connected-users">
          <h3>Connected Users</h3>
          <ul>
            <li v-for="user in connectedUsers" :key="user.id">
              {{ user.id }}
            </li>
          </ul>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { EditorState, Extension } from '@codemirror/state'
import { EditorView, keymap } from '@codemirror/view'
import { defaultKeymap } from '@codemirror/commands'

// State
const editorElement = ref<HTMLElement | null>(null)
const isConnected = ref(false)
const connectedUsers = ref<Array<{ id: string }>>([])
let editor: EditorView | null = null
let ws: WebSocket | null = null

// WebSocket setup
const setupWebSocket = () => {
  ws = new WebSocket('ws://localhost:8000/ws')
  
  ws.onopen = () => {
    isConnected.value = true
  }
  
  ws.onclose = () => {
    isConnected.value = false
    // Attempt to reconnect after a delay
    setTimeout(setupWebSocket, 1000)
  }
  
  ws.onmessage = (event) => {
    const data = JSON.parse(event.data)
    // Handle incoming operations
    // TODO: Implement CRDT operations
  }
}

// Editor setup
const setupEditor = () => {
  if (!editorElement.value) return
  
  const extensions: Extension[] = [
    keymap.of(defaultKeymap),
    EditorView.updateListener.of((update) => {
      if (update.docChanged && ws?.readyState === WebSocket.OPEN) {
        // Send changes to server
        // TODO: Implement CRDT operations
      }
    })
  ]
  
  const state = EditorState.create({
    doc: '',
    extensions
  })
  
  editor = new EditorView({
    state,
    parent: editorElement.value
  })
}

// Lifecycle hooks
onMounted(() => {
  setupEditor()
  setupWebSocket()
})

onUnmounted(() => {
  editor?.destroy()
  ws?.close()
})
</script>

<style>
.editor-container {
  height: 100vh;
  display: flex;
  flex-direction: column;
}

header {
  padding: 1rem;
  background: #f5f5f5;
  border-bottom: 1px solid #ddd;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.connection-status {
  padding: 0.5rem 1rem;
  border-radius: 4px;
  background: #ff4444;
  color: white;
}

.connection-status.connected {
  background: #44ff44;
}

main {
  flex: 1;
  display: flex;
  overflow: hidden;
}

.editor {
  flex: 1;
  overflow: auto;
  padding: 1rem;
}

.sidebar {
  width: 250px;
  border-left: 1px solid #ddd;
  padding: 1rem;
}

.connected-users ul {
  list-style: none;
  padding: 0;
}

.connected-users li {
  padding: 0.5rem 0;
  border-bottom: 1px solid #eee;
}
</style>
