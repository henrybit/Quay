<script lang="ts">
	import type { ConnectionProfile } from '$lib/types/mq';
	import { brokerMeta } from '$lib/mq/catalog';
	import Tabs from '$lib/components/ui/Tabs.svelte';
	import ExplorerHeader from './ExplorerHeader.svelte';
	import RabbitExplorer from './rabbitmq/RabbitExplorer.svelte';
	import KafkaExplorer from './kafka/KafkaExplorer.svelte';
	import RocketExplorer from './rocketmq/RocketExplorer.svelte';

	type Props = {
		connection: ConnectionProfile;
	};

	let { connection }: Props = $props();
	const meta = $derived(brokerMeta(connection.kind));
	let tab = $state('overview');
	let refreshKey = $state(0);

	$effect(() => {
		void connection.id;
		tab = brokerMeta(connection.kind).tabs[0]?.id ?? 'overview';
	});
</script>

<section class="explorer">
	<ExplorerHeader {connection} busy={false} onrefresh={() => (refreshKey += 1)} />
	<Tabs tabs={meta.tabs} bind:value={tab} />
	<div class="explorer-body">
		{#if connection.kind === 'rabbitmq'}
			<RabbitExplorer {connection} {tab} {refreshKey} />
		{:else if connection.kind === 'kafka'}
			<KafkaExplorer {connection} {tab} {refreshKey} />
		{:else}
			<RocketExplorer {connection} {tab} {refreshKey} />
		{/if}
	</div>
</section>
