use lite_graphics::{color::Color, Buffer, Drawable, Offset};

fn main() {
    let buf = Buffer::new(400, 300);
    buf.line(
        Offset { x: 100, y: 100 },
        Offset { x: 200, y: 250 },
        Color::RED,
    );
    buf.line_aa(
        Offset { x: 100, y: 150 },
        Offset { x: 300, y: 200 },
        Color::BLUE,
    );
    buf.draw();
}
