import { invoke } from './client';
import type {
	KafkaCluster,
	KafkaGroup,
	KafkaGroupDetail,
	KafkaTopicDetail,
	MessageView
} from '$lib/types/mq';

export function kafkaCluster(id: string) {
	return invoke<KafkaCluster>('kafka_cluster', { id });
}

export function kafkaTopicDetail(id: string, topic: string) {
	return invoke<KafkaTopicDetail>('kafka_topic_detail', { id, topic });
}

export function kafkaFetch(
	id: string,
	topic: string,
	partition: number,
	offset = 0,
	maxBytes = 1_000_000
) {
	return invoke<MessageView[]>('kafka_fetch', { id, topic, partition, offset, maxBytes });
}

export function kafkaProduce(
	id: string,
	topic: string,
	payload: string,
	options?: { partition?: number; key?: string; headers?: Record<string, string> }
) {
	return invoke<number>('kafka_produce', {
		id,
		topic,
		payload,
		partition: options?.partition ?? 0,
		key: options?.key,
		headers: options?.headers
	});
}

export function kafkaGroups(id: string) {
	return invoke<KafkaGroup[]>('kafka_groups', { id });
}

export function kafkaGroupDetail(id: string, groupId: string) {
	return invoke<KafkaGroupDetail>('kafka_group_detail', { id, groupId });
}
