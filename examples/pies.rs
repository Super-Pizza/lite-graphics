use lite_graphics::{color::Rgba, draw::Buffer, Offset};

fn main() {
    let buf = Buffer::new(400, 300);
    buf.circle_pie_aa(Offset { x: 200, y: 150 }, 100, 5., 1., Rgba::BLACK);

    buf.circle_pie_aa(Offset { x: 200, y: 150 }, 50, 0., 1., Rgba::PINK);
    buf.circle_pie(Offset { x: 200, y: 150 }, 50, 1., 2., Rgba::BLUE);
    buf.circle_pie_aa(Offset { x: 200, y: 150 }, 50, 2., 3., Rgba::GREEN);
    buf.circle_pie(Offset { x: 200, y: 150 }, 50, 3., 4., Rgba::BROWN);
    buf.circle_pie_aa(Offset { x: 200, y: 150 }, 50, 4., 5., Rgba::MAGENTA);
    buf.circle_pie(Offset { x: 200, y: 150 }, 50, 5., 6., Rgba::RED);

    buf.draw();
}
