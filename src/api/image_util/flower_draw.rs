use tiny_skia::{Path, PathBuilder};

struct Flower {}

impl Flower {
    fn new() -> Option<Path> {
        let mut builder = PathBuilder::new();
        builder.move_to(50.0, 0.0);
        builder.quad_to(70.0, 0.0, 70.0, 10.0);
        builder.line_to(50.0, 50.0);
        builder.line_to(80.0, 30.0);
        builder.quad_to(0.0, 30.0, 100.0, 50.0);
        builder.quad_to(100.0, 70.0, 80.8, 70.0);
        builder.line_to(50.0, 50.0);
        builder.line_to(70.0,80.0);
        builder.quad_to(70.0,100.0, 50.0, 100.0);
        builder.quad_to(30.0,100.0,30.0,80.0);
        builder.line_to(50.0, 50.0);
        builder.line_to(70.0,20.0);
        builder.quad_to(0.0,70.0,0.0,50.0);
        builder.quad_to(0.0,30.0,20.0,30.0);
        builder.line_to(50.0, 50.0);
        builder.line_to(30.0,20.0);
        builder.quad_to()
        builder.close();
        builder.finish()
    }
}
