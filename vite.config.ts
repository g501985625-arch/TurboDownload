import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import { join } from 'path'
import pkg from './package.json'

export default defineConfig({
  plugins: [react()],
  define: {
    'import.meta.env.VITE_APP_VERSION': JSON.stringify(pkg.version)
  },
  build: {
    outDir: join(__dirname, 'src-tauri/dist'),
    emptyOutDir: true
  },
  server: {
    port: 1420,
    strictPort: true
  }
})