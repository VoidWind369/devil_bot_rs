use crate::api::image_util::{Circle, Draw};
use tiny_skia::{Color, FillRule, Paint, Path, PathBuilder, Pixmap, PixmapPaint, Transform};

#[derive(Clone)]
pub struct Flower {
    size: f32,
    display: f32,
    color: Color,
}

impl Flower {
    pub fn new() -> Self {
        Self {
            size: 100.0,
            display: 0.0,
            color: Color::WHITE,
        }
    }

    pub fn set_size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn set_display(mut self, display: f32) -> Self {
        self.display = display;
        self
    }

    pub fn set_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    fn builder(self) -> Option<Path> {
        let size = self.size;
        let d = self.display;
        let mut builder = PathBuilder::new();
        builder.move_to(size * 0.5, 0.0);
        builder.quad_to(size * 0.7 - d, 0.0, size * 0.75 - d, size * 0.18 - d);
        builder.line_to(size * 0.5, size * 0.5);

        builder.line_to(size * 0.82 + d, size * 0.25 + d);
        builder.quad_to(size, size * 0.3 + d, size, size * 0.5);
        builder.quad_to(size, size * 0.7 - d, size * 0.82 + d, size * 0.75 - d);
        builder.line_to(size * 0.5, size * 0.5);

        builder.line_to(size * 0.75 - d, size * 0.82 + d);
        builder.quad_to(size * 0.7 - d, size, size * 0.5, size);
        builder.quad_to(size * 0.3 + d, size, size * 0.25 + d, size * 0.82 + d);
        builder.line_to(size * 0.5, size * 0.5);

        builder.line_to(size * 0.18 - d, size * 0.75 - d);
        builder.quad_to(0.0, size * 0.7 - d, 0.0, size * 0.5);
        builder.quad_to(0.0, size * 0.3 + d, size * 0.18 - d, size * 0.25 + d);
        builder.line_to(size * 0.5, size * 0.5);

        builder.line_to(size * 0.25 + d, size * 0.18 - d);
        builder.quad_to(size * 0.3 + d, 0.0, size * 0.5, 0.0);
        builder.close();
        builder.finish()
    }
}

impl Draw for Flower {
    // 图形元素
    fn create_pixmap(self) -> Pixmap {
        // 创建一个 Pixmap 用来渲染
        let mut pixmap = Pixmap::new(self.size as u32, self.size as u32).unwrap();

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

/// # 画朵花
fn flower_logo() -> Pixmap {
    let (circle_r, bg_size, co_size) = (70.0, 120.0, 96.0);
    let bg_xy = (circle_r - bg_size / 2.0) as i32;
    let co_xy = (circle_r - co_size / 2.0) as i32;

    let mut bg = Circle::new(circle_r)
        .set_color(Color::from_rgba8(255, 255, 255, 60))
        .create_pixmap();

    Flower::new()
        .set_size(bg_size)
        .set_color(Color::from_rgba8(140, 198, 63, 255))
        .draw(&mut bg, bg_xy, bg_xy);

    Flower::new()
        .set_size(co_size)
        .set_display(10.0)
        .set_color(Color::from_rgba8(255, 248, 202, 255))
        .draw(&mut bg, co_xy, co_xy);
    bg
}

/// # 标志
pub fn draw_logo(base_pixmap: &mut Pixmap, x: i32, y: i32) {
    base_pixmap.draw_pixmap(
        x,
        y,
        flower_logo().as_ref(),
        &PixmapPaint::default(),
        Transform::identity(),
        None,
    )
}

#[tokio::test]
async fn flower_test() {
    flower_logo().save_png("flower_test.png").unwrap();
}
