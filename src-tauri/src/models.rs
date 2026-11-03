use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum BrokerKind {
    Rabbitmq,
    Kafka,
    Rocketmq,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionProfile {
    pub id: String,
    pub name: String,
    pub kind: BrokerKind,
    #[serde(default)]
    pub note: String,
    pub created_at: String,
    pub updated_at: String,
    pub config: BrokerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum BrokerConfig {
    Rabbitmq(RabbitMqConfig),
    Kafka(KafkaConfig),
    Rocketmq(RocketMqConfig),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RabbitMqConfig {
    pub management_url: String,
    pub username: String,
    pub password: String,
    #[serde(default = "default_vhost")]
    pub vhost: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KafkaConfig {
    pub brokers: String,
    #[serde(default = "default_client_id")]
    pub client_id: String,
    #[serde(default)]
    pub sasl_username: String,
    #[serde(default)]
    pub sasl_password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RocketMqConfig {
    pub namesrv: String,
    #[serde(default)]
    pub access_key: String,
    #[serde(default)]
    pub secret_key: String,
}

fn default_vhost() -> String {
    "/".into()
}

fn default_client_id() -> String {
    "quay".into()
}

impl ConnectionProfile {
    pub fn rabbitmq(&self) -> Result<&RabbitMqConfig, String> {
        match &self.config {
            BrokerConfig::Rabbitmq(cfg) => Ok(cfg),
            _ => Err("当前连接不是 RabbitMQ".into()),
        }
    }

    pub fn kafka(&self) -> Result<&KafkaConfig, String> {
        match &self.config {
            BrokerConfig::Kafka(cfg) => Ok(cfg),
            _ => Err("当前连接不是 Kafka".into()),
        }
    }

    pub fn rocketmq(&self) -> Result<&RocketMqConfig, String> {
        match &self.config {
            BrokerConfig::Rocketmq(cfg) => Ok(cfg),
            _ => Err("当前连接不是 RocketMQ".into()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestResult {
    pub ok: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageView {
    pub offset: Option<i64>,
    pub partition: Option<i32>,
    pub key: String,
    pub payload: String,
    pub timestamp: Option<String>,
    pub properties: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KafkaTopicSummary {
    pub name: String,
    pub partition_count: usize,
    pub partitions: Vec<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KafkaPartitionDetail {
    pub partition: i32,
    pub earliest: i64,
    pub latest: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KafkaTopicDetail {
    pub name: String,
    pub partitions: Vec<KafkaPartitionDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KafkaGroup {
    pub group_id: String,
    pub protocol_type: String,
    pub broker: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KafkaGroupDetail {
    pub group_id: String,
    pub state: String,
    pub protocol_type: String,
    pub protocol: String,
    pub members: Vec<KafkaGroupMember>,
    pub lag: Vec<KafkaLagRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KafkaGroupMember {
    pub member_id: String,
    pub client_id: String,
    pub client_host: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KafkaLagRow {
    pub topic: String,
    pub partition: i32,
    pub committed: i64,
    pub latest: i64,
    pub lag: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KafkaBroker {
    pub node_id: i32,
    pub host: String,
    pub port: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KafkaCluster {
    pub brokers: Vec<KafkaBroker>,
    pub topics: Vec<KafkaTopicSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RocketCluster {
    pub namesrv: String,
    pub raw: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RocketTopicList {
    pub topics: Vec<String>,
}
