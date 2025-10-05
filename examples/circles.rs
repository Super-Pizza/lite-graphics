use lite_graphics::{color::Color, Buffer, Drawable, Offset};

fn main() {
    let buf = Buffer::new(400, 300);
    buf.circle(Offset { x: 150, y: 150 }, 50, Color::RED);
    buf.circle_aa(Offset { x: 250, y: 150 }, 50, Color::BLUE);
    buf.draw();
}
