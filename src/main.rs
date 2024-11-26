use std::hash::{DefaultHasher, Hash};
use std::ops::Deref;
use jsonwebtoken::EncodingKey;
use md5::Digest;
use md5::digest::DynDigest;
use void_log::{log_error, log_info, log_link, log_warn};

mod api;
mod link;
mod modal;
mod msg_pack;
mod om_api;
mod start;
mod util;

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
    let a = om_api::record::Record::new("#8GYUVV", "1329997614", 0).await;
    log_info!("{:?}", a)
}
