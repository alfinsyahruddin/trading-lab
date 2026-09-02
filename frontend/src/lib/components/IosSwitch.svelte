<script lang="ts">
	let {
		checked = $bindable(false),
		disabled = false,
		id = undefined,
		name = undefined,
		label = undefined,
		onchange = undefined
	}: {
		checked?: boolean;
		disabled?: boolean;
		id?: string;
		name?: string;
		label?: string;
		onchange?: (checked: boolean) => void;
	} = $props();

	function toggle() {
		if (disabled) return;
		checked = !checked;
		onchange?.(checked);
	}

	function handleKeydown(e: KeyboardEvent) {
		if (disabled) return;
		if (e.key === ' ' || e.key === 'Enter') {
			e.preventDefault();
			toggle();
		}
	}
</script>

<div class="inline-flex items-center gap-3">
	<button
		type="button"
		role="switch"
		aria-checked={checked}
		aria-label={label}
		{disabled}
		{id}
		onclick={toggle}
		onkeydown={handleKeydown}
		class="ios-switch relative inline-flex h-7 w-12 shrink-0 cursor-pointer items-center rounded-full p-0.5 transition-colors duration-250 ease-in-out focus-visible:outline-2 focus-visible:outline-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
		class:active={checked}
		style="
			background-color: {checked ? 'var(--accent)' : 'rgba(120, 120, 128, 0.28)'};
			outline-color: var(--accent);
		"
	>
		<span
			class="pointer-events-none inline-block size-6 rounded-full bg-white shadow-md transition-transform duration-250 ease-in-out"
			style="
				transform: translateX({checked ? '20px' : '0px'});
			"
		></span>
	</button>

	{#if name}
		<input type="hidden" {name} value={checked ? 'true' : 'false'} />
	{/if}
</div>

<style>
	.ios-switch {
		box-shadow: inset 0 1px 2px rgba(0, 0, 0, 0.15);
	}

	.ios-switch.active {
		box-shadow: 0 0 12px rgba(48, 180, 201, 0.35);
	}
</style>
