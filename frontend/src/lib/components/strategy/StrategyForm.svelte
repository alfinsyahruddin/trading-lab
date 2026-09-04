<script lang="ts">
	import Icon from '@iconify/svelte';
	import TextField from '$lib/components/TextField.svelte';
	import WhereConditionsBuilder from './WhereConditionsBuilder.svelte';
	import StrategyAiSuggestionsCard from './StrategyAiSuggestionsCard.svelte';
	import { formatRiskReward } from '$lib/constants';
	import { getStrategyAiSuggestions } from '$lib/api';
	import { getToken } from '$lib/helpers/session';
	import type {
		CreateStrategyPayload,
		StrategyAiSuggestion,
		StrategyRuleGroup,
		TradingStrategy
	} from '$lib/types';

	let {
		initialData,
		loading = false,
		error = '',
		submitLabel = 'Save Strategy',
		enableAiSuggestions = false,
		onsubmit,
		oncancel
	}: {
		initialData?: Partial<TradingStrategy>;
		loading?: boolean;
		error?: string;
		submitLabel?: string;
		enableAiSuggestions?: boolean;
		onsubmit: (data: CreateStrategyPayload) => void;
		oncancel: () => void;
	} = $props();

	let name = $state('');
	let description = $state('');
	let tpPercentage = $state('10');
	let slPercentage = $state('5');
	let maxHoldingPeriodDays = $state('30');
	let rules = $state<StrategyRuleGroup[]>([
		{
			id: crypto.randomUUID(),
			connector_to_next: null,
			conditions: [
				{
					id: crypto.randomUUID(),
					variable: 'price',
					operator: '>',
					value: '50',
					connector_to_next: null
				}
			]
		}
	]);

	let suggestions = $state<StrategyAiSuggestion[]>([]);
	let analyzing = $state(false);
	let suggestionsChecked = $state(false);

	// Sync when initialData changes
	let initialDataLoaded = false;
	$effect(() => {
		if (initialData && !initialDataLoaded) {
			initialDataLoaded = true;
			name = initialData.name || '';
			description = initialData.description || '';
			tpPercentage =
				initialData.tp_percentage !== undefined ? String(initialData.tp_percentage) : '10';
			slPercentage =
				initialData.sl_percentage !== undefined ? String(initialData.sl_percentage) : '5';
			maxHoldingPeriodDays =
				initialData.max_holding_period_days !== undefined
					? String(initialData.max_holding_period_days)
					: '30';
			if (initialData.rules && initialData.rules.length > 0) {
				rules = JSON.parse(JSON.stringify(initialData.rules));
			}
		}
	});

	// Computed Risk-Reward Ratio
	const riskRewardRatio = $derived.by(() => {
		const tp = parseFloat(tpPercentage);
		const sl = parseFloat(slPercentage);
		return formatRiskReward(tp, sl);
	});

	async function handleSubmit(e: Event) {
		e.preventDefault();
		const tp = parseFloat(tpPercentage);
		const sl = parseFloat(slPercentage);
		const days = parseInt(maxHoldingPeriodDays, 10);

		const payload: CreateStrategyPayload = {
			name: name.trim(),
			description: description.trim() || null,
			tp_percentage: isNaN(tp) ? 0 : tp,
			sl_percentage: isNaN(sl) ? 0 : sl,
			max_holding_period_days: isNaN(days) ? 1 : days,
			rules
		};

		if (enableAiSuggestions && !suggestionsChecked) {
			analyzing = true;
			try {
				const token = getToken();
				const result = await getStrategyAiSuggestions(token, {
					name: payload.name,
					description: payload.description,
					tp_percentage: payload.tp_percentage,
					sl_percentage: payload.sl_percentage,
					max_holding_period_days: payload.max_holding_period_days,
					rules: payload.rules
				});

				if (result && result.length > 0) {
					suggestions = result;
					suggestionsChecked = true;
					return;
				}
			} catch {
				// Proceed to submit directly if AI request fails
			} finally {
				analyzing = false;
			}
		}

		onsubmit(payload);
	}

	function handleAcceptSuggestion(suggestion: StrategyAiSuggestion) {
		if (suggestion.suggestion_type === 'RULE' && suggestion.rule_payload) {
			const { group_index, condition_index, variable, operator, value, connector_to_next } =
				suggestion.rule_payload;

			// Ensure target group exists
			while (rules.length <= group_index) {
				rules.push({
					id: crypto.randomUUID(),
					connector_to_next: null,
					conditions: []
				});
			}

			const targetGroup = rules[group_index];

			if (
				suggestion.rule_action === 'EDIT_CONDITION' &&
				condition_index !== null &&
				condition_index !== undefined &&
				targetGroup.conditions[condition_index]
			) {
				targetGroup.conditions[condition_index].variable = variable;
				targetGroup.conditions[condition_index].operator = operator;
				targetGroup.conditions[condition_index].value = value;
			} else {
				// ADD_CONDITION
				if (targetGroup.conditions.length > 0) {
					targetGroup.conditions[targetGroup.conditions.length - 1].connector_to_next =
						targetGroup.conditions[targetGroup.conditions.length - 1].connector_to_next || 'AND';
				}
				targetGroup.conditions.push({
					id: crypto.randomUUID(),
					variable,
					operator,
					value,
					connector_to_next: connector_to_next ?? null
				});
			}
			rules = [...rules];
		} else {
			if (
				suggestion.field === 'tp_percentage' &&
				suggestion.suggested_value !== null &&
				suggestion.suggested_value !== undefined
			) {
				tpPercentage = String(suggestion.suggested_value);
			} else if (
				suggestion.field === 'sl_percentage' &&
				suggestion.suggested_value !== null &&
				suggestion.suggested_value !== undefined
			) {
				slPercentage = String(suggestion.suggested_value);
			} else if (
				suggestion.field === 'max_holding_period_days' &&
				suggestion.suggested_value !== null &&
				suggestion.suggested_value !== undefined
			) {
				maxHoldingPeriodDays = String(Math.round(suggestion.suggested_value));
			}
		}
		suggestions = suggestions.filter((s) => s.id !== suggestion.id);
	}

	function handleIgnoreSuggestion(suggestion: StrategyAiSuggestion) {
		suggestions = suggestions.filter((s) => s.id !== suggestion.id);
	}
</script>

<form onsubmit={handleSubmit} class="flex flex-col gap-6">
	<!-- Top Section: Core Parameters Card -->
	<div
		class="flex flex-col rounded-2xl border p-3.5 sm:p-6"
		style="background-color: var(--bg-card); border-color: var(--border);"
	>
		<div class="mb-5 flex items-center gap-2.5">
			<div
				class="flex size-9 shrink-0 items-center justify-center rounded-xl"
				style="background-color: var(--accent-soft); color: var(--accent);"
			>
				<Icon icon="lucide:settings-2" width="18" height="18" />
			</div>
			<div>
				<h2 class="font-700 text-sm sm:text-base" style="color: var(--fg)">General Parameters</h2>
				<p class="text-xs" style="color: var(--fg-muted)">
					Define the identity, targets, and risk parameters of your strategy
				</p>
			</div>
		</div>

		<!-- Name & Description -->
		<div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
			<div class="sm:col-span-2">
				<TextField
					label="Strategy Name"
					type="text"
					placeholder="e.g. Undervalued stocks"
					bind:value={name}
					required
				/>
			</div>

			<div class="flex flex-col gap-1.5 sm:col-span-2">
				<label for="strat-desc" class="font-500 text-sm" style="color: var(--fg-muted)">
					Description (Optional)
				</label>
				<textarea
					id="strat-desc"
					rows="2"
					placeholder="Briefly describe the rationale or objective of this strategy..."
					bind:value={description}
					class="w-full rounded-lg border px-3.5 py-2.5 text-sm transition-all duration-150 outline-none"
					style="
						background-color: var(--bg-input, var(--bg));
						border-color: var(--border-strong);
						color: var(--fg);
					"></textarea>
			</div>
		</div>

		<!-- Numeric Metrics Row: TP, SL, R:R, Max Holding -->
		<div class="mt-5 grid grid-cols-1 gap-3 sm:mt-6 sm:grid-cols-2 sm:gap-4 lg:grid-cols-4">
			<!-- Take Profit -->
			<div class="flex flex-col gap-1.5">
				<label for="tp-input" class="font-500 text-sm" style="color: var(--fg-muted)">
					Take Profit (%) <span style="color: var(--danger)">*</span>
				</label>
				<div class="relative">
					<input
						id="tp-input"
						type="number"
						step="any"
						min="0.01"
						placeholder="10.0"
						bind:value={tpPercentage}
						required
						class="font-600 w-full rounded-xl border py-2.5 pr-8 pl-3.5 text-sm transition-colors duration-150 outline-none"
						style="
							background-color: var(--bg-input, var(--bg));
							border-color: var(--border-strong);
							color: var(--success);
						"
					/>
					<span
						class="font-600 pointer-events-none absolute inset-y-0 right-0 flex items-center pr-3 text-xs"
						style="color: var(--fg-muted);"
					>
						%
					</span>
				</div>
			</div>

			<!-- Stop Loss -->
			<div class="flex flex-col gap-1.5">
				<label for="sl-input" class="font-500 text-sm" style="color: var(--fg-muted)">
					Stop Loss (%) <span style="color: var(--danger)">*</span>
				</label>
				<div class="relative">
					<input
						id="sl-input"
						type="number"
						step="any"
						min="0.01"
						placeholder="5.0"
						bind:value={slPercentage}
						required
						class="font-600 w-full rounded-xl border py-2.5 pr-8 pl-3.5 text-sm transition-colors duration-150 outline-none"
						style="
							background-color: var(--bg-input, var(--bg));
							border-color: var(--border-strong);
							color: var(--danger);
						"
					/>
					<span
						class="font-600 pointer-events-none absolute inset-y-0 right-0 flex items-center pr-3 text-xs"
						style="color: var(--fg-muted);"
					>
						%
					</span>
				</div>
			</div>

			<!-- Live Risk Reward Ratio Display -->
			<div class="flex flex-col gap-1.5">
				<span class="font-500 text-sm" style="color: var(--fg-muted)"> Risk : Reward Ratio </span>
				<div
					class="flex h-10.5 items-center justify-between rounded-xl border px-3.5 shadow-xs"
					style="
						background-color: var(--accent-soft);
						border-color: rgba(48, 180, 201, 0.3);
					"
				>
					<span class="font-600 text-xs tracking-wider uppercase" style="color: var(--fg-muted)">
						Ratio
					</span>
					<span class="font-700 text-sm" style="color: var(--accent)">
						{riskRewardRatio}
					</span>
				</div>
			</div>

			<!-- Max Holding Period -->
			<div class="flex flex-col gap-1.5">
				<label for="holding-input" class="font-500 text-sm" style="color: var(--fg-muted)">
					Max Holding (Day) <span style="color: var(--danger)">*</span>
				</label>
				<div class="relative">
					<input
						id="holding-input"
						type="number"
						step="1"
						min="1"
						max="3650"
						placeholder="30"
						bind:value={maxHoldingPeriodDays}
						required
						class="font-600 w-full rounded-xl border py-2.5 pr-14 pl-3.5 text-sm transition-colors duration-150 outline-none"
						style="
							background-color: var(--bg-input, var(--bg));
							border-color: var(--border-strong);
							color: var(--fg);
						"
					/>
					<span
						class="font-500 pointer-events-none absolute inset-y-0 right-0 flex items-center pr-3 text-xs"
						style="color: var(--fg-muted);"
					>
						Days
					</span>
				</div>
			</div>
		</div>
	</div>

	<!-- Bottom Section: Where Conditions Builder -->
	<WhereConditionsBuilder bind:groups={rules} />

	<!-- Error Alert -->
	{#if error}
		<div
			class="font-500 rounded-xl border p-4 text-sm"
			style="background-color: rgba(239,68,68,0.08); border-color: rgba(239,68,68,0.3); color: var(--danger);"
		>
			<div class="flex items-center gap-2">
				<Icon icon="lucide:alert-circle" width="16" height="16" />
				<span>{error}</span>
			</div>
		</div>
	{/if}

	<!-- Floating AI Suggestions (above action buttons) -->
	{#if suggestions.length > 0}
		<StrategyAiSuggestionsCard
			{suggestions}
			onaccept={handleAcceptSuggestion}
			onignore={handleIgnoreSuggestion}
		/>
	{/if}

	<!-- Form Action Bar (Not floating) -->
	<div
		class="flex flex-col-reverse gap-2.5 border-t pt-5 sm:flex-row sm:items-center sm:justify-end sm:gap-3"
		style="border-color: var(--border);"
	>
		<button
			type="button"
			onclick={oncancel}
			class="btn-interactive font-600 w-full rounded-xl border px-5 py-2.5 text-sm transition-colors duration-150 hover:bg-(--bg-card-hover) sm:w-auto"
			style="border-color: var(--border-strong); color: var(--fg-muted);"
		>
			Cancel
		</button>
		<button
			type="submit"
			disabled={loading || analyzing || !name.trim() || suggestions.length > 0}
			class="btn-interactive font-600 inline-flex w-full items-center justify-center gap-2 rounded-xl px-6 py-2.5 text-sm text-white shadow-md transition-all duration-150 disabled:cursor-not-allowed disabled:opacity-50 sm:w-auto"
			style="background-color: var(--accent);"
		>
			{#if analyzing}
				<Icon icon="lucide:loader-2" class="animate-spin" width="18" height="18" />
				<span>Analyzing Strategy…</span>
			{:else if loading}
				<Icon icon="lucide:loader-2" class="animate-spin" width="18" height="18" />
				<span>Saving…</span>
			{:else}
				<Icon icon="lucide:check" width="18" height="18" />
				<span>{submitLabel}</span>
			{/if}
		</button>
	</div>
</form>
