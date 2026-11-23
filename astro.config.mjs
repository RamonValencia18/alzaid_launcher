// @ts-check
import { defineConfig } from 'astro/config';

// https://astro.build/config
export default defineConfig({
  // ... tu configuración actual de Astro ...
  vite: {
    server: {
      watch: {
        ignored: ['**/src-tauri/**'],
      },
    },
  },
});