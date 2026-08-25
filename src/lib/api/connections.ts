import { invoke } from './client';
import type { ConnectionProfile, TestResult } from '$lib/types/mq';

export function listConnections() {
	return invoke<ConnectionProfile[]>('list_connections');
}

export function saveConnection(profile: ConnectionProfile) {
	return invoke<ConnectionProfile>('save_connection', { profile });
}

export function deleteConnection(id: string) {
	return invoke<void>('delete_connection', { id });
}

export function getConnection(id: string) {
	return invoke<ConnectionProfile>('get_connection', { id });
}

export function testConnection(profile: ConnectionProfile) {
	return invoke<TestResult>('test_connection', { profile });
}
