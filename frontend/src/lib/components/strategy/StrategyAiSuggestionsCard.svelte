<script lang="ts">
	import Icon from '@iconify/svelte';
	import { STRATEGY_OPERATORS, STRATEGY_VARIABLES } from '$lib/constants';
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

	function getFieldLabel(field?: string | null): string {
		switch (field) {
			case 'tp_percentage':
				return 'Take Profit';
			case 'sl_percentage':
				return 'Stop Loss';
			case 'max_holding_period_days':
				return 'Max Holding Period';
			default:
				return field || 'Parameter';
		}
	}

	function formatFieldValue(field?: string | null, val?: number | null): string {
		if (val === undefined || val === null) return '';
		if (field === 'max_holding_period_days') {
			return `${Math.round(val)} Days`;
		}
		return `${val}%`;
	}

	function getVariableLabel(code: string): string {
		const found = STRATEGY_VARIABLES.find((v) => v.code === code);
		return found ? `${found.name} (${found.code})` : code;
	}

	function getOperatorDisplay(op: string): string {
		const found = STRATEGY_OPERATORS.find((o) => o.value === op);
		return found ? found.symbol : op;
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
					<svg
						width="20"
						height="20"
						viewBox="0 0 607 607"
						fill="none"
						xmlns="http://www.w3.org/2000/svg"
						class="shrink-0"
					>
						<defs>
							<linearGradient id="strategyAiIconGrad" x1="0%" y1="0%" x2="100%" y2="100%">
								<stop offset="0%" stop-color="#FF9500" />
								<stop offset="100%" stop-color="#FFCC00" />
							</linearGradient>
						</defs>
						<path
							d="M279.22 488.635C295.407 525.561 303.5 565.016 303.5 607C303.5 565.016 311.34 525.561 327.021 488.635C343.208 451.709 364.959 419.589 392.274 392.274C419.589 364.959 451.709 343.461 488.635 327.78C525.561 311.593 565.016 303.5 607 303.5C565.016 303.5 525.561 295.66 488.635 279.979C452.696 264.483 420.004 242.345 392.274 214.726C364.655 186.996 342.517 154.304 327.021 118.365C311.34 81.4392 303.5 41.9842 303.5 0C303.5 41.9842 295.407 81.4392 279.22 118.365C263.539 155.291 242.041 187.411 214.726 214.726C186.996 242.345 154.304 264.483 118.365 279.979C81.4392 295.66 41.9842 303.5 0 303.5C41.9842 303.5 81.4392 311.593 118.365 327.78C155.291 343.461 187.411 364.959 214.726 392.274C242.041 419.589 263.539 451.709 279.22 488.635Z"
							fill="url(#strategyAiIconGrad)"
						/>
					</svg>

					<h3
						class="font-700 text-sm tracking-tight sm:text-base"
						style="
							background: linear-gradient(135deg, #FF9500 0%, #FFCC00 100%);
							-webkit-background-clip: text;
							-webkit-text-fill-color: transparent;
							background-clip: text;
						"
					>
						AI Strategy Suggestions
					</h3>
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
								{#if item.suggestion_type === 'RULE' && item.rule_payload}
									<span
										class="font-600 rounded-md px-1.5 py-0.5 text-[11px]"
										style="
											background-color: var(--bg-card);
											color: var(--fg);
											border: 1px solid var(--border);
										"
									>
										<span class="font-700" style="color: #FF9500;">
											{item.rule_action === 'EDIT_CONDITION' ? 'Modify Rule:' : 'Add Rule:'}
										</span>
										<span class="ml-1 font-mono">
											{getVariableLabel(item.rule_payload.variable)}
											{getOperatorDisplay(item.rule_payload.operator)}
											{item.rule_payload.value}
										</span>
									</span>
								{:else if item.field}
									<span
										class="font-600 rounded-md px-1.5 py-0.5 text-[11px]"
										style="
											background-color: var(--bg-card);
											color: var(--fg);
											border: 1px solid var(--border);
										"
									>
										{getFieldLabel(item.field)}:
										{#if item.current_value !== null && item.current_value !== undefined}
											<span style="color: var(--fg-muted); text-decoration: line-through;"
												>{formatFieldValue(item.field, item.current_value)}</span
											>
											<Icon
												icon="lucide:arrow-right"
												class="mx-0.5 inline"
												width="10"
												height="10"
											/>
										{/if}
										{#if item.suggested_value !== null && item.suggested_value !== undefined}
											<span class="font-700" style="color: #FF9500;"
												>{formatFieldValue(item.field, item.suggested_value)}</span
											>
										{/if}
									</span>
								{/if}
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
								class="btn-interactive font-600 inline-flex items-center gap-1.5 rounded-lg border px-3 py-1.5 text-xs transition-all duration-150 hover:bg-[rgba(255,149,0,0.15)] active:scale-95"
								style="
									border-color: rgba(255, 149, 0, 0.4);
									color: #FF9500;
									background-color: rgba(255, 149, 0, 0.08);
								"
							>
								<Icon icon="lucide:x" width="13" height="13" />
								<span>Ignore</span>
							</button>

							<button
								type="button"
								onclick={() => onaccept(item)}
								class="btn-interactive font-600 inline-flex items-center gap-1.5 rounded-lg px-3.5 py-1.5 text-xs text-white shadow-xs transition-all duration-150 hover:brightness-105 active:scale-95"
								style="background-color: #FF9500;"
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
