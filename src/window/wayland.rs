#![allow(clippy::collapsible_match)]
use nix::{
    fcntl::OFlag,
    sys::{
        mman::{MapFlags, ProtFlags},
        stat::Mode,
    },
};
use std::{num::NonZeroUsize, os::fd::AsFd};
use wayland_client::{
    delegate_noop,
    globals::{registry_queue_init, GlobalListContents},
    protocol::{
        wl_buffer, wl_compositor, wl_keyboard, wl_registry, wl_seat, wl_shm, wl_shm_pool,
        wl_surface,
    },
    Dispatch, WEnum,
};
use wayland_protocols::xdg::shell::client::{xdg_surface, xdg_toplevel, xdg_wm_base};

struct State {
    running: bool,
    base_surface: Option<wl_surface::WlSurface>,
    buffer: Option<wl_buffer::WlBuffer>,
    wm_base: Option<xdg_wm_base::XdgWmBase>,
    xdg_surface: Option<(xdg_surface::XdgSurface, xdg_toplevel::XdgToplevel)>,
    configured: bool,
}

delegate_noop!(State: ignore wl_compositor::WlCompositor);
delegate_noop!(State: ignore wl_surface::WlSurface);
delegate_noop!(State: ignore wl_shm::WlShm);
delegate_noop!(State: ignore wl_shm_pool::WlShmPool);
delegate_noop!(State: ignore wl_buffer::WlBuffer);

impl Dispatch<wl_registry::WlRegistry, GlobalListContents> for State {
    fn event(
        _: &mut Self,
        _: &wl_registry::WlRegistry,
        _: wl_registry::Event,
        _: &GlobalListContents,
        _: &wayland_client::Connection,
        _: &wayland_client::QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<wl_seat::WlSeat, ()> for State {
    fn event(
        _: &mut Self,
        seat: &wl_seat::WlSeat,
        event: wl_seat::Event,
        _: &(),
        _: &wayland_client::Connection,
        qh: &wayland_client::QueueHandle<Self>,
    ) {
        if let wl_seat::Event::Capabilities {
            capabilities: WEnum::Value(capabilities),
        } = event
        {
            if capabilities.contains(wl_seat::Capability::Keyboard) {
                seat.get_keyboard(qh, ());
            }
        }
    }
}

impl Dispatch<wl_keyboard::WlKeyboard, ()> for State {
    fn event(
        this: &mut Self,
        _: &wl_keyboard::WlKeyboard,
        event: wl_keyboard::Event,
        _: &(),
        _: &wayland_client::Connection,
        _: &wayland_client::QueueHandle<Self>,
    ) {
        if let wl_keyboard::Event::Key { key, .. } = event {
            if key == 1 {
                this.running = false;
            }
        }
    }
}

impl Dispatch<xdg_wm_base::XdgWmBase, ()> for State {
    fn event(
        _: &mut Self,
        wm_base: &xdg_wm_base::XdgWmBase,
        event: xdg_wm_base::Event,
        _: &(),
        _: &wayland_client::Connection,
        _: &wayland_client::QueueHandle<Self>,
    ) {
        if let xdg_wm_base::Event::Ping { serial } = event {
            wm_base.pong(serial);
        }
    }
}

impl Dispatch<xdg_surface::XdgSurface, ()> for State {
    fn event(
        this: &mut Self,
        xdg_surface: &xdg_surface::XdgSurface,
        event: xdg_surface::Event,
        _: &(),
        _: &wayland_client::Connection,
        _: &wayland_client::QueueHandle<Self>,
    ) {
        if let xdg_surface::Event::Configure { serial } = event {
            xdg_surface.ack_configure(serial);
            this.configured = true;

            let surface = this.base_surface.as_ref().unwrap();
            if let Some(ref buffer) = this.buffer {
                surface.attach(Some(buffer), 0, 0);
                surface.commit();
            }
        }
    }
}

impl Dispatch<xdg_toplevel::XdgToplevel, ()> for State {
    fn event(
        this: &mut Self,
        _: &xdg_toplevel::XdgToplevel,
        event: xdg_toplevel::Event,
        _: &(),
        _: &wayland_client::Connection,
        _: &wayland_client::QueueHandle<Self>,
    ) {
        if let xdg_toplevel::Event::Close = event {
            this.running = false
        }
    }
}

impl State {
    fn init_xdg_surface(&mut self, qh: &wayland_client::QueueHandle<State>) {
        let wm_base = self.wm_base.as_ref().unwrap();
        let base_surface = self.base_surface.as_ref().unwrap();

        let xdg_surface = wm_base.get_xdg_surface(base_surface, qh, ());
        let toplevel = xdg_surface.get_toplevel(qh, ());
        toplevel.set_title("Example window".into());

        base_surface.commit();

        self.xdg_surface = Some((xdg_surface, toplevel));
    }
}

fn draw(tmp: &mut [u8], buf: &crate::draw::Buffer) {
    for y in 0..buf.height {
        for x in 0..buf.width {
            tmp[(x + y * buf.width) * 4..(x + y * buf.width) * 4 + 4].copy_from_slice(&[
                buf.data.borrow()[x * 3 + y * buf.width * 3 + 2],
                buf.data.borrow()[x * 3 + y * buf.width * 3 + 1],
                buf.data.borrow()[x * 3 + y * buf.width * 3],
                255,
            ]);
        }
    }
}

struct Shm<'a>(&'a str);
impl Drop for Shm<'_> {
    fn drop(&mut self) {
        nix::sys::mman::shm_unlink(self.0).unwrap();
    }
}

pub(super) fn window(buf: &crate::draw::Buffer) {
    let conn = wayland_client::Connection::connect_to_env().unwrap();
    let _dpy = conn.display();
    let (globals, mut event_queue) = registry_queue_init::<State>(&conn).unwrap();
    let qh = event_queue.handle();

    let mut state = State {
        running: true,
        base_surface: None,
        buffer: None,
        wm_base: None,
        xdg_surface: None,
        configured: false,
    };

    event_queue.roundtrip(&mut state).unwrap();

    let compositor: wl_compositor::WlCompositor = globals.bind(&qh, 4..=5, ()).unwrap();
    let wm_base: xdg_wm_base::XdgWmBase = globals.bind(&qh, 4..=5, ()).unwrap();
    state.wm_base = Some(wm_base);
    let surface = compositor.create_surface(&qh, ());
    state.base_surface = Some(surface);
    state.init_xdg_surface(&qh);

    let _: wl_seat::WlSeat = globals.bind(&qh, 6..=7, ()).unwrap();

    let shm: wl_shm::WlShm = globals.bind(&qh, 1..=2, ()).unwrap();

    let name = "lite_graphics_wayland";
    let file = nix::sys::mman::shm_open(
        name,
        OFlag::O_CREAT | OFlag::O_EXCL | OFlag::O_RDWR,
        Mode::S_IRUSR | Mode::S_IWUSR,
    )
    .unwrap();
    nix::unistd::ftruncate(file.as_fd(), (buf.width * buf.height * 4) as _).unwrap();
    let addr = unsafe {
        let ptr = nix::sys::mman::mmap(
            None,
            NonZeroUsize::new(buf.width * buf.height * 4).unwrap(),
            ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
            MapFlags::MAP_SHARED,
            file.as_fd(),
            0,
        )
        .unwrap();
        std::slice::from_raw_parts_mut(ptr.as_ptr() as *mut u8, buf.width * buf.height * 4)
    };
    draw(addr, buf);

    let _ = Shm(name);

    let pool = shm.create_pool(file.as_fd(), (buf.width * buf.height * 4) as i32, &qh, ());
    let buffer = pool.create_buffer(
        0,
        buf.width as i32,
        buf.height as i32,
        (buf.width * 4) as i32,
        wl_shm::Format::Argb8888,
        &qh,
        (),
    );
    state.buffer = Some(buffer.clone());

    if state.configured {
        let surface = state.base_surface.as_ref().unwrap();
        surface.attach(Some(&buffer), 0, 0);
        surface.commit();
    }

    while state.running {
        event_queue.blocking_dispatch(&mut state).unwrap();
    }
}
