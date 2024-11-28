use lite_graphics::{
    draw::{Buffer, Rgba},
    Offset,
};

fn main() {
    let buf = Buffer::new(400, 300);
    buf.circle(Offset { x: 150, y: 150 }, 50, Rgba::RED);
    buf.circle_aa(Offset { x: 250, y: 150 }, 50, Rgba::BLUE);
    buf.draw();
}
