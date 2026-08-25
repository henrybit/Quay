# MQ GUI 使用说明书

MQ GUI 是一款桌面端消息中间件管理工具，支持 **RabbitMQ**、**Kafka** 和 **RocketMQ** 三种主流消息队列。可视化浏览集群拓扑、查看队列状态、消费消息、发布测试消息。

---

## 目录

1. [安装与启动](#1-安装与启动)
2. [连接管理](#2-连接管理)
3. [RabbitMQ 功能指南](#3-rabbitmq-功能指南)
4. [Kafka 功能指南](#4-kafka-功能指南)
5. [RocketMQ 功能指南](#5-rocketmq-功能指南)
6. [数据存储说明](#6-数据存储说明)
7. [常见问题](#7-常见问题)

---

## 1. 安装与启动

### 系统要求

- **macOS**（当前版本主要支持）
- Windows/Linux 通过 Tauri 跨平台构建后可通用

### 安装方式

下载对应平台的安装包，双击完成安装。安装后启动应用即可。

### 启动

首次启动后，左侧栏显示已保存的连接列表（初始为空）；主区域显示欢迎页，引导新建连接。

---

## 2. 连接管理

### 新建连接

点击欢迎页中的「新建连接」按钮，或点击左侧栏顶部 `+` 图标，选择要连接的中间件类型（RabbitMQ / Kafka / RocketMQ），填写配置后保存。

### 连接配置说明

| 中间件 | 必填字段 | 说明 |
|--------|----------|------|
| RabbitMQ | Management URL、用户名、密码 | 需要开启 `rabbitmq_management` 插件 |
| Kafka | Bootstrap Servers | SASL PLAIN 认证为可选字段 |
| RocketMQ | NameServer | ACL（AccessKey/SecretKey）为可选字段 |

### 编辑 / 删除连接

在左侧栏右键或点击连接右侧的操作按钮，可对已保存的连接进行编辑或删除。

### 连接配置文件

所有连接配置保存在本地文件中：

- **macOS**：`~/Library/Application Support/com.mqgui.app/connections.json`

---

## 3. RabbitMQ 功能指南

进入 RabbitMQ 连接后，顶栏显示以下标签页：

| 标签 | 功能 |
|------|------|
| 总览 | 集群概览、节点状态、整体消息统计 |
| 队列 | 列出所有队列，显示 Ready 数、Unacked 数，消费者数 |
| 交换机 | 列出所有交换机及其类型（direct / fanout / topic / headers） |
| 绑定 | 查看交换机与队列之间的绑定关系 |
| 消息 | Peek 指定队列中的消息，支持发布新消息到指定交换机 |

### 消息预览

在「消息」标签中，选择目标队列，可浏览队列中的消息内容（AMQP 消息体直接展示）。

### 发布消息

在「消息」标签中，选择目标交换机，填写 Routing Key 和消息内容，执行发布操作。

---

## 4. Kafka 功能指南

进入 Kafka 连接后，顶栏显示以下标签页：

| 标签 | 功能 |
|------|------|
| 集群 | Broker 列表及状态 |
| Topic | 列出所有 Topic，显示分区数、副本数 |
| 消费组 | 查看所有消费组及其 Lag（消费延迟） |
| 消息浏览 | 按分区、offset 范围（Earliest / Latest）浏览消息内容 |

### 消费组 Lag

Lag 表示消费端已消费到最新消息的差距，Lag 越大说明消费积压越严重。

### 生产消息

在「消息浏览」标签中，可直接向指定 Topic 生产测试消息。

---

## 5. RocketMQ 功能指南

进入 RocketMQ 连接后，顶栏显示以下标签页：

| 标签 | 功能 |
|------|------|
| 集群 | NameServer 集群拓扑 |
| Topic | Topic 列表，包含路由信息、DLQ（死信队列）、Retry（重试队列）分类 |
| 消费进度 | 消费者组消费进度，按 Instance 分组展示 |
| 消息查询 | 按 MessageKey 查询历史消息 |

### 消息查询

在「消息查询」标签中，输入 MessageKey 可查询该条消息的详细信息（内容、属性、发送时间等）。

### 发送消息

在「消息查询」标签中，支持向指定 Topic 发送测试消息。

---

## 6. 数据存储说明

### 连接配置

连接信息以 JSON 格式存储在本机，不上传至任何服务器。配置文件路径：

```
~/Library/Application Support/com.mqgui.app/connections.json
```

### 敏感信息

密码、SASL 凭证、ACL 密钥等以明文存储在连接配置中。请注意本地电脑的安全，不要让他人随意访问此文件。

---

## 7. 常见问题

**Q: RabbitMQ 连接失败，提示 "management plugin not enabled"**

A: RabbitMQ 需要开启 management 插件。执行以下命令开启：
```bash
rabbitmq-plugins enable rabbitmq_management
```

**Q: Kafka 连接成功但看不到 Topic**

A: 检查是否配置了正确的 Bootstrap Servers 地址，并确认 Kafka 监听地址允许外部访问（检查 `listeners` 配置）。

**Q: RocketMQ 消费进度显示为 0**

A: 确认消费者组已启动并有实际消费行为。RocketMQ GUI 显示的是消费者提交的消费位点。

**Q: 连接配置丢失怎么办**

A: 配置文件位于 `~/Library/Application Support/com.mqgui.app/connections.json`，请勿手动删除或修改该文件，否则已保存的连接将丢失。
