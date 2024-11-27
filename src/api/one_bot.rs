use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::util::Config;
use crate::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OneBotData {
    // interval
    pub self_id: Option<i64>,
    pub time: Option<i64>,
    pub post_type: Option<String>,
    pub meta_event_type: Option<String>,
    pub status: Option<OneBotDataStatus>,
    pub interval: Option<i64>,
    // message
    pub user_id: Option<i64>,
    pub message_id: Option<i64>,
    pub real_id: Option<i64>,
    pub message_type: Option<String>,
    pub sender: Option<OneBotDataSender>,
    pub raw_message: Option<String>,
    pub font: Option<i64>,
    pub sub_type: Option<String>,
    pub message: Option<Vec<OneBotDataMessage>>,
    pub message_format: Option<String>,
    pub group_id: Option<i64>,
    // friend
    pub request_type: Option<String>,
    pub comment: Option<String>,
    pub flag: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OneBotDataStatus {
    pub online: Option<bool>,
    pub good: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OneBotDataSender {
    pub user_id: Option<i64>,
    pub nickname: Option<String>,
    pub card: Option<String>,
    pub role: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OneBotDataMessage {
    pub data: Option<OneBotDataMessageData>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OneBotDataMessageData {
    pub text: Option<String>,
    pub qq: Option<String>,
    pub file: Option<String>,
    pub url: Option<String>,
    pub file_size: Option<String>,
}

// 发送消息构造
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendOneBotGroup {
    user_id: Option<String>,
    group_id: Option<String>,
    message: Vec<SendOneBotGroupMessage>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendMessage {
    message_type: SendMessageType,
    user_id: Option<i64>,
    group_id: Option<i64>,
    message: Vec<SendOneBotGroupMessage>,
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
pub struct SendOneBotGroupMessage {
    r#type: String,
    data: SendOneBotGroupMessageData,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendOneBotGroupMessageData {
    id: Option<String>,
    qq: Option<String>,
    name: Option<String>,
    text: Option<String>,
    file: Option<String>,
}

pub async fn send_msg(
    message_type: SendMessageType,
    user_id: Option<i64>,
    group_id: Option<i64>,
    text: &str,
    at: i64,
) -> String {
    let config = Config::get().await;
    let url = format!("{}/send_msg", config.bot.unwrap().url.unwrap());

    let message = if text.starts_with("file://")
        || text.starts_with("http://")
        || text.starts_with("https://")
        || text.starts_with("base64://")
        || text.starts_with("data:image/png;base64")
    {
        if at < 0 {
            vec![send_image(text)]
        } else {
            vec![send_at(at), send_image(text)]
        }
    } else {
        if at < 0 {
            vec![send_text(text)]
        } else {
            vec![send_at(at), send_text(&format!(" {text}"))]
        }
    };
    let group_id = match message_type {
        SendMessageType::Group => group_id,
        SendMessageType::Private => None,
    };
    let send = SendMessage {
        message_type,
        user_id,
        group_id,
        message,
        auto_escape: None,
    };
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

pub async fn _send_group_msg(group_id: &str, text: &str, at: i64) {
    let config = Config::get().await;
    let url = format!("{}/send_group_msg", config.bot.unwrap().url.unwrap());

    let message = match at {
        -1 => vec![send_text(text)],
        _ => vec![send_at(at), send_text(&format!(" {text}"))],
    };
    let send = SendOneBotGroup {
        user_id: None,
        group_id: Option::from(group_id.to_string()),
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

pub async fn _send_user_msg(user_id: &str, group_id: Option<String>, text: &str) {
    let config = Config::get().await;
    let url = format!("{}/send_private_msg", config.bot.unwrap().url.unwrap());

    let message = vec![send_text(text)];
    let send = match group_id {
        Some(id) => SendOneBotGroup {
            user_id: Option::from(user_id.to_string()),
            group_id: Option::from(id),
            message,
        },
        None => SendOneBotGroup {
            user_id: Option::from(user_id.to_string()),
            group_id: None,
            message,
        },
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
    let url = format!(
        "{}/set_friend_add_request",
        config.bot.unwrap().url.unwrap()
    );
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

fn send_text(text: &str) -> SendOneBotGroupMessage {
    SendOneBotGroupMessage {
        r#type: "text".to_string(),
        data: SendOneBotGroupMessageData {
            id: None,
            qq: None,
            name: None,
            text: Option::from(text.to_string()),
            file: None,
        },
    }
}

fn send_image(path: &str) -> SendOneBotGroupMessage {
    SendOneBotGroupMessage {
        r#type: "image".to_string(),
        data: SendOneBotGroupMessageData {
            id: None,
            qq: None,
            name: None,
            text: None,
            file: Option::from(path.to_string()),
        },
    }
}

fn send_at(id: i64) -> SendOneBotGroupMessage {
    match id {
        0 => SendOneBotGroupMessage {
            r#type: "at".to_string(),
            data: SendOneBotGroupMessageData {
                id: None,
                qq: Option::from("all".to_string()),
                name: None,
                text: None,
                file: None,
            },
        },
        _ => SendOneBotGroupMessage {
            r#type: "at".to_string(),
            data: SendOneBotGroupMessageData {
                id: None,
                qq: Option::from(id.to_string()),
                name: None,
                text: None,
                file: None,
            },
        },
    }
}
