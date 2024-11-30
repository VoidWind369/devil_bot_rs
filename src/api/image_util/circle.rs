use crate::api::image_util::Draw;
use tiny_skia::{Color, FillRule, GradientStop, Paint, Path, PathBuilder, Pixmap, Point, RadialGradient, SpreadMode, Transform};

pub struct Circle {
    radius: f32,
    start_color: Color,
    end_color: Color,
}

impl Circle {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            start_color: Color::WHITE,
            end_color: Color::WHITE,
        }
    }

    pub fn set_color(mut self, color: Color) -> Circle {
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

    pub fn builder(self) -> Option<Path> {
        let mut builder = PathBuilder::new();
        builder.push_circle(self.radius, self.radius, self.radius);
        builder.finish()
    }
}

impl Draw for Circle {
    fn create_pixmap(self) -> Pixmap {
        let size = self.radius * 2.0;
        // 创建一个 Pixmap 用来渲染
        let mut pixmap = Pixmap::new(size as u32, size as u32).unwrap();

        // 创建渐变色
        let mut paint = Paint::default();
        paint.anti_alias = true;
        if self.start_color == self.end_color {
            paint.set_color(self.start_color);
        } else {
            paint.shader = RadialGradient::new(
                Point::from_xy(self.radius, self.radius),
                Point::from_xy(self.radius, self.radius),
                self.radius,
                vec![
                    GradientStop::new(0.0, self.start_color),
                    GradientStop::new(1.0, self.end_color),
                ],
                SpreadMode::Pad,
                Transform::identity(),
            )
                .unwrap();
        }

        // 创建路径
        let path = self.builder();

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
}
