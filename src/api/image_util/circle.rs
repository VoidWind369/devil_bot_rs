use crate::api::image_util::Draw;
use tiny_skia::{Color, FillRule, Paint, Path, PathBuilder, Pixmap, Transform};

pub struct Circle {
    radius: f32,
    color: Color,
}

impl Circle {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            color: Color::WHITE,
        }
    }

    pub fn set_color(mut self, color: Color) -> Circle {
        self.color = color;
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
        paint.set_color(self.color);

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
