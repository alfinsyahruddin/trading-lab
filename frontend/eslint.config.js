import js from '@eslint/js';
import ts from 'typescript-eslint';
import svelte from 'eslint-plugin-svelte';
import tailwind from 'eslint-plugin-tailwindcss';
import globals from 'globals';

export default ts.config(
	js.configs.recommended,
	...ts.configs.recommended,
	...svelte.configs['flat/recommended'],
	tailwind.configs.recommended,
	{
		settings: {
			tailwindcss: {
				cssConfigPath: 'src/app.css'
			}
		},
		languageOptions: {
			globals: {
				...globals.browser,
				...globals.node
			}
		}
	},
	{
		files: ['**/*.ts', '**/*.svelte.ts', '**/*.js'],
		languageOptions: {
			parser: ts.parser
		}
	},
	{
		files: ['**/*.svelte', '*.svelte'],
		languageOptions: {
			parserOptions: {
				parser: ts.parser
			}
		}
	},
	{
		rules: {
			// Tailwind rules
			'tailwindcss/no-custom-classname': 'off',
			'tailwindcss/classnames-order': 'off',

			// TypeScript rules
			'@typescript-eslint/no-unused-vars': [
				'warn',
				{ argsIgnorePattern: '^_', varsIgnorePattern: '^_' }
			],

			// SvelteKit CSR/SPA friendly adjustments
			'svelte/no-navigation-without-resolve': 'off',
			'svelte/prefer-svelte-reactivity': 'warn',
			'svelte/require-each-key': 'warn',
			'svelte/no-useless-children-snippet': 'warn'
		}
	},
	{
		ignores: ['build/', '.svelte-kit/', 'dist/', 'node_modules/', 'coverage/']
	}
);
