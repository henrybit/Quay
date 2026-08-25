<script lang="ts">
type Props = {
	message?: string | null;
};

let { message = null }: Props = $props();
</script>

{#if message}
	<div class="banner error">{message}</div>
{/if}
