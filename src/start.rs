use crate::api::one_bot::{send_msg, OneBotData, SendMessageType};
use crate::om_api::record::Record;
use crate::util::Config;
use base64::Engine;
use image::ImageBuffer;
use image::ImageFormat::Png;
use rand::Rng;
use reqwest::Client;
use serde_json::{json, Value};
use std::io::Cursor;
use base64::prelude::BASE64_STANDARD;
use futures_util::SinkExt;
use void_log::*;

pub async fn listen(ob_data: OneBotData) {
    let (mut msg, mut at) = ("".to_string(), "".to_string());
    for data_message in ob_data.message.unwrap_or_default() {
        if let Some(msg_data) = data_message.data {
            if let Some(text) = msg_data.text {
                msg = text
            }
            if let Some(qq) = msg_data.qq {
                at = qq
            }
        } else {
            continue;
        }
    }

    // *******************群聊消息*******************
    if let Some(group) = ob_data.group_id {
        log_info!("消息 {}", &msg);
        if msg.eq("指令") {
            let api = zn_api().await;
            send_msg(
                SendMessageType::Group,
                ob_data.user_id,
                Some(group),
                &api,
                -1,
            )
            .await;
        } else {
            let mut rng = rand::thread_rng();
            let y = rng.gen_range(0.00..1000.00);

            if y > 980.00 {
                let api = zn_api().await;
                send_msg(
                    SendMessageType::Group,
                    ob_data.user_id,
                    Some(group),
                    &api,
                    -1,
                )
                .await;
            }

            log_info!("{y}")
        }
        if msg.starts_with("查询日记#") {
            let split_str = msg.split('#');
            let tag = split_str.last().unwrap();
            // tokio::fs::create_dir_all("./cache/record/").await.unwrap();
            let img = Record::new(tag, 0).await.list_img(50).await;
            // let base64_str = base64::prelude::BASE64_STANDARD_NO_PAD.encode(img.as_bytes());
            let mut bytes: Vec<u8> = Vec::new();
            img.write_to(&mut Cursor::new(&mut bytes), Png).unwrap();
            send_msg(
                SendMessageType::Group,
                ob_data.user_id,
                Some(group),
                &format!("data:image/png;base64,{}", BASE64_STANDARD.encode(&bytes)),
                -1,
            )
            .await;
        }
    }
}

async fn zn_api() -> String {
    let api = Config::get().await.get_api();
    let url = format!(
        "{}?key={}",
        api.url.unwrap_or_default(),
        api.token.unwrap_or_default()
    );
    let client = Client::new().get(url).send().await.unwrap();
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
    log_info!("{:?}{:?}", response.status(), response.headers());
    let res = response.json::<String>().await.unwrap();
    res
}
