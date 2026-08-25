<script lang="ts">
	import type { ConnectionProfile, MessageView, RocketCluster, RocketTopicList } from '$lib/types/mq';
	import * as api from '$lib/api/rocketmq';
	import { formatNumber } from '$lib/utils/format';
	import MetricCard from '$lib/components/ui/MetricCard.svelte';
	import DataTable, { type Column } from '$lib/components/ui/DataTable.svelte';
	import ErrorBanner from '$lib/components/ui/ErrorBanner.svelte';
	import Spinner from '$lib/components/ui/Spinner.svelte';
	import JsonBlock from '$lib/components/ui/JsonBlock.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Field from '$lib/components/ui/Field.svelte';
	import Input from '$lib/components/ui/Input.svelte';
	import MessageTable from '$lib/components/explorer/shared/MessageTable.svelte';
	import MessageDetail from '$lib/components/explorer/shared/MessageDetail.svelte';
	import PublishPanel from '$lib/components/explorer/shared/PublishPanel.svelte';

	type Props = {
		connection: ConnectionProfile;
		tab: string;
		refreshKey?: number;
	};

	let { connection, tab, refreshKey = 0 }: Props = $props();

	let loading = $state(false);
	let error = $state<string | null>(null);
	let cluster = $state<RocketCluster | null>(null);
	let topics = $state<string[]>([]);
	let topic = $state('');
	let route = $state<unknown>(null);
	let stats = $state<unknown>(null);
	let group = $state('');
	let consumeStats = $state<unknown>(null);
	let messageKey = $state('');
	let messages = $state<MessageView[]>([]);
	let selectedMessage = $state<MessageView | null>(null);
	let querying = $state(false);

	const topicColumns: Column<{ name: string; kind: string }>[] = [
		{ key: 'name', label: 'Topic', class: 'col-wide', value: (row) => row.name },
		{ key: 'kind', label: '类型', value: (row) => row.kind }
	];

	function classify(name: string): string {
		if (name.startsWith('%RETRY%')) return 'Retry';
		if (name.startsWith('%DLQ%')) return 'DLQ';
		if (name.startsWith('%SYS%') || name.startsWith('rmq_sys_')) return 'System';
		return 'Normal';
	}

	const topicRows = $derived(
		topics.map((name) => ({
			name,
			kind: classify(name)
		}))
	);

	async function load() {
		loading = true;
		error = null;
		try {
			if (tab === 'cluster') {
				cluster = await api.rocketCluster(connection.id);
			} else if (tab === 'topics' || tab === 'query') {
				const list: RocketTopicList = await api.rocketTopics(connection.id);
				topics = list.topics;
			}
		} catch (err) {
			error = err instanceof Error ? err.message : '加载失败';
		} finally {
			loading = false;
		}
	}

	async function inspectTopic(name: string) {
		topic = name;
		error = null;
		try {
			route = await api.rocketTopicRoute(connection.id, name);
			try {
				stats = await api.rocketTopicStats(connection.id, name);
			} catch {
				stats = null;
			}
		} catch (err) {
			error = err instanceof Error ? err.message : '读取路由失败';
		}
	}

	async function loadConsume() {
		error = null;
		try {
			consumeStats = await api.rocketConsumeStats(
				connection.id,
				group,
				topic || undefined
			);
		} catch (err) {
			error = err instanceof Error ? err.message : '读取消费进度失败';
		}
	}

	async function query() {
		querying = true;
		error = null;
		try {
			messages = await api.rocketQueryMessages(connection.id, topic, messageKey);
			selectedMessage = messages[0] ?? null;
		} catch (err) {
			error = err instanceof Error ? err.message : '查询失败';
		} finally {
			querying = false;
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
{:else if tab === 'cluster' && cluster}
	<div class="metric-grid">
		<MetricCard tone="rocket" label="NameServer" value={cluster.namesrv} />
		<MetricCard
			label="Broker 组"
			value={formatNumber(
				cluster.raw && typeof cluster.raw === 'object'
					? Object.keys((cluster.raw as { brokerAddrTable?: object }).brokerAddrTable ?? {}).length
					: 0
			)}
		/>
	</div>
	<JsonBlock value={cluster.raw} />
{:else if tab === 'topics'}
	<div class="split">
		<DataTable
			columns={topicColumns}
			rows={topicRows}
			rowKey={(row) => row.name}
			activeKey={topic}
			onselect={(row) => void inspectTopic(row.name)}
			empty="NameServer 没有 Topic"
		/>
		<div class="split-detail">
			{#if route}
				<h3>{topic}</h3>
				<p class="mute">Topic 路由（queueDatas / brokerDatas）是 RocketMQ 的核心视图。</p>
				<JsonBlock value={route} />
				{#if stats}
					<h4>队列位点</h4>
					<JsonBlock value={stats} />
				{/if}
			{:else}
				<div class="mute-box">选择 Topic 查看路由与队列位点</div>
			{/if}
		</div>
	</div>
{:else if tab === 'consume'}
	<div class="toolbar">
		<Field label="Consumer Group">
			<Input bind:value={group} placeholder="GID_xxx" />
		</Field>
		<Field label="Topic（可选）">
			<Input bind:value={topic} placeholder="缩小到某个 topic" />
		</Field>
		<Button kind="rocketmq" onclick={loadConsume}>查询进度</Button>
	</div>
	{#if consumeStats}
		<JsonBlock value={consumeStats} />
	{:else}
		<p class="mute">RocketMQ 消费进度来自 Broker 的 ConsumeStats，需填写消费组。</p>
	{/if}
{:else if tab === 'query'}
	<div class="toolbar">
		<Field label="Topic">
			<Input bind:value={topic} placeholder="topic" />
		</Field>
		<Field label="Message Key">
			<Input bind:value={messageKey} placeholder="业务 Key / MessageKey" />
		</Field>
		<Button kind="rocketmq" busy={querying} onclick={query}>按 Key 查询</Button>
	</div>
	<div class="split">
		<MessageTable rows={messages} onselect={(row) => (selectedMessage = row)} />
		<MessageDetail message={selectedMessage} />
	</div>
	<div class="panel">
		<h3>发送消息</h3>
		<PublishPanel
			targetLabel="Topic"
			targetPlaceholder="topic"
			keyLabel="Keys"
			extraLabel="Tags"
			onpublish={async ({ target, key, extra, payload }) => {
				await api.rocketSend(connection.id, target, payload, {
					keys: key || undefined,
					tags: extra || undefined
				});
			}}
		/>
	</div>
{/if}
