use void_log::{log_error, log_info, log_link, log_warn};

mod start;
mod api;
mod link;
mod util;
mod tool;

#[tokio::main]
async fn main() {
    println!("=================================================================");
    log_info!("正常标识");
    log_warn!("警告标识");
    log_error!("错误标识");
    log_link!("WebSocket在线保持");
    println!("=================================================================");

    log_info!("登录中....");

    let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(5000));
    loop {
        link::conn().await;
        interval.tick().await;
    }
}
