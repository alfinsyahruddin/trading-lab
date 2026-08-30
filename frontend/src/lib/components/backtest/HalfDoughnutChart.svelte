<script lang="ts">
	let { wins, losses }: { wins: number; losses: number } = $props();

	const total = $derived(wins + losses);

	// Math for SVG arcs
	// We use a stroke-dasharray technique. Circumference of half circle = pi * r.
	// Radius = 40, so pi * 40 = 125.66
	const radius = 40;
	const circumference = Math.PI * radius;

	const winPercentage = $derived(total === 0 ? 0 : wins / total);
	const winDash = $derived(winPercentage * circumference);
	const lossDash = $derived(circumference - winDash);
</script>

<div class="flex flex-col items-center justify-center pt-2">
	<div class="relative w-full max-w-40">
		<svg viewBox="0 0 100 50" class="h-auto w-full overflow-visible">
			<!-- Background track -->
			<path
				d="M 10 50 A 40 40 0 0 1 90 50"
				fill="none"
				stroke="var(--bg-card-hover, #333)"
				stroke-width="12"
				stroke-linecap="butt"
			/>

			<!-- Losses (Red, right side) - drawn first as the background for the active part -->
			<path
				d="M 10 50 A 40 40 0 0 1 90 50"
				fill="none"
				stroke="var(--danger)"
				stroke-width="12"
				stroke-linecap="butt"
				stroke-dasharray="{circumference} {circumference}"
				stroke-dashoffset="0"
			/>

			<!-- Wins (Green, left side) -->
			<path
				d="M 10 50 A 40 40 0 0 1 90 50"
				fill="none"
				stroke="var(--success)"
				stroke-width="12"
				stroke-linecap="butt"
				stroke-dasharray="{winDash} {circumference}"
				stroke-dashoffset="0"
			/>
		</svg>

		<div
			class="absolute inset-x-0 bottom-0 -mb-2 flex flex-col items-center justify-end text-center"
		>
			<span class="font-700 text-xl leading-none" style="color: var(--fg)">{total}</span>
			<span
				class="font-500 mt-0.5 text-[10px] tracking-wider uppercase"
				style="color: var(--fg-muted)">Trades</span
			>
		</div>
	</div>

	<div class="font-500 mt-4 flex items-center justify-center gap-4 text-xs">
		<div class="flex items-center gap-1.5" style="color: var(--success)">
			<span class="size-2 rounded-full bg-current"></span>
			<span>Wins {wins}x</span>
		</div>
		<div class="flex items-center gap-1.5" style="color: var(--danger)">
			<span class="size-2 rounded-full bg-current"></span>
			<span>Losses {losses}x</span>
		</div>
	</div>
</div>
