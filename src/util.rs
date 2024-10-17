use serde::{Deserialize, Serialize};
use tokio::io::AsyncReadExt;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub server_port: Option<i64>,
    // database
    pub database_url: Option<String>,
    pub database_name: Option<String>,
    pub database_username: Option<String>,
    pub database_password: Option<String>,
    // redis
    pub redis_url: Option<String>,
    pub redis_username: Option<String>,
    pub redis_password: Option<String>,
    pub redis_expire: Option<i64>,
    // request
    pub ws_url: Option<String>,
    pub api_url: Option<String>,
    pub auth_token: Option<String>,
    // QQ Bot
    pub qq_num: Option<String>,
    pub appid: Option<String>,
    pub token: Option<String>,
    pub app_secret: Option<String>,
    pub use_group: Option<i64>,
    pub server_url: Option<String>,
}

impl Config {
    pub async fn get() -> Self {
        let mut yaml_file = tokio::fs::File::open("config.yaml").await.expect("read config error");
        let mut yaml_str = String::new();
        yaml_file.read_to_string(&mut yaml_str).await.expect("read str error");
        serde_yml::from_str::<Config>(yaml_str.as_str()).expect("config error")
    }
}
