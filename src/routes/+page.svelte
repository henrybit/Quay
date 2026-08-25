<script lang="ts">
	import EmptyState from '$lib/components/ui/EmptyState.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import { workspace } from '$lib/stores/workspace.svelte';
	import { BROKER_CATALOG, BROKER_KINDS } from '$lib/mq/catalog';
	import BrokerIcon from '$lib/components/brand/BrokerIcon.svelte';
</script>

<section class="welcome">
	<EmptyState title="选择一个消息集群" text="从左侧打开已保存的连接，或按中间件类型新建。连接配置保存在本机，可随时编辑。">
		<div class="kind-picker">
			{#each BROKER_KINDS as kind (kind)}
				<button class="kind-card" type="button" onclick={() => workspace.openCreate(kind)}>
					<BrokerIcon {kind} size={36} />
					<span class="kind-copy">
						<strong>{BROKER_CATALOG[kind].label}</strong>
						<span class="kind-tags">
							{#each BROKER_CATALOG[kind].blurb.split(' · ') as feature}
								<span>{feature}</span>
							{/each}
						</span>
					</span>
				</button>
			{/each}
		</div>
		<Button onclick={() => workspace.openCreate()}>新建连接</Button>
	</EmptyState>
</section>
