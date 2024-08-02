use serde::{Deserialize, Serialize};
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use void_log::*;

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
        let mut yaml_file = fs::File::open("config.yaml").await.expect("read config error");
        let mut yaml_str = String::new();
        yaml_file.read_to_string(&mut yaml_str).await.expect("read str error");
        serde_yml::from_str::<Self>(yaml_str.as_str()).expect("config error")
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdSetting {
    pub send: bool,
    pub file: Option<String>,
    pub time: Option<u64>,
    pub groups: Vec<String>,
}

impl Default for AdSetting {
    fn default() -> Self {
        Self {
            send: true,
            file: Some("ad.void".to_string()),
            time: Some(3600),
            ..Default::default()
        }
    }
}

impl AdSetting {
    pub async fn get() -> Self {
        let mut yaml_file = fs::File::open("ad_setting.yaml").await.expect("read setting error");
        let mut yaml_str = String::new();
        yaml_file.read_to_string(&mut yaml_str).await.expect("read str error");
        serde_yml::from_str::<Self>(yaml_str.as_str()).expect("setting error")
    }

    pub async fn set(&self) {
        let mut yaml_file = fs::File::open("ad_setting.yaml").await.expect("read setting error");
        // let mut writer = BufWriter::new(yaml_file);
        log_info!("写入配置{:?}", &self);
        yaml_file.write_all(&serde_yml::to_string(self).unwrap().as_bytes()).await.unwrap();
        yaml_file.flush().await.unwrap();
    }

    pub async fn ad_file(&self) -> String {
        let file = self.clone().file.unwrap_or_default();
        let mut ad_file = fs::File::open(file).await.unwrap();
        let mut ad_str = String::new();
        ad_file.read_to_string(&mut ad_str).await.unwrap();
        ad_str
    }
}
