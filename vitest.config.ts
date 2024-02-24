import { defineConfig } from 'vitest/config';

export default defineConfig({
	test: {
		globals: true,
		coverage: {
			enabled: false
		},
		poolOptions: {
			threads: {
				singleThread: true
			}
		}
	},
	esbuild: {
		target: 'esnext'
	}
});
