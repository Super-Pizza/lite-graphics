use lite_graphics::{
    draw::{Buffer, Rgba},
    Rect,
};

fn main() {
    let buf = Buffer::new(400, 300);
    buf.round_rect(
        Rect {
            x: 50,
            y: 50,
            w: 300,
            h: 200,
        },
        10,
        Rgba::RED,
    );
    buf.round_rect_aa(
        Rect {
            x: 100,
            y: 100,
            w: 200,
            h: 100,
        },
        10,
        Rgba::BLUE,
    );
    buf.fill_round_rect(
        Rect {
            x: 125,
            y: 125,
            w: 50,
            h: 50,
        },
        10,
        Rgba::BLACK.set_a(127),
    );
    buf.fill_round_rect_aa(
        Rect {
            x: 225,
            y: 125,
            w: 50,
            h: 50,
        },
        10,
        Rgba::BLACK.set_a(127),
    );
    buf.draw();
}
