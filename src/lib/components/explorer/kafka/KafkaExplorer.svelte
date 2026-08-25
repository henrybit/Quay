<script lang="ts">
	import type {
		ConnectionProfile,
		KafkaCluster,
		KafkaGroup,
		KafkaGroupDetail,
		KafkaTopicDetail,
		MessageView
	} from '$lib/types/mq';
	import * as api from '$lib/api/kafka';
	import { formatNumber } from '$lib/utils/format';
	import MetricCard from '$lib/components/ui/MetricCard.svelte';
	import DataTable, { type Column } from '$lib/components/ui/DataTable.svelte';
	import ErrorBanner from '$lib/components/ui/ErrorBanner.svelte';
	import Spinner from '$lib/components/ui/Spinner.svelte';
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
	let cluster = $state<KafkaCluster | null>(null);
	let topicName = $state('');
	let topicDetail = $state<KafkaTopicDetail | null>(null);
	let groups = $state<KafkaGroup[]>([]);
	let groupId = $state('');
	let groupDetail = $state<KafkaGroupDetail | null>(null);
	let partition = $state('0');
	let offset = $state('0');
	let messages = $state<MessageView[]>([]);
	let selectedMessage = $state<MessageView | null>(null);
	let fetching = $state(false);

	const topicColumns: Column<KafkaCluster['topics'][number]>[] = [
		{ key: 'name', label: 'Topic', class: 'col-wide', value: (row) => row.name },
		{ key: 'partitions', label: '分区数', value: (row) => row.partitionCount }
	];

	const partitionColumns: Column<KafkaTopicDetail['partitions'][number]>[] = [
		{ key: 'p', label: 'Partition', value: (row) => row.partition },
		{ key: 'earliest', label: 'Earliest', value: (row) => formatNumber(row.earliest) },
		{ key: 'latest', label: 'Latest / HW', value: (row) => formatNumber(row.latest) },
		{
			key: 'size',
			label: '日志量',
			value: (row) => formatNumber(Math.max(row.latest - row.earliest, 0))
		}
	];

	const groupColumns: Column<KafkaGroup>[] = [
		{ key: 'id', label: 'Group', class: 'col-wide', value: (row) => row.groupId },
		{ key: 'protocol', label: 'Protocol', value: (row) => row.protocolType || '—' },
		{ key: 'broker', label: 'Coordinator', value: (row) => row.broker }
	];

	async function load() {
		loading = true;
		error = null;
		try {
			if (tab === 'cluster' || tab === 'topics' || tab === 'browser') {
				cluster = await api.kafkaCluster(connection.id);
			}
			if (tab === 'groups') {
				groups = await api.kafkaGroups(connection.id);
			}
		} catch (err) {
			error = err instanceof Error ? err.message : '加载失败';
		} finally {
			loading = false;
		}
	}

	async function loadTopic(name: string) {
		topicName = name;
		error = null;
		try {
			topicDetail = await api.kafkaTopicDetail(connection.id, name);
		} catch (err) {
			error = err instanceof Error ? err.message : '读取分区失败';
		}
	}

	async function loadGroup(id: string) {
		groupId = id;
		error = null;
		try {
			groupDetail = await api.kafkaGroupDetail(connection.id, id);
		} catch (err) {
			error = err instanceof Error ? err.message : '读取消费组失败';
		}
	}

	async function fetchMessages() {
		fetching = true;
		error = null;
		try {
			messages = await api.kafkaFetch(
				connection.id,
				topicName,
				Number(partition) || 0,
				Number(offset) || 0
			);
			selectedMessage = messages[0] ?? null;
		} catch (err) {
			error = err instanceof Error ? err.message : '拉取消息失败';
		} finally {
			fetching = false;
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
		<MetricCard tone="kafka" label="Broker" value={cluster.brokers.length || '—'} />
		<MetricCard label="Topic" value={cluster.topics.length} />
		<MetricCard
			label="分区合计"
			value={cluster.topics.reduce((sum, item) => sum + item.partitionCount, 0)}
		/>
	</div>
	<h3>Brokers</h3>
	{#if cluster.brokers.length === 0}
		<p class="mute">当前客户端未返回 broker 元数据，Topic 列表仍可用。</p>
	{:else}
		<DataTable
			columns={[
				{ key: 'id', label: 'ID', value: (row) => row.nodeId },
				{ key: 'host', label: 'Host', class: 'col-wide', value: (row) => row.host },
				{ key: 'port', label: 'Port', value: (row) => row.port }
			]}
			rows={cluster.brokers}
			rowKey={(row) => String(row.nodeId)}
		/>
	{/if}
{:else if tab === 'topics' && cluster}
	<div class="split">
		<DataTable
			columns={topicColumns}
			rows={cluster.topics}
			rowKey={(row) => row.name}
			activeKey={topicName}
			onselect={(row) => void loadTopic(row.name)}
			empty="没有 Topic"
		/>
		<div class="split-detail">
			{#if topicDetail}
				<h3>{topicDetail.name}</h3>
				<DataTable
					columns={partitionColumns}
					rows={topicDetail.partitions}
					rowKey={(row) => String(row.partition)}
				/>
			{:else}
				<div class="mute-box">选择 Topic 查看分区 Earliest / Latest 偏移</div>
			{/if}
		</div>
	</div>
{:else if tab === 'groups'}
	<div class="split">
		<DataTable
			columns={groupColumns}
			rows={groups}
			rowKey={(row) => row.groupId}
			activeKey={groupId}
			onselect={(row) => void loadGroup(row.groupId)}
			empty="没有消费组，或 broker 未开启 GroupCoordinator"
		/>
		<div class="split-detail">
			{#if groupDetail}
				<p>
					<strong>{groupDetail.groupId}</strong>
					<span class="badge">{groupDetail.state || 'unknown'}</span>
				</p>
				<p class="mute">{groupDetail.protocolType} / {groupDetail.protocol || '—'}</p>
				<h4>成员</h4>
				<DataTable
					columns={[
						{ key: 'member', label: 'Member', value: (row) => row.memberId },
						{ key: 'client', label: 'Client', value: (row) => row.clientId },
						{ key: 'host', label: 'Host', value: (row) => row.clientHost }
					]}
					rows={groupDetail.members}
					rowKey={(row) => row.memberId}
					empty="没有在线成员"
				/>
				<h4>Lag</h4>
				<DataTable
					columns={[
						{ key: 'topic', label: 'Topic', value: (row) => row.topic },
						{ key: 'p', label: 'P', value: (row) => row.partition },
						{ key: 'c', label: 'Committed', value: (row) => row.committed },
						{ key: 'l', label: 'Latest', value: (row) => row.latest },
						{ key: 'lag', label: 'Lag', value: (row) => row.lag }
					]}
					rows={groupDetail.lag}
					rowKey={(row) => `${row.topic}:${row.partition}`}
					empty="没有已提交位移"
				/>
			{:else}
				<div class="mute-box">选择消费组查看成员与 Lag</div>
			{/if}
		</div>
	</div>
{:else if tab === 'browser'}
	<div class="toolbar">
		<Field label="Topic">
			<Input bind:value={topicName} placeholder="topic 名称" />
		</Field>
		<Field label="Partition">
			<Input bind:value={partition} />
		</Field>
		<Field label="Offset">
			<Input bind:value={offset} />
		</Field>
		<Button kind="kafka" busy={fetching} onclick={fetchMessages}>拉取</Button>
	</div>
	<div class="split">
		<MessageTable
			rows={messages}
			activeKey={selectedMessage ? `${selectedMessage.partition}:${selectedMessage.offset}` : null}
			onselect={(row) => (selectedMessage = row)}
		/>
		<MessageDetail message={selectedMessage} />
	</div>
	<div class="panel">
		<h3>生产消息</h3>
		<PublishPanel
			targetLabel="Topic"
			targetPlaceholder="topic"
			keyLabel="Key"
			extraLabel="Partition"
			onpublish={async ({ target, key, extra, payload }) => {
				await api.kafkaProduce(connection.id, target, payload, {
					key: key || undefined,
					partition: Number(extra) || 0
				});
			}}
		/>
	</div>
{/if}
