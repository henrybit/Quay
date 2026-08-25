<script lang="ts">
import type { Snippet } from 'svelte';
import { cx } from '$lib/utils/format';

type Props = {
	label: string;
	value: string | number;
	hint?: string;
	tone?: 'default' | 'rabbit' | 'kafka' | 'rocket';
	children?: Snippet;
};

let { label, value, hint, tone = 'default', children }: Props = $props();
</script>

<article class={cx('metric', tone !== 'default' && `tone-${tone}`)}>
	<div class="metric-label">{label}</div>
	<div class="metric-value">{value}</div>
	{#if hint}
		<div class="metric-hint">{hint}</div>
	{/if}
	{#if children}
		<div class="metric-extra">{@render children()}</div>
	{/if}
</article>
