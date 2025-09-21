use std::ops::{Add, Neg, Sub, SubAssign};

pub mod color;
pub mod draw;
#[cfg(feature = "window")]
pub mod window;

pub use draw::{Buffer, Drawable, Overlay};

#[derive(Clone, Copy, Default)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

impl Rect {
    pub fn new(offs: Offset, size: Size) -> Self {
        Rect {
            x: offs.x,
            y: offs.y,
            w: size.w,
            h: size.h,
        }
    }
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
        let end = self_end.min(other_end);
        let offs = self.offset().max(other.offset());
        let size = end.abs_diff(offs);
        (offs, size).into()
    }
}

#[derive(Clone, Copy, Default)]
pub struct Offset {
    pub x: i32,
    pub y: i32,
}

impl Offset {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn abs_diff(&self, rhs: Self) -> Size {
        Size {
            w: self.x.abs_diff(rhs.x),
            h: self.y.abs_diff(rhs.y),
        }
    }
    pub fn max(&self, rhs: Self) -> Self {
        Self {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
        }
    }
    pub fn min(&self, rhs: Self) -> Self {
        Self {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct Size {
    pub w: u32,
    pub h: u32,
}

impl Size {
    pub fn new(w: u32, h: u32) -> Self {
        Self { w, h }
    }
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
        Self::new(offs, size)
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

macro_rules! ref_impls {
    (impl $impl:ident<$other:ty> for $self:ty { fn $fn:ident($lhs:ident, $rhs:ident) -> $out:ty { $($code:tt)* } }) => {
        impl $impl<$other> for $self {
            type Output = $out;
            #[inline]
            fn $fn(self, rhs: $other) -> Self::Output {
                let $lhs = self;
                let $rhs = rhs;
                $($code)*
            }
        }
        impl $impl<$other> for &$self {
            type Output = $out;
            #[inline]
            fn $fn(self, rhs: $other) -> Self::Output {
                let $lhs = *self;
                let $rhs = rhs;
                $($code)*
            }
        }
        impl $impl<&$other> for $self {
            type Output = $out;
            #[inline]
            fn $fn(self, rhs: &$other) -> Self::Output {
                let $lhs = self;
                let $rhs = *rhs;
                $($code)*
            }
        }
        impl $impl<&$other> for &$self {
            type Output = $out;
            #[inline]
            fn $fn(self, rhs: &$other) -> Self::Output {
                let $lhs = *self;
                let $rhs = *rhs;
                $($code)*
            }
        }
    };
    (impl $impl:ident<$other:ty> for $self:ty { fn $fn:ident(&mut $lhs:ident, $rhs:ident) { $($code:tt)* } }) => {
        impl $impl<$other> for $self {
            #[inline]
            fn $fn(&mut self, rhs: $other) {
                let $lhs = self;
                let $rhs = rhs;
                $($code)*
            }
        }
        impl $impl<&$other> for $self {
            #[inline]
            fn $fn(&mut self, rhs: &$other) {
                let $lhs = self;
                let $rhs = *rhs;
                $($code)*
            }
        }
    };
}

ref_impls! {impl Sub<Self> for Offset {
    fn sub(this, rhs) -> Offset {
        Offset {
            x: this.x - rhs.x,
            y: this.y - rhs.y
        }
    }
}}

ref_impls! {impl SubAssign<Self> for Offset {
    fn sub_assign(&mut this, rhs) {
        *this = *this - rhs
    }
}}

ref_impls! {impl Add<Size> for Offset {
    fn add(this, rhs) -> Offset {
        Offset {
            x: this.x + rhs.w as i32,
            y: this.y + rhs.h as i32,
        }
    }
}}

ref_impls! {impl Add<Offset> for Size {
    fn add(this, rhs) -> Offset {
        Offset {
            x: this.w as i32 + rhs.x,
            y: this.h as i32 + rhs.y,
        }
    }
}}

ref_impls! {impl Add<Self> for Offset {
    fn add(this, rhs) -> Offset {
        Offset {
            x: this.x + rhs.x,
            y: this.y + rhs.y,
        }
    }
}}

ref_impls! {impl Add<Self> for Size {
    fn add(this, rhs) -> Size {
        Size {
            w: this.w + rhs.w,
            h: this.h + rhs.h,
        }
    }
}}

impl Neg for Offset {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

ref_impls! {impl Add<Offset> for Rect {
    fn add(this, rhs) -> Rect {
        Rect::from((this.offset() + rhs, this.size()))
    }
}}
