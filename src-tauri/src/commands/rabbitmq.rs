use serde_json::Value;
use tauri::AppHandle;

use crate::error::AppResult;
use crate::models::MessageView;
use crate::mq::rabbitmq::RabbitClient;
use crate::storage;

#[tauri::command]
pub async fn rabbit_overview(app: AppHandle, id: String) -> AppResult<Value> {
    client(&app, &id)?.overview().await
}

#[tauri::command]
pub async fn rabbit_queues(app: AppHandle, id: String) -> AppResult<Value> {
    client(&app, &id)?.queues().await
}

#[tauri::command]
pub async fn rabbit_exchanges(app: AppHandle, id: String) -> AppResult<Value> {
    client(&app, &id)?.exchanges().await
}

#[tauri::command]
pub async fn rabbit_bindings(app: AppHandle, id: String) -> AppResult<Value> {
    client(&app, &id)?.bindings().await
}

#[tauri::command]
pub async fn rabbit_consumers(app: AppHandle, id: String) -> AppResult<Value> {
    client(&app, &id)?.consumers().await
}

#[tauri::command]
pub async fn rabbit_connections(app: AppHandle, id: String) -> AppResult<Value> {
    client(&app, &id)?.connections().await
}

#[tauri::command]
pub async fn rabbit_get_messages(
    app: AppHandle,
    id: String,
    queue: String,
    count: Option<u32>,
    requeue: Option<bool>,
) -> AppResult<Vec<MessageView>> {
    client(&app, &id)?
        .get_messages(&queue, count.unwrap_or(20), requeue.unwrap_or(true))
        .await
}

#[tauri::command]
pub async fn rabbit_publish(
    app: AppHandle,
    id: String,
    exchange: String,
    routing_key: String,
    payload: String,
    properties: Option<Value>,
) -> AppResult<Value> {
    client(&app, &id)?
        .publish(&exchange, &routing_key, &payload, properties)
        .await
}

fn client(app: &AppHandle, id: &str) -> AppResult<RabbitClient> {
    let profile = storage::get(app, id)?;
    let cfg = profile
        .rabbitmq()
        .map_err(crate::error::AppError::msg)?;
    RabbitClient::new(cfg.clone())
}
