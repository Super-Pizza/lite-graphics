use lite_graphics::{color::Rgba, draw::Buffer, Offset};

fn main() {
    let buf = Buffer::new(400, 300);
    buf.line_h(Offset { x: 100, y: 200 }, 200, Rgba::RED);
    buf.line_v(Offset { x: 200, y: 100 }, 100, Rgba::BLUE);
    buf.draw();
}
