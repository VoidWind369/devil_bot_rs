use ab_glyph::{FontArc, PxScale};
use image::{DynamicImage, GenericImageView, Rgba};
use image::imageops::overlay;
use imageproc::definitions::HasBlack;
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
    picture: DynamicImage, // 图片
    height: u32,           // 高
    p_x: i32,              // 横轴
    p_y: i32,              // 纵轴
    aligns: Vec<Align>,    // 居中
    pixel: u32,            // dpi
}

pub enum Align {
    Horizontally, // 横向居中
    Vertically,   // 纵向居中
    Top,          // 上对齐
    Bottom,       // 下对齐
    Left,         // 左对齐
    Right,        // 右对齐
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
            color: Rgba::black(),
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

    pub fn draw(self, rgba_image: &mut DynamicImage) {
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
    pub fn new(picture: DynamicImage, height: u32) -> Self {
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
        let img = &self.picture.resize(
            picture_weight,
            picture_height,
            image::imageops::FilterType::Lanczos3,
        );

        // 居中判断
        for align in &self.aligns {
            (x, y) = align.new(x, y, (picture_weight, picture_height));
        }

        overlay(rgba_image, img, x as i64, y as i64);
    }
}