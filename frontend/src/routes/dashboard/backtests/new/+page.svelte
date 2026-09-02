<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import Icon from '@iconify/svelte';
	import SelectField from '$lib/components/SelectField.svelte';
	import SegmentedControl from '$lib/components/SegmentedControl.svelte';
	import TextField from '$lib/components/TextField.svelte';
	import { listTradingStrategies, createBacktest, ApiError } from '$lib/api';
	import { getToken } from '$lib/helpers/session';
	import { toast } from '$lib/helpers/toast.svelte';
	import { calculateBacktestDateRange } from '$lib/helpers/date';
	import type { TradingStrategy, CreateBacktestPayload } from '$lib/types';

	let strategies = $state<TradingStrategy[]>([]);
	let loadingStrats = $state(true);
	let loading = $state(false);

	let formStrategyId = $state('');
	let formYear = $state(2025);
	let formName = $state('');
	let namePristine = $state(true);
	let formInitialCash = $state('100000000');
	let formMaxHoldingStocks = $state('4');
	let formDuration = $state('12');
	let formBuyFee = $state('0.15');
	let formSellFee = $state('0.25');
	let isPublic = $state(false);

	const yearOptions = [
		{ value: 2021, label: '2021' },
		{ value: 2022, label: '2022' },
		{ value: 2023, label: '2023' },
		{ value: 2024, label: '2024' },
		{ value: 2025, label: '2025' }
	];

	const durationOptions = [
		{ value: '1', label: '1 Month' },
		{ value: '3', label: '3 Months' },
		{ value: '6', label: '6 Months' },
		{ value: '12', label: '1 Year' }
	];

	onMount(async () => {
		try {
			const token = getToken();
			if (!token) {
				goto('/login');
				return;
			}
			strategies = await listTradingStrategies(token);
			if (strategies.length > 0) {
				formStrategyId = strategies[0].id;
			}
		} catch {
			toast.error('Failed to load strategies.');
		} finally {
			loadingStrats = false;
		}
	});

	$effect(() => {
		if (namePristine && formStrategyId) {
			const strat = strategies.find((s) => s.id === formStrategyId);
			if (strat) {
				formName = `${strat.name} - ${formYear}`;
			}
		}
	});

	function handleNameInput() {
		namePristine = false;
	}

	function formatRupiah(val: number): string {
		return new Intl.NumberFormat('id-ID').format(val || 0);
	}

	const backtestDateRange = $derived(
		calculateBacktestDateRange(formYear, Number(formDuration)).formatted
	);

	async function handleSubmit(e: Event) {
		e.preventDefault();
		if (!formStrategyId) {
			toast.error('Please select a trading strategy.');
			return;
		}

		const initialCashNum = Number(formInitialCash);
		if (isNaN(initialCashNum) || initialCashNum < 1_000_000 || initialCashNum > 100_000_000_000) {
			toast.error('Initial cash must be between Rp 1.000.000 and Rp 100.000.000.000.');
			return;
		}

		const maxHoldingNum = Number(formMaxHoldingStocks);
		if (isNaN(maxHoldingNum) || maxHoldingNum < 1 || maxHoldingNum > 50) {
			toast.error('Max holding stocks must be between 1 and 50.');
			return;
		}

		const buyFeeNum = Number(formBuyFee);
		if (isNaN(buyFeeNum) || buyFeeNum < 0 || buyFeeNum > 10) {
			toast.error('Buy fee must be between 0% and 10%.');
			return;
		}

		const sellFeeNum = Number(formSellFee);
		if (isNaN(sellFeeNum) || sellFeeNum < 0 || sellFeeNum > 10) {
			toast.error('Sell fee must be between 0% and 10%.');
			return;
		}

		loading = true;
		try {
			const token = getToken();
			if (!token) return goto('/login');

			const payload: CreateBacktestPayload = {
				strategy_id: formStrategyId,
				name: formName.trim(),
				year: formYear,
				initial_cash: initialCashNum,
				max_holding_stocks: maxHoldingNum,
				backtest_duration_months: Number(formDuration),
				buy_fee_percentage: buyFeeNum,
				sell_fee_percentage: sellFeeNum,
				is_public: isPublic
			};

			await createBacktest(token, payload);
			toast.success('Backtest job queued successfully.');
			goto('/dashboard/backtests');
		} catch (err) {
			toast.error(err instanceof ApiError ? err.message : 'Failed to run backtest.');
		} finally {
			loading = false;
		}
	}
</script>

<div class="mb-4">
	<a
		href="/dashboard/backtests"
		class="btn-interactive font-600 inline-flex items-center gap-1.5 text-xs transition-colors duration-150"
		style="color: var(--fg-muted);"
	>
		<Icon icon="lucide:arrow-left" width="14" height="14" />
		<span>Back to Backtests</span>
	</a>
</div>

<div class="mb-5 sm:mb-6">
	<h1 class="font-700 text-xl sm:text-2xl" style="color: var(--fg)">Run Backtest</h1>
	<p class="mt-0.5 text-xs sm:text-sm" style="color: var(--fg-muted)">
		Configure parameters and run your strategy against historical data.
	</p>
</div>

{#if loadingStrats}
	<div class="flex min-h-40 items-center justify-center">
		<Icon icon="lucide:loader-2" class="text-accent animate-spin" width="32" height="32" />
	</div>
{:else}
	<div
		class="rounded-xl border p-4 sm:p-6"
		style="background-color: var(--bg-card); border-color: var(--border);"
	>
		<form onsubmit={handleSubmit} class="flex flex-col gap-5">
			<SelectField
				label="Trading Strategy"
				bind:value={formStrategyId}
				options={strategies.map((s) => ({ value: s.id, label: s.name }))}
				required
			/>

			<div class="flex flex-col gap-4 sm:flex-row sm:items-start sm:justify-between sm:gap-6">
				<div class="flex flex-1 flex-col gap-1.5">
					<span class="font-500 text-sm" style="color: var(--fg-muted)">Backtest Data Year</span>
					<SegmentedControl options={yearOptions} bind:value={formYear} />
				</div>

				<div class="flex shrink-0 flex-col gap-1.5">
					<span class="font-500 text-sm" style="color: var(--fg-muted)">Visibility</span>
					<SegmentedControl
						class="w-fit"
						options={[
							{ value: false, label: 'Private', icon: 'lucide:lock' },
							{ value: true, label: 'Public', icon: 'lucide:globe' }
						]}
						bind:value={isPublic}
					/>
				</div>
			</div>

			<div oninput={handleNameInput}>
				<TextField label="Backtest Name" bind:value={formName} required />
			</div>

			<div class="grid grid-cols-1 gap-5 sm:grid-cols-2">
				<div class="flex flex-col gap-1">
					<TextField
						label="Initial Cash"
						type="number"
						bind:value={formInitialCash}
						min="1000000"
						max="100000000000"
						step="any"
						required
					/>
					<span class="font-500 text-xs" style="color: var(--fg-muted)">
						Rp {formatRupiah(Number(formInitialCash))}
					</span>
				</div>

				<div class="flex flex-col gap-1">
					<TextField
						label="Max Holding Stocks"
						type="number"
						bind:value={formMaxHoldingStocks}
						min="1"
						max="50"
						step="1"
						required
					/>
					<span class="font-500 text-xs" style="color: var(--fg-muted)">
						Max Buy Value: Rp {formatRupiah(
							Number(formInitialCash) / (Number(formMaxHoldingStocks) || 1)
						)} ({(100 / (Number(formMaxHoldingStocks) || 1)).toFixed(1)}% per stock)
					</span>
				</div>
			</div>

			<div class="grid grid-cols-1 gap-5 sm:grid-cols-2">
				<SelectField
					label="Backtest Duration"
					bind:value={formDuration}
					options={durationOptions}
					required
				/>

				<div class="flex flex-col gap-1.5">
					<span class="font-500 text-sm" style="color: var(--fg-muted)">Backtest Date</span>
					<div class="font-500 flex h-[42px] items-center gap-2 text-sm" style="color: var(--fg)">
						<Icon icon="lucide:calendar" width="16" height="16" style="color: var(--fg-muted)" />
						<span>{backtestDateRange}</span>
					</div>
				</div>
			</div>

			<div class="grid grid-cols-1 gap-5 sm:grid-cols-2">
				<TextField
					label="Buy Fee (%)"
					type="number"
					bind:value={formBuyFee}
					min="0"
					max="10"
					step="any"
					required
				/>
				<TextField
					label="Sell Fee (%)"
					type="number"
					bind:value={formSellFee}
					min="0"
					max="10"
					step="any"
					required
				/>
			</div>

			<div
				class="mt-4 flex items-center justify-end gap-3 border-t pt-4"
				style="border-color: var(--border);"
			>
				<button
					type="button"
					onclick={() => goto('/dashboard/backtests')}
					class="btn-interactive font-500 rounded-lg border px-5 py-2.5 text-sm transition-colors"
					style="border-color: var(--border); color: var(--fg-muted);"
				>
					Cancel
				</button>
				<button
					type="submit"
					disabled={loading}
					class="btn-interactive font-600 inline-flex items-center gap-2 rounded-lg px-5 py-2.5 text-sm text-white transition-opacity"
					style="background-color: var(--accent); opacity: {loading ? '0.7' : '1'};"
				>
					{#if loading}
						<Icon icon="lucide:loader-2" class="animate-spin" width="16" height="16" />
					{:else}
						<Icon icon="lucide:play" width="16" height="16" />
					{/if}
					<span>Run Backtest</span>
				</button>
			</div>
		</form>
	</div>
{/if}
