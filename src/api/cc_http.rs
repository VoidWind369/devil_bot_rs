use reqwest::Client;
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::api::cq_http::SendCqGroup;
use void_log::*;
use crate::util::Config;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CcData {
    pub op: Option<i64>,
    pub body: Option<CcDataBody>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CcDataBody {
    pub id: Option<i64>,
    pub r#type: Option<String>,
    pub platform: Option<String>,
    pub self_id: Option<String>,
    pub timestamp: Option<i64>,
    pub user: Option<CcDataBodyInfo>,
    pub channel: Option<CcDataBodyInfo>,
    pub guild: Option<CcDataBodyInfo>,
    pub member: Option<Value>,
    pub message: Option<CcDataBodyInfo>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CcDataBodyInfo {
    pub id: Option<String>,
    pub name: Option<String>,
    pub r#type: Option<i64>,
    pub avatar: Option<String>,
    pub content: Option<String>,
}

pub async fn send_group_msg(group_id: &str, text: &str, at: i64) {
    let api = Config::get().await.api.unwrap_or_default();
    let url = format!("{}/message.create", &api.url.unwrap());

    let message = match at {
        -1 => text.to_string(),
        _ => send_at(at, text)
    };
    let send = json!({
        "channel_id": group_id,
        "content": message
    });
    let token = format!("Bearer {}", &api.token.unwrap_or_default());
    let mut headers = HeaderMap::new();
    headers.append(AUTHORIZATION, HeaderValue::from_str(&token).unwrap());
    log_info!("{:?}", &send);
    let response = Client::new().post(url).headers(headers).json(&send).send().await;
    match response {
        Ok(re) => {
            log_info!("CC Group {}", re.text().await.unwrap())
        }
        Err(e) => {
            log_error!("{e}")
        }
    }
}

fn send_at(id: i64, content: &str) -> String {
    format!("<at id=\"{id}\"/> {content}")
}