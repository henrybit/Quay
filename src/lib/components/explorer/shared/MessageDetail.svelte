<script lang="ts">
import JsonBlock from '$lib/components/ui/JsonBlock.svelte';
import type { MessageView } from '$lib/types/mq';

type Props = {
	message: MessageView | null;
};

let { message }: Props = $props();
</script>

{#if message}
	<div class="split-detail">
		<header>
			<strong>消息详情</strong>
			<span class="mute">{message.key || '无 Key'}</span>
		</header>
		<pre class="payload">{message.payload || '（空 payload）'}</pre>
		<JsonBlock value={message.properties} />
	</div>
{:else}
	<div class="split-detail mute-box">选择一条消息查看内容</div>
{/if}
