mod flower_draw;
mod image_draw;
mod rect_round;
mod circle;

pub use image_draw::*;
pub use rect_round::*;
pub use circle::*;
pub use flower_draw::*;
use tiny_skia::{Color, Pixmap, PixmapPaint, Transform};

pub trait Draw {
    // 图形元素
    fn create_pixmap(self) -> Pixmap;

    fn draw(self, base_pixmap: &mut Pixmap, x: i32, y: i32) where Self: Sized {
        base_pixmap.draw_pixmap(
            x,
            y,
            self.create_pixmap().as_ref(),
            &PixmapPaint::default(),
            Transform::identity(),
            None,
        );
    }
}
