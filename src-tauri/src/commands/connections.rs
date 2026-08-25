use tauri::AppHandle;

use crate::error::AppResult;
use crate::models::{BrokerConfig, BrokerKind, ConnectionProfile, TestResult};
use crate::mq::kafka::client::KafkaService;
use crate::mq::rabbitmq::RabbitClient;
use crate::mq::rocketmq::RocketClient;
use crate::storage;
use crate::util::{now_iso, require_non_empty};

#[tauri::command]
pub fn list_connections(app: AppHandle) -> AppResult<Vec<ConnectionProfile>> {
    storage::load_all(&app)
}

#[tauri::command]
pub fn save_connection(
    app: AppHandle,
    mut profile: ConnectionProfile,
) -> AppResult<ConnectionProfile> {
    require_non_empty(&profile.name, "连接名称")?;
    sync_kind(&mut profile)?;
    validate_config(&profile)?;
    if profile.id.trim().is_empty() {
        profile.id = uuid::Uuid::new_v4().to_string();
        profile.created_at = now_iso();
    } else if let Ok(existing) = storage::get(&app, &profile.id) {
        profile.created_at = existing.created_at;
    } else if profile.created_at.trim().is_empty() {
        profile.created_at = now_iso();
    }
    profile.updated_at = now_iso();
    storage::upsert(&app, profile)
}

#[tauri::command]
pub fn delete_connection(app: AppHandle, id: String) -> AppResult<()> {
    storage::remove(&app, &id)
}

#[tauri::command]
pub fn get_connection(app: AppHandle, id: String) -> AppResult<ConnectionProfile> {
    storage::get(&app, &id)
}

#[tauri::command]
pub async fn test_connection(profile: ConnectionProfile) -> AppResult<TestResult> {
    let message = match profile.kind {
        BrokerKind::Rabbitmq => {
            let cfg = profile.rabbitmq().map_err(crate::error::AppError::msg)?;
            RabbitClient::new(cfg.clone())?.overview().await?;
            "已连接到 RabbitMQ Management API".into()
        }
        BrokerKind::Kafka => {
            let cfg = profile.kafka().map_err(crate::error::AppError::msg)?;
            let cluster = KafkaService::new(cfg.clone()).cluster().await?;
            format!(
                "已连接到 Kafka，发现 {} 个 topic",
                cluster.topics.len()
            )
        }
        BrokerKind::Rocketmq => {
            let cfg = profile.rocketmq().map_err(crate::error::AppError::msg)?;
            RocketClient::new(cfg.clone()).cluster().await?;
            "已连接到 RocketMQ NameServer".into()
        }
    };
    Ok(TestResult { ok: true, message })
}

fn sync_kind(profile: &mut ConnectionProfile) -> AppResult<()> {
    profile.kind = match &profile.config {
        BrokerConfig::Rabbitmq(_) => BrokerKind::Rabbitmq,
        BrokerConfig::Kafka(_) => BrokerKind::Kafka,
        BrokerConfig::Rocketmq(_) => BrokerKind::Rocketmq,
    };
    Ok(())
}

fn validate_config(profile: &ConnectionProfile) -> AppResult<()> {
    match &profile.config {
        BrokerConfig::Rabbitmq(cfg) => {
            require_non_empty(&cfg.management_url, "Management URL")?;
            require_non_empty(&cfg.username, "用户名")?;
        }
        BrokerConfig::Kafka(cfg) => require_non_empty(&cfg.brokers, "Brokers")?,
        BrokerConfig::Rocketmq(cfg) => require_non_empty(&cfg.namesrv, "NameServer")?,
    }
    Ok(())
}
