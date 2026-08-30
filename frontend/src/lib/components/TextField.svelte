<script lang="ts">
	let {
		label = '',
		value = $bindable(''),
		type = 'text',
		placeholder = '',
		error = '',
		disabled = false,
		required = false,
		step,
		min,
		max
	}: {
		label?: string;
		value?: string;
		type?: 'text' | 'email' | 'password' | 'number';
		placeholder?: string;
		error?: string;
		disabled?: boolean;
		required?: boolean;
		step?: string | number;
		min?: string | number;
		max?: string | number;
	} = $props();

	const inputId = $derived(label ? `field-${label.toLowerCase().replace(/\s+/g, '-')}` : undefined);
	const computedStep = $derived(step !== undefined ? step : type === 'number' ? 'any' : undefined);
</script>

<div class="flex flex-col gap-1.5">
	{#if label}
		<label for={inputId} class="text-sm font-500" style="color: var(--fg-muted)">
			{label}{#if required}<span style="color: var(--danger)"> *</span>{/if}
		</label>
	{/if}
	<input
		id={inputId}
		{type}
		{placeholder}
		{disabled}
		{required}
		step={computedStep}
		{min}
		{max}
		bind:value
		class="w-full rounded-lg border px-3.5 py-2.5 text-base sm:text-sm font-400 outline-none transition-all duration-150"
		style="
			background-color: var(--bg-input, var(--bg));
			border-color: {error ? 'var(--danger)' : 'var(--border-strong)'};
			color: var(--fg);
		"
		onfocus={(e) => {
			(e.currentTarget as HTMLInputElement).style.borderColor = error
				? 'var(--danger)'
				: 'var(--accent)';
			(e.currentTarget as HTMLInputElement).style.boxShadow = error
				? '0 0 0 3px rgba(239,68,68,0.1)'
				: '0 0 0 3px rgba(48,180,201,0.1)';
		}}
		onblur={(e) => {
			(e.currentTarget as HTMLInputElement).style.borderColor = error
				? 'var(--danger)'
				: 'var(--border-strong)';
			(e.currentTarget as HTMLInputElement).style.boxShadow = 'none';
		}}
	/>
	{#if error}
		<p class="text-xs" style="color: var(--danger)">{error}</p>
	{/if}
</div>
