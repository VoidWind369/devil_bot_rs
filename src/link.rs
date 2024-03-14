use futures_util::{stream::{SplitSink, SplitStream}, SinkExt, StreamExt};
use serde_json::{json};
use tokio::{net::TcpStream};
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream, tungstenite::{Message}};
use crate::*;
use crate::api::cq_http::CqData;

pub async fn conn() {
    let config = util::Config::get().await;
    // 创建一个websockets客户端连接
    let (client, _) = connect_async(config.ws_url.unwrap())
        .await
        .expect("连接问题");
    let (mut socket, mut message) = client.split();

    let handle = handle(&mut message);
    let _intent = intent(&mut socket);
    tokio::join!(handle);
}

async fn handle(message: &mut SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>) {
    log_info!("{}", "收发线程loop");
    loop {
        if let Some(Ok(Message::Text(data))) = message.next().await {
            log_info!("WebSocket分片连接: {:?}", &data);
            let cq_data = serde_json::from_str::<CqData>(&data).unwrap_or(Default::default());
            if let None = &cq_data.raw_message {
                continue;
            }
            start::listen(cq_data.clone()).await;
        } else {
            log_error!("收发线程中断");
            break;
        }
    }
}

async fn intent(socket: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>) {
    log_info!("{}", "心跳线程loop");
    let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(41250));
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