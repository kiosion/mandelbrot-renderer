// import adapter from '@sveltejs/adapter-auto';
import adapterNode from '@sveltejs/adapter-node'; 
import { vitePreprocess } from '@sveltejs/kit/vite';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),

	kit: {
		adapter: adapterNode()
	}
};

export default config;
