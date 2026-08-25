use reqwest::{Client, Method, RequestBuilder};
use serde_json::{json, Value};

use crate::error::{AppError, AppResult};
use crate::models::{MessageView, RabbitMqConfig};

#[derive(Clone)]
pub struct RabbitClient {
    http: Client,
    config: RabbitMqConfig,
}

impl RabbitClient {
    pub fn new(config: RabbitMqConfig) -> AppResult<Self> {
        let http = Client::builder()
            .timeout(std::time::Duration::from_secs(15))
            .build()?;
        Ok(Self { http, config })
    }

    fn encode_vhost(&self) -> String {
        urlencoding::encode(self.config.vhost.trim()).into_owned()
    }

    fn url(&self, path: &str) -> String {
        let base = self.config.management_url.trim().trim_end_matches('/');
        format!("{base}{path}")
    }

    fn auth(&self, builder: RequestBuilder) -> RequestBuilder {
        builder.basic_auth(&self.config.username, Some(&self.config.password))
    }

    async fn send(&self, method: Method, path: &str, body: Option<Value>) -> AppResult<Value> {
        let mut builder = self.auth(self.http.request(method, self.url(path)));
        if let Some(payload) = body {
            builder = builder.json(&payload);
        }
        let response = builder.send().await?;
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        if !status.is_success() {
            return Err(AppError::msg(format!(
                "RabbitMQ Management API {status}: {text}"
            )));
        }
        if text.trim().is_empty() {
            return Ok(Value::Null);
        }
        serde_json::from_str(&text).map_err(|err| AppError::msg(err.to_string()))
    }

    pub async fn overview(&self) -> AppResult<Value> {
        self.send(Method::GET, "/api/overview", None).await
    }

    pub async fn queues(&self) -> AppResult<Value> {
        let path = format!("/api/queues/{}", self.encode_vhost());
        self.send(Method::GET, &path, None).await
    }

    pub async fn exchanges(&self) -> AppResult<Value> {
        let path = format!("/api/exchanges/{}", self.encode_vhost());
        self.send(Method::GET, &path, None).await
    }

    pub async fn bindings(&self) -> AppResult<Value> {
        let path = format!("/api/bindings/{}", self.encode_vhost());
        self.send(Method::GET, &path, None).await
    }

    pub async fn consumers(&self) -> AppResult<Value> {
        self.send(Method::GET, "/api/consumers", None).await
    }

    pub async fn connections(&self) -> AppResult<Value> {
        self.send(Method::GET, "/api/connections", None).await
    }

    pub async fn get_messages(
        &self,
        queue: &str,
        count: u32,
        ack_requeue: bool,
    ) -> AppResult<Vec<MessageView>> {
        let path = format!("/api/queues/{}/{}/get", self.encode_vhost(), urlencoding::encode(queue));
        let ackmode = if ack_requeue {
            "ack_requeue_true"
        } else {
            "ack_requeue_false"
        };
        let body = json!({
            "count": count.max(1),
            "ackmode": ackmode,
            "encoding": "auto",
            "truncate": 50000
        });
        let value = self.send(Method::POST, &path, Some(body)).await?;
        let rows = value.as_array().cloned().unwrap_or_default();
        Ok(rows
            .into_iter()
            .map(|row| MessageView {
                offset: row.get("message_count").and_then(Value::as_i64),
                partition: None,
                key: row
                    .pointer("/properties/message_id")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string(),
                payload: row
                    .get("payload")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string(),
                timestamp: row
                    .pointer("/properties/timestamp")
                    .and_then(|item| item.as_str().map(ToOwned::to_owned).or_else(|| item.as_i64().map(|n| n.to_string()))),
                properties: row.get("properties").cloned().unwrap_or(Value::Null),
            })
            .collect())
    }

    pub async fn publish(
        &self,
        exchange: &str,
        routing_key: &str,
        payload: &str,
        properties: Option<Value>,
    ) -> AppResult<Value> {
        let path = format!(
            "/api/exchanges/{}/{}/publish",
            self.encode_vhost(),
            urlencoding::encode(exchange)
        );
        let body = json!({
            "properties": properties.unwrap_or_else(|| json!({})),
            "routing_key": routing_key,
            "payload": payload,
            "payload_encoding": "string"
        });
        self.send(Method::POST, &path, Some(body)).await
    }
}
