use crate::api::image_util::{Align, ImagePicture, ImageText};
use crate::modal::app_qq::AppQQ;
use crate::util::Config;
use ab_glyph::FontArc;
use chrono::NaiveDateTime;
use image::{open, ColorType, DynamicImage, RgbaImage};
use imageproc::drawing::Canvas;
use jsonwebtoken::{encode, EncodingKey, Header};
use reqwest::header::HeaderMap;
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::hash::Hash;
use std::time::{SystemTime, UNIX_EPOCH};
use void_log::log_info;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Record {
    pub msg: Option<String>,
    pub code: Option<bool>,
    pub data: Vec<RecordData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct RecordClan {
    name: Option<String>,
    tag: Option<String>,
    union: Option<String>,
    state: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct RecordScore {
    record: Option<RecordScoreRecord>,
    score: Option<i64>,
    create_time: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct RecordScoreRecord {
    tag: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct Payload {
    openid: String,
    time: f64,
}

impl Record {
    pub async fn new(tag: &str, userid: impl AsRef<str>, r#type: i64) -> Self {
        let api = Config::get().await.get_om_api();

        let app_qq = AppQQ::select(userid.as_ref()).await.unwrap();
        let local_now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        let payload = Payload {
            openid: "0".to_string(),
            time: local_now,
        };
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
        let url =
            Url::parse_with_params(&format!("{}/record", api.url.unwrap_or_default()), &params)
                .unwrap();

        log_info!("{}", &url);

        let response = Client::new()
            .get(url)
            .headers(headers)
            .send()
            .await
            .unwrap();
        // response.text().await.unwrap()
        response.json::<Self>().await.unwrap()
    }

    fn list_img(self) -> DynamicImage {
        let mut img_top = open("static/image/record/top_0727.png").unwrap();
        let img_bottom = open("static/image/record/bottom_0727.png").unwrap();
        let img_win = open("static/image/record/center_win_0730.png").unwrap();
        let img_lose = open("static/image/record/center_lose_0730.png").unwrap();
        let img_fail = open("static/image/record/center_fail_0730.png").unwrap();

        let wight = img_win.width();
        let top_height = img_top.height();
        let list_height = img_win.height();
        let height = self.data.len() as u32 * list_height;

        let mut base = DynamicImage::new(wight, height, ColorType::Rgba8);

        let font = FontArc::try_from_slice(include_bytes!(
            "../../static/fonts/SourceHanSansCN-Bold.otf"
        ))
        .unwrap();

        // let font_hei = FontArc::try_from_slice(include_bytes!("simhei.ttf"));

        let data = self.data;
        ImageText::new(&data[0].clone().tag.unwrap(), &font, 45.0).draw(&mut img_top);
        // 顶部写入base
        ImagePicture::new(img_top, 0).draw(&mut base);

        for (i, datum) in data.into_iter().enumerate() {
            let result = datum.result.unwrap();
            let mut img = match result.as_str() {
                "赢" => img_win.clone(),
                "输" => img_lose.clone(),
                _ => img_fail.clone(),
            };
            let opponent_clan = datum.opponent_clan.unwrap_or_default();
            let y = top_height + list_height * i as u32;

            // 对方部落标签
            ImageText::new(&opponent_clan.tag.unwrap_or_default(), &font, 22.0)
                .set_axis(200, y as i32 + 30)
                .set_aligns(vec![Align::Horizontally])
                .draw(&mut img);

            // 对方部落联盟
            ImageText::new(&opponent_clan.union.unwrap_or_default(), &font, 22.0)
                .set_axis(200, y as i32 + 68)
                .set_aligns(vec![Align::Horizontally])
                .draw(&mut img);

            // 对方部落名字
            ImageText::new(&opponent_clan.name.unwrap_or_default(), &font, 22.0)
                .set_axis(320, y as i32 + 30)
                .draw(&mut img);

            // 匹配状态
            let state_time = format!(
                "{} | 轮次: {}",
                &datum.state.unwrap_or_default(),
                &datum.time.unwrap_or_default()
            );
            ImageText::new(&state_time, &font, 22.0)
                .set_axis(320, y as i32 + 68)
                .draw(&mut img);

            // 上轮积分
            let historical_score = datum.historical_score.unwrap_or_default().to_string();
            ImageText::new(&historical_score, &font, 22.0)
                .set_axis(845, y as i32 + 70)
                .set_aligns(vec![Align::Horizontally])
                .draw(&mut img);

            // 积分变动
            let score_change = datum.score_change.unwrap_or_default();
            let score = score_change.score.unwrap_or_default().to_string();
            ImageText::new(&score, &font, 22.0)
                .set_axis(950, y as i32 + 70)
                .set_aligns(vec![Align::Horizontally])
                .draw(&mut img);

            // 输赢
            ImageText::new(&result, &font, 22.0)
                .set_axis(1040, y as i32 + 60)
                .set_aligns(vec![Align::Horizontally, Align::Vertically])
                .draw(&mut img);

            // 序号
            ImageText::new(&(i + 1).to_string(), &font, 22.0)
                .set_axis(52, y as i32 + 60)
                .set_aligns(vec![Align::Horizontally, Align::Vertically])
                .draw(&mut img);

            // 主体写入base
            ImagePicture::new(img, 0)
                .set_axis(0, y as i32)
                .draw(&mut base);
        }

        // 底部写入base
        ImagePicture::new(img_bottom, 0)
            .set_axis(0, (top_height + height) as i32)
            .draw(&mut base);

        base
    }
}
