<script lang="ts">
	import Icon from '@iconify/svelte';
	import {
		STRATEGY_OPERATORS,
		STRATEGY_VARIABLES,
		type StrategyVariableOption
	} from '$lib/constants';
	import type { StrategyRuleCondition, StrategyRuleGroup } from '$lib/types';
	import SelectField from '$lib/components/SelectField.svelte';
	import VariablePickerModal from './VariablePickerModal.svelte';

	let {
		groups = $bindable<StrategyRuleGroup[]>([])
	}: {
		groups?: StrategyRuleGroup[];
	} = $props();

	const operatorOptions = STRATEGY_OPERATORS.map((op) => ({
		value: op.value,
		label: op.display
	}));

	const isValueOptions = [
		{ value: 'null', label: 'null' },
		{ value: 'not null', label: 'not null' },
		{ value: 'true', label: 'true' },
		{ value: 'false', label: 'false' }
	];

	// Modal state for variable picker
	let modalOpen = $state(false);
	let activeTarget = $state<{ groupIndex: number; conditionIndex: number } | null>(null);

	// Helper to find variable meta
	function getVariable(code: string): StrategyVariableOption | undefined {
		return STRATEGY_VARIABLES.find((v) => v.code === code);
	}

	function openVariableModal(groupIndex: number, conditionIndex: number) {
		activeTarget = { groupIndex, conditionIndex };
		modalOpen = true;
	}

	function handleVariableSelect(selectedCode: string) {
		if (!activeTarget) return;
		const { groupIndex, conditionIndex } = activeTarget;
		if (groups[groupIndex]?.conditions[conditionIndex]) {
			groups[groupIndex].conditions[conditionIndex].variable = selectedCode;
		}
		activeTarget = null;
	}

	function addCondition(groupIndex: number) {
		const group = groups[groupIndex];
		if (!group) return;

		const newCondition: StrategyRuleCondition = {
			id: crypto.randomUUID(),
			variable: 'price',
			operator: '>',
			value: '0',
			connector_to_next: 'AND'
		};

		// Ensure previous condition has connector
		if (group.conditions.length > 0) {
			group.conditions[group.conditions.length - 1].connector_to_next =
				group.conditions[group.conditions.length - 1].connector_to_next || 'AND';
		}

		group.conditions.push(newCondition);
	}

	function removeCondition(groupIndex: number, conditionIndex: number) {
		const group = groups[groupIndex];
		if (!group) return;

		group.conditions.splice(conditionIndex, 1);

		// If group is now empty and there is more than 1 group, remove group
		if (group.conditions.length === 0 && groups.length > 1) {
			removeGroup(groupIndex);
		} else if (group.conditions.length > 0) {
			// Clear connector of the last condition
			group.conditions[group.conditions.length - 1].connector_to_next = null;
		}
	}

	function toggleConditionConnector(groupIndex: number, conditionIndex: number) {
		const cond = groups[groupIndex]?.conditions[conditionIndex];
		if (cond) {
			cond.connector_to_next = cond.connector_to_next === 'OR' ? 'AND' : 'OR';
		}
	}

	function addGroup() {
		const newGroup: StrategyRuleGroup = {
			id: crypto.randomUUID(),
			connector_to_next: 'AND',
			conditions: [
				{
					id: crypto.randomUUID(),
					variable: 'price',
					operator: '>',
					value: '50',
					connector_to_next: null
				}
			]
		};

		if (groups.length > 0) {
			groups[groups.length - 1].connector_to_next =
				groups[groups.length - 1].connector_to_next || 'AND';
		}

		groups.push(newGroup);
	}

	function removeGroup(groupIndex: number) {
		if (groups.length <= 1) return; // Keep at least one group
		groups.splice(groupIndex, 1);
		if (groups.length > 0) {
			groups[groups.length - 1].connector_to_next = null;
		}
	}

	function toggleGroupConnector(groupIndex: number) {
		const group = groups[groupIndex];
		if (group) {
			group.connector_to_next = group.connector_to_next === 'OR' ? 'AND' : 'OR';
		}
	}
</script>

<div
	class="flex flex-col rounded-2xl border p-4 sm:p-6"
	style="background-color: var(--bg-card); border-color: var(--border);"
>
	<!-- Header Section -->
	<div class="mb-5 flex items-start gap-3">
		<div
			class="font-700 flex size-10 shrink-0 items-center justify-center rounded-xl font-mono text-sm"
			style="background-color: var(--accent-soft); color: var(--accent);"
		>
			<Icon icon="lucide:code-2" width="20" height="20" />
		</div>
		<div>
			<h3 class="font-700 text-base leading-tight" style="color: var(--fg)">Where Conditions</h3>
			<p class="font-400 text-xs sm:text-sm" style="color: var(--fg-muted)">
				Combine rules into groups to filter the universe.
			</p>
		</div>
	</div>

	<!-- Groups List -->
	<div class="flex flex-col gap-4">
		{#each groups as group, groupIdx}
			<!-- Group Container Card -->
			<div
				class="flex flex-col rounded-xl border p-3.5 transition-all duration-150 sm:p-5"
				style="
					background-color: var(--bg-card-hover, var(--bg));
					border-color: var(--border);
				"
			>
				<!-- Group Header -->
				<div class="mb-3.5 flex items-center justify-between">
					<div class="flex items-center gap-2">
						<span class="font-700 font-mono text-xs" style="color: var(--fg-muted)">{'{ }'}</span>
						<span
							class="font-700 font-mono text-xs tracking-wider uppercase"
							style="color: var(--fg-muted)"
						>
							GROUP {groupIdx + 1}
						</span>
					</div>

					{#if groups.length > 1}
						<button
							type="button"
							onclick={() => removeGroup(groupIdx)}
							class="btn-interactive flex size-7 items-center justify-center rounded-lg transition-colors duration-150 hover:bg-red-50 hover:text-red-500 dark:hover:bg-red-950"
							style="color: var(--fg-muted);"
							title="Delete Group {groupIdx + 1}"
							aria-label="Delete Group {groupIdx + 1}"
						>
							<Icon icon="lucide:trash-2" width="14" height="14" />
						</button>
					{/if}
				</div>

				<!-- Group Conditions -->
				<div class="flex flex-col gap-3">
					{#each group.conditions as condition, condIdx}
						{@const varMeta = getVariable(condition.variable)}
						<!-- Condition Row -->
						<div
							class="flex flex-col gap-2 rounded-xl border p-2.5 sm:flex-row sm:items-center sm:gap-2.5 sm:border-0 sm:p-0"
							style="border-color: var(--border);"
						>
							<!-- 1. Variable Selector Button (Full width on mobile, auto width on desktop) -->
							<button
								type="button"
								onclick={() => openVariableModal(groupIdx, condIdx)}
								class="btn-interactive font-500 flex h-10 w-full min-w-0 items-center justify-between gap-2 rounded-xl border px-3 text-left text-sm transition-colors duration-150 sm:w-auto sm:max-w-xs sm:flex-1 md:max-w-sm"
								style="
									background-color: var(--bg-card);
									border-color: var(--border-strong);
									color: var(--fg);
								"
							>
								<div class="flex min-w-0 flex-1 items-center gap-2">
									<Icon
										icon="lucide:activity"
										class="shrink-0"
										style="color: var(--accent);"
										width="16"
										height="16"
									/>
									<span class="font-500 min-w-0 truncate">
										{varMeta ? varMeta.name : condition.variable || 'Select variable'}
									</span>
									{#if varMeta}
										<span
											class="font-700 shrink-0 rounded px-1.5 py-0.5 font-mono text-[11px]"
											style="background-color: var(--accent-soft); color: var(--accent);"
										>
											{varMeta.code}
										</span>
									{/if}
								</div>
								<Icon
									icon="lucide:chevron-down"
									class="ml-1.5 shrink-0"
									style="color: var(--fg-muted);"
									width="16"
									height="16"
								/>
							</button>

							<!-- Operator + Value Input + Remove Button Row on Mobile (Inline on Desktop) -->
							<div class="flex w-full min-w-0 flex-1 items-center gap-2 sm:w-auto">
								<!-- 2. Operator Selector Dropdown -->
								<div class="min-w-0 flex-1 sm:w-48 sm:flex-initial lg:w-56">
									<SelectField
										value={condition.operator}
										options={operatorOptions}
										placeholder="Operator"
										buttonClass="h-10 text-xs sm:text-sm rounded-xl"
										buttonStyle="background-color: var(--bg-card);"
										onchange={(val) => {
											condition.operator = val;
											if (
												val === 'is' &&
												!['null', 'not null', 'true', 'false'].includes(condition.value)
											) {
												condition.value = 'null';
											}
										}}
									/>
								</div>

								<!-- 3. Value Input (Adapts based on operator) -->
								<div class="flex min-w-0 flex-1 items-center">
									{#if condition.operator === 'is'}
										<!-- Selector for null / true / false -->
										<div class="w-full min-w-0">
											<SelectField
												value={condition.value}
												options={isValueOptions}
												placeholder="Select value"
												buttonClass="h-10 text-xs sm:text-sm rounded-xl"
												buttonStyle="background-color: var(--bg-card);"
												onchange={(val) => {
													condition.value = val;
												}}
											/>
										</div>
									{:else if condition.operator === 'in'}
										<input
											type="text"
											placeholder="e.g. BBCA, BBRI"
											value={condition.value}
											oninput={(e) => (condition.value = e.currentTarget.value)}
											class="h-10 w-full min-w-0 rounded-xl border px-3 text-xs transition-colors duration-150 outline-none sm:text-sm"
											style="
												background-color: var(--bg-card);
												border-color: var(--border-strong);
												color: var(--fg);
											"
										/>
									{:else if condition.operator === '~~'}
										<input
											type="text"
											placeholder="e.g. %bank%"
											value={condition.value}
											oninput={(e) => (condition.value = e.currentTarget.value)}
											class="h-10 w-full min-w-0 rounded-xl border px-3 text-xs transition-colors duration-150 outline-none sm:text-sm"
											style="
												background-color: var(--bg-card);
												border-color: var(--border-strong);
												color: var(--fg);
											"
										/>
									{:else}
										<input
											type="text"
											placeholder="0"
											value={condition.value}
											oninput={(e) => (condition.value = e.currentTarget.value)}
											class="h-10 w-full min-w-0 rounded-xl border px-3 text-xs transition-colors duration-150 outline-none sm:text-sm"
											style="
												background-color: var(--bg-card);
												border-color: var(--border-strong);
												color: var(--fg);
											"
										/>
									{/if}
								</div>

								<!-- 4. Remove Condition Button -->
								<button
									type="button"
									onclick={() => removeCondition(groupIdx, condIdx)}
									class="btn-interactive flex size-10 shrink-0 items-center justify-center rounded-xl border transition-colors duration-150 hover:bg-(--bg-card) hover:text-red-500 sm:size-9 sm:rounded-lg sm:border-0"
									style="border-color: var(--border-strong); color: var(--fg-muted);"
									title="Remove condition"
									aria-label="Remove condition"
								>
									<Icon icon="lucide:x" width="16" height="16" />
								</button>
							</div>
						</div>

						<!-- Connector pill between conditions inside group -->
						{#if condIdx < group.conditions.length - 1}
							<div class="my-1 flex items-center gap-3">
								<div
									class="relative inline-flex rounded-lg border p-0.5 shadow-xs select-none"
									style="background-color: var(--bg-card); border-color: var(--border-strong);"
								>
									<!-- Sliding Pill Indicator -->
									<div
										class="pointer-events-none absolute rounded-md"
										style="
											top: 2px;
											bottom: 2px;
											left: 2px;
											width: calc((100% - 4px) / 2);
											transform: translate3d(calc({condition.connector_to_next === 'OR' ? 1 : 0} * 100%), 0, 0);
											transition: transform 0.25s cubic-bezier(0.32, 0.72, 0, 1);
											will-change: transform;
											background-color: var(--accent);
										"
									></div>

									<button
										type="button"
										onclick={() => toggleConditionConnector(groupIdx, condIdx)}
										class="btn-interactive font-700 relative z-10 rounded-md px-2.5 py-0.5 text-xs transition-colors duration-150"
										style="
											color: {condition.connector_to_next === 'AND' || !condition.connector_to_next
											? '#ffffff'
											: 'var(--fg-muted)'};
										"
									>
										AND
									</button>
									<button
										type="button"
										onclick={() => toggleConditionConnector(groupIdx, condIdx)}
										class="btn-interactive font-700 relative z-10 rounded-md px-2.5 py-0.5 text-xs transition-colors duration-150"
										style="
											color: {condition.connector_to_next === 'OR' ? '#ffffff' : 'var(--fg-muted)'};
										"
									>
										OR
									</button>
								</div>
								<div class="h-px flex-1" style="background-color: var(--border);"></div>
							</div>
						{/if}
					{/each}

					<!-- Add Condition Button -->
					<div class="pt-2">
						<button
							type="button"
							onclick={() => addCondition(groupIdx)}
							class="btn-interactive font-600 inline-flex items-center gap-1.5 rounded-lg px-2.5 py-1.5 text-xs transition-colors duration-150 hover:bg-(--bg-card)"
							style="color: var(--accent);"
						>
							<Icon icon="lucide:plus" width="14" height="14" />
							<span>Add condition</span>
						</button>
					</div>
				</div>
			</div>

			<!-- Connector between groups -->
			{#if groupIdx < groups.length - 1}
				<div class="my-1 flex items-center justify-center gap-3">
					<div class="h-px flex-1" style="background-color: var(--border-strong);"></div>
					<div
						class="relative inline-flex rounded-lg border p-0.5 shadow-xs select-none"
						style="background-color: var(--bg-card); border-color: var(--border-strong);"
					>
						<!-- Sliding Pill Indicator -->
						<div
							class="pointer-events-none absolute rounded-md"
							style="
								top: 2px;
								bottom: 2px;
								left: 2px;
								width: calc((100% - 4px) / 2);
								transform: translate3d(calc({group.connector_to_next === 'OR' ? 1 : 0} * 100%), 0, 0);
								transition: transform 0.25s cubic-bezier(0.32, 0.72, 0, 1);
								will-change: transform;
								background-color: var(--accent);
							"
						></div>

						<button
							type="button"
							onclick={() => toggleGroupConnector(groupIdx)}
							class="btn-interactive font-700 relative z-10 rounded-md px-3 py-1 text-xs transition-colors duration-150"
							style="
								color: {group.connector_to_next === 'AND' || !group.connector_to_next
								? '#ffffff'
								: 'var(--fg-muted)'};
							"
						>
							AND
						</button>
						<button
							type="button"
							onclick={() => toggleGroupConnector(groupIdx)}
							class="btn-interactive font-700 relative z-10 rounded-md px-3 py-1 text-xs transition-colors duration-150"
							style="
								color: {group.connector_to_next === 'OR' ? '#ffffff' : 'var(--fg-muted)'};
							"
						>
							OR
						</button>
					</div>
					<div class="h-px flex-1" style="background-color: var(--border-strong);"></div>
				</div>
			{/if}
		{/each}

		<!-- Add Group Button (Dashed box) -->
		<button
			type="button"
			onclick={addGroup}
			class="btn-interactive font-600 flex w-full items-center justify-center gap-2 rounded-xl border-2 border-dashed py-3.5 text-sm transition-colors duration-150 hover:bg-(--bg-card-hover)"
			style="border-color: var(--border-strong); color: var(--fg);"
		>
			<Icon icon="lucide:plus" width="16" height="16" />
			<span>Add Group</span>
		</button>
	</div>
</div>

<!-- Variable Picker Modal -->
<VariablePickerModal
	bind:open={modalOpen}
	selectedVariable={activeTarget
		? groups[activeTarget.groupIndex]?.conditions[activeTarget.conditionIndex]?.variable || ''
		: ''}
	onselect={handleVariableSelect}
/>
