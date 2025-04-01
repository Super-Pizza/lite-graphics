#[cfg(all(
    unix,
    feature = "wayland",
    not(any(
        target_os = "redox",
        target_family = "wasm",
        target_os = "android",
        target_os = "ios",
        target_os = "macos"
    ))
))]
mod wayland;
#[cfg(all(
    unix,
    feature = "x11rb",
    not(any(
        target_os = "redox",
        target_family = "wasm",
        target_os = "android",
        target_os = "ios",
        target_os = "macos"
    ))
))]
mod x11;

impl crate::draw::Buffer {
    pub fn draw(&self) {
        #[cfg(all(
            unix,
            not(any(
                target_os = "redox",
                target_family = "wasm",
                target_os = "android",
                target_os = "ios",
                target_os = "macos"
            ))
        ))]
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
