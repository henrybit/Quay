<script lang="ts">
import type { BrokerKind } from '$lib/types/mq';

type Props = {
	kind: BrokerKind;
	size?: number;
};

let { kind, size = 28 }: Props = $props();
</script>

<span class="broker-icon {kind}" style="width:{size}px;height:{size}px" aria-hidden="true">
	{#if kind === 'rabbitmq'}
		<svg viewBox="0 0 32 32" fill="none">
			<path d="M10 14c0-6 2.2-10 6-10s6 4 6 10" stroke="currentColor" stroke-width="2.2" />
			<path d="M8.5 16.5c0 7.2 3.4 11.5 7.5 11.5s7.5-4.3 7.5-11.5" stroke="currentColor" stroke-width="2.2" />
			<circle cx="13.2" cy="18.2" r="1.2" fill="currentColor" />
			<circle cx="18.8" cy="18.2" r="1.2" fill="currentColor" />
			<path d="M14.4 22.2c1.1.9 2.1.9 3.2 0" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" />
			<path d="M10 8.2 7.4 5.4M22 8.2 24.6 5.4" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" />
		</svg>
	{:else if kind === 'kafka'}
		<svg viewBox="0 0 32 32" fill="none">
			<rect x="6" y="6" width="20" height="5" rx="1.4" fill="currentColor" opacity="0.95" />
			<rect x="6" y="13.5" width="20" height="5" rx="1.4" fill="currentColor" opacity="0.7" />
			<rect x="6" y="21" width="20" height="5" rx="1.4" fill="currentColor" opacity="0.45" />
		</svg>
	{:else}
		<svg viewBox="0 0 32 32" fill="none">
			<path d="M16 4 26 22H6L16 4Z" stroke="currentColor" stroke-width="2.2" stroke-linejoin="round" />
			<path d="M16 12v16" stroke="currentColor" stroke-width="2.2" stroke-linecap="round" />
			<path d="M16 20c-3 2.4-5.2 6-5.2 8.6M16 20c3 2.4 5.2 6 5.2 8.6" stroke="currentColor" stroke-width="1.8" />
		</svg>
	{/if}
</span>
