<script lang="ts">
import type { Snippet } from 'svelte';
import { cx } from '$lib/utils/format';
import type { BrokerKind } from '$lib/types/mq';

type Props = {
	variant?: 'primary' | 'ghost' | 'danger' | 'quiet';
	kind?: BrokerKind;
	size?: 'sm' | 'md';
	disabled?: boolean;
	busy?: boolean;
	type?: 'button' | 'submit';
	onclick?: (event: MouseEvent) => void;
	children: Snippet;
};

let {
	variant = 'primary',
	kind,
	size = 'md',
	disabled = false,
	busy = false,
	type = 'button',
	onclick,
	children
}: Props = $props();
</script>

<button
	{type}
	class={cx(
		'btn',
		`btn-${variant}`,
		size === 'sm' && 'btn-sm',
		kind && `btn-kind-${kind}`,
		busy && 'is-busy'
	)}
	disabled={disabled || busy}
	{onclick}
>
	{@render children()}
</button>
