# MQ GUI

桌面端消息查看工具，覆盖 **RabbitMQ / Kafka / RocketMQ**。界面用 SvelteKit，桌面壳用 Tauri 2，连接配置保存在本机。

## 能力

- 保存、编辑、删除集群连接，左侧用图标区分三种中间件
- 切换连接后进入对应 Explorer，每种中间件有独立功能页
- **RabbitMQ**：Management API 总览、队列 Ready/Unacked、交换机类型、绑定、Peek 消息、发布
- **Kafka**：Broker/Topic、分区 Earliest/Latest、消费组与 Lag、按 offset 浏览、生产消息
- **RocketMQ**：NameServer 集群、Topic 路由/DLQ/Retry 分类、消费进度、按 MessageKey 查询、发送

## 开发

需要 Node 24+、pnpm、Rust。

```sh
pnpm install
pnpm desktop
```

只跑前端（无法调用本机 Tauri 命令）：

```sh
pnpm dev
```

打包：

```sh
pnpm desktop:build
```

## 连接说明

| 类型 | 必填 | 说明 |
| --- | --- | --- |
| RabbitMQ | Management URL、用户名 | 需要开启 `rabbitmq_management` |
| Kafka | Bootstrap brokers | SASL PLAIN 可选 |
| RocketMQ | NameServer | 走 Remoting 协议；ACL 字段可选 |

连接 JSON 位于系统应用配置目录（macOS: `~/Library/Application Support/com.mqgui.app/connections.json`）。
