<script lang="ts">
	import { toasts } from '$lib/helpers/toast';
</script>

{#if toasts.length > 0}
	<div
		class="fixed right-4 top-4 z-100 flex flex-col gap-2"
		style="min-width: 280px; max-width: 360px;"
	>
		{#each toasts as toast (toast.id)}
			<div
				class="flex items-start gap-3 rounded-xl border px-4 py-3.5 shadow-lg"
				style="
					background-color: var(--bg-card);
					border-color: {toast.type === 'success'
					? 'rgba(34,197,94,0.3)'
					: toast.type === 'error'
						? 'rgba(239,68,68,0.3)'
						: 'rgba(48,180,201,0.3)'};
					border-left: 3px solid {toast.type === 'success'
					? 'var(--success)'
					: toast.type === 'error'
						? 'var(--danger)'
						: 'var(--accent)'};
				"
			>
				<!-- Icon -->
				<div
					class="mt-0.5 shrink-0"
					style="color: {toast.type === 'success'
						? 'var(--success)'
						: toast.type === 'error'
							? 'var(--danger)'
							: 'var(--accent)'}"
				>
					{#if toast.type === 'success'}
						<svg
							width="16"
							height="16"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2.5"
						>
							<path d="M20 6L9 17l-5-5" />
						</svg>
					{:else if toast.type === 'error'}
						<svg
							width="16"
							height="16"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2.5"
						>
							<circle cx="12" cy="12" r="10" />
							<path d="M12 8v4M12 16h.01" />
						</svg>
					{:else}
						<svg
							width="16"
							height="16"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="2.5"
						>
							<circle cx="12" cy="12" r="10" />
							<path d="M12 16v-4M12 8h.01" />
						</svg>
					{/if}
				</div>
				<!-- Message -->
				<p class="text-sm font-500 leading-snug" style="color: var(--fg)">{toast.message}</p>
			</div>
		{/each}
	</div>
{/if}
