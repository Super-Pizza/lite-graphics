use lite_graphics::{
    color::{DirectionalGradient, Rgba},
    Buffer, Drawable, Offset, Rect,
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
        DirectionalGradient::new(
            &[(0.0, Rgba::RED), (1.0, Rgba::BLUE)],
            false,
            0.,
            150.,
            Offset { x: 100, y: 100 },
        ),
    );
    buf.fill_rect(
        Rect {
            x: 150,
            y: 150,
            w: 150,
            h: 100,
        },
        DirectionalGradient::new(
            &[(0.0, Rgba::BLUE.set_a(0)), (1.0, Rgba::BLUE)],
            false,
            0.,
            150.,
            Offset { x: 150, y: 150 },
        ),
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
