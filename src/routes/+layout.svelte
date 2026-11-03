<script lang="ts">
	import './layout.css';
	import { onMount } from 'svelte';
	import { workspace } from '$lib/stores/workspace.svelte';
	import Sidebar from '$lib/components/layout/Sidebar.svelte';
	import ConnectionEditor from '$lib/components/connection/ConnectionEditor.svelte';

	let { children } = $props();

	onMount(() => {
		void workspace.refresh();
	});
</script>

<svelte:head>
	<title>Quay</title>
</svelte:head>

<div class="app-shell">
	<Sidebar />
	<main class="stage">
		{@render children()}
	</main>
</div>

<ConnectionEditor />
