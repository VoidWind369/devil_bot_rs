use image::RgbaImage;
use imageproc::drawing::Canvas;
use tiny_skia::{Color, FillRule, GradientStop, LinearGradient, Paint, Path, PathBuilder, Pixmap, PixmapPaint, Point, SpreadMode, Transform};

pub struct RectRound {
    width: u32,
    height: u32,
    radius: RectRadius,
    padding: f32,
    start_color: Color,
    end_color: Color,
}

impl RectRound {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            radius: RectRadius::new(0.0),
            padding: 0.0,
            start_color: Color::WHITE,
            end_color: Color::WHITE,
        }
    }

    pub fn set_radius(mut self, radius: RectRadius) -> Self {
        self.radius = radius;
        self
    }

    pub fn set_padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    pub fn set_color(mut self, color: Color) -> Self {
        self.start_color = color;
        self.end_color = color;
        self
    }

    pub fn set_start_color(mut self, start_color: Color) -> Self {
        self.start_color = start_color;
        self
    }

    pub fn set_end_color(mut self, end_color: Color) -> Self {
        self.end_color = end_color;
        self
    }

    // 图形元素
    pub fn create_pixmap(self) -> Pixmap {
        // 创建一个 Pixmap 用来渲染
        let mut pixmap = Pixmap::new(self.width, self.height).unwrap();

        // 创建路径
        let path = self.radius.create_rounded_rect_path(
            self.padding,
            self.padding,
            self.width as f32 - self.padding,
            self.height as f32 - self.padding,
        );

        // 创建渐变色
        let mut paint = Paint::default();
        paint.anti_alias = true;

        if self.start_color == self.end_color {
            paint.set_color(self.start_color);
        } else {
            paint.shader = LinearGradient::new(
                Point::from_xy(self.width as f32 / 2.0, 0.0),
                Point::from_xy(self.width as f32 / 2.0, self.height as f32),
                vec![
                    GradientStop::new(0.0, self.start_color),
                    GradientStop::new(1.0, self.end_color),
                ],
                SpreadMode::Pad,
                Transform::identity(),
            )
                .unwrap();
        }

        // 绘制路径到 Pixmap 上
        pixmap.fill_path(
            &path.unwrap(),
            &paint,
            FillRule::default(),
            Transform::identity(),
            None,
        );

        pixmap
    }

    pub fn draw(self, base_pixmap: &mut Pixmap, x: i32, y: i32) {
        base_pixmap.draw_pixmap(
            x,
            y,
            self.create_pixmap().as_ref(),
            &PixmapPaint::default(),
            Transform::identity(),
            None,
        );
    }

    pub fn rgba_image(self) -> Option<RgbaImage> {
        // 转化成RgbaImage
        RgbaImage::from_raw(self.width, self.height, self.create_pixmap().encode_png().unwrap())
    }
}

pub struct RectRadius {
    left_top: f32,     //左上
    right_top: f32,    //右上
    right_bottom: f32, //右下
    left_bottom: f32,  //右下
}

impl RectRadius {
    pub fn new(round: f32) -> Self {
        Self {
            left_top: round,
            right_top: round,
            right_bottom: round,
            left_bottom: round,
        }
    }

    pub fn new_top(round: f32) -> Self {
        Self {
            left_top: round,
            right_top: round,
            right_bottom: 0.0,
            left_bottom: 0.0,
        }
    }

    pub fn new_bottom(round: f32) -> Self {
        Self {
            left_top: 0.0,
            right_top: 0.0,
            right_bottom: round,
            left_bottom: round,
        }
    }

    pub fn new_left(round: f32) -> Self {
        Self {
            left_top: round,
            right_top: 0.0,
            right_bottom: 0.0,
            left_bottom: round,
        }
    }

    fn create_rounded_rect_path(self, x: f32, y: f32, width: f32, height: f32) -> Option<Path> {
        let mut builder = PathBuilder::new();
        builder.move_to(x + self.left_top, y);
        builder.line_to(x + width - self.right_top, y);
        builder.quad_to(x + width, y, x + width, y + self.right_top);
        builder.line_to(x + width, y + height - self.right_bottom);
        builder.quad_to(
            x + width,
            y + height,
            x + width - self.right_bottom,
            y + height,
        );
        builder.line_to(x + self.left_bottom, y + height);
        builder.quad_to(x, y + height, x, y + height - self.left_bottom);
        builder.line_to(x, y + self.left_top);
        builder.quad_to(x, y, x + self.left_top, y);
        builder.close();
        builder.finish()
    }
}