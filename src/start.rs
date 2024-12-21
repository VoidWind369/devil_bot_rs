use crate::api::one_bot::{send_msg, OneBotData, SendMessageType};
use crate::modal::app_admin_power::AppAdminPower;
use crate::modal::app_qq::AppQQ;
use crate::modal::openid::Openid;
use crate::om_api::menu::Menu;
use crate::om_api::record::{base64img, Record};
use crate::util::Config;
use rand::Rng;
use reqwest::Client;
use serde_json::{json, Value};
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

    let user_i64 = ob_data.user_id.unwrap_or_default();

    // *******************群聊消息*******************
    if let Some(group) = ob_data.group_id {
        log_info!("消息 {}", &msg);

        if msg.eq("指令") {
            log_info!("Read menu");
            let img = Menu::from_file("menu.json").await.list_img().await;
            log_info!("Send menu");
            send_msg(
                SendMessageType::Group,
                ob_data.user_id,
                Some(group),
                &format!("data:image/png;base64,{}", base64img(img).await),
                None,
            )
            .await;
            log_info!("Send end")
        }

        if at.starts_with(ob_data.self_id.unwrap().to_string().as_str()) {
            let api = zn_api().await;
            send_msg(
                SendMessageType::Group,
                ob_data.user_id,
                Some(group),
                &api,
                Some(&ob_data.user_id.unwrap().to_string()),
            )
            .await;
        }
        if msg.starts_with("查询日记#") {
            let mut split_str = msg.split('#').skip(1);
            let tag = split_str.next().unwrap_or_default();
            let type_str = split_str
                .next()
                .unwrap_or("0")
                .parse::<char>()
                .unwrap_or_default();
            let img = Record::new_json(tag, type_str).await.list_img(50).await;
            send_msg(
                SendMessageType::Group,
                ob_data.user_id,
                Some(group),
                &format!("data:image/png;base64,{}", base64img(img).await),
                None,
            )
            .await;
        }
        if msg.starts_with("查身份号#") {
            let qq = msg.split('#').nth(1).unwrap_or_default();
            let text = Openid::select_qq(qq).await.unwrap_or_default().to_string();
            send_msg(
                SendMessageType::Group,
                ob_data.user_id,
                Some(group),
                &text,
                None,
            )
            .await;
        }
        if msg.starts_with("修改名称#") {
            let mut split = msg.split('#').skip(1);
            let qq = split.next().unwrap_or_default();
            let name = split.next().unwrap_or_default();
            // 查权限
            if let Ok(app_qq) = AppQQ::select(&user_i64.to_string()).await {
                let openid_id = app_qq.id.unwrap_or_default();
                if let Err(_) = AppAdminPower::select_admin_id(openid_id, 1).await {
                    send_msg(
                        SendMessageType::Group,
                        ob_data.user_id,
                        Some(group),
                        "没有权限",
                        None,
                    )
                    .await;
                    return;
                };
            } else {
                send_msg(
                    SendMessageType::Group,
                    ob_data.user_id,
                    Some(group),
                    "你还没有绑定小程序",
                    None,
                )
                .await;
                return;
            };
            // 执行
            let result = Openid::update_name(qq, name).await.unwrap();
            let text = if result.rows_affected() > 0 {
                Openid::select_qq(qq).await.unwrap_or_default().to_string()
            } else {
                "修改失败".to_string()
            };
            send_msg(
                SendMessageType::Group,
                ob_data.user_id,
                Some(group),
                &text,
                None,
            )
            .await;
        }

        // 彩蛋
        let mut rng = rand::thread_rng();
        let z = rng.gen_range(0.00..1000.00);
        if msg.eq("指令") && z > 800.00 {
            let api = zn_api().await;
            send_msg(
                SendMessageType::Group,
                ob_data.user_id,
                Some(group),
                &api,
                Some(&ob_data.user_id.unwrap().to_string()),
            )
            .await;
        } else {
            let mut rng = rand::thread_rng();
            let y = rng.gen_range(0.00..1000.00);

            if y > 999.90 {
                let api = zn_api().await;
                send_msg(
                    SendMessageType::Group,
                    ob_data.user_id,
                    Some(group),
                    &api,
                    None,
                )
                .await;
            }

            log_info!("{y}")
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
