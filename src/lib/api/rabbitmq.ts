import { invoke } from './client';
import type { MessageView } from '$lib/types/mq';

export function rabbitOverview(id: string) {
	return invoke<Record<string, unknown>>('rabbit_overview', { id });
}

export function rabbitQueues(id: string) {
	return invoke<unknown[]>('rabbit_queues', { id });
}

export function rabbitExchanges(id: string) {
	return invoke<unknown[]>('rabbit_exchanges', { id });
}

export function rabbitBindings(id: string) {
	return invoke<unknown[]>('rabbit_bindings', { id });
}

export function rabbitConsumers(id: string) {
	return invoke<unknown[]>('rabbit_consumers', { id });
}

export function rabbitConnections(id: string) {
	return invoke<unknown[]>('rabbit_connections', { id });
}

export function rabbitGetMessages(id: string, queue: string, count = 20, requeue = true) {
	return invoke<MessageView[]>('rabbit_get_messages', { id, queue, count, requeue });
}

export function rabbitPublish(
	id: string,
	exchange: string,
	routingKey: string,
	payload: string,
	properties?: unknown
) {
	return invoke<unknown>('rabbit_publish', {
		id,
		exchange,
		routingKey,
		payload,
		properties
	});
}
