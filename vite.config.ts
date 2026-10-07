import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import path from 'path';

export default defineConfig({
  root: path.resolve(import.meta.dirname, 'svelte'), // Set svelte/ as root
  plugins: [sveltekit()],
  resolve: {
    alias: {
      $lib: path.resolve(import.meta.dirname, 'svelte/src/lib'),
      $routes: path.resolve(import.meta.dirname, 'svelte/src/routes'),
      $components: path.resolve(import.meta.dirname, 'svelte/src/components'),
    },
  },
  server: {
    fs: {
      allow: [
        // Allow serving files from these directories
        path.resolve(import.meta.dirname, 'svelte'),
        path.resolve(import.meta.dirname, 'svelte/src'),
        path.resolve(import.meta.dirname, 'svelte/static'),
      ],
    },
  },
});
