use reqwest::Client;
use serde_json::{json, Value};
use crate::log_info;
use crate::api::{EventsBody, send_message};
use crate::config::Config;

pub async fn listen_msg(events_body: EventsBody, config: &Config) {
    log_info!("群聊消息");
    let Some(channel) = events_body.channel else { todo!() };
    let Some(message) = events_body.message else { todo!() };

    if Some("指令".to_string()).eq(&message.content) {
        let res = send_message(&channel.id, "CRAZY TEST", config).await;
        log_info!("{res}")
    }

    if Some("时间".to_string()).eq(&message.content) {
        set_xin().await;
        let response = Client::new()
            .get("http://get.cocsnipe.top/listTime".to_string())
            .send()
            .await
            .unwrap();

        let res: Vec<Value> = response.json().await.expect("获取失败");

        let mut text = String::new();
        text.push_str("⟦ 时间集 ⟧");
        for re in res {
            let union = re["union"].as_str().expect("名称空");
            let day = re["day"].as_str().expect("时间空");
            let time = &re["un_time"].as_str().expect("时间空")[0..5];
            text.push_str(&format!("\r\n{} {}{}", union, day, time))
        }
        let res = send_message(&channel.id, &text, config).await;
        log_info!("{res}")
    }

    if Some("爱玩".to_string()).eq(&message.content) || Some("启动码".to_string()).eq(&message.content) {
        let qdm = get_aw_qdm().await;
        let mut text = String::new();
        text.push_str("启动码: ");
        text.push_str(&qdm[0]);
        text.push_str("\r\n下次刷新: ");
        text.push_str(&qdm[1]);
        let res = send_message(&channel.id, &text, config).await;
        log_info!("{res}")
    }
}

pub async fn listen_user_msg(events_body: EventsBody, config: &Config) {
    log_info!("私信消息");
    let Some(channel) = events_body.channel else { todo!() };
    let Some(message) = events_body.message else { todo!() };

    // 更新#s盟#2024-01-01 10:00
    if message.clone().content.unwrap_or("".to_string()).contains("更新#") {
        if let Some(msg) = message.content {
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

            set_time(json).await;
        }
        log_info!("发信人 {:?}", &channel.id);
        let text = "Updating";
        let res = send_message(&channel.id, text, config).await;
        log_info!("{res}")
    }
}

async fn set_time(json: Value) {
    let response = Client::new()
        .post("http://get.cocsnipe.top/setTime")
        .json(&json)
        .send()
        .await
        .unwrap();
    log_info!("{}", response.text().await.unwrap_or("没有更新".to_string()))
}

async fn set_xin() {
    let response = Client::new()
        .get("http://get.cocsnipe.top/setXm")
        .send()
        .await
        .unwrap();
    log_info!("鑫盟{}", response.text().await.unwrap_or("没有更新".to_string()))
}

async fn get_aw_qdm() -> [String; 2] {
    let response = Client::new()
        .get("http://get.cocsnipe.top/aw")
        .send().await.expect("getAwErr");
    let res = response.json().await.unwrap();
    log_info!("启动码{:?}", res);
    res
}
