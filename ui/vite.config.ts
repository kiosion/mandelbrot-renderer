import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import Wasm from 'vite-plugin-wasm';
import TopLevelAwait from 'vite-plugin-top-level-await';

export default defineConfig({
  plugins: [
    sveltekit(),
    Wasm(),
    TopLevelAwait()
  ]
});
