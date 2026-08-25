use serde_json::Value;
use tauri::AppHandle;

use crate::error::AppResult;
use crate::models::{MessageView, RocketCluster, RocketTopicList};
use crate::mq::rocketmq::RocketClient;
use crate::storage;

#[tauri::command]
pub async fn rocket_cluster(app: AppHandle, id: String) -> AppResult<RocketCluster> {
    client(&app, &id)?.cluster().await
}

#[tauri::command]
pub async fn rocket_topics(app: AppHandle, id: String) -> AppResult<RocketTopicList> {
    client(&app, &id)?.topics().await
}

#[tauri::command]
pub async fn rocket_topic_route(app: AppHandle, id: String, topic: String) -> AppResult<Value> {
    client(&app, &id)?.topic_route(&topic).await
}

#[tauri::command]
pub async fn rocket_topic_stats(app: AppHandle, id: String, topic: String) -> AppResult<Value> {
    client(&app, &id)?.topic_stats(&topic).await
}

#[tauri::command]
pub async fn rocket_consume_stats(
    app: AppHandle,
    id: String,
    group: String,
    topic: Option<String>,
) -> AppResult<Value> {
    client(&app, &id)?
        .consume_stats(&group, topic.as_deref())
        .await
}

#[tauri::command]
pub async fn rocket_query_messages(
    app: AppHandle,
    id: String,
    topic: String,
    key: String,
    begin: Option<i64>,
    end: Option<i64>,
    max_num: Option<i32>,
) -> AppResult<Vec<MessageView>> {
    let now = chrono::Utc::now().timestamp_millis();
    client(&app, &id)?
        .query_messages(
            &topic,
            &key,
            begin.unwrap_or(now - 86_400_000),
            end.unwrap_or(now),
            max_num.unwrap_or(32),
        )
        .await
}

#[tauri::command]
pub async fn rocket_send(
    app: AppHandle,
    id: String,
    topic: String,
    payload: String,
    keys: Option<String>,
    tags: Option<String>,
) -> AppResult<Value> {
    client(&app, &id)?
        .send(&topic, &payload, keys.as_deref(), tags.as_deref())
        .await
}

fn client(app: &AppHandle, id: &str) -> AppResult<RocketClient> {
    let profile = storage::get(app, id)?;
    let cfg = profile
        .rocketmq()
        .map_err(crate::error::AppError::msg)?;
    Ok(RocketClient::new(cfg.clone()))
}
