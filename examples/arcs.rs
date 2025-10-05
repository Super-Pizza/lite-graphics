use lite_graphics::{color::Color, Buffer, Drawable, Offset};

fn main() {
    let buf = Buffer::new(400, 300);
    buf.circle_arc_aa(Offset { x: 200, y: 150 }, 50, 0., 1., Color::ORANGE);
    buf.circle_arc(Offset { x: 200, y: 150 }, 50, 1., 2., Color::BLUE);
    buf.circle_arc_aa(Offset { x: 200, y: 150 }, 50, 2., 3., Color::GREEN);
    buf.circle_arc(Offset { x: 200, y: 150 }, 50, 3., 4., Color::CYAN);
    buf.circle_arc_aa(Offset { x: 200, y: 150 }, 50, 4., 5., Color::MAGENTA);
    buf.circle_arc(Offset { x: 200, y: 150 }, 50, 5., 6., Color::BLACK);

    buf.circle_arc_aa(Offset { x: 200, y: 150 }, 100, 5., 1., Color::BLACK);
    buf.draw();
}
