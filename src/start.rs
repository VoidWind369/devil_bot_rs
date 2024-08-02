use chrono::{Datelike, Local, NaiveDateTime};
use reqwest::{Client};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::fs;
use tokio::io::AsyncReadExt;
use void_log::*;
use crate::api::*;
use crate::api::cc_http::*;
use crate::util::{AdSetting, Config};

pub async fn ad_loop() {
    let ad_setting = AdSetting::get().await;
    let groups = &ad_setting.groups;

    let ad_str = ad_setting.ad_file().await;

    if ad_setting.send {
        for group in groups {
            let _time = Local::now().naive_local().format("%Y-%m-%d %H:%M:%S").to_string();
            send_group_msg(group, &ad_str, -1).await;
        }
    }
    tokio::time::sleep(tokio::time::Duration::from_millis(ad_setting.time.unwrap_or_default() * 1000)).await;
}

pub async fn listen(cc_body: CcDataBody, config: &Config) {
    let use_group = config.use_group.unwrap_or_default();
    let sender = cc_body.user.unwrap().id.unwrap_or_default();
    let msg = cc_body.message.unwrap_or_default().content.unwrap_or_default();

    // *******************群聊消息*******************
    if let Some(group) = cc_body.channel.unwrap().id {
        if msg.eq("指令") {
            send_group_msg(&group, "CRAZY TEST", -1).await;
        }

        // if msg.eq("广告开关") {
        //     let ad_setting = AdSetting::get().await;
        //     let mut ad_set = ad_setting.clone();
        //     match ad_set.send {
        //         true => false,
        //         false => true
        //     };
        //     ad_set.set().await;
        // }
        //
        // if msg.starts_with("添加群#") {
        //     let vec = msg.split("#").collect::<Vec<&str>>();
        //     let ad_setting = AdSetting::get().await;
        //     let mut ad_set = ad_setting.clone();
        //     if !ad_set.groups.contains(&vec[1].to_string()) {
        //         ad_set.groups.push(vec[1].to_string());
        //         log_info!("添加群号{:?}", &ad_setting);
        //         ad_set.set().await;
        //     };
        // }

        if msg.contains("查部落#") || msg.contains("部落配置#") {
            let vec = msg.split("#").collect::<Vec<&str>>();
            let img_url = format!("http://get.cocsnipe.top/coc_clan_img/{}", vec[1]);
            let text = format!("<img src='{}'/>", img_url);
            send_group_msg(&group, &text, -1).await;
        }

        if msg.contains("查玩家#") {
            let vec = msg.split("#").collect::<Vec<&str>>();
            let img_url = format!("http://get.cocsnipe.top/coc_player_img/{}", vec[1]);
            let text = format!("<img src='{}'/>", img_url);
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

async fn set_time(json: Value) -> String {
    let response = Client::new()
        .post("http://get.cocsnipe.top/setTime")
        .json(&json)
        .send()
        .await
        .unwrap();
    format!("{}", response.text().await.unwrap_or("没有更新".to_string()))
}
