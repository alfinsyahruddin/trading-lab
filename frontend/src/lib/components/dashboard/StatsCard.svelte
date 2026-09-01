<script lang="ts">
	import Icon from '@iconify/svelte';

	let {
		icon,
		label,
		value,
		subtitle,
		href,
		orbColor,
		iconFg,
		iconBg,
		border = '',
		hoverBorder = '',
		smallValue = false
	}: {
		icon: string;
		label: string;
		value: string;
		subtitle?: string;
		href?: string;
		orbColor: string;
		iconFg: string;
		iconBg: string;
		border?: string;
		hoverBorder?: string;
		smallValue?: boolean;
	} = $props();
</script>

{#if href}
	<a
		{href}
		class="group relative block overflow-hidden rounded-2xl border border-(--border) p-5 transition-colors duration-200 sm:p-6 {border} {hoverBorder}"
		style="background-color: var(--bg-card);"
	>
		<!-- Radial glow -->
		<div
			class="pointer-events-none absolute inset-0 rounded-2xl"
			style="background: radial-gradient(ellipse 90% 80% at 100% 0%, {orbColor} 0%, transparent 70%);"
		></div>

		<!-- Content -->
		<div class="relative flex flex-col">
			<!-- Top row: icon left, arrow right -->
			<div class="mb-4 flex items-start justify-between sm:mb-5">
				<div
					class="inline-flex size-10 shrink-0 items-center justify-center rounded-xl sm:size-11 {iconBg}"
				>
					<Icon {icon} class="size-5 sm:size-5.5 {iconFg}" />
				</div>

				<Icon
					icon="lucide:arrow-up-right"
					class="mt-0.5 size-4 text-(--fg-muted)/30 transition-colors duration-200 group-hover:text-(--fg-muted)/80"
				/>
			</div>

			<!-- Value (large) -->
			<p
				class="leading-tight font-bold tracking-tight {smallValue
					? 'text-lg sm:text-xl'
					: 'text-3xl tabular-nums sm:text-4xl'}"
				style="color: var(--fg)"
			>
				{value}
			</p>

			<!-- Subtitle (medium) -->
			{#if subtitle}
				<p class="mt-1 text-xs font-medium tabular-nums sm:text-sm" style="color: var(--fg-muted)">
					{subtitle}
				</p>
			{/if}

			<!-- Label (small) -->
			<p class="mt-1 text-xs font-medium sm:text-sm" style="color: var(--fg-muted)">{label}</p>
		</div>
	</a>
{:else}
	<div
		class="group relative overflow-hidden rounded-2xl border border-(--border) p-5 transition-colors duration-200 sm:p-6 {border} {hoverBorder}"
		style="background-color: var(--bg-card);"
	>
		<!-- Radial glow -->
		<div
			class="pointer-events-none absolute inset-0 rounded-2xl"
			style="background: radial-gradient(ellipse 90% 80% at 100% 0%, {orbColor} 0%, transparent 70%);"
		></div>

		<!-- Content -->
		<div class="relative flex flex-col">
			<!-- Top row: icon left -->
			<div class="mb-4 flex items-start justify-between sm:mb-5">
				<div
					class="inline-flex size-10 shrink-0 items-center justify-center rounded-xl sm:size-11 {iconBg}"
				>
					<Icon {icon} class="size-5 sm:size-5.5 {iconFg}" />
				</div>
			</div>

			<!-- Value (large) -->
			<p
				class="leading-tight font-bold tracking-tight {smallValue
					? 'text-lg sm:text-xl'
					: 'text-3xl tabular-nums sm:text-4xl'}"
				style="color: var(--fg)"
			>
				{value}
			</p>

			<!-- Subtitle (medium) -->
			{#if subtitle}
				<p class="mt-1 text-xs font-medium tabular-nums sm:text-sm" style="color: var(--fg-muted)">
					{subtitle}
				</p>
			{/if}

			<!-- Label (small) -->
			<p class="mt-1 text-xs font-medium sm:text-sm" style="color: var(--fg-muted)">{label}</p>
		</div>
	</div>
{/if}
