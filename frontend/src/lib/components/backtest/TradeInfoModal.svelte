<script lang="ts">
	import Icon from '@iconify/svelte';
	import { SvelteSet } from 'svelte/reactivity';
	import { STRATEGY_VARIABLES, type StrategyVariableOption } from '$lib/constants';
	import type { TradeHistoryEntry, TradingStrategy } from '$lib/types';

	let {
		open = $bindable(false),
		trade = null,
		strategy = null,
		year = 2024
	}: {
		open?: boolean;
		trade?: TradeHistoryEntry | null;
		strategy?: TradingStrategy | null;
		year?: number;
	} = $props();

	let copied = $state(false);

	function close() {
		open = false;
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') close();
	}

	function stripJK(symbol: string): string {
		return symbol.replace(/\.JK$/i, '');
	}

	function formatEntryDate(dateStr: string): string {
		const safeStr = dateStr.includes('T') ? dateStr : `${dateStr}T00:00:00`;
		const d = new Date(safeStr);
		if (isNaN(d.getTime())) return dateStr;
		return d.toLocaleDateString('en-GB', {
			weekday: 'short',
			day: 'numeric',
			month: 'short',
			year: 'numeric'
		});
	}

	function getVariableInfo(code: string): {
		name: string;
		category?: string;
		description?: string;
	} {
		const found = STRATEGY_VARIABLES.find(
			(v: StrategyVariableOption) => v.code.toLowerCase() === code.toLowerCase()
		);
		return found
			? { name: found.name, category: found.category, description: found.description }
			: { name: code };
	}

	function findActualValue(variable: string): { key: string; value: unknown } | null {
		if (!trade?.query_values) return null;
		const qv = trade.query_values;
		const vLower = variable.toLowerCase();

		// 1. Direct match
		if (vLower in qv) return { key: vLower, value: qv[vLower] };
		if (variable in qv) return { key: variable, value: qv[variable] };

		// 2. Price aliases
		if (vLower === 'price') {
			if ('last_close_price' in qv)
				return { key: 'last_close_price', value: qv['last_close_price'] };
			if ('price' in qv) return { key: 'price', value: qv['price'] };
		}

		// 3. Year-indexed e.g. pe[2024]
		const yearKey = `${vLower}[${year}]`;
		if (yearKey in qv) return { key: yearKey, value: qv[yearKey] };

		// 4. Case-insensitive key scan
		for (const k of Object.keys(qv)) {
			const kLower = k.toLowerCase();
			if (kLower === vLower || kLower === yearKey) {
				return { key: k, value: qv[k] };
			}
		}

		// 5. Prefix scan for indexed fields
		for (const k of Object.keys(qv)) {
			if (k.toLowerCase().startsWith(`${vLower}[`)) {
				return { key: k, value: qv[k] };
			}
		}

		return null;
	}

	function formatCompactNumber(value: number): string {
		const abs = Math.abs(value);
		if (abs >= 1e12) return `${(value / 1e12).toFixed(2)} T`;
		if (abs >= 1e9) return `${(value / 1e9).toFixed(2)} B`;
		if (abs >= 1e6) return `${(value / 1e6).toFixed(2)} M`;
		if (abs >= 1e3) return `${(value / 1e3).toFixed(2)} K`;
		return value.toLocaleString('id-ID');
	}

	const CURRENCY_VARS = new Set([
		'market_cap',
		'revenue',
		'cost_of_revenue',
		'gross_profit',
		'operating_expense',
		'operating_pnl',
		'ebit',
		'ebitda',
		'earnings_before_tax',
		'tax',
		'net_income',
		'total_assets',
		'total_liabilities',
		'total_equity',
		'cash_and_equivalents',
		'free_cash_flow',
		'operating_cash_flow',
		'capex',
		'price',
		'last_close_price',
		'value'
	]);

	const RATIO_VARS = new Set([
		'pe',
		'pb',
		'ps',
		'pcf',
		'peg',
		'enterprise_to_ebitda',
		'enterprise_to_revenue',
		'current_ratio',
		'quick_ratio',
		'cash_ratio',
		'dar',
		'der',
		'debt_to_ebitda',
		'asset_turnover',
		'inventory_turnover',
		'pe_peer_avg',
		'pb_peer_avg',
		'ps_peer_avg'
	]);

	const PERCENT_VARS = new Set([
		'roe',
		'roa',
		'roce',
		'npm',
		'gpm',
		'opm',
		'dividend_yield',
		'revenue_growth',
		'net_income_growth',
		'eps_growth'
	]);

	function formatDisplayValue(variable: string, val: unknown): { formatted: string; raw?: string } {
		if (val === null || val === undefined) return { formatted: '-' };
		const vClean = variable.replace(/\[\d+\]$/, '').toLowerCase();

		if (typeof val === 'number') {
			if (CURRENCY_VARS.has(vClean)) {
				if (Math.abs(val) >= 1_000_000) {
					return {
						formatted: `Rp ${formatCompactNumber(val)}`,
						raw: `Rp ${val.toLocaleString('id-ID')}`
					};
				}
				return { formatted: `Rp ${val.toLocaleString('id-ID')}` };
			}
			if (RATIO_VARS.has(vClean)) {
				return { formatted: `${val.toFixed(2)}x` };
			}
			if (PERCENT_VARS.has(vClean)) {
				const pct = Math.abs(val) <= 1.0 ? val * 100 : val;
				return { formatted: `${pct.toFixed(2)}%` };
			}
			if (vClean === 'volume') {
				return {
					formatted: `${formatCompactNumber(val)} shares`,
					raw: `${val.toLocaleString('id-ID')} shares (${Math.round(val / 100).toLocaleString('id-ID')} Lot)`
				};
			}
			if (Math.abs(val) >= 1_000_000) {
				return {
					formatted: formatCompactNumber(val),
					raw: val.toLocaleString('id-ID')
				};
			}
			return { formatted: val.toLocaleString('id-ID', { maximumFractionDigits: 2 }) };
		}
		return { formatted: String(val) };
	}

	function formatTargetRule(variable: string, operator: string, value: string): string {
		const num = parseFloat(value.replace(/['",]/g, ''));
		if (!isNaN(num)) {
			const disp = formatDisplayValue(variable, num);
			return `${operator} ${disp.formatted}`;
		}
		return `${operator} ${value}`;
	}

	function evaluateCondition(actual: unknown, operator: string, targetStr: string): boolean | null {
		if (actual === null || actual === undefined) return null;
		const cleanTarget = targetStr.replace(/['"]/g, '').trim();

		if (typeof actual === 'number') {
			const targetNum = parseFloat(cleanTarget);
			if (!isNaN(targetNum)) {
				switch (operator) {
					case '>':
						return actual > targetNum;
					case '>=':
						return actual >= targetNum;
					case '<':
						return actual < targetNum;
					case '<=':
						return actual <= targetNum;
					case '=':
					case '==':
						return Math.abs(actual - targetNum) < 1e-6;
					case '!=':
						return Math.abs(actual - targetNum) >= 1e-6;
					default:
						return null;
				}
			}
		}

		const actStr = String(actual).toLowerCase();
		const tarStr = cleanTarget.toLowerCase();
		switch (operator) {
			case '=':
			case '==':
				return actStr === tarStr;
			case '!=':
				return actStr !== tarStr;
			case '~~':
			case 'in':
				return actStr.includes(tarStr);
			default:
				return null;
		}
	}

	const unmappedQueryValues = $derived.by(() => {
		if (!trade?.query_values) return [];
		const matchedKeys = new SvelteSet<string>();

		if (strategy?.rules) {
			for (const group of strategy.rules) {
				for (const cond of group.conditions) {
					const res = findActualValue(cond.variable);
					if (res) matchedKeys.add(res.key);
				}
			}
		}

		const unmapped: { key: string; value: unknown }[] = [];
		for (const [key, value] of Object.entries(trade.query_values)) {
			if (!matchedKeys.has(key)) {
				unmapped.push({ key, value });
			}
		}
		return unmapped;
	});

	async function copyJson() {
		if (!trade?.query_values) return;
		await navigator.clipboard.writeText(JSON.stringify(trade.query_values, null, 2));
		copied = true;
		setTimeout(() => {
			copied = false;
		}, 2000);
	}
</script>

<svelte:window onkeydown={handleKeydown} />

{#if open && trade}
	<!-- Backdrop -->
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<div
		class="animate-fade-in fixed inset-0 z-50 flex items-center justify-center p-3 sm:p-4"
		style="background-color: rgba(0,0,0,0.6); backdrop-filter: blur(4px);"
		onclick={(e) => {
			if (e.target === e.currentTarget) close();
		}}
	>
		<!-- Dialog Container -->
		<div
			class="animate-modal relative flex max-h-[calc(100dvh-2rem)] w-full max-w-2xl flex-col overflow-hidden rounded-2xl shadow-2xl"
			style="background-color: var(--bg-card); border: 1px solid var(--border-strong);"
			role="dialog"
			aria-modal="true"
			aria-labelledby="trade-info-title"
		>
			<!-- Header -->
			<div
				class="flex shrink-0 items-center justify-between border-b p-4 sm:px-6 sm:py-4"
				style="border-color: var(--border);"
			>
				<div class="flex items-center gap-3">
					<div
						class="flex size-10 items-center justify-center rounded-xl"
						style="background-color: var(--accent-soft); color: var(--accent);"
					>
						<Icon icon="lucide:sliders-horizontal" width="20" height="20" />
					</div>
					<div>
						<div class="flex items-center gap-2">
							<h2 id="trade-info-title" class="font-800 text-lg" style="color: var(--fg)">
								{stripJK(trade.code)}
							</h2>
							{#if trade.company_name}
								<span
									class="font-500 max-w-64 truncate text-xs sm:max-w-xs"
									style="color: var(--fg-muted)"
								>
									{trade.company_name}
								</span>
							{/if}
						</div>
						<p class="text-xs" style="color: var(--fg-muted)">
							Screening criteria & actual metrics evaluated for entry
						</p>
					</div>
				</div>

				<button
					type="button"
					onclick={close}
					class="btn-interactive rounded-lg p-1.5 transition-all duration-200 hover:rotate-90 hover:bg-(--bg-card-hover)"
					style="color: var(--fg-muted);"
					aria-label="Close modal"
				>
					<Icon icon="lucide:x" width="18" height="18" />
				</button>
			</div>

			<!-- Body -->
			<div class="flex-1 space-y-5 overflow-y-auto p-4 sm:p-6">
				<!-- Trade Context Pill Bar -->
				<div
					class="flex flex-wrap items-center justify-between gap-2 rounded-xl border p-3 text-xs"
					style="background-color: var(--bg-card-hover, var(--bg)); border-color: var(--border);"
				>
					<div class="flex items-center gap-1.5">
						<Icon icon="lucide:calendar" width="14" height="14" style="color: var(--fg-muted);" />
						<span style="color: var(--fg-muted)">Entry Date:</span>
						<span class="font-700" style="color: var(--fg)">{formatEntryDate(trade.buy_date)}</span>
					</div>
					<div class="flex items-center gap-1.5">
						<Icon icon="lucide:coins" width="14" height="14" style="color: var(--fg-muted);" />
						<span style="color: var(--fg-muted)">Entry Price:</span>
						<span class="font-700" style="color: var(--fg)"
							>Rp {trade.buy_price.toLocaleString('id-ID')}</span
						>
					</div>
					<div class="flex items-center gap-1.5">
						<Icon icon="lucide:layers" width="14" height="14" style="color: var(--fg-muted);" />
						<span style="color: var(--fg-muted)">Lots:</span>
						<span class="font-700" style="color: var(--fg)"
							>{trade.lot.toLocaleString('id-ID')}</span
						>
					</div>
					<div class="flex items-center gap-1.5">
						<span
							class="font-700"
							style="color: {trade.pnl >= 0 ? 'var(--success)' : 'var(--danger)'}"
						>
							{trade.pnl >= 0 ? '+' : ''}{trade.pnl_percentage.toFixed(2)}%
						</span>
					</div>
				</div>

				<!-- Strategy Rules Section -->
				{#if strategy?.rules && strategy.rules.length > 0}
					<div class="space-y-4">
						<div class="flex items-center justify-between">
							<h3 class="font-700 text-sm tracking-wide uppercase" style="color: var(--accent);">
								Rules & Entry Conditions
							</h3>
							<span class="text-xs" style="color: var(--fg-muted)">
								Evaluated for {strategy.name}
							</span>
						</div>

						<div class="space-y-4">
							{#each strategy.rules as group, gIdx (group.id || gIdx)}
								{#if gIdx > 0}
									<div class="flex items-center gap-3">
										<div class="h-px flex-1" style="background-color: var(--border);"></div>
										<span
											class="font-800 rounded-md px-2.5 py-0.5 font-mono text-[11px] uppercase shadow-2xs"
											style="background-color: var(--accent-soft); color: var(--accent);"
										>
											{strategy.rules[gIdx - 1]?.connector_to_next || 'AND'}
										</span>
										<div class="h-px flex-1" style="background-color: var(--border);"></div>
									</div>
								{/if}

								<div
									class="space-y-3 rounded-xl border p-3.5 sm:p-4"
									style="background-color: var(--bg-card-2, var(--bg)); border-color: var(--border);"
								>
									<div class="flex items-center justify-between text-xs">
										<span class="font-600 tracking-wider uppercase" style="color: var(--fg-muted)">
											Condition Group #{gIdx + 1}
										</span>
										<span class="text-[11px]" style="color: var(--fg-muted)">
											{group.conditions.length}
											{group.conditions.length === 1 ? 'condition' : 'conditions'}
										</span>
									</div>

									<div class="space-y-2.5">
										{#each group.conditions as condition, cIdx (condition.id || cIdx)}
											{#if cIdx > 0}
												<div class="flex items-center justify-center py-0.5">
													<span
														class="font-700 font-mono text-[10px]"
														style="color: var(--accent);"
													>
														{group.conditions[cIdx - 1]?.connector_to_next || 'AND'}
													</span>
												</div>
											{/if}

											{@const varInfo = getVariableInfo(condition.variable)}
											{@const actualEntry = findActualValue(condition.variable)}
											{@const isManualRule =
												condition.variable.toLowerCase() === 'volume' ||
												condition.variable.toLowerCase() === 'value'}
											{@const actualDisplay = actualEntry
												? formatDisplayValue(condition.variable, actualEntry.value)
												: null}
											{@const matched = actualEntry
												? evaluateCondition(actualEntry.value, condition.operator, condition.value)
												: null}

											<div
												class="rounded-lg border p-3 transition-colors"
												style="background-color: var(--bg-card); border-color: var(--border);"
											>
												<!-- Top: Variable Header & Match Badge -->
												<div
													class="flex items-center justify-between gap-2 border-b pb-2"
													style="border-color: var(--border);"
												>
													<div class="flex flex-wrap items-center gap-1.5">
														<span
															class="font-700 rounded px-2 py-0.5 font-mono text-xs"
															style="background-color: var(--accent-soft); color: var(--accent);"
														>
															{condition.variable}
														</span>
														{#if varInfo.name && varInfo.name !== condition.variable}
															<span class="font-600 text-xs" style="color: var(--fg)">
																{varInfo.name}
															</span>
														{/if}
														{#if varInfo.category}
															<span
																class="rounded-full px-2 py-0.5 text-[10px]"
																style="background-color: var(--bg-card-hover); color: var(--fg-muted);"
															>
																{varInfo.category}
															</span>
														{/if}
													</div>

													<div>
														{#if matched === true}
															<span
																class="font-700 inline-flex items-center gap-1 rounded-full px-2 py-0.5 text-[10px]"
																style="background-color: rgba(16, 185, 129, 0.15); color: var(--success);"
															>
																<Icon icon="lucide:check-circle-2" width="12" height="12" />
																Matched
															</span>
														{:else if matched === false}
															<span
																class="font-700 inline-flex items-center gap-1 rounded-full px-2 py-0.5 text-[10px]"
																style="background-color: rgba(239, 68, 68, 0.15); color: var(--danger);"
															>
																<Icon icon="lucide:x-circle" width="12" height="12" />
																Unmatched
															</span>
														{:else if isManualRule}
															<span
																class="font-600 inline-flex items-center gap-1 rounded-full px-2 py-0.5 text-[10px]"
																style="background-color: var(--bg-card-hover); color: var(--fg-muted);"
															>
																<Icon icon="lucide:clock" width="12" height="12" />
																Daily Candle
															</span>
														{:else}
															<span
																class="font-600 inline-flex items-center gap-1 rounded-full px-2 py-0.5 text-[10px]"
																style="background-color: var(--bg-card-hover); color: var(--fg-muted);"
															>
																Screened
															</span>
														{/if}
													</div>
												</div>

												<!-- Bottom: Strategy Target vs Actual Value -->
												<div class="mt-2.5 grid grid-cols-1 gap-3 text-xs sm:grid-cols-2">
													<!-- Strategy Target -->
													<div>
														<p
															class="font-600 text-[10px] tracking-wide uppercase"
															style="color: var(--fg-muted)"
														>
															Strategy Target Rule
														</p>
														<div class="mt-1 flex items-center gap-1.5">
															<span
																class="font-700 rounded border px-1.5 py-0.5 font-mono text-[11px]"
																style="background-color: var(--bg-card-hover); border-color: var(--border); color: var(--fg);"
															>
																{condition.operator}
															</span>
															<span class="font-700" style="color: var(--fg)">
																{formatTargetRule(condition.variable, '', condition.value).trim()}
															</span>
														</div>
													</div>

													<!-- Actual Screened Value -->
													<div class="sm:border-l sm:pl-3" style="border-color: var(--border);">
														<p
															class="font-600 text-[10px] tracking-wide uppercase"
															style="color: var(--fg-muted)"
														>
															Actual Value at Screening
														</p>
														{#if actualDisplay}
															<div class="mt-1">
																<p class="font-800 text-sm" style="color: var(--accent)">
																	{actualDisplay.formatted}
																</p>
																{#if actualDisplay.raw && actualDisplay.raw !== actualDisplay.formatted}
																	<p
																		class="mt-0.5 font-mono text-[10px]"
																		style="color: var(--fg-muted)"
																	>
																		{actualDisplay.raw}
																	</p>
																{/if}
															</div>
														{:else if isManualRule}
															<p class="mt-1 text-xs italic" style="color: var(--fg-muted)">
																Evaluated on daily entry candle ({formatEntryDate(trade.buy_date)})
															</p>
														{:else}
															<p class="mt-1 text-xs italic" style="color: var(--fg-muted)">
																Not captured in screener payload
															</p>
														{/if}
													</div>
												</div>
											</div>
										{/each}
									</div>
								</div>
							{/each}
						</div>
					</div>
				{:else}
					<!-- Fallback when strategy rules are empty -->
					<div
						class="rounded-xl border p-4 text-center"
						style="background-color: var(--bg-card-2); border-color: var(--border);"
					>
						<p class="text-xs" style="color: var(--fg-muted)">
							No condition groups found for this strategy.
						</p>
					</div>
				{/if}

				<!-- Additional Screened Metrics (if any unmapped) -->
				{#if unmappedQueryValues.length > 0}
					<div
						class="space-y-2.5 rounded-xl border p-4"
						style="background-color: var(--bg-card-2); border-color: var(--border);"
					>
						<h4 class="font-700 text-xs tracking-wider uppercase" style="color: var(--fg-muted)">
							Other Screened Metrics ({unmappedQueryValues.length})
						</h4>
						<div class="grid grid-cols-1 gap-2 sm:grid-cols-2">
							{#each unmappedQueryValues as item (item.key)}
								{@const disp = formatDisplayValue(item.key, item.value)}
								<div
									class="flex items-center justify-between rounded-lg border p-2.5 text-xs"
									style="background-color: var(--bg-card); border-color: var(--border);"
								>
									<span class="font-600 font-mono text-[11px]" style="color: var(--fg-muted)">
										{item.key}
									</span>
									<span class="font-700" style="color: var(--fg)">
										{disp.formatted}
									</span>
								</div>
							{/each}
						</div>
					</div>
				{/if}

				<!-- Collapsible Raw JSON Drawer -->
				<details
					class="group rounded-xl border transition-all"
					style="background-color: var(--bg-card-2); border-color: var(--border);"
				>
					<summary
						class="font-600 flex cursor-pointer items-center justify-between p-3.5 text-xs transition-colors select-none hover:text-(--accent)"
						style="color: var(--fg-muted);"
					>
						<span class="flex items-center gap-2">
							<Icon icon="lucide:braces" width="14" height="14" />
							Raw Screener Data (JSON)
						</span>
						<Icon
							icon="lucide:chevron-down"
							width="14"
							height="14"
							class="transition-transform duration-200 group-open:rotate-180"
						/>
					</summary>
					<div class="space-y-2 border-t p-3 sm:p-4" style="border-color: var(--border);">
						<div class="flex justify-end">
							<button
								type="button"
								onclick={copyJson}
								class="btn-interactive inline-flex items-center gap-1.5 rounded-lg px-2.5 py-1 text-xs transition-colors"
								style="background-color: var(--bg-card-hover); color: var(--fg);"
							>
								<Icon icon={copied ? 'lucide:check' : 'lucide:copy'} width="12" height="12" />
								<span>{copied ? 'Copied!' : 'Copy JSON'}</span>
							</button>
						</div>
						<pre
							class="max-h-56 overflow-auto rounded-lg p-3 font-mono text-[11px] leading-relaxed"
							style="background-color: var(--bg); color: var(--fg); border: 1px solid var(--border);">{JSON.stringify(
								trade.query_values,
								null,
								2
							)}</pre>
					</div>
				</details>
			</div>

			<!-- Footer -->
			<div
				class="flex shrink-0 items-center justify-end border-t p-3 sm:px-6 sm:py-3.5"
				style="border-color: var(--border);"
			>
				<button
					type="button"
					onclick={close}
					class="btn-interactive font-600 cursor-pointer rounded-xl px-4 py-2 text-xs transition-colors"
					style="background-color: var(--bg-card-hover); color: var(--fg);"
				>
					Close
				</button>
			</div>
		</div>
	</div>
{/if}
