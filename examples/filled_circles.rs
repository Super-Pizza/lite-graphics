use lite_graphics::{color::Rgba, draw::Buffer, Offset};

fn main() {
    let buf = Buffer::new(400, 300);
    buf.fill_circle(Offset { x: 150, y: 150 }, 50, Rgba::RED);
    buf.fill_circle_aa(Offset { x: 250, y: 150 }, 50, Rgba::BLUE);
    buf.draw();
}
