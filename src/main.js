// Import Tauri APIs
import { invoke } from '@tauri-apps/api/core';

// Get the button and status elements
const greetButton = document.getElementById('greet-button');
const statusElement = document.getElementById('status');

// Add click handler to the button
greetButton.addEventListener('click', async () => {
  // Call the Rust command
  try {
    const response = await invoke('greet', { name: 'Tauri User' });
    statusElement.textContent = response;
  } catch (error) {
    console.error('Error calling Tauri command:', error);
    statusElement.textContent = `Error: ${error.message}`;
  }
});

// Initialize the app
document.addEventListener('DOMContentLoaded', () => {
  statusElement.textContent = 'TurboDownload is ready!';
});