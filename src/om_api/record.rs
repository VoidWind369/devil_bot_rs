use crate::modal::app_qq::AppQQ;
use crate::util::Config;
use chrono::NaiveDateTime;
use jsonwebtoken::{encode, EncodingKey, Header};
use md5::Digest;
use reqwest::header::HeaderMap;
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::hash::Hash;
use std::time::{SystemTime, UNIX_EPOCH};
use void_log::log_info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    pub msg: Option<String>,
    pub code: Option<bool>,
    pub data: Vec<RecordData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RecordData {
    pub id: Option<i64>,
    pub state: Option<String>,
    pub result: Option<String>,
    pub opponent_clan: Option<RecordClan>,
    #[serde(rename = "scorechange")]
    pub score_change: Option<RecordScore>,
    pub tag: Option<String>,
    pub historical_score: Option<i64>,
    #[serde(rename = "is_bzlm")]
    pub is_bz: Option<bool>,
    pub create_time: Option<String>,
    pub time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RecordClan {
    name: Option<String>,
    tag: Option<String>,
    union: Option<String>,
    state: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RecordScore {
    record: Option<RecordScoreRecord>,
    score: Option<i64>,
    create_time: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RecordScoreRecord {
    tag: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Payload {
    openid: i64,
    time: f64,
}

impl Record {
    pub async fn new(tag: &str, userid: &str, r#type: i64) -> String {
        let api = Config::get().await.get_om_api();

        let app_qq = AppQQ::select(userid).await.unwrap();
        let local_now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        let payload = Payload {
            openid: 0,
            time: local_now,
        };
        let mut key = md5::Md5::new();
        key.update(b"leinuococ");
        let key = key.finalize();
        let token = encode(
            &Header::default(),
            &payload,
            &EncodingKey::from_secret(b"50edc303ef5c5d3bf2f1a20ba66c47e9"),
        )
        .unwrap();

        log_info!("{} {}", &token, local_now);

        let mut headers = HeaderMap::new();
        headers.insert("token", token.parse().unwrap());

        let mut params: BTreeMap<String, String> = BTreeMap::new();
        // params.insert("format".to_string(), "json".to_string());
        params.insert("tag".to_owned(), tag.to_string());
        params.insert("type".to_owned(), r#type.to_string());
        let url = Url::parse_with_params(
            &format!("{}/record", api.url.unwrap_or_default()),
            &params,
        )
        .unwrap();

        log_info!("{}", &url);

        let response = Client::new()
            .get(url)
            .headers(headers)
            .send()
            .await
            .unwrap();
        response.text().await.unwrap()
        // response.json::<Self>().await.unwrap()
    }
}

fn list_img() {
    let img_top = image::open("image/record/top_0727.png")
        .unwrap()
        .into_rgba8();
    let img_bottom = image::open("image/record/bottom_0727.png")
        .unwrap()
        .into_rgba8();
    let img_win = image::open("image/record/center_win_0730.png")
        .unwrap()
        .into_rgba8();
    let img_lose = image::open("image/record/center_lose_0730.png")
        .unwrap()
        .into_rgba8();
    let img_fail = image::open("image/record/center_fail_0730.png")
        .unwrap()
        .into_rgba8();
}
