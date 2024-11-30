use ab_glyph::{point, Font, FontArc, PxScale};
use image::imageops::overlay;
use image::{DynamicImage, Rgba, RgbaImage};
use imageproc::definitions::HasWhite;
use imageproc::drawing::{draw_text_mut, text_size};

pub struct ImageText<'a> {
    text: String,       // 文本
    font: &'a FontArc,  // 字体
    scale: PxScale,     // 字体大小
    color: Rgba<u8>,    // 颜色
    p_x: i32,           // 横轴
    p_y: i32,           // 纵轴
    aligns: Vec<Align>, // 居中
    pixel: u32,         // dpi
}

pub struct ImagePicture {
    picture: RgbaImage, // 图片
    height: u32,        // 高
    p_x: i32,           // 横轴
    p_y: i32,           // 纵轴
    aligns: Vec<Align>, // 居中
    pixel: u32,         // dpi
}

#[derive(Debug, PartialEq)]
pub enum Align {
    Horizontally, // 横向居中
    Vertically,   // 纵向居中
    Top,          // 上对齐
    Bottom,       // 下对齐
    Left,         // 左对齐
    Right,        // 右对齐
}

impl<'a> ImageText<'a> {
    ///
    ///
    /// # 水印文字
    ///
    /// * `text`: 文字
    /// * `font`: 字体
    /// * `scale`: 字号
    ///
    /// returns: ImageText
    ///
    /// # Examples
    ///
    /// ```
    /// let font = FontArc::try_from_slice(include_bytes!("../fonts/Exo2-Light.otf"))?;
    /// let image_text = ImageText::new("abc", font, PxScale::from(24.0))
    /// ```
    pub fn new(text: &str, font: &'a FontArc, scale: f32) -> Self {
        Self {
            text: text.to_string(),
            font,
            scale: PxScale::from(scale),
            color: Rgba::white(),
            p_x: 0,
            p_y: 0,
            aligns: vec![],
            pixel: 72,
        }
    }

    pub fn set_color(mut self, color: Rgba<u8>) -> Self {
        self.color = color;
        self
    }

    pub fn set_axis(mut self, x: i32, y: i32) -> Self {
        self.p_x = x;
        self.p_y = y;
        self
    }

    pub fn set_aligns(mut self, aligns: Vec<Align>) -> Self {
        self.aligns = aligns;
        self
    }

    pub fn set_pixel(mut self, pixel: u32) -> Self {
        self.pixel = pixel;
        self
    }

    pub fn draw(self, rgba_image: &mut RgbaImage) {
        let pixel = &self.pixel / 72;
        let (width, height) = rgba_image.dimensions();
        let (mut x, mut y) = (self.p_x * pixel as i32, self.p_y * pixel as i32);
        // 计算文字大小和位置
        let text_scale = text_size(self.scale, &self.font, &self.text);

        for align in &self.aligns {
            (x, y) = align.new(x, y, text_scale)
        }

        x = x.clamp(0, (width - 1) as i32);
        y = y.clamp(0, (height - 1) as i32);
        // 在图像上绘制文字
        draw_text_mut(
            rgba_image, self.color, x, y, self.scale, &self.font, &self.text,
        );
    }

    pub fn draw_with(self, rgba_image: &mut RgbaImage, letter_spacing: i32) {
        let pixel = &self.pixel / 72;
        let (width, height) = rgba_image.dimensions();
        let (mut x, mut y) = (self.p_x * pixel as i32, self.p_y * pixel as i32);
        let (t_wight, t_height) = text_size(self.scale, self.font, &self.text);
        let text_wight = t_wight as i32 + letter_spacing * (self.text.chars().count() - 1) as i32;

        // 居中判定
        for align in &self.aligns {
            (x, y) = align.new(x, y, (text_wight as u32, t_height))
        }

        // 遍历每个字符并渲染
        for c in self.text.chars() {
            let t = c.to_string();
            let c_scale = text_size(self.scale, self.font, &t);
            x = x.clamp(0, (width - 1) as i32);
            y = y.clamp(0, (height - 1) as i32);
            // 在图像上绘制文字
            draw_text_mut(rgba_image, self.color, x, y, self.scale, &self.font, &t);
            x += c_scale.0 as i32 + letter_spacing;
        }
    }

    pub fn draw_chars(self, rgba_image: &mut RgbaImage, letter_spacing: u32) {
        // let letter_spacing = 10.0;
        let (mut width, _) = text_size(self.scale, self.font, &self.text);
        width += letter_spacing * (self.text.chars().count() - 1) as u32;

        let width_spacing = if self.aligns.contains(&Align::Horizontally) {
            width / 2
        } else {
            0
        };

        let (mut x_position, ty) = (self.p_x as f32, self.p_y as f32);
        for char in self.text.chars() {
            let g = self
                .font
                .glyph_id(char)
                .with_scale_and_position(self.scale, point(x_position, ty));
            if let Some(g) = self.font.outline_glyph(g) {
                g.draw(|x, y, c| {
                    // 通过覆盖度决定像素的颜色
                    let color = Rgba([
                        (c * 255.0) as u8, // alpha 通过覆盖度控制
                        (c * 255.0) as u8, // 红色部分
                        (c * 255.0) as u8, // 绿色部分
                        255,               // 不透明度
                    ]);

                    // 将像素 `(x, y)` 绘制为颜色
                    if x < rgba_image.width() && y < rgba_image.height() {
                        rgba_image.put_pixel(x + x_position as u32 - width_spacing, y, color);
                    }
                })
            }
            // 更新 x 坐标，为下一个字符留出空间
            let (glyph_width, _) = text_size(self.scale, self.font, char.to_string().as_str());
            x_position += (glyph_width + letter_spacing) as f32; // 字形宽度加上字间距
        }
    }
}

impl ImagePicture {
    ///
    ///
    /// # Arguments
    ///
    /// * `picture`: DynamicImage图片
    /// * `height`: 高度(0为原值)
    ///
    /// returns: ImagePicture
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn new(picture: RgbaImage, height: u32) -> Self {
        let height = if height == 0 {
            picture.height()
        } else {
            height
        };
        Self {
            picture,
            height,
            p_x: 0,
            p_y: 0,
            aligns: vec![],
            pixel: 72,
        }
    }

    pub fn set_axis(mut self, x: i32, y: i32) -> Self {
        self.p_x = x;
        self.p_y = y;
        self
    }

    pub fn set_aligns(mut self, aligns: Vec<Align>) -> Self {
        self.aligns = aligns;
        self
    }

    pub fn set_pixel(mut self, pixel: u32) -> Self {
        self.pixel = pixel;
        self
    }

    pub fn draw(&self, rgba_image: &mut DynamicImage) {
        let pixel = &self.pixel / 72;
        let (mut x, mut y) = (self.p_x * pixel as i32, self.p_y * pixel as i32);

        let picture_weight = &self.height * pixel * &self.picture.width() / &self.picture.height();
        let picture_height = &self.height * pixel;
        // let img = &self.picture.resize(
        //     picture_weight,
        //     picture_height,
        //     image::imageops::FilterType::Lanczos3,
        // );
        let img = &self.picture;

        // 居中判断
        for align in &self.aligns {
            (x, y) = align.new(x, y, (picture_weight, picture_height));
        }

        overlay(rgba_image, img, x as i64, y as i64);
    }
}

impl Align {
    fn new(&self, mut x: i32, mut y: i32, (weight, height): (u32, u32)) -> (i32, i32) {
        match &self {
            Align::Horizontally => {
                x = x - weight as i32 / 2;
            }
            Align::Vertically => {
                y = y - height as i32 / 2;
            }
            Align::Bottom => {
                y = y - height as i32;
            }
            Align::Right => {
                x = x - weight as i32;
            }
            _ => {}
        }
        (x, y)
    }
}

#[tokio::test]
async fn test_d_text() {
    let mut r = RgbaImage::new(400, 400);
    let f = include_bytes!("../../../static/fonts/FZSHHJW.TTF");
    let f = FontArc::try_from_slice(f).unwrap();
    ImageText::new("QAS", &f, 48.0)
        .set_axis(100, 0)
        .draw_chars(&mut r, 10);
    r.save("ll.png").unwrap();
}
