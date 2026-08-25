mod commands;
mod error;
mod models;
mod mq;
mod storage;
mod util;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::connections::list_connections,
            commands::connections::save_connection,
            commands::connections::delete_connection,
            commands::connections::get_connection,
            commands::connections::test_connection,
            commands::rabbitmq::rabbit_overview,
            commands::rabbitmq::rabbit_queues,
            commands::rabbitmq::rabbit_exchanges,
            commands::rabbitmq::rabbit_bindings,
            commands::rabbitmq::rabbit_consumers,
            commands::rabbitmq::rabbit_connections,
            commands::rabbitmq::rabbit_get_messages,
            commands::rabbitmq::rabbit_publish,
            commands::kafka::kafka_cluster,
            commands::kafka::kafka_topic_detail,
            commands::kafka::kafka_fetch,
            commands::kafka::kafka_produce,
            commands::kafka::kafka_groups,
            commands::kafka::kafka_group_detail,
            commands::rocketmq::rocket_cluster,
            commands::rocketmq::rocket_topics,
            commands::rocketmq::rocket_topic_route,
            commands::rocketmq::rocket_topic_stats,
            commands::rocketmq::rocket_consume_stats,
            commands::rocketmq::rocket_query_messages,
            commands::rocketmq::rocket_send,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
