use crate::api::cq_http::*;
use crate::api::*;
use crate::util::Config;
use crate::*;
use chrono::{Datelike, Local, NaiveDateTime};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub async fn listen(cq_data: CqData<'_>, msg: String, config: Config) {
    let chat_use = config.chat_use.unwrap();
    let use_groups = chat_use.group.unwrap_or_default();
    let use_user = chat_use.user.unwrap_or_default();
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
                let result = set_jin_time(Option::from(time.to_string()), None, Some(1329997614.to_string())).await;
                if result > 0 {
                    for use_group in &use_groups {
                        send_msg(SendMessageType::Group, cq_data.user_id, Some(*use_group), "新一轮时间已更新，请回复指令 40时间 获取时间！", 0).await;
                    }
                }
            }
        }
        if msg.contains("艾特") {
            send_msg(SendMessageType::Group, cq_data.user_id, cq_data.group_id, "有事没事艾特一下", 0).await;
        }
        if msg.eq("指令") {
            let mut text = String::from("指令");
            text.push_str("\n40时间");
            send_msg(SendMessageType::Group, cq_data.user_id, cq_data.group_id, &text, -1).await;
        }
        if msg.eq("40时间") && (use_groups.contains(&group) || group == 622678662) {
            let result = get_jin_time(sender.unwrap(), group).await;
            let send_res = send_msg(SendMessageType::Private, cq_data.user_id, cq_data.group_id, &format!("40时间 {result}"), -1).await;
            if send_res.eq("ok") {
                send_msg(SendMessageType::Group, cq_data.user_id, cq_data.group_id, "请查看私聊", -1).await;
            } else {
                send_msg(SendMessageType::Group, cq_data.user_id, cq_data.group_id, "请按公告说明添加机器人好友", -1).await;
            }
        }
    }
    if let Some(userid) = sender {
        if msg.eq("指令") && cq_data.group_id == None && use_user.contains(&userid) {
            let mut text = String::from("指令");
            text.push_str("\n发布时间#1970-10-01 08:00");
            text.push_str("\n偏差时间#<number>");
            text.push_str("\n成员列表/群列表");
            text.push_str("\n更新成员#<qq_number>#<number/type>");
            text.push_str("\n更新成员type参数：");
            text.push_str("\n  白名单");
            text.push_str("\n  内部群");
            text.push_str("\n  外部群");
            text.push_str("\n  黑名单");
            send_msg(SendMessageType::Private, Option::from(userid), group_id, &text, -1).await;
        }
        if msg.starts_with("发布时间#") && use_user.contains(&userid) {
            let time_str = msg.split('#').last().unwrap_or("2024-10-01 00:00");
            log_info!("提取时间 {time_str}");
            let result = match NaiveDateTime::parse_from_str(time_str, "%Y-%m-%d %H:%M") {
                Ok(parse) => {
                    set_jin_time(Option::from(parse.to_string()), None, Some(userid.to_string())).await
                }
                Err(_) => {
                    0
                }
            };
            if result > 0 {
                for use_group in &use_groups {
                    send_msg(SendMessageType::Group, cq_data.user_id, Some(*use_group), "新一轮时间已更新，请回复指令 40时间 获取时间！", 0).await;
                }
            }
        }
        if msg.eq("成员列表") && use_user.contains(&userid) {
            let users = get_users(userid, 2).await;
            let black_users = get_users(userid, 9).await;
            let mut text = String::from("成员列表");
            for user in users {
                let word = format!("\n{} | 偏差 {} | 白名单", user.name.unwrap(), user.view.unwrap());
                text.push_str(&word);
            }
            for user in black_users {
                let word = format!("\n{} | 偏差 {} | 黑名单", user.name.unwrap(), user.view.unwrap());
                text.push_str(&word);
            }
            send_msg(SendMessageType::Private, Option::from(userid), group_id, &text, -1).await;
        }
        if msg.eq("群列表") && use_user.contains(&userid) {
            let groups = get_users(userid, 4).await;
            let out_groups = get_users(userid, 5).await;
            let mut text = String::from("群列表");
            for group in groups {
                let word = format!("\n{} | 偏差 {} | 内部群", group.name.unwrap(), group.view.unwrap());
                text.push_str(&word);
            }
            for group in out_groups {
                let word = format!("\n{} | 偏差 {} | 外部群", group.name.unwrap(), group.view.unwrap());
                text.push_str(&word);
            }
            send_msg(SendMessageType::Private, Option::from(userid), group_id, &text, -1).await;
        }
        if msg.starts_with("更新成员#") && use_user.contains(&userid) {
            let split = msg.split("#").collect::<Vec<&str>>();
            let user = *split.get(1).unwrap();

            let result = if let Ok(view) = split.get(2).unwrap_or(&"0").parse::<i64>() {
                set_user_view(user, view).await
            } else {
                let type_id = match split.get(2) {
                    Some(&"白名单") => 2,
                    Some(&"内部群") => 4,
                    Some(&"外部群") => 5,
                    Some(&"黑名单") => 9,
                    _ => -1
                };
                set_user_type(user, type_id).await
            };
            if result > 0 {
                send_msg(SendMessageType::Private, cq_data.user_id, cq_data.group_id, "修改成功", -1).await;
            }
        }
        if msg.starts_with("批量更新成员#") {
            let split = msg.split("#").collect::<Vec<&str>>();
            let users = split.get(1).unwrap();
            let mut result = 0;
            if let Ok(view) = split.get(2).unwrap_or(&"0").parse::<i64>() {
                let mut users = users.trim_end_matches(",").split(",");
                while let Some(user) = users.next() {
                    result += set_user_view(user, view).await
                }
            };
            if result > 0 {
                send_msg(SendMessageType::Private, cq_data.user_id, cq_data.group_id, "修改成功", -1).await;
            };
        }
        if msg.starts_with("偏差时间#") && use_user.contains(&userid) {
            let deviate_time = msg.split("#").collect::<Vec<&str>>();
            let deviate_time = deviate_time[1].parse::<i64>().unwrap();
            let result = set_jin_time(None, Some(deviate_time), Some(userid.to_string())).await;
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct JinApi {
    status: i64,
    message: String,
    data: Option<Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserModel {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub user_type: Option<i64>,
    pub status: Option<i64>,
    pub view: Option<i64>,
}

async fn get_jin_time(user: i64, group: i64) -> String {
    let config = Config::get().await;
    let url = format!("{}/get_time", config.api.unwrap().url.unwrap());
    let json = json!({
        "id": 1,
        "user": user.to_string(),
        "group": group.to_string(),
    });
    let response = Client::new().post(url).json(&json).send().await;
    match response {
        Ok(re) => {
            let res = re.json::<JinApi>().await.unwrap();
            log_info!("Set Result {}", &res.message);
            res.data.unwrap()["up_time"].as_str().unwrap_or("时间获取失败").to_string()
        }
        Err(e) => {
            log_warn!("Not Res {}", e);
            "时间接口失败".to_string()
        }
    }
}

async fn set_jin_time(up_time: Option<String>, deviate_time: Option<i64>, user: Option<String>) -> i64 {
    let config = Config::get().await;
    let url = format!("{}/set_time", config.api.unwrap().url.unwrap());
    let up = UpJinTime {
        id: 1,
        up_time,
        deviate_time,
        user,
    };
    let response = Client::new().post(url)
        .json(&up).send().await;
    match response {
        Ok(re) => {
            let res = re.json::<JinApi>().await.unwrap();
            log_info!("Set Result {}", &res.message);
            res.data.unwrap().as_i64().unwrap()
        }
        Err(e) => {
            log_warn!("Not Res {}", e);
            0
        }
    }
}

///
///
/// # Arguments
///
/// * `user`: 操作用户
/// * `user_type`: 要查看的权限组
///
/// returns: i64
///
/// # Examples
///
/// ```
///
/// ```
async fn get_users(user: i64, user_type: i64) -> Vec<UserModel> {
    let config = Config::get().await;
    let url = format!("{}/get_users", config.api.unwrap().url.unwrap());
    let json = json!({
        "id": 0,
        "name": user.to_string(),
        "user_type": user_type
    });
    match Client::new().post(url).json(&json).send().await {
        Ok(re) => {
            let res = re.json::<JinApi>().await.unwrap();
            log_info!("Get Users {}", &res.message);
            serde_json::from_value(res.data.unwrap()).unwrap()
        }
        Err(e) => {
            log_warn!("Not Res {}", e);
            vec![]
        }
    }
}

async fn set_user_view(user: &str, view: i64) -> i64 {
    let config = Config::get().await;
    let url = format!("{}/set_user_view", config.api.unwrap().url.unwrap());
    let json = json!({
        "id": 0,
        "name": user,
        "view": view
    });
    match Client::new().post(url).json(&json).send().await {
        Ok(re) => {
            let res = re.json::<JinApi>().await.unwrap();
            log_info!("Set UserView {}", &res.message);
            res.data.unwrap().as_i64().unwrap()
        }
        Err(e) => {
            log_warn!("Not Res {}", e);
            0
        }
    }
}

async fn set_user_type(user: &str, user_type: i64) -> i64 {
    //小于0跳过
    if user_type < 0 {
        return 0;
    }
    let config = Config::get().await;
    let url = format!("{}/set_user_type", config.api.unwrap().url.unwrap());
    let json = json!({
        "id": 0,
        "name": user,
        "user_type": user_type
    });
    match Client::new().post(url).json(&json).send().await {
        Ok(re) => {
            let res = re.json::<JinApi>().await.unwrap();
            log_info!("Set UserType {}", &res.message);
            res.data.unwrap().as_i64().unwrap()
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
