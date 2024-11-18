use void_log::{log_error, log_info, log_link, log_warn};

mod start;
mod api;
mod link;
mod util;
mod tool;

#[tokio::main]
async fn main() {
    let mut banner = String::from("--------------");
    banner.push_str("\n   ██████╗ ██████╗  █████╗ ███╗   ██╗ ██████╗ ███████╗  ");
    banner.push_str("\n  ██╔═══██╗██╔══██╗██╔══██╗████╗  ██║██╔════╝ ██╔════╝  ");
    banner.push_str("\n  ██║   ██║██████╔╝███████║██╔██╗ ██║██║  ███╗█████╗    ");
    banner.push_str("\n  ██║   ██║██╔══██╗██╔══██║██║╚██╗██║██║   ██║██╔══╝    ");
    banner.push_str("\n  ╚██████╔╝██║  ██║██║  ██║██║ ╚████║╚██████╔╝███████╗  ");
    banner.push_str("\n   ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝ ╚═════╝ ╚══════╝  ");
    banner.push_str("\n--------------------------------------------------------");
    log_info!("{}", banner);

    log_info!("登录中....");

    let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(5000));
    loop {
        link::conn().await;
        interval.tick().await;
    }
}
