<script lang="ts">
import type { TabDef } from '$lib/types/mq';

type Props = {
	tabs: TabDef[];
	value: string;
};

let { tabs, value = $bindable() }: Props = $props();
</script>

<div class="tabs" role="tablist">
	{#each tabs as tab (tab.id)}
		<button
			type="button"
			class="tab"
			class:active={value === tab.id}
			role="tab"
			aria-selected={value === tab.id}
			onclick={() => (value = tab.id)}
		>
			{tab.label}
		</button>
	{/each}
</div>
