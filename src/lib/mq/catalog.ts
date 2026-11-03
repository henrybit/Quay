import type { BrokerKind, BrokerMeta, ConnectionProfile } from '$lib/types/mq';

export const BROKER_CATALOG: Record<BrokerKind, BrokerMeta> = {
	rabbitmq: {
		kind: 'rabbitmq',
		label: 'RabbitMQ',
		short: 'AMQP',
		blurb: '交换机 · 队列 · 绑定 · 消息预览',
		accentVar: '--color-rabbit',
		tabs: [
			{ id: 'overview', label: '总览' },
			{ id: 'queues', label: '队列' },
			{ id: 'exchanges', label: '交换机' },
			{ id: 'bindings', label: '绑定' },
			{ id: 'messages', label: '消息' }
		],
		fields: [
			{
				key: 'managementUrl',
				label: 'Management URL',
				placeholder: 'http://127.0.0.1:15672',
				hint: '需要开启 management 插件'
			},
			{ key: 'username', label: '用户名', placeholder: 'guest' },
			{ key: 'password', label: '密码', secret: true, placeholder: 'guest' },
			{ key: 'vhost', label: 'Virtual Host', placeholder: '/' }
		]
	},
	kafka: {
		kind: 'kafka',
		label: 'Kafka',
		short: 'LOG',
		blurb: 'Topic · 分区偏移 · 消费组 Lag · 消息浏览',
		accentVar: '--color-kafka',
		tabs: [
			{ id: 'cluster', label: '集群' },
			{ id: 'topics', label: 'Topic' },
			{ id: 'groups', label: '消费组' },
			{ id: 'browser', label: '消息浏览' }
		],
		fields: [
			{
				key: 'brokers',
				label: 'Bootstrap Brokers',
				placeholder: '127.0.0.1:9092',
				multiline: true,
				hint: '多个地址用逗号或换行分隔'
			},
			{ key: 'clientId', label: 'Client ID', placeholder: 'quay' },
			{ key: 'saslUsername', label: 'SASL 用户名', placeholder: '可选' },
			{ key: 'saslPassword', label: 'SASL 密码', secret: true, placeholder: '可选' }
		]
	},
	rocketmq: {
		kind: 'rocketmq',
		label: 'RocketMQ',
		short: 'NS',
		blurb: 'NameServer · Topic 路由 · 消费进度 · 按 Key 查询',
		accentVar: '--color-rocket',
		tabs: [
			{ id: 'cluster', label: '集群' },
			{ id: 'topics', label: 'Topic' },
			{ id: 'consume', label: '消费进度' },
			{ id: 'query', label: '消息查询' }
		],
		fields: [
			{
				key: 'namesrv',
				label: 'NameServer',
				placeholder: '127.0.0.1:9876',
				multiline: true,
				hint: '多个地址用逗号或换行分隔'
			},
			{ key: 'accessKey', label: 'AccessKey', placeholder: '可选，ACL' },
			{ key: 'secretKey', label: 'SecretKey', secret: true, placeholder: '可选，ACL' }
		]
	}
};

export const BROKER_KINDS = Object.keys(BROKER_CATALOG) as BrokerKind[];

export function brokerMeta(kind: BrokerKind): BrokerMeta {
	return BROKER_CATALOG[kind];
}

export function emptyConfig(kind: BrokerKind): ConnectionProfile['config'] {
	if (kind === 'rabbitmq') {
		return {
			type: 'rabbitmq',
			managementUrl: 'http://127.0.0.1:15672',
			username: 'guest',
			password: 'guest',
			vhost: '/'
		};
	}
	if (kind === 'kafka') {
		return {
			type: 'kafka',
			brokers: '127.0.0.1:9092',
			clientId: 'quay',
			saslUsername: '',
			saslPassword: ''
		};
	}
	return {
		type: 'rocketmq',
		namesrv: '127.0.0.1:9876',
		accessKey: '',
		secretKey: ''
	};
}

export function emptyProfile(kind: BrokerKind = 'rabbitmq'): ConnectionProfile {
	return {
		id: '',
		name: '',
		kind,
		note: '',
		createdAt: '',
		updatedAt: '',
		config: emptyConfig(kind)
	};
}

export function endpointHint(profile: ConnectionProfile): string {
	const cfg = profile.config;
	if (cfg.type === 'rabbitmq') return cfg.managementUrl;
	if (cfg.type === 'kafka') return cfg.brokers.split(/[,\n]/)[0]?.trim() ?? '';
	return cfg.namesrv.split(/[,\n]/)[0]?.trim() ?? '';
}
