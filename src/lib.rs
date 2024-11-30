use std::ops::{Add, Sub};

pub mod draw;
pub mod window;

#[derive(Clone, Copy, Default)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

impl Rect {
    #[inline]
    pub fn size(&self) -> Size {
        Size {
            w: self.w,
            h: self.h,
        }
    }
    #[inline]
    pub fn offset(&self) -> Offset {
        Offset {
            x: self.x,
            y: self.y,
        }
    }
    /// Returns the opposite corner of [`Self::offset`]
    #[inline]
    pub fn offset_2(&self) -> Offset {
        self.offset() + self.size()
    }
    /// Clamps `self` to fit inside `other`. If `other` fails to contain `self`, returns `other` with zero size.
    pub fn clamp(&self, other: Self) -> Rect {
        let self_end = self.offset_2();
        let other_end = other.offset_2();
        if self.x >= other_end.x
            || self.y >= other_end.y
            || self_end.x <= other.x
            || self_end.y <= other.y
        {
            return (other.offset(), Size::default()).into();
        }
        let end = Offset {
            x: self_end.x.min(other_end.x),
            y: self_end.y.min(other_end.y),
        };
        let offs = Offset {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        };
        let size = end - offs;
        (offs, size).into()
    }
}

#[derive(Clone, Copy, Default)]
pub struct Offset {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Default)]
pub struct Size {
    pub w: u32,
    pub h: u32,
}

impl From<Size> for Rect {
    #[inline]
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
    #[inline]
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
    #[inline]
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

impl From<(i32, i32, u32, u32)> for Rect {
    #[inline]
    fn from((x, y, w, h): (i32, i32, u32, u32)) -> Self {
        Rect { x, y, w, h }
    }
}

impl From<Rect> for (i32, i32, u32, u32) {
    #[inline]
    fn from(r: Rect) -> Self {
        (r.x, r.y, r.w, r.h)
    }
}

impl From<(u32, u32)> for Size {
    #[inline]
    fn from((w, h): (u32, u32)) -> Self {
        Size { w, h }
    }
}

impl From<Size> for (u32, u32) {
    #[inline]
    fn from(s: Size) -> Self {
        (s.w, s.h)
    }
}

impl From<(i32, i32)> for Offset {
    #[inline]
    fn from((x, y): (i32, i32)) -> Self {
        Offset { x, y }
    }
}

impl From<Offset> for (i32, i32) {
    #[inline]
    fn from(o: Offset) -> Self {
        (o.x, o.y)
    }
}

impl Sub<Self> for Offset {
    type Output = Size;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Size {
            w: (self.x - rhs.x).max(0) as u32,
            h: (self.y - rhs.y).max(0) as u32,
        }
    }
}

impl Add<Size> for Offset {
    type Output = Offset;
    #[inline]
    fn add(self, rhs: Size) -> Self::Output {
        Offset {
            x: self.x + rhs.w as i32,
            y: self.y + rhs.h as i32,
        }
    }
}
