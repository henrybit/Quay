<script lang="ts">
import type { MessageView } from '$lib/types/mq';
import DataTable, { type Column } from '$lib/components/ui/DataTable.svelte';
import { clip, formatTime } from '$lib/utils/format';

type Props = {
	rows: MessageView[];
	onselect?: (row: MessageView) => void;
	activeKey?: string | null;
};

let { rows, onselect, activeKey = null }: Props = $props();

const columns: Column<MessageView>[] = [
	{ key: 'offset', label: 'Offset', value: (row) => row.offset ?? '—' },
	{ key: 'partition', label: '分区', value: (row) => row.partition ?? '—' },
	{ key: 'key', label: 'Key', value: (row) => clip(row.key || '—', 36) },
	{ key: 'payload', label: 'Payload', class: 'col-wide', value: (row) => clip(row.payload || '—', 72) },
	{ key: 'time', label: '时间', value: (row) => formatTime(row.timestamp) }
];

function rowKey(row: MessageView, index: number) {
	return `${row.partition ?? 0}:${row.offset ?? index}:${row.key}`;
}
</script>

<DataTable
	{columns}
	{rows}
	{onselect}
	{activeKey}
	rowKey={(row) => rowKey(row, rows.indexOf(row))}
	empty="没有消息"
/>
