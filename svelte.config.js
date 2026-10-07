import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  // Consult https://svelte.dev/docs/kit/integrations
  // for more information about preprocessors
  preprocess: vitePreprocess(),

  kit: {
    files: {
      routes: 'svelte/src/routes',
      lib: 'svelte/src/lib',
      assets: 'svelte/static',
      appTemplate: 'svelte/src/app.html',
    },
    alias: {
      $lib: 'svelte/src/lib',
      $routes: 'svelte/src/routes',
      $components: 'svelte/src/components',
    },
    adapter: adapter({
      pages: 'build',
    }),
    output: {
      preloadStrategy: 'modulepreload',
      bundleStrategy: 'split',
    },
  },
};

export default config;
