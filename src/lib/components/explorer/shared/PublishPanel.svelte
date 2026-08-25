<script lang="ts">
import Button from '$lib/components/ui/Button.svelte';
import Field from '$lib/components/ui/Field.svelte';
import Input from '$lib/components/ui/Input.svelte';
import Textarea from '$lib/components/ui/Textarea.svelte';

type Props = {
	targetLabel: string;
	targetPlaceholder: string;
	keyLabel?: string;
	extraLabel?: string;
	busy?: boolean;
	onpublish: (input: { target: string; key: string; extra: string; payload: string }) => Promise<void>;
};

let {
	targetLabel,
	targetPlaceholder,
	keyLabel = 'Key / Routing Key',
	extraLabel,
	busy = false,
	onpublish
}: Props = $props();

let target = $state('');
let key = $state('');
let extra = $state('');
let payload = $state('');
let message = $state('');

async function submit() {
	message = '';
	try {
		await onpublish({ target, key, extra, payload });
		message = '已发送';
	} catch (error) {
		message = error instanceof Error ? error.message : '发送失败';
	}
}
</script>

<form
	class="publish-panel"
	onsubmit={(event) => {
		event.preventDefault();
		void submit();
	}}
>
	<div class="form-grid">
		<Field label={targetLabel}>
			<Input bind:value={target} placeholder={targetPlaceholder} />
		</Field>
		<Field label={keyLabel}>
			<Input bind:value={key} placeholder="可选" />
		</Field>
		{#if extraLabel}
			<Field label={extraLabel}>
				<Input bind:value={extra} placeholder="可选" />
			</Field>
		{/if}
	</div>
	<Field label="Payload">
		<Textarea bind:value={payload} rows={5} placeholder="消息正文" />
	</Field>
	<div class="row-actions">
		<Button type="submit" {busy}>发送</Button>
		{#if message}
			<span class="mute">{message}</span>
		{/if}
	</div>
</form>
