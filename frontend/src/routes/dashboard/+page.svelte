<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import Icon from '@iconify/svelte';
	import StatsCard from '$lib/components/dashboard/StatsCard.svelte';
	import LeaderboardCard from '$lib/components/dashboard/LeaderboardCard.svelte';
	import SegmentedControl from '$lib/components/SegmentedControl.svelte';
	import {
		getDashboardStats,
		getLeaderboard,
		getTopStars,
		starBacktest,
		unstarBacktest,
		ApiError
	} from '$lib/api';
	import { getToken } from '$lib/helpers/session';
	import { toast } from '$lib/helpers/toast.svelte';
	import type { DashboardStats, LeaderboardEntry } from '$lib/types';

	let loading = $state(true);
	let stats = $state<DashboardStats | null>(null);
	let leaderboard = $state<LeaderboardEntry[]>([]);
	let topStars = $state<LeaderboardEntry[]>([]);

	let activeTab = $state<'LEADERBOARD' | 'TOP_STARS'>('LEADERBOARD');

	const tabOptions = [
		{ value: 'LEADERBOARD', label: 'Leaderboard' },
		{ value: 'TOP_STARS', label: 'Top Stars' }
	];

	async function loadDashboard() {
		const token = getToken();
		if (!token) {
			goto('/login');
			return;
		}

		try {
			const [statsRes, leaderboardRes, topStarsRes] = await Promise.all([
				getDashboardStats(token),
				getLeaderboard(token),
				getTopStars(token)
			]);

			stats = statsRes;
			leaderboard = leaderboardRes;
			topStars = topStarsRes;
		} catch (error) {
			if (error instanceof ApiError && error.status === 401) {
				goto('/login');
			} else {
				toast.error(error instanceof Error ? error.message : 'Failed to load dashboard');
			}
		} finally {
			loading = false;
		}
	}

	async function handleStarToggle(id: string, starred: boolean) {
		const token = getToken();
		if (!token) return;

		// Optimistic update function
		const updateEntry = (entry: LeaderboardEntry) => {
			if (entry.id !== id) return entry;
			return {
				...entry,
				is_starred_by_me: starred,
				star_count: entry.star_count + (starred ? 1 : -1)
			};
		};

		// Apply optimistic update
		const prevLeaderboard = [...leaderboard];
		const prevTopStars = [...topStars];
		leaderboard = leaderboard.map(updateEntry);
		topStars = topStars.map(updateEntry);

		try {
			if (starred) {
				await starBacktest(token, id);
			} else {
				await unstarBacktest(token, id);
			}
		} catch {
			// Revert optimistic update
			leaderboard = prevLeaderboard;
			topStars = prevTopStars;
			toast.error('Failed to update star rating');
		}
	}

	onMount(() => {
		loadDashboard();
	});

	let activeList = $derived(activeTab === 'LEADERBOARD' ? leaderboard : topStars);

	function formatTimeJoined(dateStr: string): string {
		const d = new Date(dateStr);
		if (isNaN(d.getTime())) return '18.00';
		const hours = String(d.getHours()).padStart(2, '0');
		const minutes = String(d.getMinutes()).padStart(2, '0');
		return `${hours}.${minutes}`;
	}

	function formatDateJoined(dateStr: string): string {
		const d = new Date(dateStr);
		if (isNaN(d.getTime())) return dateStr;
		const weekday = d.toLocaleDateString('en-US', { weekday: 'short' });
		const day = d.toLocaleDateString('en-US', { day: 'numeric' });
		const month = d.toLocaleDateString('en-US', { month: 'short' });
		const year = d.toLocaleDateString('en-US', { year: 'numeric' });
		return `${weekday}, ${day} ${month} ${year}`;
	}
</script>

<div class="mx-auto max-w-5xl space-y-8 pb-12">
	<div class="flex flex-col gap-1">
		<h1 class="text-2xl font-bold" style="color: var(--fg)">Dashboard</h1>
		<p class="text-sm" style="color: var(--fg-muted)">
			Overview of your activity and top community backtests.
		</p>
	</div>

	{#if loading}
		<div class="flex h-64 items-center justify-center">
			<Icon icon="lucide:loader-2" class="size-8 animate-spin" style="color: var(--accent)" />
		</div>
	{:else}
		<!-- Section 1: Header + Stats Cards -->
		{#if stats}
			<div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
				<StatsCard
					icon="lucide:star"
					label="Total Stars"
					value={stats.total_stars_received.toString()}
					orbColor="rgba(245,158,11,0.22)"
					iconFg="text-amber-500 dark:text-amber-400"
					iconBg="bg-amber-500/15 ring-1 ring-inset ring-amber-500/30"
					border="border-amber-500/20"
					hoverBorder="hover:border-amber-500/50 dark:hover:border-amber-400/50"
				/>
				<StatsCard
					icon="lucide:candlestick-chart"
					label="Trading Strategies"
					value={stats.total_strategies.toString()}
					href="/dashboard/strategies"
					orbColor="rgba(48,180,201,0.22)"
					iconFg="text-(--accent)"
					iconBg="bg-(--accent)/15 ring-1 ring-inset ring-(--accent)/30"
					border="border-(--accent)/20"
					hoverBorder="hover:border-(--accent)/60 dark:hover:border-(--accent)/60"
				/>
				<StatsCard
					icon="lucide:flask-conical"
					label="Total Backtests"
					value={stats.total_backtests.toString()}
					href="/dashboard/backtests"
					orbColor="rgba(139,92,246,0.22)"
					iconFg="text-violet-500 dark:text-violet-400"
					iconBg="bg-violet-500/15 ring-1 ring-inset ring-violet-500/30"
					border="border-violet-500/20"
					hoverBorder="hover:border-violet-500/50 dark:hover:border-violet-400/50"
				/>
				<StatsCard
					icon="lucide:calendar-check"
					label="Date Joined"
					value={formatDateJoined(stats.date_joined)}
					subtitle={formatTimeJoined(stats.date_joined)}
					smallValue={true}
					orbColor="rgba(34,197,94,0.22)"
					iconFg="text-emerald-500 dark:text-emerald-400"
					iconBg="bg-emerald-500/15 ring-1 ring-inset ring-emerald-500/30"
					border="border-emerald-500/20"
					hoverBorder="hover:border-emerald-500/50 dark:hover:border-emerald-400/50"
				/>
			</div>
		{/if}

		<!-- Section 2: Community -->
		<div class="space-y-6">
			<div class="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
				<h2 class="text-xl font-bold" style="color: var(--fg)">Community</h2>
				<div class="w-full sm:w-64">
					<SegmentedControl options={tabOptions} bind:value={activeTab} isInsideCard={false} />
				</div>
			</div>

			<div class="flex flex-col gap-4">
				{#if activeList.length === 0}
					<div
						class="flex flex-col items-center justify-center rounded-xl border border-dashed py-16 text-center"
						style="border-color: var(--border-strong); background-color: var(--bg-card)"
					>
						<Icon icon="lucide:inbox" class="mb-4 size-12 opacity-20" style="color: var(--fg)" />
						<p class="text-sm font-medium" style="color: var(--fg)">No public backtests found</p>
						<p class="mt-1 max-w-sm text-xs" style="color: var(--fg-muted)">
							Be the first to share your winning strategy with the community.
						</p>
					</div>
				{:else}
					{#each activeList as entry, i (entry.id)}
						<LeaderboardCard {entry} rank={i + 1} onstar={handleStarToggle} />
					{/each}
				{/if}
			</div>
		</div>
	{/if}
</div>
