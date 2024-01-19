use futures::{SinkExt, StreamExt};
use serde_json::{json, Value};
use tokio_tungstenite::{connect_async, tungstenite, tungstenite::Message};

use crate::{config, log_error, log_info, log_link};
use crate::api::Events;
use crate::start::{listen_msg, listen_user_msg};

pub async fn conn() -> Result<(), tungstenite::Error> {
    let config = config::Config::new().await;

    // 创建一个websockets客户端连接
    let (client, _) = connect_async(config.ws_url)
        .await
        .expect("连接问题");
    let (mut socket, mut message) = client.split();

    let first_conn = json!({ "op": 3, "body": { "token": config.auth_token } });
    let first_send = Message::text(first_conn.to_string());
    // 发送消息给服务端
    socket.send(first_send).await.unwrap();

    let handle = tokio::spawn(async move {
        log_info!("{}", "收发线程");

        log_info!("{}", "收发线程loop");
        loop {
            match message.next().await {
                Some(Ok(Message::Text(data))) => {
                    log_info!("保持WebSocket分片连接: {}", data.clone());
                    let config = config::Config::new().await;
                    let events = Events::from(&data);
                    if !events.op.eq(&0) {
                        continue;
                    }
                    let Some(body) = events.body else { panic!("NONE") };
                    if let Some(channel) = body.clone().channel {
                        if let Some(channel_type) = channel.r#type {
                            match channel_type {
                                0 => listen_msg(body, &config).await,
                                3 => listen_user_msg(body, &config).await,
                                _ => { continue; }
                            }
                        }
                    };
                }
                _ => { break; }
            };
        }
    });

    let int = tokio::spawn(async move {
        log_info!("{}", "心跳线程");
        let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(10000));
        log_info!("{}", "心跳线程loop");
        loop {
            interval.tick().await;

            {
                // 创建一个ping消息
                let ping_n = json!({ "op": 1 });
                let ping = Message::text(ping_n.to_string());
                // 发送ping消息给服务端
                match socket.send(ping).await {
                    Ok(_) => { log_link!("Ping {}", &ping_n); }
                    Err(_) => {
                        log_error!("Ping {}", &ping_n);
                        break;
                    }
                };
            }
        }
    });

    let _ = tokio::join!(handle, int);

    Ok(())
}

// 定义一个异步函数，用于根据服务端回传的数据发送数据给服务端
pub fn _json_to_value(data: &str) -> Value {
    serde_json::from_str(data).expect("JSON格式错误")
}
