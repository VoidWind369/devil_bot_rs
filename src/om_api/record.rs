use crate::util::Config;
use chrono::{Local, NaiveDateTime};
use hmac::{Hmac, Mac};
use jsonwebtoken::{encode, EncodingKey, Header};
use jwt::SignWithKey;
use md5::Digest;
use reqwest::header::HeaderMap;
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::collections::BTreeMap;
use crate::modal::app_qq::AppQQ;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    pub code: Option<i64>,
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
    pub create_time: Option<NaiveDateTime>,
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
    openid: String,
    time: f64,
}

impl Record {
    pub async fn new(tag: &str, userid: &str, r#type: i64) -> Self {
        let api = Config::get().await.get_api();

        let app_qq = AppQQ::select(userid).await.unwrap();
        let payload = Payload {
            openid: app_qq.openid.unwrap_or("0".to_string()),
            time: Local::now().timestamp_micros() as f64 / 1000.0,
        };
        let key = md5::compute(b"leinuococ").0;
        let token = encode(&Header::default(), &payload, &EncodingKey::from_secret(&key)).unwrap();

        let mut headers = HeaderMap::new();
        headers.insert("token", token.parse().unwrap());

        let mut params: BTreeMap<String, String> = BTreeMap::new();
        params.insert("tag".to_owned(), tag.to_string());
        params.insert("type".to_owned(), r#type.to_string());
        let url =
            Url::parse_with_params(&format!("{}/record", api.url.unwrap_or_default()), &params)
                .unwrap();

        let response = Client::new().get(url).headers(headers).send().await;
        response.unwrap().json::<Self>().await.unwrap()
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
