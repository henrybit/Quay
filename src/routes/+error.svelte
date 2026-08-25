<script lang="ts">
	import { page } from '$app/state';
	import EmptyState from '$lib/components/ui/EmptyState.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import { goto } from '$app/navigation';
</script>

<EmptyState title="页面出错" text={page.error?.message ?? '未知错误'}>
	<Button onclick={() => goto('/')}>回到工作台</Button>
</EmptyState>
