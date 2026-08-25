export type BrokerKind = 'rabbitmq' | 'kafka' | 'rocketmq';

export type RabbitMqConfig = {
	type: 'rabbitmq';
	managementUrl: string;
	username: string;
	password: string;
	vhost: string;
};

export type KafkaConfig = {
	type: 'kafka';
	brokers: string;
	clientId: string;
	saslUsername: string;
	saslPassword: string;
};

export type RocketMqConfig = {
	type: 'rocketmq';
	namesrv: string;
	accessKey: string;
	secretKey: string;
};

export type BrokerConfig = RabbitMqConfig | KafkaConfig | RocketMqConfig;

export type ConnectionProfile = {
	id: string;
	name: string;
	kind: BrokerKind;
	note: string;
	createdAt: string;
	updatedAt: string;
	config: BrokerConfig;
};

export type FieldDef = {
	key: string;
	label: string;
	placeholder?: string;
	secret?: boolean;
	multiline?: boolean;
	hint?: string;
};

export type TabDef = {
	id: string;
	label: string;
};

export type BrokerMeta = {
	kind: BrokerKind;
	label: string;
	short: string;
	blurb: string;
	accentVar: string;
	tabs: TabDef[];
	fields: FieldDef[];
};

export type MessageView = {
	offset?: number | null;
	partition?: number | null;
	key: string;
	payload: string;
	timestamp?: string | null;
	properties: unknown;
};

export type KafkaTopicSummary = {
	name: string;
	partitionCount: number;
	partitions: number[];
};

export type KafkaPartitionDetail = {
	partition: number;
	earliest: number;
	latest: number;
};

export type KafkaTopicDetail = {
	name: string;
	partitions: KafkaPartitionDetail[];
};

export type KafkaBroker = {
	nodeId: number;
	host: string;
	port: number;
};

export type KafkaCluster = {
	brokers: KafkaBroker[];
	topics: KafkaTopicSummary[];
};

export type KafkaGroup = {
	groupId: string;
	protocolType: string;
	broker: string;
};

export type KafkaGroupMember = {
	memberId: string;
	clientId: string;
	clientHost: string;
};

export type KafkaLagRow = {
	topic: string;
	partition: number;
	committed: number;
	latest: number;
	lag: number;
};

export type KafkaGroupDetail = {
	groupId: string;
	state: string;
	protocolType: string;
	protocol: string;
	members: KafkaGroupMember[];
	lag: KafkaLagRow[];
};

export type RocketCluster = {
	namesrv: string;
	raw: unknown;
};

export type RocketTopicList = {
	topics: string[];
};

export type TestResult = {
	ok: boolean;
	message: string;
};
