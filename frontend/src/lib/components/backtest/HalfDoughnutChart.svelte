<script lang="ts">
	import Icon from '@iconify/svelte';

	let { wins, losses }: { wins: number; losses: number } = $props();

	const ARC_LEN = Math.PI * 80; // semicircle arc length for r=80

	const totalTrades = $derived(wins + losses);
	const winsArcLen = $derived(totalTrades > 0 ? (wins / totalTrades) * ARC_LEN : 0);
	const lossesArcLen = $derived(totalTrades > 0 ? ARC_LEN - winsArcLen : 0);
</script>

<div class="flex flex-col items-center justify-center pt-2">
	<svg viewBox="0 18 200 96" class="h-auto w-full max-w-36 overflow-visible">
		<!-- Background track -->
		<path
			d="M 20,105 A 80,80 0 0,1 180,105"
			fill="none"
			stroke="var(--bg-card-hover, var(--border))"
			stroke-width="13"
			stroke-linecap="round"
		/>

		<!-- Losses (Red) — drawn first so green sits on top -->
		{#if lossesArcLen > 0}
			<path
				d="M 20,105 A 80,80 0 0,1 180,105"
				fill="none"
				stroke="var(--danger)"
				stroke-width="13"
				stroke-linecap="round"
				stroke-dasharray="{lossesArcLen} {ARC_LEN + 10}"
				stroke-dashoffset={-winsArcLen}
			/>
		{/if}

		<!-- Wins (Green) — drawn last so it renders on top -->
		{#if winsArcLen > 0}
			<path
				d="M 20,105 A 80,80 0 0,1 180,105"
				fill="none"
				stroke="var(--success)"
				stroke-width="13"
				stroke-linecap="round"
				stroke-dasharray="{winsArcLen} {ARC_LEN + 10}"
				stroke-dashoffset="0"
			/>
		{/if}

		<text
			x="100"
			y="88"
			text-anchor="middle"
			font-size="28"
			font-weight="700"
			style="fill: var(--fg);"
			font-family="inherit"
		>
			{totalTrades}
		</text>
		<text
			x="100"
			y="108"
			text-anchor="middle"
			font-size="13"
			font-weight="500"
			style="fill: var(--fg-muted);"
			font-family="inherit"
		>
			Trades
		</text>
	</svg>

	<div class="mt-2 flex items-center justify-center gap-4 text-xs font-medium">
		<span class="inline-flex items-center gap-1" style="color: var(--success)">
			<Icon icon="lucide:check-circle-2" width="13" height="13" />
			<span>Wins <strong>{wins}x</strong></span>
		</span>
		<span class="inline-flex items-center gap-1" style="color: var(--danger)">
			<Icon icon="lucide:x-circle" width="13" height="13" />
			<span>Losses <strong>{losses}x</strong></span>
		</span>
	</div>
</div>
