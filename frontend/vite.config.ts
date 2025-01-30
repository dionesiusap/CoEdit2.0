/*
 * File: vite.config.ts
 * Purpose: Configuration file for Vite build tool
 * 
 * Responsibilities:
 * - Configure Vue.js plugin
 * - Set up development server
 * - Configure WebSocket proxy for backend communication
 * - Define build settings and optimizations
 * 
 * This file manages the build and development environment configuration,
 * ensuring proper communication between frontend and backend services.
 */

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

export default defineConfig({
  plugins: [vue()],
  server: {
    proxy: {
      '/ws': {
        target: 'ws://localhost:8000',
        ws: true,
      }
    }
  }
})
