<script lang="ts">
	import { workspace } from '$lib/stores/workspace.svelte';
	import Modal from '$lib/components/ui/Modal.svelte';
	import ConnectionForm from '$lib/components/connection/ConnectionForm.svelte';
	import type { ConnectionProfile } from '$lib/types/mq';
	import { goto } from '$app/navigation';

	async function save(profile: ConnectionProfile) {
		const saved = await workspace.persist(profile);
		await goto(`/c/${saved.id}`);
	}
</script>

<Modal
	open={workspace.editorOpen}
	title={workspace.editing ? '编辑连接' : '新建连接'}
	onclose={() => workspace.closeEditor()}
>
	<ConnectionForm
		profile={workspace.editing}
		initialKind={workspace.pendingKind}
		oncancel={() => workspace.closeEditor()}
		onsave={save}
	/>
</Modal>
