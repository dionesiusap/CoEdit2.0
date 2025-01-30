/*
 * File: main.ts
 * Purpose: Entry point for the Vue.js application
 * 
 * Responsibilities:
 * - Initialize the Vue application
 * - Mount the root component
 * - Set up any global plugins or configurations
 * 
 * This file serves as the bootstrap point for the frontend application,
 * setting up the necessary Vue.js infrastructure.
 */

import { createApp } from 'vue'
import App from './App.vue'

createApp(App).mount('#app')
