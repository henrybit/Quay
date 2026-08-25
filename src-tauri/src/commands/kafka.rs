use std::collections::BTreeMap;

use tauri::AppHandle;

use crate::error::AppResult;
use crate::models::{
    KafkaCluster, KafkaGroup, KafkaGroupDetail, KafkaTopicDetail, MessageView,
};
use crate::mq::kafka::client::KafkaService;
use crate::storage;

#[tauri::command]
pub async fn kafka_cluster(app: AppHandle, id: String) -> AppResult<KafkaCluster> {
    service(&app, &id)?.cluster().await
}

#[tauri::command]
pub async fn kafka_topic_detail(
    app: AppHandle,
    id: String,
    topic: String,
) -> AppResult<KafkaTopicDetail> {
    service(&app, &id)?.topic_detail(&topic).await
}

#[tauri::command]
pub async fn kafka_fetch(
    app: AppHandle,
    id: String,
    topic: String,
    partition: i32,
    offset: Option<i64>,
    max_bytes: Option<i32>,
) -> AppResult<Vec<MessageView>> {
    service(&app, &id)?
        .fetch(&topic, partition, offset.unwrap_or(0), max_bytes.unwrap_or(1_000_000))
        .await
}

#[tauri::command]
pub async fn kafka_produce(
    app: AppHandle,
    id: String,
    topic: String,
    partition: Option<i32>,
    key: Option<String>,
    payload: String,
    headers: Option<BTreeMap<String, String>>,
) -> AppResult<i64> {
    service(&app, &id)?
        .produce(
            &topic,
            partition.unwrap_or(0),
            key,
            payload,
            headers.unwrap_or_default(),
        )
        .await
}

#[tauri::command]
pub async fn kafka_groups(app: AppHandle, id: String) -> AppResult<Vec<KafkaGroup>> {
    service(&app, &id)?.admin()?.list_groups().await
}

#[tauri::command]
pub async fn kafka_group_detail(
    app: AppHandle,
    id: String,
    group_id: String,
) -> AppResult<KafkaGroupDetail> {
    let mut detail = service(&app, &id)?
        .admin()?
        .describe_group(&group_id)
        .await?;
    if let Ok(svc) = service(&app, &id) {
        for row in detail.lag.iter_mut() {
            if let Ok(topic) = svc.topic_detail(&row.topic).await {
                if let Some(part) = topic
                    .partitions
                    .iter()
                    .find(|item| item.partition == row.partition)
                {
                    row.latest = part.latest;
                    row.lag = (part.latest - row.committed).max(0);
                }
            }
        }
    }
    Ok(detail)
}

fn service(app: &AppHandle, id: &str) -> AppResult<KafkaService> {
    let profile = storage::get(app, id)?;
    let cfg = profile.kafka().map_err(crate::error::AppError::msg)?;
    Ok(KafkaService::new(cfg.clone()))
}
