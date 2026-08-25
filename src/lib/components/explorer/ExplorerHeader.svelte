<script lang="ts">
	import type { ConnectionProfile } from '$lib/types/mq';
	import { brokerMeta } from '$lib/mq/catalog';
	import BrokerIcon from '$lib/components/brand/BrokerIcon.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import { workspace } from '$lib/stores/workspace.svelte';
	import { goto } from '$app/navigation';
	import { endpointHint } from '$lib/mq/catalog';

	type Props = {
		connection: ConnectionProfile;
		onrefresh?: () => void;
		busy?: boolean;
	};

	let { connection, onrefresh, busy = false }: Props = $props();
	const meta = $derived(brokerMeta(connection.kind));

	async function remove() {
		if (!confirm(`删除连接「${connection.name}」？`)) return;
		await workspace.remove(connection.id);
		await goto('/');
	}
</script>

<header class="explorer-head">
	<div class="explorer-title">
		<BrokerIcon kind={connection.kind} size={32} />
		<div>
			<div class="kicker">{meta.label}</div>
			<h1>{connection.name}</h1>
			<p>{endpointHint(connection)}</p>
		</div>
	</div>
	<div class="row-actions">
		<Button variant="ghost" size="sm" {busy} onclick={() => onrefresh?.()}>刷新</Button>
		<Button variant="quiet" size="sm" onclick={() => workspace.openEdit(connection)}>编辑</Button>
		<Button variant="danger" size="sm" onclick={remove}>删除</Button>
	</div>
</header>
