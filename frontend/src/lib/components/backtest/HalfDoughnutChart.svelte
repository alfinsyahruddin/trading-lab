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
	<div class="relative w-full max-w-[160px]">
		<svg viewBox="0 0 100 50" class="w-full h-auto overflow-visible">
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
			class="absolute bottom-0 left-0 right-0 flex flex-col items-center justify-end text-center mb-[-8px]"
		>
			<span class="text-xl font-700 leading-none" style="color: var(--fg)">{total}</span>
			<span
				class="text-[10px] font-500 uppercase tracking-wider mt-0.5"
				style="color: var(--fg-muted)">Trades</span
			>
		</div>
	</div>

	<div class="mt-4 flex items-center justify-center gap-4 text-xs font-500">
		<div class="flex items-center gap-1.5" style="color: var(--success)">
			<span class="h-2 w-2 rounded-full bg-current"></span>
			<span>Wins {wins}x</span>
		</div>
		<div class="flex items-center gap-1.5" style="color: var(--danger)">
			<span class="h-2 w-2 rounded-full bg-current"></span>
			<span>Losses {losses}x</span>
		</div>
	</div>
</div>
