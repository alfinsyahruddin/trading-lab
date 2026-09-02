<script lang="ts">
	interface Option {
		label: string;
		value: string;
	}

	let {
		label = '',
		value = $bindable<string>(''),
		options = [] as Option[],
		error = '',
		disabled = false,
		required = false
	}: {
		label?: string;
		value?: string;
		options?: Option[];
		error?: string;
		disabled?: boolean;
		required?: boolean;
	} = $props();

	const selectId = $derived(
		label ? `select-${label.toLowerCase().replace(/\s+/g, '-')}` : undefined
	);
</script>

<div class="flex flex-col gap-1.5">
	{#if label}
		<label for={selectId} class="font-500 text-sm" style="color: var(--fg-muted)">
			{label}{#if required}<span style="color: var(--danger)"> *</span>{/if}
		</label>
	{/if}
	<select
		id={selectId}
		bind:value
		{disabled}
		{required}
		class="select-field font-400 w-full appearance-none rounded-lg border px-3.5 py-2.5 text-base transition-all duration-150 outline-none disabled:cursor-not-allowed sm:text-sm"
		style="
			background-color: {disabled ? 'var(--bg-input-disabled, var(--bg))' : 'var(--bg-input, var(--bg))'};
			border-color: {error ? 'var(--danger)' : disabled ? 'var(--border)' : 'var(--border-strong)'};
			color: {disabled ? 'var(--fg-muted)' : 'var(--fg)'};
			cursor: {disabled ? 'not-allowed' : 'pointer'};
		"
		onfocus={(e) => {
			if (disabled) return;
			(e.currentTarget as HTMLSelectElement).style.borderColor = 'var(--accent)';
			(e.currentTarget as HTMLSelectElement).style.boxShadow = '0 0 0 3px rgba(48,180,201,0.1)';
		}}
		onblur={(e) => {
			if (disabled) return;
			(e.currentTarget as HTMLSelectElement).style.borderColor = error
				? 'var(--danger)'
				: 'var(--border-strong)';
			(e.currentTarget as HTMLSelectElement).style.boxShadow = 'none';
		}}
	>
		{#each options as opt (opt.value)}
			<option value={opt.value}>{opt.label}</option>
		{/each}
	</select>
	{#if error}
		<p class="text-xs" style="color: var(--danger)">{error}</p>
	{/if}
</div>

<style>
	/* Custom dropdown arrow via CSS pseudo-element to avoid SVG URL escaping issues */
	.select-field {
		background-image: url("data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 12 12'><path fill='%2364748b' d='M6 8L1 3h10z'/></svg>");
		background-repeat: no-repeat;
		background-position: right 12px center;
		padding-right: 36px;
	}

	.select-field:disabled {
		background-image: url("data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 12 12'><path fill='%2394a3b8' d='M6 8L1 3h10z'/></svg>");
	}
</style>
