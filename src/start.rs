use chrono::{Datelike, Local, NaiveDateTime};
use reqwest::{Client};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::*;
use crate::api::*;
use crate::api::cq_http::*;
use crate::util::Config;

pub async fn listen(cq_data: CqData<'_>, msg: String, config: &Config) {
    let use_group = config.use_group.unwrap();
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
                    send_group_msg(use_group, "新一轮时间已更新，请回复指令 40时间 获取时间！", 0).await;
                }
            }
        }
        if msg.contains("艾特") {
            send_group_msg(group, "嘎嘎", 0).await;
        }
        if msg.eq("指令") {
            send_user_msg(sender.unwrap(), group_id, "zl").await;
        }
        if msg.eq("40时间") && (group_id == Some(use_group) || group_id == Some(622678662)) {
            let result = get_jin_time(sender.unwrap()).await;
            send_group_msg(group, "请查看私聊", sender.unwrap()).await;
            send_user_msg(sender.unwrap(), group_id, &format!("40时间 {result}")).await;
        }
    }
    if let Some(userid) = sender {
        if msg.eq("指令") {
            send_user_msg(userid, group_id, "zl").await;
        }
        if msg.contains("偏差时间#") {
            let deviate_time = msg.split("#").collect::<Vec<&str>>();
            let deviate_time = deviate_time[1].parse::<i64>().unwrap();
            let result = set_jin_time(None, Some(deviate_time)).await;
            if result > 0 {
                send_user_msg(userid, group_id, "修改成功").await;
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
    let url = format!("{}/get_time", config.server_url.unwrap());
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
    let url = format!("{}/set_time", config.server_url.unwrap());
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

// pub async fn listen_msg(events_body: EventsBody, config: &Config) {
//     log_info!("群聊消息");
//     let Some(channel) = events_body.channel else { panic!("NONE") };
//     let Some(message) = events_body.message else { panic!("NONE") };
//     let Some(user) = events_body.user else { panic!("NONE") };
//
//     log_info!("{:?}", &message.content);
//     if Some("指令".to_string()).eq(&message.content) {
//         let res = send_message(&channel.id, "CRAZY TEST", config).await;
//         log_info!("{res}")
//     }
//
//     if Some("时间".to_string()).eq(&message.content) {
//         set_xin().await;
//         let text = "<img src=\"http://get.cocsnipe.top/listTimeImg\"/>";
//         let res = send_message(&channel.id, &text, config).await;
//         log_info!("{res}")
//     }
//
//     // if Some("艾特".to_string()).eq(&message.content) {
//     //     let sender = Some(format!("{}", user.id.clone().unwrap()));
//     //     let text = "<at id=\"all\" name=\"全体成员\"/> 嘎嘎";
//     //     let res = send_message(&channel.id, &text, config).await;
//     //     log_info!("{res}")
//     // }
//
//     if message.clone().content.unwrap_or("".to_string()).contains("涩图#") {
//         if let Some(msg) = &message.content {
//             let vec = msg.split("#").collect::<Vec<&str>>();
//
//             let img_url = get_comfy(vec[1].to_string()).await.replace("127.0.0.1:8188", "1.orgvoid.top:50009");
//             let text = format!("<img src='{}'/>", img_url);
//             let res = send_message(&channel.id, &text, config).await;
//             log_info!("{res}")
//         }
//     }
//
//     // if Some("私聊".to_string()).eq(&message.content) {
//     //     let sender = Some(format!("private:{}", user.id.clone().unwrap()));
//     //     let text = "撩骚";
//     //     let res = send_message(&sender, &text, config).await;
//     //     log_info!("{res}")
//     // }
//
//     // if message.clone().content.unwrap_or("".to_string()).contains("查配置#") {
//     //     if let Some(msg) = &message.content {
//     //         let vec = msg.split("#").collect::<Vec<&str>>();
//     //
//     //         let img_url = format!("http://app.orgvoid.top/clan/{}",vec[1]);
//     //         let text = format!("<img src='{}'/>", img_url);
//     //         let res = send_message(&channel.id, &text, config).await;
//     //         log_info!("{res}")
//     //     }
//     // }
//
//     // if Some("爱玩".to_string()).eq(&message.content) || Some("启动码".to_string()).eq(&message.content) {
//     //     let qdm = get_aw_qdm().await;
//     //     let mut text = String::new();
//     //     text.push_str("启动码: ");
//     //     text.push_str(&qdm[0]);
//     //     text.push_str("\r\n下次刷新: ");
//     //     text.push_str(&qdm[1]);
//     //     let res = send_message(&channel.id, &text, config).await;
//     //     log_info!("{res}")
//     // }
// }

// pub async fn listen_user_msg(events_body: EventsBody, config: &Config) {
//     log_info!("私信消息");
//     let Some(channel) = events_body.channel else { panic!("NONE") };
//     let Some(message) = events_body.message else { panic!("NONE") };
//
//     // if Some("私聊".to_string()).eq(&message.content) {
//     //     let text = "撩骚";
//     //     let res = send_message(&channel.id, &text, config).await;
//     // }
//     // 更新#s盟#2024-01-01 10:00
//     if message.clone().content.unwrap_or("".to_string()).contains("更新#") {
//         if let Some(msg) = message.content {
//             let vec = msg.split("#").collect::<Vec<&str>>();
//             let time = vec[2].replace("：", ":");
//             let union_id = match vec[1] {
//                 "zero" => 11,
//                 "积分" => 21,
//                 "鑫盟" => 41,
//                 "g盟" => 52,
//                 "g盟高配" => 53,
//                 "fwa" => 81,
//                 "s盟" => 100,
//                 "都城" => 201,
//                 _ => 0
//             };
//             let json = json!({
//                 "id": union_id,
//                 "time": time
//             });
//             log_info!("{json}");
//
//             set_time(json).await;
//         }
//         log_info!("发信人 {:?}", &channel.id);
//         let text = "Updating";
//         log_info!("{res}")
//     }
// }

async fn _get_comfy(text: String) -> String {
    let url = "http://127.0.0.1:50000/get_comfy";
    let json = json!({
        "prompt": text
    });
    let response = Client::new().post(url).json(&json).send().await.unwrap();
    log_info!("{:?}{:?}",response.status(), response.headers());
    let res = response.json::<String>().await.unwrap();
    res
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

async fn _set_xin() {
    let response = Client::new()
        .get("http://get.cocsnipe.top/setXm")
        .send()
        .await
        .unwrap();
    log_info!("鑫盟{}", response.text().await.unwrap_or("没有更新".to_string()))
}

async fn _get_aw_qdm() -> [String; 2] {
    let response = Client::new()
        .get("http://get.cocsnipe.top/aw")
        .send().await.expect("getAwErr");
    let res = response.json().await.unwrap();
    log_info!("启动码{:?}", res);
    res
}

fn _formal_fwa(string: String) -> String {
    //FWA 开搜时间
    //Saturday, January 20, 2024 8:40 AM
    let binding = string.replace("FWA 开搜时间\n", "");
    let time_str = binding.trim_start_matches(" ");
    let binding = time_str.replace(',', "");
    let vec = binding.split(" ").collect::<Vec<&str>>();
    vec[0].parse().unwrap()
}
