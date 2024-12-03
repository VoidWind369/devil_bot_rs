use crate::api::image_util::{
    draw_logo, Align, Direction, Draw, ImagePicture, ImageText, RectRadius, RectRound,
};
use ab_glyph::FontArc;
use image::{open, ColorType, DynamicImage, Rgba, RgbaImage};
use imageproc::definitions::{HasBlack, HasWhite};
use serde::{Deserialize, Serialize};
use tiny_skia::{Color, Pixmap};
use tokio::io::AsyncReadExt;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Menu {
    title: String,
    body: Vec<MenuBody>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MenuBody {
    name: String,
    remark: Vec<String>,
}

impl Menu {
    pub(crate) async fn from_file(filename: &str) -> Self {
        let mut yaml_file = tokio::fs::File::open(filename)
            .await
            .expect("read config error");
        let mut yaml_str = String::new();
        yaml_file
            .read_to_string(&mut yaml_str)
            .await
            .expect("read str error");
        serde_json::from_str(yaml_str.as_str()).expect("config error")
    }

    pub async fn list_img(self) -> DynamicImage {
        let mut img_top = top().await.unwrap();
        let img_body = body_data().await.unwrap();
        let mut img_bottom = bottom().await.unwrap();

        let top_height = img_top.height();
        let body_height = if self.body.len() % 2 == 0 {
            self.body.len() as u32 * 160 / 2
        } else {
            (self.body.len() + 1) as u32 * 160 / 2
        } + 20;
        let bottom_height = img_bottom.height();

        let mut base = DynamicImage::new(
            1080,
            top_height + body_height + bottom_height,
            ColorType::Rgba8,
        );
        let menu_bg = open("./static/image/menu_bg.png").unwrap().to_rgba8();
        ImagePicture::new(menu_bg, top_height + body_height + bottom_height)
            .set_aligns(vec![Align::Horizontally])
            .draw(&mut base);

        let source_han_sans_cn = include_bytes!("../../static/fonts/SourceHanSansCN-Bold.otf");
        let fz_shh_jw = include_bytes!("../../static/fonts/FZSHHJW.TTF");
        let fz_zq_jw = include_bytes!("../../static/fonts/FZZQJW.TTF");
        let source_han_sans_cn = FontArc::try_from_slice(source_han_sans_cn).unwrap();
        let fz_shh_jw = FontArc::try_from_slice(fz_shh_jw).unwrap();
        let fz_zq_jw = FontArc::try_from_slice(fz_zq_jw).unwrap();

        // 标题
        ImageText::new("Orange Menu", &fz_shh_jw, 88.0)
            .set_axis(241, 55)
            .set_color(Rgba::white())
            .draw_with(&mut img_top, 5);

        // 顶部写入base
        ImagePicture::new(img_top, 0).draw(&mut base);

        let (mut x, mut y) = (0, 0);
        for (i, body) in self.body.iter().enumerate() {
            let mut img = img_body.clone();
            ImageText::new(&body.name, &source_han_sans_cn, 26.0)
                .set_axis(40, 10)
                .draw(&mut img);

            for (line, remark) in body.remark.iter().enumerate() {
                ImageText::new(remark, &fz_zq_jw, 18.0)
                    .set_color(Rgba::black())
                    .set_axis(30, 55 + line as i32 * 24)
                    .draw(&mut img);
            }

            // 底部写入base
            ImagePicture::new(img, 0)
                .set_axis(35 + x, 230 + y)
                .draw(&mut base);

            if i % 2 == 0 {
                x = 530;
            } else {
                x = 0;
                y += 160
            }
        }

        ImageText::new("橘子科技提供技术支持", &fz_shh_jw, 30.0)
            .set_axis(540, 37)
            .set_aligns(vec![Align::Horizontally, Align::Vertically])
            .draw_with(&mut img_bottom, 12);

        // 底部写入base
        ImagePicture::new(img_bottom, 0)
            .set_axis(0, (top_height + body_height) as i32)
            .draw(&mut base);

        base
    }
}

/// # 顶部
async fn top() -> Option<RgbaImage> {
    let mut bg = RectRound::new(1080, 200)
        .set_color(Color::from_rgba8(228, 215, 255, 0))
        .create_pixmap();

    RectRound::new(1080, 200)
        .set_start_color(Color::from_rgba8(195, 13, 35, 255 / 5 * 2))
        .set_end_color(Color::from_rgba8(234, 85, 20, 255 / 5 * 2))
        .set_radius(RectRadius::new_bottom(30.0))
        .draw(&mut bg, 0, 0);

    // 标志
    draw_logo(&mut bg, 54, 30, "menu");
    RgbaImage::from_raw(bg.width(), bg.height(), bg.take())
}

async fn body_data() -> Option<RgbaImage> {
    let mut body = Pixmap::new(480, 130).unwrap();

    RectRound::new(480, 50)
        .set_start_color(Color::from_rgba8(195, 13, 35, 255 / 5 * 4))
        .set_end_color(Color::from_rgba8(234, 85, 20, 255 / 5 * 4))
        .set_radius(RectRadius::new_top(30.0))
        .set_direction(Direction::Vertically)
        .draw(&mut body, 0, 0);

    RectRound::new(480, 80)
        .set_radius(RectRadius::new_bottom(30.0))
        .draw(&mut body, 0, 50);

    RgbaImage::from_raw(body.width(), body.height(), body.take())
}

/// # 底部
async fn bottom() -> Option<RgbaImage> {
    let mut bg = RectRound::new(1080, 70)
        .set_color(Color::from_rgba8(150, 215, 255, 0))
        .create_pixmap();
    RectRound::new(1080, 60)
        .set_start_color(Color::from_rgba8(195, 13, 35, 255 / 5 * 3))
        .set_end_color(Color::from_rgba8(234, 85, 20, 255 / 5 * 3))
        .draw(&mut bg, 0, 10);
    RgbaImage::from_raw(bg.width(), bg.height(), bg.take())
}

#[tokio::test]
async fn test() {
    let m = Menu::from_file("menu.json").await.list_img().await;
    m.save("menu.png").unwrap();
}
