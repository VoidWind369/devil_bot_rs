use serde::{Deserialize, Serialize};
use tokio::io::AsyncReadExt;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub server: Option<ConfigServer>,
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigServer {
    pub url: Option<String>,
    pub path: Option<String>,
    pub port: Option<u16>,
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

impl Config {
    pub async fn get() -> Self {
        let mut yaml_file = tokio::fs::File::open("config.yaml").await.expect("read config error");
        let mut yaml_str = String::new();
        yaml_file.read_to_string(&mut yaml_str).await.expect("read str error");
        serde_yml::from_str(yaml_str.as_str()).expect("config error")
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

impl Default for ConfigServer {
    fn default() -> Self {
        Self {
            url: Option::from("localhost:50000".to_string()),
            path: Option::from("localhost".to_string()),
            port: Some(50000),
        }
    }
}
