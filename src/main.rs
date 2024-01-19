mod ws_link;
mod log;
mod start;
mod config;
mod api;

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
        ws_link::conn().await.unwrap();
    }
}
