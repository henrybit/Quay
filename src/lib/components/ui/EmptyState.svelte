<script lang="ts">
import type { Snippet } from 'svelte';

type Props = {
	title: string;
	text: string;
	children?: Snippet;
};

let { title, text, children }: Props = $props();
</script>

<div class="empty">
	<div class="empty-mark"></div>
	<h3>{title}</h3>
	<p>{text}</p>
	{#if children}
		<div class="empty-actions">{@render children()}</div>
	{/if}
</div>
