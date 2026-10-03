<script lang="ts">
	import '../app.css';
	import ToastViewport from '#lib/components/ToastViewport.svelte';
	import { initTheme } from '#lib/helpers/theme.js';
	import { onMount, type Snippet } from 'svelte';

	let { children }: { children: Snippet } = $props();

	onMount(() => {
		initTheme();
	});
</script>

{@render children()}
<ToastViewport />
