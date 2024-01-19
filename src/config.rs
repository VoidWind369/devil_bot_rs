use serde::{Deserialize, Serialize};
use tokio::fs;
use tokio::io::AsyncReadExt;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub ws_url: String,
    pub send_url: String,
    pub auth_token: String,
}

impl Config {
    pub async fn new() -> Self {
        serde_yaml::from_str::<Config>(&get().await).expect("Serde")
    }
}

async fn get() -> String {
    let mut open = fs::File::open("config.yaml").await.expect("Open");
    let mut config_str = String::new();
    open.read_to_string(&mut config_str).await.expect("Read");
    config_str
}