use std::borrow::Cow;

use x11rb::{
    connect,
    connection::Connection,
    image::{BitsPerPixel, Image, ImageOrder, ScanlinePad},
    protocol::{
        xproto::{
            AtomEnum, ConnectionExt as _, CreateGCAux, CreateWindowAux, EventMask, PropMode,
            WindowClass,
        },
        Event,
    },
    wrapper::ConnectionExt as _,
    COPY_DEPTH_FROM_PARENT,
};

x11rb::atom_manager! {
    pub Atoms: AtomsCookie {
        WM_PROTOCOLS,
        WM_DELETE_WINDOW,
        _NET_WM_NAME,
        UTF8_STRING,
    }
}

pub(super) fn window(buf: &crate::draw::Buffer) {
    let (conn, screen_num) = connect(None).unwrap();
    let screen = &conn.setup().roots[screen_num];
    let window = conn.generate_id().unwrap();
    let gc = conn.generate_id().unwrap();

    let atoms = Atoms::new(&conn).unwrap().reply().unwrap();

    conn.create_window(
        COPY_DEPTH_FROM_PARENT,
        window,
        screen.root,
        0,
        0,
        buf.width as _,
        buf.height as _,
        0,
        WindowClass::INPUT_OUTPUT,
        0,
        &CreateWindowAux::new()
            .background_pixel(screen.white_pixel)
            .event_mask(
                EventMask::EXPOSURE
                    | EventMask::STRUCTURE_NOTIFY
                    | EventMask::NO_EVENT
                    | EventMask::KEY_PRESS,
            ),
    )
    .unwrap();

    let title = "Example Window";
    conn.change_property8(
        PropMode::REPLACE,
        window,
        AtomEnum::WM_NAME,
        AtomEnum::STRING,
        title.as_bytes(),
    )
    .unwrap();
    conn.change_property8(
        PropMode::REPLACE,
        window,
        atoms._NET_WM_NAME,
        atoms.UTF8_STRING,
        title.as_bytes(),
    )
    .unwrap();
    conn.change_property32(
        PropMode::REPLACE,
        window,
        atoms.WM_PROTOCOLS,
        AtomEnum::ATOM,
        &[atoms.WM_DELETE_WINDOW],
    )
    .unwrap();
    conn.change_property8(
        PropMode::REPLACE,
        window,
        AtomEnum::WM_CLASS,
        AtomEnum::STRING,
        b"simple_window\0",
    )
    .unwrap();

    conn.create_gc(gc, window, &CreateGCAux::new()).unwrap();

    conn.map_window(window).unwrap();
    conn.flush().unwrap();
    loop {
        match conn.wait_for_event().unwrap() {
            Event::Expose(_) => {
                let data = buf.data.borrow();
                let img = Image::new(
                    buf.width as _,
                    buf.height as _,
                    ScanlinePad::Pad8,
                    24,
                    BitsPerPixel::B24,
                    ImageOrder::MsbFirst,
                    Cow::Borrowed(&data),
                )
                .unwrap();
                let img = img.native(conn.setup()).unwrap();
                img.put(&conn, window, gc, 0, 0).unwrap();
                conn.flush().unwrap();
            }
            Event::ClientMessage(event) => {
                let data = event.data.as_data32();
                if event.format == 32 && event.window == window && data[0] == atoms.WM_DELETE_WINDOW
                {
                    return;
                }
            }
            _ => {}
        }
    }
}
