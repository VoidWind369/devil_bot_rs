use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::{log_error, log_info};
use crate::config::Config;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Events {
    pub op: i64,
    pub body: Option<EventsBody>,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct EventsBody {
    pub id: Option<i64>,
    pub r#type: Option<String>,
    pub platform: Option<String>,
    pub self_id: Option<String>,
    pub timestamp: Option<i64>,
    pub user: Option<EventsBodyUser>,
    pub channel: Option<EventsBodyChannel>,
    pub message: Option<EventsBodyMessage>,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct EventsBodyUser {
    pub id: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct EventsBodyChannel {
    pub r#type: Option<i64>,
    pub id: Option<String>,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct EventsBodyMessage {
    pub id: Option<String>,
    pub content: Option<String>,
}

impl Events {
    pub fn from(json_str: &str) -> Self {
        serde_json::from_str(json_str).expect("JSON格式错误")
    }
}

pub async fn send_message(channel_id: &Option<String>, content: &str, config: &Config) -> String {
    let json = json!({
            "channel_id": channel_id,
            "content": content
        });
    let res = Client::new().post(format!("{}/message.create", config.send_url))
        .header("Authorization", format!("Bearer {}", config.auth_token))
        .json(&json).send().await;
    log_info!("response end");
    match res {
        Ok(r) => {
            r.text().await.unwrap()
        }
        Err(e) => {
            log_error!("{}", e);
            e.to_string()
        }
    }
}