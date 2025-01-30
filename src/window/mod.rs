#[cfg(feature = "wayland")]
mod wayland;
#[cfg(feature = "x11rb")]
mod x11;

impl crate::draw::Buffer {
    pub fn draw(&self) {
        if std::env::var("WAYLAND_DISPLAY")
            .unwrap_or_default()
            .is_empty()
        {
            #[cfg(feature = "x11rb")]
            x11::window(self);
        } else {
            #[cfg(feature = "wayland")]
            wayland::window(self);
        }
    }
}
