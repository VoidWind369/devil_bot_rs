mod image_api;

use axum::Router;
use axum::routing::get;
use void_log::log_info;
use crate::util::Config;

pub async fn run() {
    let config = Config::get().await.server.unwrap_or_default();
    let address = format!("{}:{}", &config.path.unwrap_or("0.0.0.0".to_string()), &config.port.unwrap_or(50000));
    log_info!("启动参数: {}", &address);

    let mut app = Router::new()
        .route("/", get(|| async { "Hello, World!" }));
    app = image_api::router(app);

    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}