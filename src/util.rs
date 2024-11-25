use serde::{Deserialize, Serialize};
use tokio::io::AsyncReadExt;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub server_port: Option<i64>,
    // database
    pub database: Option<ConfigDatabase>,
    pub redis: Option<ConfigRedis>,
    // request
    pub bot: Option<ConfigApi>,
    pub api: Option<ConfigApi>,
    pub om_api: Option<ConfigApi>,
    // QQ Bot
    pub chat_use: Option<ConfigChatUse>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfigDatabase {
    pub url: Option<String>,
    pub name: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfigRedis {
    pub url: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub expire: Option<i64>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfigApi {
    pub ws: Option<String>,
    pub url: Option<String>,
    pub token: Option<String>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ConfigChatUse {
    pub user: Option<String>,
    pub group: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server_port: Some(50000),
            ..Default::default()
        }
    }
}

impl Config {
    pub async fn get() -> Self {
        let mut yaml_file = tokio::fs::File::open("config.yaml").await.expect("read config error");
        let mut yaml_str = String::new();
        yaml_file.read_to_string(&mut yaml_str).await.expect("read str error");
        serde_yml::from_str::<Config>(yaml_str.as_str()).expect("config error")
    }

    pub fn get_api(self) -> ConfigApi {
        self.api.unwrap_or_default()
    }

    pub fn get_om_api(self) -> ConfigApi {
        self.om_api.unwrap_or_default()
    }

    pub fn get_database(self) -> ConfigDatabase {
        self.database.unwrap_or_default()
    }
}
