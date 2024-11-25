#[cfg(feature = "x11rb")]
mod x11;

impl crate::draw::Buffer {
    pub fn draw(&self) {
        #[cfg(feature = "x11rb")]
        x11::window(self);
    }
}
