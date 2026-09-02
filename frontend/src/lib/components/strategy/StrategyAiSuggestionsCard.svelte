<script lang="ts">
	import Icon from '@iconify/svelte';
	import type { StrategyAiSuggestion } from '$lib/types';

	let {
		suggestions = [],
		onaccept,
		onignore
	}: {
		suggestions: StrategyAiSuggestion[];
		onaccept: (suggestion: StrategyAiSuggestion) => void;
		onignore: (suggestion: StrategyAiSuggestion) => void;
	} = $props();

	function getFieldLabel(field: string): string {
		switch (field) {
			case 'tp_percentage':
				return 'Take Profit';
			case 'sl_percentage':
				return 'Stop Loss';
			case 'max_holding_period_days':
				return 'Max Holding Period';
			default:
				return field;
		}
	}

	function formatFieldValue(field: string, val: number): string {
		if (field === 'max_holding_period_days') {
			return `${Math.round(val)} Days`;
		}
		return `${val}%`;
	}
</script>

{#if suggestions.length > 0}
	<div class="ai-suggestion-card relative mb-4 rounded-2xl p-[1.5px] shadow-lg">
		<!-- Static Orange-Yellow Border -->
		<div class="ai-suggestion-border absolute inset-0 rounded-2xl"></div>

		<!-- Inner Container -->
		<div
			class="ai-suggestion-inner relative z-10 flex flex-col rounded-2xl p-4 sm:p-5"
			style="background-color: var(--bg-card);"
		>
			<!-- Header -->
			<div
				class="mb-4 flex flex-wrap items-center justify-between gap-2 border-b pb-3"
				style="border-color: rgba(255, 149, 0, 0.2);"
			>
				<div class="flex items-center gap-2.5">
					<div
						class="flex size-7 items-center justify-center rounded-lg shadow-xs"
						style="
							background: linear-gradient(135deg, #FF9500 0%, #FFCC00 100%);
							color: #ffffff;
						"
					>
						<Icon icon="lucide:sparkles" width="15" height="15" />
					</div>
					<div>
						<h3 class="font-700 text-sm sm:text-base" style="color: var(--fg)">
							AI Strategy Suggestions
						</h3>
					</div>
				</div>

				<div class="flex items-center gap-2">
					<span
						class="font-600 inline-flex items-center gap-1 rounded-full px-2.5 py-0.5 text-[11px]"
						style="
							background-color: rgba(255, 149, 0, 0.15);
							color: #FF9500;
							border: 1px solid rgba(255, 149, 0, 0.3);
						"
					>
						<Icon icon="lucide:alert-circle" width="12" height="12" />
						<span>Action Required ({suggestions.length} remaining)</span>
					</span>
				</div>
			</div>

			<p class="font-400 mb-3 text-xs" style="color: var(--fg-muted);">
				Please <strong>Accept</strong> or <strong>Ignore</strong> each suggestion below before creating
				your strategy.
			</p>

			<!-- Suggestions List -->
			<div class="flex flex-col gap-3">
				{#each suggestions as item (item.id)}
					<div
						class="flex flex-col gap-3 rounded-xl border p-3.5 transition-all duration-150 sm:flex-row sm:items-center sm:justify-between"
						style="
							background-color: rgba(255, 149, 0, 0.04);
							border-color: rgba(255, 149, 0, 0.2);
						"
					>
						<!-- Left: details -->
						<div class="flex flex-col gap-1 sm:max-w-[65%]">
							<div class="flex flex-wrap items-center gap-2">
								<h4 class="font-700 text-xs sm:text-sm" style="color: var(--fg);">
									{item.title}
								</h4>
								<span
									class="font-600 rounded-md px-1.5 py-0.5 text-[11px]"
									style="
										background-color: var(--bg-card);
										color: var(--fg);
										border: 1px solid var(--border);
									"
								>
									{getFieldLabel(item.field)}:
									<span style="color: var(--fg-muted); text-decoration: line-through;"
										>{formatFieldValue(item.field, item.current_value)}</span
									>
									<Icon icon="lucide:arrow-right" class="mx-0.5 inline" width="10" height="10" />
									<span class="font-700" style="color: #FF9500;"
										>{formatFieldValue(item.field, item.suggested_value)}</span
									>
								</span>
							</div>

							<p class="text-xs leading-relaxed" style="color: var(--fg-muted);">
								{item.reason}
							</p>
						</div>

						<!-- Right: action buttons -->
						<div class="flex items-center gap-2 self-end sm:self-center">
							<button
								type="button"
								onclick={() => onignore(item)}
								class="btn-interactive font-600 inline-flex items-center gap-1.5 rounded-lg border px-3 py-1.5 text-xs transition-colors duration-150 hover:bg-(--bg-card-hover)"
								style="
									border-color: var(--border-strong);
									color: var(--fg-muted);
								"
							>
								<Icon icon="lucide:x" width="13" height="13" />
								<span>Ignore</span>
							</button>

							<button
								type="button"
								onclick={() => onaccept(item)}
								class="btn-interactive font-600 inline-flex items-center gap-1.5 rounded-lg px-3.5 py-1.5 text-xs text-white shadow-xs transition-all duration-150 active:scale-95"
								style="
									background: linear-gradient(135deg, #FF9500 0%, #FFCC00 100%);
								"
							>
								<Icon icon="lucide:check" width="13" height="13" />
								<span>Accept</span>
							</button>
						</div>
					</div>
				{/each}
			</div>
		</div>
	</div>
{/if}

<style>
	.ai-suggestion-card {
		overflow: hidden;
	}

	.ai-suggestion-border {
		background: linear-gradient(135deg, #ff9500 0%, #ffcc00 100%);
	}

	.ai-suggestion-inner {
		background:
			linear-gradient(135deg, rgba(255, 149, 0, 0.05) 0%, rgba(255, 204, 0, 0.05) 100%),
			var(--bg-card);
	}
</style>
