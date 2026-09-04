<script lang="ts">
	import { onMount } from 'svelte';
	import Icon from '@iconify/svelte';

	interface Option {
		label: string;
		value: string;
	}

	let {
		label = '',
		value = $bindable<string>(''),
		options = [] as Option[],
		placeholder = 'Select an option',
		error = '',
		disabled = false,
		required = false,
		class: className = '',
		buttonClass = '',
		buttonStyle = '',
		onchange
	}: {
		label?: string;
		value?: string;
		options?: Option[];
		placeholder?: string;
		error?: string;
		disabled?: boolean;
		required?: boolean;
		class?: string;
		buttonClass?: string;
		buttonStyle?: string;
		onchange?: (val: string) => void;
	} = $props();

	let isOpen = $state(false);
	let openUpwards = $state(false);
	let searchQuery = $state('');
	let containerRef = $state<HTMLDivElement | null>(null);
	let searchInputRef = $state<HTMLInputElement | null>(null);
	let highlightedIndex = $state(0);

	const selectId = $derived(
		label ? `select-${label.toLowerCase().replace(/\s+/g, '-')}` : 'select-field'
	);

	const listboxId = $derived(`${selectId}-listbox`);

	const selectedOption = $derived(options.find((opt) => String(opt.value) === String(value)));

	const filteredOptions = $derived(
		options.filter((opt) => opt.label.toLowerCase().includes(searchQuery.trim().toLowerCase()))
	);

	function toggleDropdown() {
		if (disabled) return;
		isOpen = !isOpen;
		if (isOpen) {
			searchQuery = '';
			if (containerRef && typeof window !== 'undefined') {
				const rect = containerRef.getBoundingClientRect();
				const spaceBelow = window.innerHeight - rect.bottom;
				const spaceAbove = rect.top;
				openUpwards = spaceBelow < 260 && spaceAbove > spaceBelow;
			}
			const idx = filteredOptions.findIndex((opt) => String(opt.value) === String(value));
			highlightedIndex = idx >= 0 ? idx : 0;
			setTimeout(() => {
				searchInputRef?.focus();
			}, 50);
		}
	}

	function selectOption(opt: Option) {
		if (disabled) return;
		value = opt.value;
		isOpen = false;
		searchQuery = '';
		onchange?.(opt.value);
	}

	function handleKeyDown(e: KeyboardEvent) {
		if (disabled) return;

		if (!isOpen) {
			if (e.key === 'Enter' || e.key === ' ' || e.key === 'ArrowDown') {
				e.preventDefault();
				toggleDropdown();
			}
			return;
		}

		if (e.key === 'Escape') {
			e.preventDefault();
			isOpen = false;
		} else if (e.key === 'ArrowDown') {
			e.preventDefault();
			if (filteredOptions.length > 0) {
				highlightedIndex = (highlightedIndex + 1) % filteredOptions.length;
			}
		} else if (e.key === 'ArrowUp') {
			e.preventDefault();
			if (filteredOptions.length > 0) {
				highlightedIndex = (highlightedIndex - 1 + filteredOptions.length) % filteredOptions.length;
			}
		} else if (e.key === 'Enter') {
			e.preventDefault();
			if (filteredOptions[highlightedIndex]) {
				selectOption(filteredOptions[highlightedIndex]);
			}
		}
	}

	onMount(() => {
		function handleClickOutside(e: MouseEvent) {
			if (containerRef && !containerRef.contains(e.target as Node)) {
				isOpen = false;
			}
		}

		document.addEventListener('mousedown', handleClickOutside);
		return () => {
			document.removeEventListener('mousedown', handleClickOutside);
		};
	});
</script>

<div
	class="relative flex flex-col gap-1.5 {isOpen ? 'z-50' : 'z-0'} {className}"
	bind:this={containerRef}
>
	{#if label}
		<label for={selectId} class="font-500 text-sm" style="color: var(--fg-muted)">
			{label}{#if required}<span style="color: var(--danger)"> *</span>{/if}
		</label>
	{/if}

	<!-- Custom Trigger Button -->
	<button
		type="button"
		role="combobox"
		id={selectId}
		aria-haspopup="listbox"
		aria-controls={listboxId}
		aria-expanded={isOpen}
		{disabled}
		onclick={toggleDropdown}
		onkeydown={handleKeyDown}
		class="group flex w-full items-center justify-between rounded-lg border px-3.5 py-2.5 text-left text-base transition-all duration-150 outline-none disabled:cursor-not-allowed sm:text-sm {buttonClass}"
		style="
			background-color: {disabled ? 'var(--bg-input-disabled, var(--bg))' : 'var(--bg-input, var(--bg))'};
			border-color: {error
			? 'var(--danger)'
			: isOpen
				? 'var(--accent)'
				: disabled
					? 'var(--border)'
					: 'var(--border-strong)'};
			color: {disabled ? 'var(--fg-muted)' : selectedOption ? 'var(--fg)' : 'var(--fg-muted)'};
			box-shadow: {isOpen ? '0 0 0 3px rgba(48,180,201,0.1)' : 'none'};
			{buttonStyle}
		"
	>
		<span class="truncate">
			{selectedOption ? selectedOption.label : placeholder}
		</span>
		<Icon
			icon="lucide:chevron-down"
			class="shrink-0 transition-transform duration-200 {isOpen
				? 'rotate-180 text-(--accent)'
				: 'text-(--fg-muted)'}"
			width="16"
			height="16"
		/>
	</button>

	<!-- Hidden input for form values -->
	<input type="hidden" name={label} {value} {required} />

	<!-- Floating Dropdown Menu -->
	{#if isOpen}
		<div
			id={listboxId}
			role="listbox"
			tabindex="-1"
			class="absolute {openUpwards
				? 'bottom-full mb-1.5'
				: 'top-full mt-1.5'} left-0 z-50 flex max-h-64 w-full min-w-50 flex-col overflow-hidden rounded-xl border shadow-xl backdrop-blur-md"
			style="
				background-color: var(--bg-card);
				border-color: var(--border);
				box-shadow: 0 12px 30px -4px rgba(0, 0, 0, 0.25), 0 4px 12px -2px rgba(0, 0, 0, 0.15);
			"
		>
			<!-- Search Header -->
			<div
				class="sticky top-0 z-10 border-b p-2"
				style="background-color: var(--bg-card); border-color: var(--border);"
			>
				<div class="relative flex items-center">
					<Icon
						icon="lucide:search"
						class="pointer-events-none absolute left-2.5 text-(--fg-muted)"
						width="14"
						height="14"
					/>
					<input
						bind:this={searchInputRef}
						type="text"
						bind:value={searchQuery}
						onkeydown={handleKeyDown}
						placeholder="Search..."
						class="w-full rounded-md border bg-transparent py-1.5 pr-3 pl-8 text-xs text-(--fg) transition-colors outline-none"
						style="border-color: var(--border);"
					/>
					{#if searchQuery}
						<button
							type="button"
							onclick={() => (searchQuery = '')}
							class="btn-interactive absolute right-2 text-(--fg-muted) hover:text-(--fg)"
						>
							<Icon icon="lucide:x" width="12" height="12" />
						</button>
					{/if}
				</div>
			</div>

			<!-- Options List -->
			<div class="overflow-y-auto p-1 text-sm">
				{#if filteredOptions.length === 0}
					<div class="py-4 text-center text-xs" style="color: var(--fg-muted)">
						No options found
					</div>
				{:else}
					{#each filteredOptions as opt, index (opt.value)}
						{@const isSelected = String(opt.value) === String(value)}
						{@const isHighlighted = index === highlightedIndex}
						<button
							type="button"
							role="option"
							aria-selected={isSelected}
							onclick={() => selectOption(opt)}
							onmouseenter={() => (highlightedIndex = index)}
							class="flex w-full items-center justify-between rounded-lg px-3 py-2 text-left text-xs transition-colors duration-100"
							style="
								background-color: {isSelected
								? 'var(--accent-soft)'
								: isHighlighted
									? 'var(--bg-card-hover, rgba(255,255,255,0.05))'
									: 'transparent'};
								color: {isSelected ? 'var(--accent)' : 'var(--fg)'};
							"
						>
							<span class="truncate font-medium">{opt.label}</span>
							{#if isSelected}
								<Icon icon="lucide:check" width="14" height="14" class="shrink-0 text-(--accent)" />
							{/if}
						</button>
					{/each}
				{/if}
			</div>
		</div>
	{/if}

	{#if error}
		<p class="text-xs" style="color: var(--danger)">{error}</p>
	{/if}
</div>
