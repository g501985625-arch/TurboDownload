import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import { join } from 'path'

export default defineConfig({
  plugins: [react()],
  build: {
    outDir: join(__dirname, 'src-tauri/dist'),
    emptyOutDir: true
  },
  server: {
    port: 1420,
    strictPort: true
  }
})