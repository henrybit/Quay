<script lang="ts">
	import type { ConnectionProfile, MessageView } from '$lib/types/mq';
	import * as api from '$lib/api/rabbitmq';
	import { asArray, formatNumber, readPath } from '$lib/utils/format';
	import MetricCard from '$lib/components/ui/MetricCard.svelte';
	import DataTable, { type Column } from '$lib/components/ui/DataTable.svelte';
	import ErrorBanner from '$lib/components/ui/ErrorBanner.svelte';
	import Spinner from '$lib/components/ui/Spinner.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Input from '$lib/components/ui/Input.svelte';
	import Field from '$lib/components/ui/Field.svelte';
	import MessageTable from '$lib/components/explorer/shared/MessageTable.svelte';
	import MessageDetail from '$lib/components/explorer/shared/MessageDetail.svelte';
	import PublishPanel from '$lib/components/explorer/shared/PublishPanel.svelte';

	type QueueRow = {
		name: string;
		state: string;
		messages: number;
		ready: number;
		unacked: number;
		consumers: number;
	};

	type ExchangeRow = {
		name: string;
		type: string;
		durable: string;
		autoDelete: string;
	};

	type BindingRow = {
		source: string;
		destination: string;
		destinationType: string;
		routingKey: string;
	};

	type Props = {
		connection: ConnectionProfile;
		tab: string;
		refreshKey?: number;
	};

	let { connection, tab, refreshKey = 0 }: Props = $props();

	let loading = $state(false);
	let error = $state<string | null>(null);
	let overview = $state<Record<string, unknown> | null>(null);
	let queues = $state<QueueRow[]>([]);
	let exchanges = $state<ExchangeRow[]>([]);
	let bindings = $state<BindingRow[]>([]);
	let selectedQueue = $state('');
	let messages = $state<MessageView[]>([]);
	let selectedMessage = $state<MessageView | null>(null);
	let peeking = $state(false);

	const queueColumns: Column<QueueRow>[] = [
		{ key: 'name', label: '队列', class: 'col-wide', value: (row) => row.name },
		{ key: 'state', label: '状态', value: (row) => row.state },
		{ key: 'ready', label: 'Ready', value: (row) => formatNumber(row.ready) },
		{ key: 'unacked', label: 'Unacked', value: (row) => formatNumber(row.unacked) },
		{ key: 'messages', label: '总数', value: (row) => formatNumber(row.messages) },
		{ key: 'consumers', label: '消费者', value: (row) => formatNumber(row.consumers) }
	];

	const exchangeColumns: Column<ExchangeRow>[] = [
		{ key: 'name', label: '交换机', class: 'col-wide', value: (row) => row.name || '(default)' },
		{ key: 'type', label: '类型', value: (row) => row.type },
		{ key: 'durable', label: '持久化', value: (row) => row.durable },
		{ key: 'autoDelete', label: '自动删除', value: (row) => row.autoDelete }
	];

	const bindingColumns: Column<BindingRow>[] = [
		{ key: 'source', label: 'Source', value: (row) => row.source || '(default)' },
		{ key: 'destination', label: 'Destination', class: 'col-wide', value: (row) => row.destination },
		{ key: 'type', label: '类型', value: (row) => row.destinationType },
		{ key: 'key', label: 'Routing Key', value: (row) => row.routingKey || '—' }
	];

	async function load() {
		loading = true;
		error = null;
		try {
			if (tab === 'overview') {
				overview = await api.rabbitOverview(connection.id);
			} else if (tab === 'queues') {
				queues = asArray<Record<string, unknown>>(await api.rabbitQueues(connection.id)).map(
					(item) => ({
						name: String(item.name ?? ''),
						state: String(item.state ?? 'running'),
						messages: Number(item.messages ?? 0),
						ready: Number(item.messages_ready ?? 0),
						unacked: Number(item.messages_unacknowledged ?? 0),
						consumers: Number(item.consumers ?? 0)
					})
				);
			} else if (tab === 'exchanges') {
				exchanges = asArray<Record<string, unknown>>(await api.rabbitExchanges(connection.id)).map(
					(item) => ({
						name: String(item.name ?? ''),
						type: String(item.type ?? ''),
						durable: item.durable ? '是' : '否',
						autoDelete: item.auto_delete ? '是' : '否'
					})
				);
			} else if (tab === 'bindings') {
				bindings = asArray<Record<string, unknown>>(await api.rabbitBindings(connection.id)).map(
					(item) => ({
						source: String(item.source ?? ''),
						destination: String(item.destination ?? ''),
						destinationType: String(item.destination_type ?? ''),
						routingKey: String(item.routing_key ?? '')
					})
				);
			}
		} catch (err) {
			error = err instanceof Error ? err.message : '加载失败';
		} finally {
			loading = false;
		}
	}

	async function peek() {
		if (!selectedQueue) return;
		peeking = true;
		error = null;
		try {
			messages = await api.rabbitGetMessages(connection.id, selectedQueue, 20, true);
			selectedMessage = messages[0] ?? null;
		} catch (err) {
			error = err instanceof Error ? err.message : '读取消息失败';
		} finally {
			peeking = false;
		}
	}

	$effect(() => {
		void connection.id;
		void tab;
		void refreshKey;
		void load();
	});
</script>

<ErrorBanner message={error} />
{#if loading}
	<Spinner />
{:else if tab === 'overview' && overview}
	<div class="metric-grid">
		<MetricCard
			label="队列"
			tone="rabbit"
			value={formatNumber(Number(readPath(overview, 'object_totals.queues') ?? 0))}
		/>
		<MetricCard
			label="交换机"
			value={formatNumber(Number(readPath(overview, 'object_totals.exchanges') ?? 0))}
		/>
		<MetricCard
			label="连接"
			value={formatNumber(Number(readPath(overview, 'object_totals.connections') ?? 0))}
		/>
		<MetricCard
			label="通道"
			value={formatNumber(Number(readPath(overview, 'object_totals.channels') ?? 0))}
		/>
		<MetricCard
			label="Ready"
			value={formatNumber(Number(readPath(overview, 'queue_totals.messages_ready') ?? 0))}
		/>
		<MetricCard
			label="Unacked"
			value={formatNumber(Number(readPath(overview, 'queue_totals.messages_unacknowledged') ?? 0))}
		/>
	</div>
	<p class="mute">
		集群 {String(readPath(overview, 'cluster_name') ?? '—')} · RabbitMQ {String(
			readPath(overview, 'rabbitmq_version') ?? ''
		)}
	</p>
{:else if tab === 'queues'}
	<DataTable
		columns={queueColumns}
		rows={queues}
		rowKey={(row) => row.name}
		activeKey={selectedQueue}
		onselect={(row) => (selectedQueue = row.name)}
		empty="当前 vhost 没有队列"
	/>
{:else if tab === 'exchanges'}
	<DataTable
		columns={exchangeColumns}
		rows={exchanges}
		rowKey={(row) => `${row.name}:${row.type}`}
		empty="没有交换机"
	/>
{:else if tab === 'bindings'}
	<DataTable
		columns={bindingColumns}
		rows={bindings}
		rowKey={(row) => `${row.source}:${row.destination}:${row.routingKey}`}
		empty="没有绑定"
	/>
{:else if tab === 'messages'}
	<div class="toolbar">
		<Field label="队列名">
			<Input bind:value={selectedQueue} placeholder="选择或输入队列" />
		</Field>
		<Button kind="rabbitmq" busy={peeking} onclick={peek}>Peek 消息</Button>
	</div>
	<div class="split">
		<MessageTable rows={messages} activeKey={selectedMessage?.payload ?? null} onselect={(row) => (selectedMessage = row)} />
		<MessageDetail message={selectedMessage} />
	</div>
	<div class="panel">
		<h3>发布到交换机</h3>
		<PublishPanel
			targetLabel="Exchange"
			targetPlaceholder="amq.default 可留空为默认交换机"
			keyLabel="Routing Key"
			onpublish={async ({ target, key, payload }) => {
				await api.rabbitPublish(connection.id, target, key, payload);
			}}
		/>
	</div>
{/if}
