import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  build: {
    manifest: true,
    rollupOptions: {
      input: "/resources/js/main.js"
    }
  },
  server: {
    origin: "http://127.0.0.1:5000"
  }
})
