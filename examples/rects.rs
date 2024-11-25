use lite_graphics::{
    draw::{Buffer, Rgba},
    Rect,
};

fn main() {
    let buf = Buffer::new(400, 300);
    buf.fill_rect(
        Rect {
            x: 0,
            y: 0,
            w: 400,
            h: 300,
        },
        Rgba::WHITE,
    );
    buf.fill_rect(
        Rect {
            x: 100,
            y: 100,
            w: 150,
            h: 100,
        },
        Rgba::RED,
    );
    buf.fill_rect(
        Rect {
            x: 150,
            y: 150,
            w: 150,
            h: 100,
        },
        Rgba::from([0, 0, 255, 128]),
    );
    buf.draw();
}
