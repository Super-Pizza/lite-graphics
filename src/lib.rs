pub mod draw;
pub mod window;

#[derive(Clone, Copy, Default)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
    pub fn size(&self) -> Size {
        Size {
            w: self.w,
            h: self.h,
        }
    }
    pub fn offset(&self) -> Offset {
        Offset {
            x: self.x,
            y: self.y,
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct Offset {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Default)]
pub struct Size {
    pub w: i32,
    pub h: i32,
}

impl From<Size> for Rect {
    fn from(size: Size) -> Self {
        Rect {
            x: 0,
            y: 0,
            w: size.w,
            h: size.h,
        }
    }
}

impl From<(Offset, Size)> for Rect {
    fn from((offs, size): (Offset, Size)) -> Self {
        Rect {
            x: offs.x,
            y: offs.y,
            w: size.w,
            h: size.h,
        }
    }
}

impl From<Rect> for (Offset, Size) {
    fn from(rect: Rect) -> Self {
        (
            Offset {
                x: rect.x,
                y: rect.y,
            },
            Size {
                w: rect.w,
                h: rect.h,
            },
        )
    }
}

impl From<(i32, i32, i32, i32)> for Rect {
    fn from((x, y, w, h): (i32, i32, i32, i32)) -> Self {
        Rect { x, y, w, h }
    }
}

impl From<Rect> for (i32, i32, i32, i32) {
    fn from(r: Rect) -> Self {
        (r.x, r.y, r.w, r.h)
    }
}

impl From<(i32, i32)> for Size {
    fn from((w, h): (i32, i32)) -> Self {
        Size { w, h }
    }
}

impl From<Size> for (i32, i32) {
    fn from(s: Size) -> Self {
        (s.w, s.h)
    }
}

impl From<(i32, i32)> for Offset {
    fn from((x, y): (i32, i32)) -> Self {
        Offset { x, y }
    }
}

impl From<Offset> for (i32, i32) {
    fn from(o: Offset) -> Self {
        (o.x, o.y)
    }
}
