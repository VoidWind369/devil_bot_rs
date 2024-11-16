use crate::api::cq_http::*;
use crate::api::*;
use crate::util::Config;
use chrono::NaiveDateTime;
use void_log::log_info;
use crate::tool::{get_jin_time, get_users, set_jin_time, set_user_type, set_user_view, to_native_dt};

pub async fn listen(cq_data: CqData<'_>, msg: String, config: Config) {
    let chat_use = config.chat_use.unwrap();
    let use_groups = chat_use.group.unwrap_or_default();
    let use_user = chat_use.user.unwrap_or_default();
    let sender = cq_data.sender.unwrap().user_id;
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
            text.push_str("\n更新成员#<qq>#<number/type>");
            text.push_str("\n批量更新成员#<qq1,qq2>#<number>");
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
            let users = split.get(1).unwrap().replace("，", ",");
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
