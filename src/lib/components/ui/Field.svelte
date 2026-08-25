<script lang="ts">
import type { Snippet } from 'svelte';
import { cx } from '$lib/utils/format';

type Props = {
	label: string;
	hint?: string;
	children: Snippet;
};

let { label, hint, children }: Props = $props();
</script>

<label class="field">
	<span class="field-label">{label}</span>
	{@render children()}
	{#if hint}
		<span class={cx('field-hint')}>{hint}</span>
	{/if}
</label>
