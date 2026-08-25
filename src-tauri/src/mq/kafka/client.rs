use std::collections::BTreeMap;

use chrono::Utc;
use rskafka::client::partition::{Compression, OffsetAt, UnknownTopicHandling};
use rskafka::client::{ClientBuilder, Credentials, SaslConfig};
use rskafka::record::Record;

use crate::error::{AppError, AppResult};
use crate::models::{
    KafkaCluster, KafkaConfig, KafkaPartitionDetail, KafkaTopicDetail, KafkaTopicSummary,
    MessageView,
};
use crate::util::{bytes_preview, split_endpoints};

use super::protocol::KafkaAdmin;

pub struct KafkaService {
    config: KafkaConfig,
}

impl KafkaService {
    pub fn new(config: KafkaConfig) -> Self {
        Self { config }
    }

    fn brokers(&self) -> AppResult<Vec<String>> {
        let items = split_endpoints(&self.config.brokers);
        if items.is_empty() {
            Err(AppError::msg("Kafka brokers 不能为空"))
        } else {
            Ok(items)
        }
    }

    async fn client(&self) -> AppResult<rskafka::client::Client> {
        let mut builder = ClientBuilder::new(self.brokers()?);
        if !self.config.client_id.trim().is_empty() {
            builder = builder.client_id(self.config.client_id.clone());
        }
        if !self.config.sasl_username.trim().is_empty() {
            builder = builder.sasl_config(SaslConfig::Plain(Credentials::new(
                self.config.sasl_username.clone(),
                self.config.sasl_password.clone(),
            )));
        }
        builder
            .build()
            .await
            .map_err(|err| AppError::msg(format!("连接 Kafka 失败: {err}")))
    }

    pub async fn cluster(&self) -> AppResult<KafkaCluster> {
        let admin = KafkaAdmin::new(self.brokers()?[0].clone(), self.config.client_id.clone());
        match admin.metadata().await {
            Ok((brokers, topics)) => Ok(KafkaCluster {
                brokers,
                topics: topics
                    .into_iter()
                    .map(|topic| KafkaTopicSummary {
                        name: topic.name,
                        partition_count: topic.partitions.len(),
                        partitions: topic.partitions.into_iter().map(|item| item.id).collect(),
                    })
                    .collect(),
            }),
            Err(_) => {
                let client = self.client().await?;
                let topics = client
                    .list_topics()
                    .await
                    .map_err(|err| AppError::msg(err.to_string()))?;
                Ok(KafkaCluster {
                    brokers: vec![],
                    topics: topics
                        .into_iter()
                        .map(|topic| KafkaTopicSummary {
                            name: topic.name,
                            partition_count: topic.partitions.len(),
                            partitions: topic.partitions.into_iter().collect(),
                        })
                        .collect(),
                })
            }
        }
    }

    pub async fn topic_detail(&self, topic: &str) -> AppResult<KafkaTopicDetail> {
        let client = self.client().await?;
        let listed = client
            .list_topics()
            .await
            .map_err(|err| AppError::msg(err.to_string()))?;
        let found = listed
            .into_iter()
            .find(|item| item.name == topic)
            .ok_or_else(|| AppError::msg("Topic 不存在"))?;
        let mut partitions = Vec::new();
        for partition in found.partitions {
            let pc = client
                .partition_client(topic.to_string(), partition, UnknownTopicHandling::Retry)
                .await
                .map_err(|err| AppError::msg(err.to_string()))?;
            let earliest = pc
                .get_offset(OffsetAt::Earliest)
                .await
                .map_err(|err| AppError::msg(err.to_string()))?;
            let latest = pc
                .get_offset(OffsetAt::Latest)
                .await
                .map_err(|err| AppError::msg(err.to_string()))?;
            partitions.push(KafkaPartitionDetail {
                partition,
                earliest,
                latest,
            });
        }
        partitions.sort_by_key(|item| item.partition);
        Ok(KafkaTopicDetail {
            name: topic.to_string(),
            partitions,
        })
    }

    pub async fn fetch(
        &self,
        topic: &str,
        partition: i32,
        offset: i64,
        max_bytes: i32,
    ) -> AppResult<Vec<MessageView>> {
        let client = self.client().await?;
        let pc = client
            .partition_client(topic.to_string(), partition, UnknownTopicHandling::Retry)
            .await
            .map_err(|err| AppError::msg(err.to_string()))?;
        let start = if offset < 0 {
            pc.get_offset(OffsetAt::Earliest)
                .await
                .map_err(|err| AppError::msg(err.to_string()))?
        } else {
            offset
        };
        let (records, _hw) = pc
            .fetch_records(start, 1..max_bytes.max(1024), 1_000)
            .await
            .map_err(|err| AppError::msg(err.to_string()))?;
        Ok(records
            .into_iter()
            .map(|item| MessageView {
                offset: Some(item.offset),
                partition: Some(partition),
                key: bytes_preview(item.record.key.as_deref()),
                payload: bytes_preview(item.record.value.as_deref()),
                timestamp: Some(item.record.timestamp.to_rfc3339()),
                properties: serde_json::to_value(
                    item.record
                        .headers
                        .into_iter()
                        .map(|(k, v)| (k, bytes_preview(Some(&v))))
                        .collect::<BTreeMap<_, _>>(),
                )
                .unwrap_or(serde_json::Value::Null),
            })
            .collect())
    }

    pub async fn produce(
        &self,
        topic: &str,
        partition: i32,
        key: Option<String>,
        payload: String,
        headers: BTreeMap<String, String>,
    ) -> AppResult<i64> {
        let client = self.client().await?;
        let pc = client
            .partition_client(topic.to_string(), partition, UnknownTopicHandling::Retry)
            .await
            .map_err(|err| AppError::msg(err.to_string()))?;
        let record = Record {
            key: key.map(|text| text.into_bytes()),
            value: Some(payload.into_bytes()),
            headers: headers
                .into_iter()
                .map(|(k, v)| (k, v.into_bytes()))
                .collect(),
            timestamp: Utc::now(),
        };
        let offsets = pc
            .produce(vec![record], Compression::NoCompression)
            .await
            .map_err(|err| AppError::msg(err.to_string()))?;
        Ok(offsets.first().copied().unwrap_or(-1))
    }

    pub fn admin(&self) -> AppResult<KafkaAdmin> {
        Ok(KafkaAdmin::new(
            self.brokers()?[0].clone(),
            self.config.client_id.clone(),
        ))
    }
}
