use void_log::{log_error, log_info};

mod api;
mod controller;
mod link;
mod modal;
mod om_api;
mod start;
mod util;

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

    // 启动服务器
    let server = controller::run();

    // 启动机器人
    let bot = async {
        loop {
            link::conn().await;
        }
    };

    tokio::join!(server, bot);
}

#[tokio::test]
async fn test() {
    let a = om_api::record::Record::new_text("#2PVQ9UY2Q", '2').await;
    log_info!("{:?}", &a);
}
