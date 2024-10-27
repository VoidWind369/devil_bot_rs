use chrono::{Datelike, Local, NaiveDateTime};
use reqwest::{Client};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::*;
use crate::api::*;
use crate::api::cq_http::*;
use crate::util::Config;

pub async fn listen(cq_data: CqData<'_>, msg: String, config: Config) {
    let use_groups = config.chat_use.unwrap().group.unwrap_or_default();
    let sender = cq_data.sender.unwrap().user_id;
    // let msg = cq_data.raw_message.unwrap_or("".to_string());
    let group_id = cq_data.group_id;
    if let Some(group) = cq_data.group_id {
        if msg.contains("群公告") && msg.contains("开战40人匹配") && msg.contains("输赢") {
            if let Ok(msg) = cq_util::RawMessageJson::format_json(&msg) {
                let prompt = msg.prompt.unwrap();
                log_info!("prompt {}", &prompt);
                let prompt_split = prompt
                    .trim_start_matches("[群公告]🌿")
                    .split("～").collect::<Vec<&str>>();
                let time = to_native_dt(prompt_split[0].trim_end());
                let result = set_jin_time(Option::from(time.to_string()), None).await;
                if result > 0 {
                    for use_group in &use_groups {
                        send_msg(SendMessageType::Group, cq_data.user_id, Some(*use_group), "新一轮时间已更新，请回复指令 40时间 获取时间！", 0).await;
                    }
                }
            }
        }
        if msg.contains("艾特") {
            send_msg(SendMessageType::Group, cq_data.user_id, cq_data.group_id, "嘎嘎", 0).await;
        }
        if msg.eq("指令") {
            send_msg(SendMessageType::Private, cq_data.user_id, cq_data.group_id, "zl", -1).await;
        }
        if msg.eq("40时间") && (use_groups.contains(&group) || group == 622678662) {
            let result = get_jin_time(sender.unwrap()).await;
            let send_res = send_msg(SendMessageType::Private, cq_data.user_id, cq_data.group_id, &format!("40时间 {result}"), -1).await;
            if send_res.eq("ok") {
                send_msg(SendMessageType::Group, cq_data.user_id, cq_data.group_id, "请查看私聊", -1).await;
            } else {
                send_msg(SendMessageType::Group, cq_data.user_id, cq_data.group_id, "请按公告说明添加机器人好友", -1).await;
            }
        }
    }
    if let Some(userid) = sender {
        if msg.eq("指令") {
            send_user_msg(userid, group_id, "zl").await;
        }
        if msg.starts_with("发布时间#") {
            let time_str = msg.split('#').last().unwrap_or("2024-10-01 00:00:00");
            let time = to_native_dt(time_str);
            let result = set_jin_time(Option::from(time.to_string()), None).await;
            if result > 0 {
                for use_group in &use_groups {
                    send_msg(SendMessageType::Group, cq_data.user_id, Some(*use_group), "新一轮时间已更新，请回复指令 40时间 获取时间！", 0).await;
                }
            }
        }
        if msg.contains("偏差时间#") {
            let deviate_time = msg.split("#").collect::<Vec<&str>>();
            let deviate_time = deviate_time[1].parse::<i64>().unwrap();
            let result = set_jin_time(None, Some(deviate_time)).await;
            if result > 0 {
                send_msg(SendMessageType::Private, cq_data.user_id, cq_data.group_id, "修改成功", -1).await;
            }
        }
    }
}

pub async fn listen_request(cq_data: CqData<'_>, request_type: &str) {
    let sender = cq_data.user_id;
    if request_type.eq("friend") {
        log_info!("添加好友 {}", &sender.unwrap());
        match cq_data.comment {
            Some("40时间") => {
                set_friend_add_request(cq_data.flag.unwrap(), true).await;
            }
            _ => {
                set_friend_add_request(cq_data.flag.unwrap(), false).await;
            }
        }
    }
}

fn to_native_dt(time_str: &str) -> NaiveDateTime {
    let full_str = format!("{}-{time_str}", Local::now().year());
    let fmt = "%Y-%m月%d号 %H:%M";
    match NaiveDateTime::parse_from_str(&full_str, fmt) {
        Ok(ndt) => { ndt }
        Err(e) => {
            log_warn!("Format Time Error {e}");
            Default::default()
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct UpJinTime {
    id: i64,
    up_time: Option<String>,
    deviate_time: Option<i64>,
    user: Option<String>,
}

async fn get_jin_time(user: i64) -> String {
    let config = Config::get().await;
    let url = format!("{}/get_time", config.api.unwrap().url.unwrap());
    let json = json!({
        "id": 1,
        "user": user.to_string()
    });
    let response = Client::new().post(url).json(&json).send().await;
    match response {
        Ok(re) => {
            let res = re.json::<Value>().await.unwrap();
            log_info!("Set Result {}", &res);
            res["up_time"].as_str().unwrap_or("时间获取失败").to_string()
        }
        Err(e) => {
            log_warn!("Not Res {}", e);
            "时间接口失败".to_string()
        }
    }
}

async fn set_jin_time(up_time: Option<String>, deviate_time: Option<i64>) -> i64 {
    let config = Config::get().await;
    let url = format!("{}/set_time", config.api.unwrap().url.unwrap());
    let up = UpJinTime {
        id: 1,
        up_time,
        deviate_time,
        user: Option::from("1329997614".to_string()),
    };
    let response = Client::new().post(url)
        .json(&up).send().await;
    match response {
        Ok(re) => {
            let res = re.text().await.unwrap();
            log_info!("Set Result {}", &res);
            res.parse::<i64>().unwrap()
        }
        Err(e) => {
            log_warn!("Not Res {}", e);
            0
        }
    }
}

async fn _set_time(json: Value) {
    let response = Client::new()
        .post("http://get.cocsnipe.top/setTime")
        .json(&json)
        .send()
        .await
        .unwrap();
    log_info!("{}", response.text().await.unwrap_or("没有更新".to_string()))
}
