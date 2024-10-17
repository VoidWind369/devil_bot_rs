use crate::api::cc_http::*;
use crate::util::Config;
use chrono::{Datelike, Local, NaiveDateTime};
use futures_util::AsyncReadExt;
use reqwest::Client;
use serde_json::{json, Value};
use void_log::*;

pub async fn listen(cc_body: CcDataBody) {
    let config = Config::get().await;
    let url = config.api.unwrap_or_default().url.unwrap_or_default();
    // let use_group = config.chat_use.unwrap_or_default().group.unwrap_or_default();
    let sender = cc_body.user.unwrap().id.unwrap_or_default();
    let msg = cc_body.message.unwrap_or_default().content.unwrap_or_default();

    // *******************群聊消息*******************
    if let Some(group) = cc_body.channel.unwrap().id {
        if msg.eq("指令") {
            send_group_msg(&group, "CRAZY TEST", -1).await;
        }
        if msg.eq("时间") {
            //set_xin().await;
            let text = format!("<img src='{}/listTimeImg'/>", &url);
            send_group_msg(&group, &text, -1).await;
        }
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
            let res = set_time(&url, json).await;
            log_info!("发信人：{sender}");
            send_group_msg(&group, &res, -1).await;
        }
        // comfy ui
        // if msg.contains("涩图#") {
        //     let vec = msg.split("#").collect::<Vec<&str>>();
        //     let img_url = get_comfy(vec[1].to_string()).await.replace("127.0.0.1:8188", "1.orgvoid.top:50009");
        //     // let img_url = get_comfy(vec[1].to_string()).await;
        //     let text = format!("<img src='{}'/>", img_url);
        //     send_group_msg(&group, &text, -1).await;
        // }
        if msg.contains("查部落#") || msg.contains("部落配置#") {
            let vec = msg.split("#").collect::<Vec<&str>>();
            let text = format!("<img src='{}/coc/coc_clan_img/{}'/>", &url, vec[1]);
            send_group_msg(&group, &text, -1).await;
        }
        if msg.contains("查玩家#") {
            let vec = msg.split("#").collect::<Vec<&str>>();
            let text = format!("<img src='{}/coc/coc_player_img/{}'/>", &url, vec[1]);
            send_group_msg(&group, &text, -1).await;
        }
    }

    log_info!("消息 {}", &msg);
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

async fn set_time(url: &str, json: Value) -> String {
    let response = Client::new()
        .post(format!("{url}/setTime"))
        .json(&json)
        .send()
        .await
        .unwrap();
    format!("{}", response.text().await.unwrap_or("没有更新".to_string()))
}

async fn set_xin() {
    let config = Config::get().await;
    let url = config.api.unwrap_or_default().url.unwrap_or_default();
    let response = Client::new()
        .get(format!("{url}/setXm"))
        .send()
        .await
        .unwrap();
    log_info!("鑫盟{}", response.text().await.unwrap_or("没有更新".to_string()))
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
