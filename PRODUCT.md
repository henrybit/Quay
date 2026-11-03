# Quay 产品说明

**Quay**（读作 *key*）是一款本地桌面端消息中间件查看与调试工具。名称取自「码头 / 泊位」：消息在 broker 里进出，Quay 是你停靠查看、排查与试发的工作台。

---

## 产品定位

面向开发、测试与运维人员，在本机统一管理 **RabbitMQ / Kafka / RocketMQ** 连接，快速浏览集群状态、积压与消息内容，并支持向队列或 Topic 发送测试消息。

- **不做云端托管**：连接与凭证只保存在本机
- **不做替代运维控制台**：聚焦查看、排查、试发，而不是完整集群治理
- **一种工作流**：左侧选连接 → 右侧按中间件类型进入专用 Explorer

---

## 核心特点

| 特点 | 说明 |
| --- | --- |
| 三栈一体 | 同一应用覆盖 RabbitMQ、Kafka、RocketMQ，左侧图标区分类型 |
| 本机优先 | 基于 Tauri 2 桌面壳，配置落在系统应用目录，无需登录账号 |
| 按协议深耕 | 每种中间件独立功能页与交互，而不是一套通用表格硬套三种协议 |
| 可读可发 | 既能 Peek / 按 offset / 按 Key 查看消息，也能发布测试消息 |
| 轻量启动 | Web 前端 + Rust 后端，体积与资源占用相对 Electron 方案更小 |
| 跨平台 | 支持 macOS（Apple Silicon / Intel）、Windows、Linux 安装包 |

---

## 功能一览

### 连接管理

- 新建 / 编辑 / 删除连接配置
- 按中间件类型筛选，支持搜索连接名
- 保存后可随时切换，无需每次重填地址与凭证
- 连接前可做连通性测试（按类型校验必要字段与可达性）

### RabbitMQ

| 模块 | 能力 |
| --- | --- |
| 总览 | Management API 集群概览、节点与消息统计 |
| 队列 | 列表查看 Ready / Unacked、消费者数等状态 |
| 交换机 | 查看交换机及类型（direct / fanout / topic / headers） |
| 绑定 | 查看交换机与队列的绑定关系 |
| 消息 | Peek 队列消息内容；向指定交换机发布消息（Routing Key + Body） |

**接入要求**：Management URL + 用户名密码；需开启 `rabbitmq_management` 插件。

### Kafka

| 模块 | 能力 |
| --- | --- |
| 集群 | Broker 列表与基础状态 |
| Topic | Topic 列表，分区数、副本等信息 |
| 消费组 | 消费组列表与 Lag，便于发现积压 |
| 消息浏览 | 按分区与 Earliest / Latest offset 浏览消息；可向 Topic 生产测试消息 |

**接入要求**：Bootstrap Brokers；可选 SASL PLAIN；默认 Client ID 为 `quay`。

### RocketMQ

| 模块 | 能力 |
| --- | --- |
| 集群 | NameServer 侧集群拓扑 |
| Topic | Topic 列表与路由；区分普通 / DLQ / Retry |
| 消费进度 | 按消费组查看 ConsumeStats / 位点进度 |
| 消息查询 | 按 MessageKey 查询消息详情；可向 Topic 发送测试消息 |

**接入要求**：NameServer 地址；可选 ACL（AccessKey / SecretKey）；走 Remoting 协议。

---

## 典型使用场景

1. **本地联调**：起好中间件后，用 Quay 确认队列/Topic 是否创建成功、消息是否入队  
2. **积压排查**：看 RabbitMQ Ready/Unacked，或 Kafka 消费组 Lag，快速定位消费侧问题  
3. **消息抽查**：不写临时脚本，直接 Peek / 按 offset / 按 Key 查看 payload  
4. **故障复现**：向指定交换机或 Topic 发送一条已知内容的测试消息  
5. **多环境切换**：开发 / 测试 / 预发连接保存在侧边栏，一键切换

---

## 技术架构（简要）

| 层 | 技术 |
| --- | --- |
| UI | SvelteKit + Vite + Tailwind |
| 桌面壳 | Tauri 2 |
| 后端能力 | Rust（HTTP / Kafka 协议客户端 / RocketMQ Remoting） |
| 数据 | 本机 `connections.json`，按应用标识目录存放 |

应用标识：`com.quay.app`  
macOS 配置目录示例：`~/Library/Application Support/com.quay.app/`

---

## 非目标（当前版本）

以下能力不在当前产品范围，避免误解预期：

- 云端账号同步、团队协作与权限体系  
- 集群安装、扩缩容、监控告警大盘  
- 消费组完整运维（重置 offset、再均衡策略编排等深度操作）  
- 消息轨迹全链路审计与长期存储  

---

## 版本与发布

- 通过 GitHub Actions 在推送 `v*` tag 时构建 Windows / macOS / Linux 安装包  
- macOS 发布包支持 Developer ID 签名与公证（需配置 Apple 相关 Secrets）  

更多操作步骤见 [USER_GUIDE.md](./USER_GUIDE.md)，开发与构建见 [README.md](./README.md)。
