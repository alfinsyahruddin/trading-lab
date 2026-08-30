import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';

export default defineConfig({
	plugins: [sveltekit()],
	resolve: process.env.VITEST ? { conditions: ['browser'] } : undefined,
	test: {
		environment: 'jsdom',
		include: ['tests/unit/**/*.{test,spec}.ts'],
		setupFiles: ['tests/unit/setup.ts'],
		coverage: {
			provider: 'v8',
			reportsDirectory: './coverage',
			include: ['src/lib/**/*.{ts,svelte}', 'src/routes/**/*.{ts,svelte}']
		}
	}
});
