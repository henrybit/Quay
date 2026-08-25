use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};

use crate::error::{AppError, AppResult};
use crate::models::{KafkaBroker, KafkaGroup, KafkaGroupDetail, KafkaGroupMember, KafkaLagRow};

const API_METADATA: i16 = 3;
const API_FIND_COORDINATOR: i16 = 10;
const API_OFFSET_FETCH: i16 = 9;
const API_LIST_GROUPS: i16 = 16;
const API_DESCRIBE_GROUPS: i16 = 15;

pub struct KafkaAdmin {
    bootstrap: String,
    client_id: String,
}

impl KafkaAdmin {
    pub fn new(bootstrap: String, client_id: String) -> Self {
        Self {
            bootstrap,
            client_id,
        }
    }

    pub async fn metadata(&self) -> AppResult<(Vec<KafkaBroker>, Vec<TopicMeta>)> {
        let mut conn = self.connect(&self.bootstrap).await?;
        let mut body = Vec::new();
        write_i32(&mut body, -1);
        let payload = conn.request(API_METADATA, 0, &self.client_id, &body).await?;
        let mut cur = Cursor::new(payload);
        let broker_count = cur.i32()?;
        let mut brokers = Vec::new();
        for _ in 0..broker_count {
            brokers.push(KafkaBroker {
                node_id: cur.i32()?,
                host: cur.string()?.unwrap_or_default(),
                port: cur.i32()?,
            });
        }
        let topic_count = cur.i32()?;
        let mut topics = Vec::new();
        for _ in 0..topic_count {
            let error = cur.i16()?;
            let name = cur.string()?.unwrap_or_default();
            let partition_count = cur.i32()?;
            let mut partitions = Vec::new();
            for _ in 0..partition_count {
                let _p_err = cur.i16()?;
                let id = cur.i32()?;
                let leader = cur.i32()?;
                let replica_len = cur.i32()?;
                let mut replicas = Vec::new();
                for _ in 0..replica_len {
                    replicas.push(cur.i32()?);
                }
                let isr_len = cur.i32()?;
                for _ in 0..isr_len {
                    let _ = cur.i32()?;
                }
                partitions.push(PartitionMeta {
                    id,
                    _leader: leader,
                    _replicas: replicas,
                });
            }
            if error == 0 {
                topics.push(TopicMeta { name, partitions });
            }
        }
        Ok((brokers, topics))
    }

    pub async fn list_groups(&self) -> AppResult<Vec<KafkaGroup>> {
        let (brokers, _) = self.metadata().await?;
        let mut groups = Vec::new();
        for broker in brokers {
            let addr = format!("{}:{}", broker.host, broker.port);
            let payload = match self.connect(&addr).await {
                Ok(mut conn) => conn.request(API_LIST_GROUPS, 0, &self.client_id, &[]).await,
                Err(_) => continue,
            };
            if let Ok(payload) = payload {
                let mut cur = Cursor::new(payload);
                let _error = cur.i16()?;
                let count = cur.i32().unwrap_or(0);
                for _ in 0..count {
                    if let (Ok(Some(group_id)), Ok(Some(protocol_type))) =
                        (cur.string(), cur.string())
                    {
                        groups.push(KafkaGroup {
                            group_id,
                            protocol_type,
                            broker: addr.clone(),
                        });
                    }
                }
            }
        }
        groups.sort_by(|a, b| a.group_id.cmp(&b.group_id));
        groups.dedup_by(|a, b| a.group_id == b.group_id);
        Ok(groups)
    }

    pub async fn describe_group(&self, group_id: &str) -> AppResult<KafkaGroupDetail> {
        let coord = self.find_coordinator(group_id).await?;
        let mut conn = self.connect(&coord).await?;
        let mut body = Vec::new();
        write_i32(&mut body, 1);
        write_string(&mut body, Some(group_id));
        let payload = conn
            .request(API_DESCRIBE_GROUPS, 0, &self.client_id, &body)
            .await?;
        let mut cur = Cursor::new(payload);
        let count = cur.i32()?;
        if count <= 0 {
            return Err(AppError::msg("消费组不存在"));
        }
        let error = cur.i16()?;
        if error != 0 {
            return Err(AppError::msg(format!("DescribeGroups error_code={error}")));
        }
        let group_id = cur.string()?.unwrap_or_default();
        let state = cur.string()?.unwrap_or_default();
        let protocol_type = cur.string()?.unwrap_or_default();
        let protocol = cur.string()?.unwrap_or_default();
        let member_count = cur.i32()?;
        let mut members = Vec::new();
        for _ in 0..member_count {
            let member_id = cur.string()?.unwrap_or_default();
            let client_id = cur.string()?.unwrap_or_default();
            let client_host = cur.string()?.unwrap_or_default();
            let _ = cur.bytes()?;
            let _ = cur.bytes()?;
            members.push(KafkaGroupMember {
                member_id,
                client_id,
                client_host,
            });
        }
        let lag = self.offset_lag(&coord, &group_id).await.unwrap_or_default();
        Ok(KafkaGroupDetail {
            group_id,
            state,
            protocol_type,
            protocol,
            members,
            lag,
        })
    }

    async fn find_coordinator(&self, group_id: &str) -> AppResult<String> {
        let mut conn = self.connect(&self.bootstrap).await?;
        let mut body = Vec::new();
        write_string(&mut body, Some(group_id));
        let payload = conn
            .request(API_FIND_COORDINATOR, 0, &self.client_id, &body)
            .await?;
        let mut cur = Cursor::new(payload);
        let error = cur.i16()?;
        if error != 0 {
            return Err(AppError::msg(format!(
                "FindCoordinator error_code={error}"
            )));
        }
        let _node_id = cur.i32()?;
        let host = cur.string()?.unwrap_or_default();
        let port = cur.i32()?;
        Ok(format!("{host}:{port}"))
    }

    async fn offset_lag(&self, coordinator: &str, group_id: &str) -> AppResult<Vec<KafkaLagRow>> {
        let (_, topics) = self.metadata().await?;
        let mut conn = self.connect(coordinator).await?;
        let mut body = Vec::new();
        write_string(&mut body, Some(group_id));
        write_i32(&mut body, topics.len() as i32);
        for topic in &topics {
            write_string(&mut body, Some(&topic.name));
            write_i32(&mut body, topic.partitions.len() as i32);
            for part in &topic.partitions {
                write_i32(&mut body, part.id);
            }
        }
        let payload = conn
            .request(API_OFFSET_FETCH, 1, &self.client_id, &body)
            .await?;
        let mut cur = Cursor::new(payload);
        let topic_count = cur.i32()?;
        let mut rows = Vec::new();
        for _ in 0..topic_count {
            let topic = cur.string()?.unwrap_or_default();
            let part_count = cur.i32()?;
            for _ in 0..part_count {
                let partition = cur.i32()?;
                let committed = cur.i64()?;
                let _metadata = cur.string()?;
                let _error = cur.i16()?;
                if committed >= 0 {
                    rows.push(KafkaLagRow {
                        topic: topic.clone(),
                        partition,
                        committed,
                        latest: -1,
                        lag: 0,
                    });
                }
            }
        }
        Ok(rows)
    }

    async fn connect(&self, addr: &str) -> AppResult<KafkaConn> {
        let stream = timeout(Duration::from_secs(8), TcpStream::connect(addr))
            .await
            .map_err(|_| AppError::msg(format!("连接 Kafka {addr} 超时")))?
            .map_err(|err| AppError::msg(format!("连接 Kafka {addr} 失败: {err}")))?;
        Ok(KafkaConn { stream })
    }
}

#[derive(Debug, Clone)]
pub struct TopicMeta {
    pub name: String,
    pub partitions: Vec<PartitionMeta>,
}

#[derive(Debug, Clone)]
pub struct PartitionMeta {
    pub id: i32,
    pub _leader: i32,
    pub _replicas: Vec<i32>,
}

struct KafkaConn {
    stream: TcpStream,
}

impl KafkaConn {
    async fn request(
        &mut self,
        api_key: i16,
        api_version: i16,
        client_id: &str,
        body: &[u8],
    ) -> AppResult<Vec<u8>> {
        let mut header = Vec::new();
        write_i16(&mut header, api_key);
        write_i16(&mut header, api_version);
        write_i32(&mut header, 1);
        write_string(&mut header, Some(client_id));
        header.extend_from_slice(body);

        let mut frame = Vec::with_capacity(4 + header.len());
        write_i32(&mut frame, header.len() as i32);
        frame.extend_from_slice(&header);
        self.stream.write_all(&frame).await?;

        let mut size_buf = [0u8; 4];
        timeout(Duration::from_secs(8), self.stream.read_exact(&mut size_buf))
            .await
            .map_err(|_| AppError::msg("读取 Kafka 响应超时"))??;
        let size = i32::from_be_bytes(size_buf);
        if size <= 4 {
            return Err(AppError::msg("Kafka 响应过短"));
        }
        let mut payload = vec![0u8; size as usize];
        self.stream.read_exact(&mut payload).await?;
        Ok(payload[4..].to_vec())
    }
}

struct Cursor {
    buf: Vec<u8>,
    pos: usize,
}

impl Cursor {
    fn new(buf: Vec<u8>) -> Self {
        Self { buf, pos: 0 }
    }

    fn remain(&self) -> usize {
        self.buf.len().saturating_sub(self.pos)
    }

    fn take(&mut self, n: usize) -> AppResult<&[u8]> {
        if self.remain() < n {
            return Err(AppError::msg("Kafka 协议解码越界"));
        }
        let start = self.pos;
        self.pos += n;
        Ok(&self.buf[start..self.pos])
    }

    fn i16(&mut self) -> AppResult<i16> {
        let bytes = self.take(2)?;
        Ok(i16::from_be_bytes([bytes[0], bytes[1]]))
    }

    fn i32(&mut self) -> AppResult<i32> {
        let bytes = self.take(4)?;
        Ok(i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    fn i64(&mut self) -> AppResult<i64> {
        let bytes = self.take(8)?;
        Ok(i64::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]))
    }

    fn string(&mut self) -> AppResult<Option<String>> {
        let len = self.i16()?;
        if len < 0 {
            return Ok(None);
        }
        let bytes = self.take(len as usize)?;
        Ok(Some(String::from_utf8_lossy(bytes).into_owned()))
    }

    fn bytes(&mut self) -> AppResult<Option<Vec<u8>>> {
        let len = self.i32()?;
        if len < 0 {
            return Ok(None);
        }
        Ok(Some(self.take(len as usize)?.to_vec()))
    }
}

fn write_i16(buf: &mut Vec<u8>, value: i16) {
    buf.extend_from_slice(&value.to_be_bytes());
}

fn write_i32(buf: &mut Vec<u8>, value: i32) {
    buf.extend_from_slice(&value.to_be_bytes());
}

fn write_string(buf: &mut Vec<u8>, value: Option<&str>) {
    match value {
        None => buf.extend_from_slice(&(-1i16).to_be_bytes()),
        Some(text) => {
            buf.extend_from_slice(&(text.len() as i16).to_be_bytes());
            buf.extend_from_slice(text.as_bytes());
        }
    }
}
