<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { workspace } from '$lib/stores/workspace.svelte';
	import { BROKER_CATALOG, BROKER_KINDS, endpointHint } from '$lib/mq/catalog';
	import type { BrokerKind } from '$lib/types/mq';
	import Logo from '$lib/components/brand/Logo.svelte';
	import BrokerIcon from '$lib/components/brand/BrokerIcon.svelte';
	import SearchInput from '$lib/components/ui/SearchInput.svelte';
	import Button from '$lib/components/ui/Button.svelte';

	let query = $state('');

	$effect(() => {
		workspace.query = query;
	});

	function setFilter(kind: BrokerKind | 'all') {
		workspace.kindFilter = kind;
	}
</script>

<aside class="sidebar">
	<div class="sidebar-brand">
		<Logo />
		<p>消息队列码头</p>
	</div>

	<div class="sidebar-toolbar">
		<SearchInput bind:value={query} placeholder="搜索连接" />
		<Button size="sm" onclick={() => workspace.openCreate()}>新建</Button>
	</div>

	<div class="filter-row">
		<button
			class="chip"
			class:active={workspace.kindFilter === 'all'}
			onclick={() => setFilter('all')}>全部</button
		>
		{#each BROKER_KINDS as kind (kind)}
			<button
				class="chip {kind}"
				class:active={workspace.kindFilter === kind}
				onclick={() => setFilter(kind)}>{BROKER_CATALOG[kind].label}</button
			>
		{/each}
	</div>

	<div class="conn-list">
		{#if workspace.filtered.length === 0}
			<p class="sidebar-empty">还没有连接，点右上角新建。</p>
		{:else}
			{#each workspace.filtered as item (item.id)}
			<div
				class="conn-item {item.kind}"
				class:active={page.params.id === item.id}
			>
				<button
					type="button"
					class="conn-main"
					onclick={() => goto(`/c/${item.id}`)}
				>
					<BrokerIcon kind={item.kind} size={26} />
					<span class="conn-copy">
						<strong>{item.name}</strong>
						<small>{endpointHint(item)}</small>
					</span>
				</button>
				<button
					class="icon-btn tiny"
					type="button"
					onclick={() => workspace.openEdit(item)}
					aria-label="编辑"
				>
					✎
				</button>
			</div>
			{/each}
		{/if}
	</div>
</aside>
