use lite_graphics::{
    draw::{Buffer, Rgba},
    Rect,
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
    buf.rect(
        Rect {
            x: 50,
            y: 50,
            w: 300,
            h: 200,
        },
        Rgba::GREEN,
    );
    buf.draw();
}
