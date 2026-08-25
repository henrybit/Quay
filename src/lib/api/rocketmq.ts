import { invoke } from './client';
import type { MessageView, RocketCluster, RocketTopicList } from '$lib/types/mq';

export function rocketCluster(id: string) {
	return invoke<RocketCluster>('rocket_cluster', { id });
}

export function rocketTopics(id: string) {
	return invoke<RocketTopicList>('rocket_topics', { id });
}

export function rocketTopicRoute(id: string, topic: string) {
	return invoke<unknown>('rocket_topic_route', { id, topic });
}

export function rocketTopicStats(id: string, topic: string) {
	return invoke<unknown>('rocket_topic_stats', { id, topic });
}

export function rocketConsumeStats(id: string, group: string, topic?: string) {
	return invoke<unknown>('rocket_consume_stats', { id, group, topic });
}

export function rocketQueryMessages(
	id: string,
	topic: string,
	key: string,
	options?: { begin?: number; end?: number; maxNum?: number }
) {
	return invoke<MessageView[]>('rocket_query_messages', {
		id,
		topic,
		key,
		begin: options?.begin,
		end: options?.end,
		maxNum: options?.maxNum
	});
}

export function rocketSend(
	id: string,
	topic: string,
	payload: string,
	options?: { keys?: string; tags?: string }
) {
	return invoke<unknown>('rocket_send', {
		id,
		topic,
		payload,
		keys: options?.keys,
		tags: options?.tags
	});
}
