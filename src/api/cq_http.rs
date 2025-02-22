use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::*;
use crate::util::Config;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CqData<'a> {
    // interval
    pub self_id: Option<i64>,
    pub time: Option<i64>,
    pub post_type: Option<&'a str>,
    pub meta_event_type: Option<&'a str>,
    pub status: Option<CqDataStatus>,
    pub interval: Option<i64>,
    // message
    pub user_id: Option<i64>,
    pub message_id: Option<i64>,
    pub real_id: Option<i64>,
    pub message_type: Option<&'a str>,
    pub sender: Option<CqDataSender<'a>>,
    pub raw_message: Option<String>,
    pub font: Option<i64>,
    pub sub_type: Option<&'a str>,
    pub message: Option<Vec<CqDataMessage>>,
    pub message_format: Option<&'a str>,
    pub group_id: Option<i64>,
    // friend
    pub request_type: Option<&'a str>,
    pub comment: Option<&'a str>,
    pub flag: Option<&'a str>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CqDataStatus {
    pub online: Option<bool>,
    pub good: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CqDataSender<'a> {
    pub user_id: Option<i64>,
    pub nickname: Option<&'a str>,
    pub card: Option<&'a str>,
    pub role: Option<&'a str>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CqDataMessage {
    pub data: Option<CqDataMessageData>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CqDataMessageData {
    pub file: Option<String>,
    pub url: Option<String>,
    pub file_size: Option<String>,
}

// 发送消息构造
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendCqGroup {
    user_id: Option<i64>,
    group_id: Option<i64>,
    message: Vec<SendCqGroupMessage>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendMessage {
    message_type: SendMessageType,
    user_id: Option<i64>,
    group_id: Option<i64>,
    message: Vec<SendCqGroupMessage>,
    auto_escape: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SendMessageType {
    Group,
    #[default]
    Private,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendCqGroupMessage {
    r#type: String,
    data: SendCqGroupMessageData,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendCqGroupMessageData {
    id: Option<String>,
    qq: Option<String>,
    name: Option<String>,
    text: Option<String>,
}

pub async fn send_msg(message_type: SendMessageType, user_id: Option<i64>, group_id: Option<i64>, text: &str, at: i64) -> String {
    let config = Config::get().await;
    let url = format!("{}/send_msg", config.bot.unwrap().url.unwrap());

    let message = match at {
        -1 => vec![send_text(text)],
        _ => vec![send_at(at), send_text(&format!(" {text}"))]
    };
    let group_id = match message_type {
        SendMessageType::Group => { group_id }
        SendMessageType::Private => { None }
    };
    let send = SendMessage {
        message_type,
        user_id,
        group_id,
        message,
        auto_escape: None,
    };
    log_info!("{:?}", &send);
    let response = Client::new().post(url).json(&send).send().await;
    match response {
        Ok(re) => {
            let res = re.json::<Value>().await.unwrap();
            let status = res["status"].as_str().unwrap();
            log_info!("SendResult {}", status);
            status.to_string()
        }
        Err(e) => {
            log_error!("SendError {e}");
            "err".to_string()
        }
    }
}

pub async fn _send_group_msg(group_id: i64, text: &str, at: i64) {
    let config = Config::get().await;
    let url = format!("{}/send_group_msg", config.bot.unwrap().url.unwrap());

    let message = match at {
        -1 => vec![send_text(text)],
        _ => vec![send_at(at), send_text(&format!(" {text}"))]
    };
    let send = SendCqGroup {
        user_id: None,
        group_id: Option::from(group_id),
        message,
    };
    log_info!("{:?}", &send);
    let response = Client::new().post(url).json(&send).send().await;
    match response {
        Ok(re) => {
            log_info!("Group {}", re.text().await.unwrap())
        }
        Err(e) => {
            log_error!("{e}")
        }
    }
}

pub async fn _send_user_msg(user_id: i64, group_id: Option<i64>, text: &str) {
    let config = Config::get().await;
    let url = format!("{}/send_private_msg", config.bot.unwrap().url.unwrap());

    let message = vec![send_text(text)];
    let send = match group_id {
        Some(id) => SendCqGroup {
            user_id: Option::from(user_id),
            group_id: Option::from(id),
            message,
        },
        None => SendCqGroup {
            user_id: Option::from(user_id),
            group_id: None,
            message,
        }
    };
    log_info!("{:?}", &send);
    let response = Client::new().post(url).json(&send).send().await;
    match response {
        Ok(re) => {
            log_info!("User {}", re.text().await.unwrap())
        }
        Err(e) => {
            log_error!("{e}")
        }
    }
}

pub async fn _get_group_member_info(group_id: i64, user_id: i64) -> Value {
    let config = Config::get().await;
    let url = format!("{}/get_group_member_info", config.bot.unwrap().url.unwrap());
    let json = json!({
        "group_id": group_id,
        "user_id": user_id,
        "no_cache": true
    });
    let response = Client::new().post(url).json(&json).send().await;
    match response {
        Ok(re) => {
            let value = re.json::<Value>().await.unwrap_or_default();
            log_info!("Group Member {}", value);
            value
        }
        Err(e) => {
            log_error!("{e}");
            Default::default()
        }
    }
}

pub async fn set_friend_add_request(flag: &str, approve: bool) {
    let config = Config::get().await;
    let url = format!("{}/set_friend_add_request", config.bot.unwrap().url.unwrap());
    let json = json!({
        "flag": flag,
        "approve": approve,
    });
    let response = Client::new().post(url).json(&json).send().await;
    match response {
        Ok(re) => {
            log_info!("Friend {}", re.text().await.unwrap())
        }
        Err(e) => {
            log_error!("{e}")
        }
    }
}

fn send_text(text: &str) -> SendCqGroupMessage {
    SendCqGroupMessage {
        r#type: "text".to_string(),
        data: SendCqGroupMessageData {
            id: None,
            qq: None,
            name: None,
            text: Option::from(text.to_string()),
        },
    }
}

fn send_at(id: i64) -> SendCqGroupMessage {
    match id {
        0 => SendCqGroupMessage {
            r#type: "at".to_string(),
            data: SendCqGroupMessageData {
                id: None,
                qq: Option::from("all".to_string()),
                name: None,
                text: None,
            },
        },
        _ => SendCqGroupMessage {
            r#type: "at".to_string(),
            data: SendCqGroupMessageData {
                id: None,
                qq: Option::from(id.to_string()),
                name: None,
                text: None,
            },
        }
    }
}