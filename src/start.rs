use chrono::{Datelike, Local, NaiveDateTime};
use futures_util::AsyncReadExt;
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

    // *******************群聊消息*******************
    if let Some(group) = cq_data.group_id {
        if msg.eq("指令") {
            send_group_msg(group, "CRAZY TEST", -1).await;
        }
        if msg.eq("时间") {
            set_xin().await;
            let text = "<img src=\"http://get.cocsnipe.top/listTimeImg\"/>";
            send_group_msg(group, &text, -1).await;
        }
        if msg.contains("涩图#") {
            let vec = msg.split("#").collect::<Vec<&str>>();
            let img_url = get_comfy(vec[1].to_string()).await.replace("127.0.0.1:8188", "1.orgvoid.top:50009");
            let text = format!("<img src='{}'/>", img_url);
            send_group_msg(group, &text, -1).await;
        }
        if msg.contains("查配置#") {
            let vec = msg.split("#").collect::<Vec<&str>>();
            let img_url = format!("http://app.orgvoid.top/clan/{}", vec[1]);
            let text = format!("<img src='{}'/>", img_url);
            send_group_msg(group, &text, -1).await;
        }
    }

    // *******************私聊消息*******************
    if let Some(userid) = sender {
        // 更新#s盟#2024-01-01 10:00
        if msg.contains("更新#") {
            let vec = msg.split("#").collect::<Vec<&str>>();
            let time = vec[2].replace("：", ":");
            let union_id = match vec[1] {
                "zero" => 11,
                "积分" => 21,
                "鑫盟" => 41,
                "g盟" => 52,
                "g盟高配" => 53,
                "fwa" => 81,
                "s盟" => 100,
                "都城" => 201,
                _ => 0
            };
            let json = json!({
                "id": union_id,
                "time": time
            });
            log_info!("{json}");
            let res = set_time(json).await;
            log_info!("发信人 {:?}", &userid);
            send_user_msg(userid, group_id, &res).await;
        }
    }

    log_info!("{}", &msg);
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

async fn get_comfy(text: String) -> String {
    let url = "http://127.0.0.1:50000/get_comfy";
    let json = json!({
        "prompt": text
    });
    let response = Client::new().post(url).json(&json).send().await.unwrap();
    log_info!("{:?}{:?}",response.status(), response.headers());
    let res = response.json::<String>().await.unwrap();
    res
}

async fn set_time(json: Value) -> String {
    let response = Client::new()
        .post("http://get.cocsnipe.top/setTime")
        .json(&json)
        .send()
        .await
        .unwrap();
    format!("{}", response.text().await.unwrap_or("没有更新".to_string()))
}

async fn set_xin() {
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
