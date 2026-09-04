<script lang="ts">
	interface Props {
		size?: 'sm' | 'md';
		alt?: string;
	}

	let { size = 'md', alt = 'Trading Lab' }: Props = $props();

	const isSm = $derived(size === 'sm');
	const imgClass = $derived(isSm ? 'h-6' : 'h-7 sm:h-8');
	const maxWidth = $derived(isSm ? '130px' : '150px');
</script>

<img src="/logo-dark.svg" {alt} class="logo-dark-theme {imgClass}" style="max-width: {maxWidth};" />
<img
	src="/logo-light.svg"
	{alt}
	class="logo-light-theme {imgClass}"
	style="max-width: {maxWidth};"
/>
