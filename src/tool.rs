use chrono::{Datelike, Local, NaiveDateTime};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use void_log::{log_info, log_warn};
use crate::util::Config;

pub fn to_native_dt(time_str: &str) -> NaiveDateTime {
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

pub async fn get_jin_time(user: i64, group: i64) -> String {
    let config = Config::get().await;
    let url = format!("{}/time", config.api.unwrap().url.unwrap());
    let json = json!({
        "id": 1,
        "user": user.to_string(),
        "group": group.to_string(),
    });
    let response = Client::new().get(url).json(&json).send().await;
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

pub async fn set_jin_time(up_time: Option<String>, deviate_time: Option<i64>, user: Option<String>) -> i64 {
    let config = Config::get().await;
    let url = format!("{}/time", config.api.unwrap().url.unwrap());
    let up = UpJinTime {
        id: 1,
        up_time,
        deviate_time,
        user,
    };
    let response = Client::new().put(url)
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
pub async fn get_users(user: i64, user_type: i64) -> Vec<UserModel> {
    let config = Config::get().await;
    let url = format!("{}/users", config.api.unwrap().url.unwrap());
    let json = json!({
        "id": 0,
        "name": user.to_string(),
        "user_type": user_type
    });
    match Client::new().get(url).json(&json).send().await {
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

pub async fn set_user_view(user: &str, view: i64) -> i64 {
    let config = Config::get().await;
    let url = format!("{}/user", config.api.unwrap().url.unwrap());
    let json = json!({
        "id": 0,
        "name": user,
        "view": view
    });
    match Client::new().patch(url).json(&json).send().await {
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

pub async fn set_user_type(user: &str, user_type: i64) -> i64 {
    //小于0跳过
    if user_type < 0 {
        return 0;
    }
    let config = Config::get().await;
    let url = format!("{}/user", config.api.unwrap().url.unwrap());
    let json = json!({
        "id": 0,
        "name": user,
        "user_type": user_type
    });
    match Client::new().patch(url).json(&json).send().await {
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

pub async fn _set_time(json: Value) {
    let response = Client::new()
        .post("http://get.cocsnipe.top/setTime")
        .json(&json)
        .send()
        .await
        .unwrap();
    log_info!("{}", response.text().await.unwrap_or("没有更新".to_string()))
}