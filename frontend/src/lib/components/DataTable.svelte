<script lang="ts" generics="T">
	import type { Snippet } from 'svelte';
	import EmptyState from './EmptyState.svelte';

	interface Column<Row> {
		key: keyof Row | string;
		label: string;
		render?: (row: Row) => string;
	}

	let {
		columns,
		rows,
		loading = false,
		emptyMessage = 'No records found.',
		actions
	}: {
		columns: Column<T>[];
		rows: T[];
		loading?: boolean;
		emptyMessage?: string;
		actions?: Snippet<[T]>;
	} = $props();

	function getCellValue(row: T, col: Column<T>): string {
		if (col.render) return col.render(row);
		const val = (row as Record<string, unknown>)[col.key as string];
		if (val === null || val === undefined) return '—';
		return String(val);
	}
</script>

<div
	class="overflow-hidden rounded-xl border"
	style="border-color: var(--border); background-color: var(--bg-card);"
>
	<div class="overflow-x-auto">
		<table class="w-full text-sm">
			<thead>
				<tr style="border-bottom: 1px solid var(--border); background-color: var(--bg-card);">
					{#each columns as col}
						<th
							class="px-4 py-3.5 text-left text-xs font-700 uppercase tracking-wider"
							style="color: var(--fg-muted);"
						>
							{col.label}
						</th>
					{/each}
					{#if actions}
						<th
							class="px-4 py-3.5 text-right text-xs font-700 uppercase tracking-wider"
							style="color: var(--fg-muted);"
						>
							Actions
						</th>
					{/if}
				</tr>
			</thead>
			<tbody>
				{#if loading}
					{#each { length: 5 } as _, i}
						<tr style="border-bottom: 1px solid var(--border);">
							{#each columns as _col}
								<td class="px-4 py-3.5">
									<div
										class="h-4 w-3/4 animate-pulse rounded"
										style="background-color: var(--border-strong); opacity: 0.5;"
									></div>
								</td>
							{/each}
							{#if actions}
								<td class="px-4 py-3.5">
									<div
										class="ml-auto h-4 w-16 animate-pulse rounded"
										style="background-color: var(--border-strong); opacity: 0.5;"
									></div>
								</td>
							{/if}
						</tr>
					{/each}
				{:else if rows.length === 0}
					<tr>
						<td colspan={columns.length + (actions ? 1 : 0)}>
							<EmptyState message={emptyMessage} />
						</td>
					</tr>
				{:else}
					{#each rows as row, i}
						<tr
							style="border-bottom: {i < rows.length - 1
								? '1px solid var(--border)'
								: 'none'}; transition: background-color 0.1s;"
							onmouseover={(e) =>
								((e.currentTarget as HTMLTableRowElement).style.backgroundColor =
									'var(--bg-card-hover)')}
							onmouseout={(e) =>
								((e.currentTarget as HTMLTableRowElement).style.backgroundColor = 'transparent')}
							onfocus={(e) =>
								((e.currentTarget as HTMLTableRowElement).style.backgroundColor =
									'var(--bg-card-hover)')}
							onblur={(e) =>
								((e.currentTarget as HTMLTableRowElement).style.backgroundColor = 'transparent')}
						>
							{#each columns as col}
								<td class="px-4 py-3.5 font-400" style="color: var(--fg)">
									{getCellValue(row, col)}
								</td>
							{/each}
							{#if actions}
								<td class="px-4 py-3.5 text-right">
									{@render actions(row)}
								</td>
							{/if}
						</tr>
					{/each}
				{/if}
			</tbody>
		</table>
	</div>
</div>
