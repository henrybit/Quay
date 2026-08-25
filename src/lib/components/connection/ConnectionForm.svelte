<script lang="ts">
	import type { BrokerKind, ConnectionProfile } from '$lib/types/mq';
	import { BROKER_CATALOG, brokerMeta, emptyConfig, emptyProfile } from '$lib/mq/catalog';
	import { testConnection } from '$lib/api/connections';
	import Button from '$lib/components/ui/Button.svelte';
	import Field from '$lib/components/ui/Field.svelte';
	import Input from '$lib/components/ui/Input.svelte';
	import Textarea from '$lib/components/ui/Textarea.svelte';
	import BrokerIcon from '$lib/components/brand/BrokerIcon.svelte';

	type Props = {
		profile?: ConnectionProfile | null;
		initialKind?: BrokerKind;
		oncancel: () => void;
		onsave: (profile: ConnectionProfile) => Promise<void>;
	};

	let { profile = null, initialKind = 'rabbitmq', oncancel, onsave }: Props = $props();

	let draft = $state<ConnectionProfile>(emptyProfile());
	let busy = $state(false);
	let testing = $state(false);
	let notice = $state('');

	$effect(() => {
		draft = profile ? structuredClone(profile) : emptyProfile(initialKind);
	});

	function switchKind(kind: BrokerKind) {
		draft = {
			...draft,
			kind,
			config: emptyConfig(kind)
		};
	}

	function setField(key: string, value: string) {
		draft = {
			...draft,
			config: {
				...draft.config,
				[key]: value
			}
		};
	}

	async function save() {
		busy = true;
		notice = '';
		try {
			await onsave(draft);
		} catch (error) {
			notice = error instanceof Error ? error.message : '保存失败';
		} finally {
			busy = false;
		}
	}

	async function test() {
		testing = true;
		notice = '';
		try {
			const result = await testConnection(draft);
			notice = result.message;
		} catch (error) {
			notice = error instanceof Error ? error.message : '测试失败';
		} finally {
			testing = false;
		}
	}

	const meta = $derived(brokerMeta(draft.kind));
</script>

<form
	class="conn-form"
	onsubmit={(event) => {
		event.preventDefault();
		void save();
	}}
>
	<div class="kind-picker">
		{#each Object.values(BROKER_CATALOG) as item (item.kind)}
			<button
				type="button"
				class="kind-card"
				class:active={draft.kind === item.kind}
				onclick={() => switchKind(item.kind)}
			>
				<BrokerIcon kind={item.kind} size={22} />
				<span>
					<strong>{item.label}</strong>
					<small>{item.short}</small>
				</span>
			</button>
		{/each}
	</div>

	<Field label="连接名称">
		<Input bind:value={draft.name} placeholder="例如 生产环境订单集群" />
	</Field>
	<Field label="备注">
		<Input bind:value={draft.note} placeholder="可选" />
	</Field>

	{#each meta.fields as field (field.key)}
		<Field label={field.label} hint={field.hint}>
			{#if field.multiline}
				<Textarea
					value={(draft.config as Record<string, string>)[field.key] ?? ''}
					placeholder={field.placeholder}
					rows={3}
					oninput={(value) => setField(field.key, value)}
				/>
			{:else}
				<Input
					type={field.secret ? 'password' : 'text'}
					value={(draft.config as Record<string, string>)[field.key] ?? ''}
					placeholder={field.placeholder}
					oninput={(value) => setField(field.key, value)}
				/>
			{/if}
		</Field>
	{/each}

	{#if notice}
		<p class="banner">{notice}</p>
	{/if}

	<div class="row-actions">
		<Button variant="quiet" onclick={oncancel}>取消</Button>
		<Button variant="ghost" busy={testing} onclick={test}>测试连接</Button>
		<Button type="submit" kind={draft.kind} {busy}>保存</Button>
	</div>
</form>
