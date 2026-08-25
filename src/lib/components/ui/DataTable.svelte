<script lang="ts" generics="T">
import type { Snippet } from 'svelte';

export type Column<T> = {
	key: string;
	label: string;
	class?: string;
	value: (row: T) => string | number;
};

type Props<T> = {
	columns: Column<T>[];
	rows: T[];
	rowKey: (row: T) => string;
	activeKey?: string | null;
	onselect?: (row: T) => void;
	empty?: string;
	actions?: Snippet<[T]>;
};

let {
	columns,
	rows,
	rowKey,
	activeKey = null,
	onselect,
	empty = '没有数据',
	actions
}: Props<T> = $props();
</script>

<div class="table-wrap">
	<table class="grid">
		<thead>
			<tr>
				{#each columns as col (col.key)}
					<th class={col.class}>{col.label}</th>
				{/each}
				{#if actions}
					<th class="col-actions"></th>
				{/if}
			</tr>
		</thead>
		<tbody>
			{#if rows.length === 0}
				<tr>
					<td colspan={columns.length + (actions ? 1 : 0)} class="table-empty">{empty}</td>
				</tr>
			{:else}
				{#each rows as row (rowKey(row))}
					<tr
						class:active={activeKey === rowKey(row)}
						onclick={() => onselect?.(row)}
					>
						{#each columns as col (col.key)}
							<td class={col.class}>{col.value(row)}</td>
						{/each}
						{#if actions}
							<td class="col-actions">{@render actions(row)}</td>
						{/if}
					</tr>
				{/each}
			{/if}
		</tbody>
	</table>
</div>
