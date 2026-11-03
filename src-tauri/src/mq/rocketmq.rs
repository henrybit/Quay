use std::sync::atomic::{AtomicI32, Ordering};

use serde_json::{json, Value};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};

use crate::error::{AppError, AppResult};
use crate::models::{MessageView, RocketCluster, RocketMqConfig, RocketTopicList};
use crate::util::{bytes_preview, split_endpoints};

const GET_ROUTEINFO_BY_TOPIC: i32 = 105;
const GET_BROKER_CLUSTER_INFO: i32 = 106;
const GET_ALL_TOPIC_LIST_FROM_NAMESERVER: i32 = 206;
const GET_TOPIC_STATS_INFO: i32 = 202;
const GET_CONSUME_STATS: i32 = 208;
const QUERY_MESSAGE: i32 = 12;
const SEND_MESSAGE: i32 = 10;
const SUCCESS: i32 = 0;

static OPAQUE: AtomicI32 = AtomicI32::new(1);

pub struct RocketClient {
    config: RocketMqConfig,
}

impl RocketClient {
    pub fn new(config: RocketMqConfig) -> Self {
        Self { config }
    }

    fn namesrv_list(&self) -> AppResult<Vec<String>> {
        let items = split_endpoints(&self.config.namesrv);
        if items.is_empty() {
            Err(AppError::msg("NameServer 不能为空"))
        } else {
            Ok(items)
        }
    }

    async fn invoke_namesrv(&self, code: i32, ext: Value, body: Option<Vec<u8>>) -> AppResult<RemotingResponse> {
        let mut last_error = AppError::msg("没有可用的 NameServer");
        for addr in self.namesrv_list()? {
            match invoke(&addr, code, ext.clone(), body.clone(), &self.config).await {
                Ok(resp) => return Ok(resp),
                Err(err) => last_error = err,
            }
        }
        Err(last_error)
    }

    pub async fn cluster(&self) -> AppResult<RocketCluster> {
        let resp = self
            .invoke_namesrv(GET_BROKER_CLUSTER_INFO, json!({}), None)
            .await?;
        resp.ensure_ok()?;
        Ok(RocketCluster {
            namesrv: self.config.namesrv.clone(),
            raw: resp.body_json(),
        })
    }

    pub async fn topics(&self) -> AppResult<RocketTopicList> {
        let resp = self
            .invoke_namesrv(GET_ALL_TOPIC_LIST_FROM_NAMESERVER, json!({}), None)
            .await?;
        resp.ensure_ok()?;
        let value = resp.body_json();
        let topics = value
            .get("topicList")
            .and_then(Value::as_array)
            .map(|items| {
                items
                    .iter()
                    .filter_map(Value::as_str)
                    .map(ToOwned::to_owned)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        Ok(RocketTopicList { topics })
    }

    pub async fn topic_route(&self, topic: &str) -> AppResult<Value> {
        let resp = self
            .invoke_namesrv(GET_ROUTEINFO_BY_TOPIC, json!({ "topic": topic }), None)
            .await?;
        resp.ensure_ok()?;
        Ok(resp.body_json())
    }

    pub async fn topic_stats(&self, topic: &str) -> AppResult<Value> {
        let broker = self.pick_broker(topic).await?;
        let resp = invoke(
            &broker,
            GET_TOPIC_STATS_INFO,
            json!({ "topic": topic }),
            None,
            &self.config,
        )
        .await?;
        resp.ensure_ok()?;
        Ok(resp.body_json())
    }

    pub async fn consume_stats(&self, group: &str, topic: Option<&str>) -> AppResult<Value> {
        let route_topic = topic.unwrap_or("TBW102");
        let broker = match self.pick_broker(route_topic).await {
            Ok(addr) => addr,
            Err(_) => self.pick_any_broker().await?,
        };
        let mut ext = json!({ "consumerGroup": group });
        if let Some(topic) = topic {
            ext["topic"] = json!(topic);
        }
        let resp = invoke(&broker, GET_CONSUME_STATS, ext, None, &self.config).await?;
        resp.ensure_ok()?;
        Ok(resp.body_json())
    }

    pub async fn query_messages(
        &self,
        topic: &str,
        key: &str,
        begin: i64,
        end: i64,
        max_num: i32,
    ) -> AppResult<Vec<MessageView>> {
        let broker = self.pick_broker(topic).await?;
        let ext = json!({
            "topic": topic,
            "key": key,
            "maxNum": max_num.max(1).to_string(),
            "beginTimestamp": begin.to_string(),
            "endTimestamp": end.to_string()
        });
        let resp = invoke(&broker, QUERY_MESSAGE, ext, None, &self.config).await?;
        if resp.code != SUCCESS {
            return Err(AppError::msg(
                resp.remark
                    .unwrap_or_else(|| format!("查询消息失败 code={}", resp.code)),
            ));
        }
        Ok(vec![MessageView {
            offset: None,
            partition: None,
            key: key.to_string(),
            payload: bytes_preview(resp.body.as_deref()),
            timestamp: None,
            properties: resp.body_json(),
        }])
    }

    pub async fn send(
        &self,
        topic: &str,
        payload: &str,
        keys: Option<&str>,
        tags: Option<&str>,
    ) -> AppResult<Value> {
        let broker = self.pick_broker(topic).await?;
        let mut properties = String::new();
        if let Some(keys) = keys.filter(|item| !item.is_empty()) {
            properties.push_str(&format!("KEYS\u{1}{keys}\u{2}"));
        }
        if let Some(tags) = tags.filter(|item| !item.is_empty()) {
            properties.push_str(&format!("TAGS\u{1}{tags}\u{2}"));
        }
        let ext = json!({
            "producerGroup": "quay-producer",
            "topic": topic,
            "defaultTopic": "TBW102",
            "defaultTopicQueueNums": "4",
            "queueId": "0",
            "sysFlag": "0",
            "bornTimestamp": chrono::Utc::now().timestamp_millis().to_string(),
            "flag": "0",
            "properties": properties,
            "reconsumeTimes": "0",
            "unitMode": "false",
            "batch": "false"
        });
        let resp = invoke(
            &broker,
            SEND_MESSAGE,
            ext,
            Some(payload.as_bytes().to_vec()),
            &self.config,
        )
        .await?;
        resp.ensure_ok()?;
        Ok(json!({
            "remark": resp.remark,
            "extFields": resp.ext_fields
        }))
    }

    async fn pick_broker(&self, topic: &str) -> AppResult<String> {
        let route = self.topic_route(topic).await?;
        extract_broker_addr(&route).ok_or_else(|| AppError::msg(format!("Topic {topic} 没有可用 Broker")))
    }

    async fn pick_any_broker(&self) -> AppResult<String> {
        let cluster = self.cluster().await?;
        extract_broker_addr(&cluster.raw).ok_or_else(|| AppError::msg("集群中没有可用 Broker"))
    }
}

fn extract_broker_addr(value: &Value) -> Option<String> {
    if let Some(table) = value.get("brokerAddrTable").and_then(Value::as_object) {
        for broker in table.values() {
            if let Some(addrs) = broker.get("brokerAddrs").and_then(Value::as_object) {
                if let Some(master) = addrs.get("0").and_then(Value::as_str) {
                    return Some(master.to_string());
                }
                if let Some((_, addr)) = addrs.iter().next() {
                    return addr.as_str().map(ToOwned::to_owned);
                }
            }
        }
    }
    if let Some(brokers) = value.get("brokerDatas").and_then(Value::as_array) {
        for broker in brokers {
            if let Some(addrs) = broker.get("brokerAddrs").and_then(Value::as_object) {
                if let Some(master) = addrs.get("0").and_then(Value::as_str) {
                    return Some(master.to_string());
                }
            }
        }
    }
    None
}

struct RemotingResponse {
    code: i32,
    remark: Option<String>,
    ext_fields: Value,
    body: Option<Vec<u8>>,
}

impl RemotingResponse {
    fn ensure_ok(&self) -> AppResult<()> {
        if self.code == SUCCESS {
            Ok(())
        } else {
            Err(AppError::msg(
                self.remark
                    .clone()
                    .unwrap_or_else(|| format!("RocketMQ 返回错误 code={}", self.code)),
            ))
        }
    }

    fn body_json(&self) -> Value {
        self.body
            .as_ref()
            .and_then(|buf| serde_json::from_slice(buf).ok())
            .unwrap_or(Value::Null)
    }
}

async fn invoke(
    addr: &str,
    code: i32,
    ext_fields: Value,
    body: Option<Vec<u8>>,
    config: &RocketMqConfig,
) -> AppResult<RemotingResponse> {
    let opaque = OPAQUE.fetch_add(1, Ordering::Relaxed);
    let mut header = json!({
        "code": code,
        "language": "JAVA",
        "version": 399,
        "opaque": opaque,
        "flag": 0,
        "remark": "",
        "extFields": merge_acl(ext_fields, config)
    });
    if header["extFields"].as_object().is_some_and(|map| map.is_empty()) {
        header["extFields"] = Value::Null;
    }
    let header_bytes = serde_json::to_vec(&header)?;
    let body_len = body.as_ref().map(|item| item.len()).unwrap_or(0);
    let header_len_marked = header_bytes.len() as u32;
    let length = 4 + header_bytes.len() + body_len;

    let mut frame = Vec::with_capacity(4 + length);
    frame.extend_from_slice(&(length as u32).to_be_bytes());
    frame.extend_from_slice(&header_len_marked.to_be_bytes());
    frame.extend_from_slice(&header_bytes);
    if let Some(body) = &body {
        frame.extend_from_slice(body);
    }

    let mut stream = timeout(Duration::from_secs(8), TcpStream::connect(addr))
        .await
        .map_err(|_| AppError::msg(format!("连接 RocketMQ {addr} 超时")))?
        .map_err(|err| AppError::msg(format!("连接 RocketMQ {addr} 失败: {err}")))?;
    stream.write_all(&frame).await?;

    let mut len_buf = [0u8; 4];
    timeout(Duration::from_secs(8), stream.read_exact(&mut len_buf))
        .await
        .map_err(|_| AppError::msg("读取 RocketMQ 响应超时"))??;
    let payload_len = u32::from_be_bytes(len_buf) as usize;
    let mut payload = vec![0u8; payload_len];
    stream.read_exact(&mut payload).await?;

    let header_len_raw = u32::from_be_bytes(payload[0..4].try_into().unwrap());
    let header_len = (header_len_raw & 0x00FF_FFFF) as usize;
    let header_json = serde_json::from_slice::<Value>(&payload[4..4 + header_len])?;
    let body = if payload.len() > 4 + header_len {
        Some(payload[4 + header_len..].to_vec())
    } else {
        None
    };
    Ok(RemotingResponse {
        code: header_json.get("code").and_then(Value::as_i64).unwrap_or(-1) as i32,
        remark: header_json
            .get("remark")
            .and_then(Value::as_str)
            .map(ToOwned::to_owned)
            .filter(|item| !item.is_empty()),
        ext_fields: header_json.get("extFields").cloned().unwrap_or(Value::Null),
        body,
    })
}

fn merge_acl(mut ext: Value, config: &RocketMqConfig) -> Value {
    if !config.access_key.trim().is_empty() {
        ext["AccessKey"] = json!(config.access_key);
        if !config.secret_key.trim().is_empty() {
            ext["SecretKey"] = json!(config.secret_key);
        }
    }
    ext
}
