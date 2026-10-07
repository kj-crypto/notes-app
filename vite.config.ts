import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import path from 'path';

export default defineConfig({
  root: path.resolve(__dirname, 'svelte'), // Set svelte/ as root
  plugins: [sveltekit()],
  resolve: {
    alias: {
      $lib: path.resolve(__dirname, 'svelte/src/lib'),
      $routes: path.resolve(__dirname, 'svelte/src/routes'),
      $components: path.resolve(__dirname, 'svelte/src/components'),
    },
  },
  server: {
    fs: {
      allow: [
        // Allow serving files from these directories
        path.resolve(__dirname, 'svelte'),
        path.resolve(__dirname, 'svelte/src'),
        path.resolve(__dirname, 'svelte/static'),
      ],
    },
  },
});
