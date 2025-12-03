import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [svelte()],
  server: {
    proxy: {
      '/checks': 'http://localhost:3000',
      '/history': 'http://localhost:3000'
    }
  }
})
