use lite_graphics::{
    color::{Color, Rgba},
    Buffer, Drawable, Rect,
};

fn main() {
    let buf = Buffer::new(400, 300);
    buf.fill_rect(
        Rect {
            x: 100,
            y: 100,
            w: 150,
            h: 100,
        },
        Color::RED,
    );
    buf.fill_rect(
        Rect {
            x: 150,
            y: 150,
            w: 150,
            h: 100,
        },
        Rgba::from([0, 0, 255, 128]).into(),
    );
    buf.rect(
        Rect {
            x: 50,
            y: 50,
            w: 300,
            h: 200,
        },
        Color::GREEN,
    );
    buf.draw();
}
