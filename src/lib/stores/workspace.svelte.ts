import type { BrokerKind, ConnectionProfile } from '$lib/types/mq';
import * as api from '$lib/api/connections';

class Workspace {
	connections = $state<ConnectionProfile[]>([]);
	loading = $state(false);
	error = $state<string | null>(null);
	kindFilter = $state<BrokerKind | 'all'>('all');
	query = $state('');
	editorOpen = $state(false);
	editing = $state<ConnectionProfile | null>(null);

	get filtered(): ConnectionProfile[] {
		const q = this.query.trim().toLowerCase();
		return this.connections.filter((item) => {
			if (this.kindFilter !== 'all' && item.kind !== this.kindFilter) return false;
			if (!q) return true;
			return `${item.name} ${item.note}`.toLowerCase().includes(q);
		});
	}

	async refresh() {
		this.loading = true;
		this.error = null;
		try {
			this.connections = await api.listConnections();
		} catch (error) {
			this.error = error instanceof Error ? error.message : '加载连接失败';
		} finally {
			this.loading = false;
		}
	}

	openCreate(kind?: BrokerKind) {
		this.editing = null;
		this.pendingKind = kind ?? 'rabbitmq';
		this.editorOpen = true;
	}

	pendingKind = $state<BrokerKind>('rabbitmq');

	openEdit(profile: ConnectionProfile) {
		this.editing = structuredClone(profile);
		this.pendingKind = profile.kind;
		this.editorOpen = true;
	}

	closeEditor() {
		this.editorOpen = false;
		this.editing = null;
	}

	async persist(profile: ConnectionProfile) {
		const saved = await api.saveConnection(profile);
		await this.refresh();
		this.closeEditor();
		return saved;
	}

	async remove(id: string) {
		await api.deleteConnection(id);
		await this.refresh();
	}
}

export const workspace = new Workspace();
