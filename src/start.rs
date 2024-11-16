use crate::api::cc_http::*;
use crate::util::Config;
use chrono::{Datelike, Local, NaiveDateTime};
use futures_util::AsyncReadExt;
use rand::Rng;
use reqwest::Client;
use serde_json::{json, Value};
use void_log::*;

pub async fn listen(cc_body: CcDataBody) {
    let msg = cc_body.message.unwrap_or_default().content.unwrap_or_default();

    // *******************群聊消息*******************
    if let Some(group) = cc_body.channel.unwrap().id {
        if msg.eq("指令") {
            let api = zn_api().await;
            send_group_msg(&group, &api, -1).await;
        } else {
            let mut rng = rand::thread_rng();
            let y = rng.gen_range(0.00..1000.00);

            if y > 980.00 {
                let api = zn_api().await;
                send_group_msg(&group, &api, -1).await;
            }

            log_info!("{y}")
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

async fn zn_api() -> String {
    let url = "https://whyta.cn/api/tx/saylove?key=cc8cba0a7069";
    let client = Client::new()
        .get(url).send().await.unwrap();
    let value = client.json::<Value>().await.unwrap();
    log_info!("{:?}", value);
    value["result"]["content"].as_str().unwrap().to_string()
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
