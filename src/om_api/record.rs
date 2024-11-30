use crate::api::image_util::{
    draw_logo, Align, Draw, ImagePicture, ImageText, RectRadius, RectRound,
};
use crate::util::Config;
use ab_glyph::FontArc;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use chrono::NaiveDateTime;
use image::{ColorType, DynamicImage, ImageFormat, Rgba, RgbaImage};
use imageproc::definitions::HasWhite;
use jsonwebtoken::{encode, EncodingKey, Header};
use reqwest::header::HeaderMap;
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::io::Cursor;
use std::time::{SystemTime, UNIX_EPOCH};
use tiny_skia::{Color, Pixmap};
use void_log::log_info;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Record {
    msg: Option<String>,
    code: Option<bool>,
    data: Vec<RecordData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct RecordData {
    id: Option<i64>,
    state: Option<String>,
    result: Option<String>,
    opponent_clan: Option<RecordClan>,
    #[serde(rename = "scorechange")]
    score_change: Option<RecordScore>,
    tag: Option<String>,
    historical_score: Option<i64>,
    #[serde(rename = "is_bzlm")]
    is_bz: Option<bool>,
    create_time: Option<String>,
    time: Option<i64>,
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

// jwt校验
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct Payload {
    openid: String,
    time: f64,
}

// body
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct BodyRecord {
    tag: String,
    r#type: char,
}

impl Record {
    pub async fn new(tag: &str, r#type: char) -> Response {
        let api = Config::get().await.get_om_api();
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
        params.insert("tag".to_owned(), format!("#{}", tag.replace("#", "")));
        params.insert("type".to_owned(), r#type.to_string());

        let params = BodyRecord {
            tag: format!("#{}", tag.replace("#", "")),
            r#type,
        };
        // let url = Url::parse_with_params(&format!("{}/record/", api.url.unwrap_or_default()), &params).unwrap();
        let url = format!("{}/record/", api.url.unwrap_or_default());

        log_info!("{}", &url);

        Client::new()
            .get(url)
            .headers(headers)
            .json(&params)
            .send()
            .await
            .unwrap()
    }

    pub async fn new_text(tag: &str, r#type: char) -> String {
        Self::new(tag, r#type).await.text().await.unwrap()
    }

    pub async fn new_json(tag: &str, r#type: char) -> Self {
        Self::new(tag, r#type).await.json::<Self>().await.unwrap()
    }

    pub async fn list_img(self, size: usize) -> DynamicImage {
        log_info!("Records Self {:?}", &self);
        let data = if self.data.len() < size {
            self.data
        } else {
            self.data[0..size].to_vec()
        };
        log_info!("Records Data {:?}", &data);

        let mut img_top = top().await.unwrap();
        let mut img_bottom = bottom().await.unwrap();
        let img_win = body_win().await.unwrap();
        let img_lose = body_lose().await.unwrap();
        let img_fail = body_fail().await.unwrap();

        let wight = img_win.width();
        let top_height = img_top.height();
        let list_height = img_win.height();
        let body_height = data.len() as u32 * list_height;
        let bottom_height = img_bottom.height();
        let height = top_height + data.len() as u32 * list_height + bottom_height;

        let mut base = DynamicImage::new(wight, height, ColorType::Rgba8);

        let source_han_sans_cn = include_bytes!("../../static/fonts/SourceHanSansCN-Bold.otf");
        let fz_shh_jw = include_bytes!("../../static/fonts/FZSHHJW.TTF");
        let fzy3jw = include_bytes!("../../static/fonts/FZY3JW.TTF");
        let source_han_sans_cn = FontArc::try_from_slice(source_han_sans_cn).unwrap();
        let fz_shh_jw = FontArc::try_from_slice(fz_shh_jw).unwrap();
        let fzy3jw = FontArc::try_from_slice(fzy3jw).unwrap();

        // 标题
        ImageText::new("Orange", &fz_shh_jw, 88.0)
            .set_axis(241, 55)
            .set_color(Rgba([251, 176, 59, 255]))
            .draw_with(&mut img_top, 5);
        ImageText::new("对战日记", &fzy3jw, 72.0)
            .set_axis(556, 70)
            .draw_with(&mut img_top, 5);

        // 本方标签
        let data0tag = if let Some(data0) = data.first().cloned() {
            data0.tag.unwrap()
        } else {
            "无数据".to_string()
        };
        ImageText::new(&data0tag, &source_han_sans_cn, 45.0)
            .set_axis(380, 165)
            .set_color(Rgba([65, 40, 145, 255]))
            .set_aligns(vec![Align::Horizontally])
            .draw(&mut img_top);
        // 顶部写入base
        ImagePicture::new(img_top, 0).draw(&mut base);

        let font_size = 26.0;
        let body_down_y = 65;

        for (i, datum) in data.into_iter().enumerate() {
            let result = datum.result.unwrap();
            let (mut img, text_color) = match result.as_str() {
                "赢" => (img_win.clone(), Rgba([0, 61, 30, 255])),
                "输" => (img_lose.clone(), Rgba([122, 0, 9, 255])),
                _ => (img_fail.clone(), Rgba([30, 30, 30, 255])),
            };
            let opponent_clan = datum.opponent_clan.unwrap_or_default();
            let y = top_height + list_height * i as u32;

            // 对方部落标签
            ImageText::new(
                &opponent_clan.tag.unwrap_or_default(),
                &source_han_sans_cn,
                font_size,
            )
            .set_axis(200, 30)
            .set_color(text_color)
            .set_aligns(vec![Align::Horizontally])
            .draw(&mut img);

            // 对方部落联盟
            ImageText::new(
                &opponent_clan.union.unwrap_or_default(),
                &source_han_sans_cn,
                font_size,
            )
            .set_axis(200, body_down_y)
            .set_color(Rgba([200, 254, 255, 255]))
            .set_aligns(vec![Align::Horizontally])
            .draw(&mut img);

            // 对方部落名字
            ImageText::new(
                &opponent_clan.name.unwrap_or_default(),
                &source_han_sans_cn,
                font_size,
            )
            .set_axis(320, 30)
            .set_color(Rgba::white())
            .draw(&mut img);

            // 匹配状态
            let state_time = format!(
                "{} | 轮次: {}",
                &datum.state.unwrap_or_default(),
                &datum.time.unwrap_or_default()
            );
            ImageText::new(&state_time, &source_han_sans_cn, font_size)
                .set_axis(320, body_down_y)
                .set_color(Rgba::white())
                .draw(&mut img);

            // 上轮积分
            let historical_score = datum.historical_score.unwrap_or_default().to_string();
            let score_change = datum.score_change.unwrap_or_default();
            let score = score_change.score.unwrap_or_default().to_string();
            ImageText::new("积分              当场", &fzy3jw, font_size - 6.0)
                .set_axis(780, body_down_y + 7)
                .set_color(text_color)
                .draw(&mut img);
            ImageText::new(&historical_score, &source_han_sans_cn, font_size)
                .set_axis(845, body_down_y + 2)
                .set_color(text_color)
                .set_aligns(vec![Align::Horizontally])
                .draw(&mut img);
            // 积分变动
            ImageText::new(&score, &source_han_sans_cn, font_size)
                .set_axis(950, body_down_y + 2)
                .set_color(text_color)
                .set_aligns(vec![Align::Horizontally])
                .draw(&mut img);

            // 输赢
            ImageText::new(&result, &source_han_sans_cn, font_size + 2.0)
                .set_axis(1040, 55)
                .set_color(Rgba::white())
                .set_aligns(vec![Align::Horizontally, Align::Vertically])
                .draw(&mut img);

            // 序号
            ImageText::new(&(i + 1).to_string(), &fz_shh_jw, 52.0)
                .set_axis(52, 48)
                .set_color(Rgba([255, 210, 130, 255]))
                .set_aligns(vec![Align::Horizontally, Align::Vertically])
                .draw(&mut img);

            // 主体写入base
            ImagePicture::new(img, 0)
                .set_axis(0, y as i32)
                .draw(&mut base);
        }

        ImageText::new("橘子科技提供技术支持", &fz_shh_jw, 30.0)
            .set_axis(540, 37)
            .set_aligns(vec![Align::Horizontally, Align::Vertically])
            .draw_with(&mut img_bottom, 16);

        // 底部写入base
        ImagePicture::new(img_bottom, 0)
            .set_axis(0, (top_height + body_height) as i32)
            .draw(&mut base);

        base
    }
}

pub async fn base64img(dynamic_image: DynamicImage) -> String {
    let mut bytes: Vec<u8> = Vec::new();
    dynamic_image
        .write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)
        .unwrap();
    BASE64_STANDARD.encode(&bytes)
}

impl RectRound {
    fn body() -> Self {
        Self::new(1060, 100).set_radius(RectRadius::new(10.0))
    }

    fn label() -> Self {
        Self::new(200, 35)
    }

    fn right() -> Self {
        Self::new(60, 50).set_radius(RectRadius::new(20.0))
    }

    fn right_bg() -> Self {
        Self::new(70, 70)
            .set_color(Color::from_rgba8(228, 215, 255, 255))
            .set_radius(RectRadius::new_left(30.0))
    }

    fn body_background() -> Self {
        Self::new(1080, 110).set_color(Color::from_rgba8(228, 215, 255, 255))
    }

    fn left_top_label() -> Self {
        Self::label()
            .set_color(Color::from_rgba8(242, 242, 242, 200))
            .set_radius(RectRadius::new_top(10.0))
    }

    fn left_bottom_label() -> Self {
        Self::label()
            .set_color(Color::from_rgba8(242, 242, 242, 76))
            .set_radius(RectRadius::new_bottom(10.0))
    }

    fn right_bottom_label() -> Self {
        Self::new(215, 28)
            .set_color(Color::from_rgba8(242, 242, 242, 230))
            .set_radius(RectRadius::new(10.0))
    }

    fn draw_ele(self, right_color: Color) -> Pixmap {
        let mut bg = RectRound::body_background().create_pixmap();

        let right = Self::new(60, 50)
            .set_radius(RectRadius::new(20.0))
            .set_color(right_color);

        // 将元素绘制到body
        let padding = 10;
        self.draw(&mut bg, padding, padding);
        RectRound::left_top_label().draw(&mut bg, padding + 90, padding + 15);
        RectRound::left_bottom_label().draw(&mut bg, padding + 90, padding + 50);
        RectRound::right_bottom_label().draw(&mut bg, padding + 757, padding + 57);
        RectRound::right_bg().draw(&mut bg, padding + 990, padding + 15);
        right.draw(&mut bg, padding + 1000, padding + 25);
        bg
    }
}

/// # 顶部
async fn top() -> Option<RgbaImage> {
    let mut bg = RectRound::new(1080, 240)
        .set_color(Color::from_rgba8(228, 215, 255, 255))
        .create_pixmap();

    RectRound::new(1080, 240)
        .set_start_color(Color::from_rgba8(46, 49, 146, 255))
        .set_end_color(Color::from_rgba8(102, 45, 145, 255))
        .set_radius(RectRadius::new_bottom(30.0))
        .draw(&mut bg, 0, 0);

    // 标志
    draw_logo(&mut bg, 54, 50);

    // tag栏
    RectRound::new(285, 60)
        .set_radius(RectRadius::new(30.0))
        .set_color(Color::from_rgba8(230, 230, 230, 255))
        .draw(&mut bg, 240, 160);
    RgbaImage::from_raw(bg.width(), bg.height(), bg.take())
}

/// # 赢局
async fn body_win() -> Option<RgbaImage> {
    let main_color = Color::from_rgba8(0, 104, 55, 255);
    let body = RectRound::body()
        .set_start_color(Color::from_rgba8(0, 146, 69, 255))
        .set_end_color(main_color)
        .draw_ele(main_color);
    RgbaImage::from_raw(body.width(), body.height(), body.take())
}

/// # 输局
async fn body_lose() -> Option<RgbaImage> {
    let main_color = Color::from_rgba8(158, 0, 93, 255);
    let body = RectRound::body()
        .set_start_color(Color::from_rgba8(193, 39, 45, 255))
        .set_end_color(main_color)
        .draw_ele(main_color);
    RgbaImage::from_raw(body.width(), body.height(), body.take())
}

/// # 失败
async fn body_fail() -> Option<RgbaImage> {
    let main_color = Color::from_rgba8(51, 51, 51, 255);
    let body = RectRound::body()
        .set_start_color(Color::from_rgba8(77, 77, 77, 255))
        .set_end_color(main_color)
        .draw_ele(main_color);
    RgbaImage::from_raw(body.width(), body.height(), body.take())
}

/// # 底部
async fn bottom() -> Option<RgbaImage> {
    let mut bg = RectRound::new(1080, 70)
        .set_color(Color::from_rgba8(228, 215, 255, 255))
        .create_pixmap();
    RectRound::new(1080, 60)
        .set_start_color(Color::from_rgba8(46, 49, 146, 255))
        .set_end_color(Color::from_rgba8(102, 45, 145, 255))
        .set_radius(RectRadius::new_top(30.0))
        .draw(&mut bg, 0, 10);
    RgbaImage::from_raw(bg.width(), bg.height(), bg.take())
}

#[tokio::test]
async fn test1() {
    Record::new_json("#2PVQ9UY2Q", '2')
        .await
        .list_img(50)
        .await
        .save("clan.png")
        .unwrap()
}

#[tokio::test]
async fn test2() {
    body_lose().await.unwrap().save("body_lose.png").unwrap();
}
