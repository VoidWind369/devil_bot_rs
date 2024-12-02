use crate::api::image_util::{Direction, Draw, ImagePicture, ImageText, RectRadius, RectRound};
use ab_glyph::FontArc;
use image::{ColorType, DynamicImage, Rgba, RgbaImage};
use imageproc::definitions::HasBlack;
use imageproc::drawing::Canvas;
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
    remark: String,
}

impl Menu {
    async fn from_file(filename: &str) -> Self {
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
        let (width, height) = (1080, self.body.len() as u32 * 100 / 2);
        let mut base = DynamicImage::new(width, height, ColorType::Rgba8);

        let img_body = body_data().await.unwrap();

        let source_han_sans_cn = include_bytes!("../../static/fonts/SourceHanSansCN-Bold.otf");
        let fz_shh_jw = include_bytes!("../../static/fonts/FZSHHJW.TTF");
        let fzy3jw = include_bytes!("../../static/fonts/FZY3JW.TTF");
        let source_han_sans_cn = FontArc::try_from_slice(source_han_sans_cn).unwrap();
        let fz_shh_jw = FontArc::try_from_slice(fz_shh_jw).unwrap();
        let fzy3jw = FontArc::try_from_slice(fzy3jw).unwrap();

        let (mut x, mut y) = (0, 0);
        for (i, body) in self.body.iter().enumerate() {
            let mut img = img_body.clone();
            ImageText::new(&body.name, &source_han_sans_cn, 24.0)
                .set_axis(30, 10)
                .draw(&mut img);

            ImageText::new(&body.remark, &source_han_sans_cn, 24.0)
                .set_color(Rgba::black())
                .set_axis(30, 50)
                .draw(&mut img);

            // 底部写入base
            ImagePicture::new(img, 0)
                .set_axis(20 + x, 20 + y)
                .draw(&mut base);

            if i % 2 == 0 {
                x = 540;
            } else {
                x = 0;
                y += 160
            }
        }
        base
    }
}

async fn body_data() -> Option<RgbaImage> {
    let mut body = Pixmap::new(480, 130).unwrap();

    RectRound::new(480, 50)
        .set_start_color(Color::from_rgba8(0, 146, 69, 255))
        .set_end_color(Color::from_rgba8(0, 104, 55, 255))
        .set_radius(RectRadius::new_top(30.0))
        .set_direction(Direction::Vertically)
        .draw(&mut body, 0, 0);

    RectRound::new(480, 80)
        .set_radius(RectRadius::new_bottom(30.0))
        .draw(&mut body, 0, 50);

    RgbaImage::from_raw(body.width(), body.height(), body.take())
}

#[tokio::test]
async fn test() {
    let m = Menu::from_file("menu.json").await.list_img().await;
    m.save("menu.png").unwrap();
}
