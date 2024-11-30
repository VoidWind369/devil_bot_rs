use crate::api::image_util::{Circle, Direction, Draw};
use tiny_skia::{
    Color, FillRule, GradientStop, LinearGradient, Paint, Path, PathBuilder, Pixmap, PixmapPaint,
    Point, RadialGradient, SpreadMode, Transform,
};

#[derive(Clone)]
pub struct Flower {
    size: f32,
    display: f32,
    start_color: Color,
    end_color: Color,
}

impl Flower {
    pub fn new() -> Self {
        Self {
            size: 100.0,
            display: 0.0,
            start_color: Color::WHITE,
            end_color: Color::WHITE,
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

    fn builder(self) -> Option<Path> {
        let size = self.size;
        let d = self.display;
        let mut builder = PathBuilder::new();
        builder.move_to(size * 0.5, 0.0);
        builder.quad_to(size * 0.7 - d, 0.0, size * 0.7 - d, size * 0.18 - d);
        builder.line_to(size * 0.5, size * 0.5);

        builder.line_to(size * 0.82 + d, size * 0.3 + d);
        builder.quad_to(size, size * 0.3 + d, size, size * 0.5);
        builder.quad_to(size, size * 0.7 - d, size * 0.82 + d, size * 0.7 - d);
        builder.line_to(size * 0.5, size * 0.5);

        builder.line_to(size * 0.7 - d, size * 0.82 + d);
        builder.quad_to(size * 0.7 - d, size, size * 0.5, size);
        builder.quad_to(size * 0.3 + d, size, size * 0.3 + d, size * 0.82 + d);
        builder.line_to(size * 0.5, size * 0.5);

        builder.line_to(size * 0.18 - d, size * 0.7 - d);
        builder.quad_to(0.0, size * 0.7 - d, 0.0, size * 0.5);
        builder.quad_to(0.0, size * 0.3 + d, size * 0.18 - d, size * 0.3 + d);
        builder.line_to(size * 0.5, size * 0.5);

        builder.line_to(size * 0.3 + d, size * 0.18 - d);
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
        if self.start_color == self.end_color {
            paint.set_color(self.start_color);
        } else {
            paint.shader = RadialGradient::new(
                Point::from_xy(self.size / 2.0, self.size / 2.0),
                Point::from_xy(self.size / 2.0, self.size / 2.0),
                self.size / 2.0,
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

/// # 画朵花
fn flower_logo() -> Pixmap {
    let (circle_r, bg_size, co_size, co1_size) = (70.0, 130.0, 116.0, 75.0);
    let bg_xy = (circle_r - bg_size / 2.0) as i32;
    let co_xy = (circle_r - co_size / 2.0) as i32;
    let co_xy1 = (circle_r - co1_size / 2.0) as i32;

    let mut bg = Circle::new(circle_r)
        .set_color(Color::from_rgba8(255, 255, 255, 100))
        .create_pixmap();

    Circle::new(circle_r - 20.0)
        .set_color(Color::from_rgba8(46, 49, 146, 200))
        .draw(&mut bg, 20, 20);

    Flower::new()
        .set_size(bg_size)
        .set_start_color(Color::from_rgba8(255, 255, 145, 180))
        .set_end_color(Color::from_rgba8(46, 49, 146, 230))
        .draw(&mut bg, bg_xy, bg_xy);

    Flower::new()
        .set_size(co_size)
        .set_display(6.0)
        .set_start_color(Color::from_rgba8(251, 176, 59, 150))
        .set_end_color(Color::from_rgba8(240, 90, 36, 150))
        .draw(&mut bg, co_xy, co_xy);

    Flower::new()
        .set_size(co1_size)
        .set_display(10.0)
        .set_start_color(Color::from_rgba8(240, 90, 36, 255))
        .set_end_color(Color::from_rgba8(240, 90, 36, 100))
        .draw(&mut bg, co_xy1, co_xy1);
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
