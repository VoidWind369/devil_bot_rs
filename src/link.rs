use crate::api::one_bot::OneBotData;
use crate::start;
use crate::util::Config;
use futures_util::{stream::{SplitSink, SplitStream}, SinkExt, StreamExt};
use serde_json::json;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use void_log::*;

pub async fn conn() {
    let bot = Config::get().await.bot.unwrap_or_default();
    // 创建一个websockets客户端连接
    let (client, _) = connect_async(bot.ws.unwrap_or_default())
        .await
        .expect("连接问题");
    let (mut socket, mut message) = client.split();

    let handle = handle(&mut message);
    let _intent = intent(&mut socket);
    tokio::join!(handle);
}

async fn handle(message: &mut SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>) {
    log_info!("{}", "收发线程loop");
    while let Some(Ok(Message::Text(data))) = message.next().await {
        log_msg!("WebSocket分片连接: {}", &data);
        if let Ok(value_data) = serde_json::from_str::<OneBotData>(&data) {
            start::listen(value_data.clone()).await;
        }
        if let Err(e) = serde_json::from_str::<OneBotData>(&data) {
            panic!("{}", e);
        };
    }
}

async fn intent(socket: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>) {
    let config = Config::get().await;
    let token = config.bot.unwrap().token;
    log_info!("{}", "心跳线程loop");
    let op3 = json!({
        "op": 3, "body": { "token": token }
    });
    socket.send(Message::text(op3.to_string())).await.unwrap();
    let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(10000));
    loop {
        interval.tick().await;
        // 创建一个ping消息
        let ping_n = json!({
            "op": 1,
        });
        let ping = Message::text(ping_n.to_string());
        // 发送ping消息给服务端
        match socket.send(ping).await {
            Ok(_) => { log_link!("Ping {}", &ping_n); }
            Err(e) => {
                log_error!("Ping {} {}", &ping_n, e);
                break;
            }
        };
    }
}