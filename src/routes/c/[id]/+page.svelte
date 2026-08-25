<script lang="ts">
	import { page } from '$app/state';
	import { workspace } from '$lib/stores/workspace.svelte';
	import Explorer from '$lib/components/explorer/Explorer.svelte';
	import EmptyState from '$lib/components/ui/EmptyState.svelte';
	import Spinner from '$lib/components/ui/Spinner.svelte';
	import Button from '$lib/components/ui/Button.svelte';

	const connection = $derived(
		workspace.connections.find((item) => item.id === page.params.id) ?? null
	);
</script>

{#if workspace.loading && !connection}
	<Spinner label="读取连接" />
{:else if connection}
	<Explorer {connection} />
{:else}
	<EmptyState title="连接不存在" text="它可能已被删除，或配置文件尚未同步。">
		<Button variant="ghost" onclick={() => workspace.refresh()}>重新加载</Button>
	</EmptyState>
{/if}
