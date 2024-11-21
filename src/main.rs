use rmp_serde::Serializer;
use serde::Serialize;
use void_log::*;

mod start;
mod api;
mod link;
mod util;
mod msg_pack;

#[tokio::main]
async fn main() {
    println!("=================================================================");
    log_info!("正常标识");
    log_warn!("警告标识");
    log_error!("错误标识");
    log_link!("WebSocket在线保持");
    println!("=================================================================");

    log_info!("登录中....");

    loop {
        link::conn().await;
    }
}

#[tokio::test]
async fn test() {
    let mut buf = Vec::new();
    let mp = msg_pack::Message::new("虚無风", "测试cececec");
    mp.serialize(&mut Serializer::new(&mut buf)).unwrap();
    log_info!("{:?}", buf);

    reqwest::Client::new().post("").body(buf).send().await.unwrap();
}
