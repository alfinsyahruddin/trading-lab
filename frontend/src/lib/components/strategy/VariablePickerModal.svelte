<script lang="ts">
	import Icon from '@iconify/svelte';
	import {
		STRATEGY_VARIABLES,
		VARIABLE_CATEGORIES,
		type StrategyVariableOption
	} from '$lib/constants';

	let {
		open = $bindable(false),
		selectedVariable = '',
		onselect
	}: {
		open?: boolean;
		selectedVariable?: string;
		onselect?: (variableCode: string) => void;
	} = $props();

	let searchQuery = $state('');
	let tempSelected = $state('');

	// Sync tempSelected whenever the modal opens or selectedVariable changes
	$effect(() => {
		if (open) {
			tempSelected = selectedVariable;
			searchQuery = '';
		}
	});

	const hasDifference = $derived(tempSelected !== '' && tempSelected !== selectedVariable);

	const filteredCategories = $derived.by(() => {
		const q = searchQuery.trim().toLowerCase();
		const results: { category: string; variables: StrategyVariableOption[] }[] = [];

		for (const category of VARIABLE_CATEGORIES) {
			const vars = STRATEGY_VARIABLES.filter((v) => {
				if (v.category !== category) return false;
				if (!q) return true;
				return (
					v.code.toLowerCase().includes(q) ||
					v.name.toLowerCase().includes(q) ||
					v.description.toLowerCase().includes(q)
				);
			});

			if (vars.length > 0) {
				results.push({ category, variables: vars });
			}
		}

		return results;
	});

	function close() {
		open = false;
	}

	function handleSave() {
		if (hasDifference) {
			onselect?.(tempSelected);
			open = false;
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') close();
	}
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open}
	<!-- Backdrop -->
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div
		class="animate-fade-in fixed inset-0 z-50 flex items-center justify-center p-2.5 sm:p-4"
		style="background-color: rgba(0,0,0,0.6); backdrop-filter: blur(4px);"
		onclick={(e) => {
			if (e.target === e.currentTarget) close();
		}}
	>
		<!-- Dialog -->
		<div
			class="animate-modal relative flex max-h-[calc(100dvh-1.25rem)] w-full max-w-5xl flex-col overflow-hidden rounded-2xl border shadow-2xl sm:max-h-[calc(100dvh-2rem)]"
			style="background-color: var(--bg-card); border-color: var(--border-strong);"
			role="dialog"
			aria-modal="true"
			aria-labelledby="var-modal-title"
		>
			<!-- Header -->
			<div
				class="flex shrink-0 items-center justify-between border-b px-3.5 py-3 sm:px-6 sm:py-4"
				style="border-color: var(--border);"
			>
				<div class="flex items-center gap-2.5">
					<div
						class="flex size-8 shrink-0 items-center justify-center rounded-lg"
						style="background-color: var(--accent-soft); color: var(--accent);"
					>
						<Icon icon="lucide:sliders-horizontal" width="18" height="18" />
					</div>
					<div>
						<h2 id="var-modal-title" class="font-700 text-sm sm:text-base" style="color: var(--fg)">
							Select Variable
						</h2>
						<p class="text-xs" style="color: var(--fg-muted)">
							Choose a metric to construct your rule condition
						</p>
					</div>
				</div>
				<button
					onclick={close}
					class="btn-interactive rounded-lg p-1.5 transition-all duration-200 hover:rotate-90 hover:bg-(--bg-card-hover)"
					style="color: var(--fg-muted);"
					aria-label="Close modal"
				>
					<Icon icon="lucide:x" width="18" height="18" />
				</button>
			</div>

			<!-- Search input -->
			<div class="border-b px-3.5 py-2.5 sm:px-6 sm:py-3" style="border-color: var(--border);">
				<div class="relative">
					<div
						class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3.5"
						style="color: var(--fg-muted);"
					>
						<Icon icon="lucide:search" width="16" height="16" />
					</div>
					<input
						type="text"
						placeholder="Search by code or description (e.g. pb, revenue, roa, price)..."
						bind:value={searchQuery}
						class="w-full rounded-xl border py-2 pr-4 pl-9 text-xs transition-all duration-150 outline-none sm:pl-10 sm:text-sm"
						style="
							background-color: var(--bg-input, var(--bg));
							border-color: var(--border-strong);
							color: var(--fg);
						"
					/>
					{#if searchQuery}
						<button
							type="button"
							onclick={() => (searchQuery = '')}
							class="absolute inset-y-0 right-0 flex items-center pr-3"
							style="color: var(--fg-muted);"
							aria-label="Clear search"
						>
							<Icon icon="lucide:x" width="14" height="14" />
						</button>
					{/if}
				</div>
			</div>

			<!-- Body (Categorized list) -->
			<div class="flex-1 divide-y divide-transparent overflow-y-auto px-3.5 py-3 sm:px-6">
				{#if filteredCategories.length === 0}
					<div class="flex flex-col items-center justify-center py-12 text-center">
						<div
							class="mb-3 flex size-12 items-center justify-center rounded-xl"
							style="background-color: var(--accent-soft); color: var(--accent);"
						>
							<Icon icon="lucide:search-x" width="24" height="24" />
						</div>
						<p class="font-600 text-sm" style="color: var(--fg)">No variables found</p>
						<p class="mt-1 text-xs" style="color: var(--fg-muted)">
							Try searching with different keywords
						</p>
					</div>
				{:else}
					<div class="flex flex-col gap-6 py-2">
						{#each filteredCategories as { category, variables }}
							<div class="flex flex-col gap-2.5">
								<div class="flex items-center gap-2">
									<h3
										class="font-700 text-xs tracking-wider uppercase"
										style="color: var(--accent);"
									>
										{category}
									</h3>
									<div class="h-px flex-1" style="background-color: var(--border);"></div>
								</div>

								<div class="grid grid-cols-1 gap-2.5 sm:grid-cols-2 lg:grid-cols-3">
									{#each variables as v}
										{@const isSelected = tempSelected === v.code}
										<button
											type="button"
											onclick={() => (tempSelected = v.code)}
											class="btn-interactive flex items-start justify-between gap-2.5 rounded-xl border p-3 text-left transition-all duration-150"
											style="
												background-color: {isSelected ? 'var(--accent-soft)' : 'var(--bg-card-hover)'};
												border-color: {isSelected ? 'var(--accent)' : 'var(--border)'};
												box-shadow: {isSelected ? '0 0 0 1.5px var(--accent)' : 'none'};
											"
										>
											<div class="flex min-w-0 flex-1 flex-col items-start gap-1">
												<span
													class="font-700 inline-block shrink-0 rounded px-1.5 py-0.5 font-mono text-xs"
													style="
														background-color: {isSelected ? 'var(--accent)' : 'var(--border)'};
														color: {isSelected ? '#ffffff' : 'var(--fg)'};
													"
												>
													{v.code}
												</span>

												<p
													class="font-500 line-clamp-1 w-full truncate text-xs leading-snug"
													style="color: var(--fg-muted);"
													title={v.description}
												>
													{v.description}
												</p>
											</div>

											{#if isSelected}
												<div
													class="mt-0.5 flex size-5 shrink-0 items-center justify-center rounded-full text-white"
													style="background-color: var(--accent);"
												>
													<Icon icon="lucide:check" width="12" height="12" />
												</div>
											{/if}
										</button>
									{/each}
								</div>
							</div>
						{/each}
					</div>
				{/if}
			</div>

			<!-- Footer -->
			<div
				class="flex shrink-0 flex-col-reverse gap-2 border-t px-4 py-3.5 sm:flex-row sm:justify-end sm:gap-3 sm:px-6 sm:py-4"
				style="border-color: var(--border);"
			>
				<button
					type="button"
					onclick={close}
					class="btn-interactive font-500 w-full rounded-xl border px-4 py-2 text-sm transition-colors duration-150 hover:bg-(--bg-card-hover) sm:w-auto"
					style="border-color: var(--border-strong); color: var(--fg-muted);"
				>
					Cancel
				</button>
				<button
					type="button"
					onclick={handleSave}
					disabled={!hasDifference}
					class="btn-interactive font-600 inline-flex w-full items-center justify-center gap-1.5 rounded-xl px-5 py-2 text-sm text-white shadow-sm transition-all duration-150 disabled:cursor-not-allowed disabled:opacity-40 sm:w-auto"
					style="background-color: var(--accent);"
				>
					<Icon icon="lucide:check" width="16" height="16" />
					Save
				</button>
			</div>
		</div>
	</div>
{/if}
