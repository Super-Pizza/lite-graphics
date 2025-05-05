use std::{cell::RefCell, mem, rc::Rc};

use crate::{Offset, Rect, Size};

#[derive(Clone, Copy)]
pub struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl From<[u8; 4]> for Rgba {
    fn from(value: [u8; 4]) -> Self {
        Rgba {
            r: value[0],
            g: value[1],
            b: value[2],
            a: value[3],
        }
    }
}

impl From<[u8; 3]> for Rgba {
    fn from(value: [u8; 3]) -> Self {
        Rgba {
            r: value[0],
            g: value[1],
            b: value[2],
            a: 255,
        }
    }
}

impl From<Rgba> for [u8; 4] {
    fn from(this: Rgba) -> Self {
        [this.r, this.g, this.b, this.a]
    }
}

impl From<Rgba> for [u8; 3] {
    fn from(this: Rgba) -> Self {
        [this.r, this.g, this.b]
    }
}

impl Rgba {
    pub const RED: Self = Self::hex("#f00").unwrap();
    pub const GREEN: Self = Self::hex("#0f0").unwrap();
    pub const BLUE: Self = Self::hex("#00f").unwrap();
    pub const YELLOW: Self = Self::hex("#ff0").unwrap();
    pub const CYAN: Self = Self::hex("#0ff").unwrap();
    pub const MAGENTA: Self = Self::hex("#f0f").unwrap();
    pub const BLACK: Self = Self::hex("#000").unwrap();
    pub const WHITE: Self = Self::hex("#fff").unwrap();

    // Parse hex string (const fn).
    pub const fn hex(val: &'static str) -> Option<Self> {
        const fn u8_from_nibs(n1: &u8, n2: &u8) -> u8 {
            const TABLE: [u8; 128] = [
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0xa, 0xb, 0xc, 0xd, 0xe, 0xf, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
                0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            ];
            TABLE[*n1 as usize] * 16 + TABLE[*n2 as usize]
        }
        const fn get<T>(s: &[T], n: usize) -> Option<&T> {
            if s.len() > n {
                Some(&s[n])
            } else {
                None
            }
        }
        let iter = val.as_bytes();
        if iter[0] != 35 {
            return None;
        }
        let r1 = get(iter, 1);
        let r2 = get(iter, 2);
        let g1 = get(iter, 3);
        let g2 = get(iter, 4);
        let b1 = get(iter, 5);
        let b2 = get(iter, 6);
        let a1 = get(iter, 7);
        let a2 = get(iter, 8);
        if g1.is_none() {
            return None;
        }
        #[allow(clippy::unnecessary_unwrap)]
        let (r, g, b, a) = if b1.is_none() {
            let a = if let Some(a) = g2 {
                u8_from_nibs(a, a)
            } else {
                255
            };
            // 12/16 bits
            (
                u8_from_nibs(r1.unwrap(), r1.unwrap()),
                u8_from_nibs(r2.unwrap(), r2.unwrap()),
                u8_from_nibs(g1.unwrap(), g1.unwrap()),
                a,
            )
        } else if b2.is_none() || a1.is_some() && a2.is_none() {
            return None;
        } else {
            let a = if let (Some(a1), Some(a2)) = (a1, a2) {
                u8_from_nibs(a1, a2)
            } else {
                255
            };
            // 24/32 bits
            (
                u8_from_nibs(r1.unwrap(), r2.unwrap()),
                u8_from_nibs(g1.unwrap(), g2.unwrap()),
                u8_from_nibs(b1.unwrap(), b2.unwrap()),
                a,
            )
        };
        Some(Self { r, g, b, a })
    }
    pub const fn set_a(self, a: u8) -> Self {
        Self {
            r: self.r,
            g: self.g,
            b: self.b,
            a: (a as u16 * self.a as u16 / 255) as u8,
        }
    }
    pub const fn lerp(self, other: Self, t: u8) -> Self {
        Self {
            r: ((other.r as i32 - self.r as i32) * t as i32 / 255 + self.r as i32) as u8,
            g: ((other.g as i32 - self.g as i32) * t as i32 / 255 + self.g as i32) as u8,
            b: ((other.b as i32 - self.b as i32) * t as i32 / 255 + self.b as i32) as u8,
            a: ((other.a as i32 - self.a as i32) * t as i32 / 255 + self.a as i32) as u8,
        }
    }
}

macro_rules! quadrant {
    ($($fn:ident).+($cx:expr,$cy:expr,$x:expr,$y:expr,$color:expr)) => {
        if $x != 0 {
            if $y != 0 {
                $($fn).+($cx - $x, $cy - $y, $color);
            }
            $($fn).+($cx - $x, $cy + $y, $color);
        }
        if $y != 0 {
            $($fn).+($cx + $x, $cy - $y, $color);
        }
        $($fn).+($cx + $x, $cy + $y, $color);
    };
}
pub struct Buffer {
    pub(crate) data: Rc<RefCell<Vec<u8>>>,
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) offs: Offset,
}

impl Buffer {
    /// Creates a buffer, filled with black
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: Rc::new(RefCell::new(vec![255; width * height * 3])),
            width,
            height,
            offs: Default::default(),
        }
    }
    pub fn with_offset(&self, offset: Offset) -> Self {
        Self {
            data: self.data.clone(),
            width: self.width,
            height: self.height,
            offs: offset + self.offs,
        }
    }
    pub fn data(&self) -> std::cell::Ref<'_, Vec<u8>> {
        self.data.borrow()
    }
    pub fn size(&self) -> Size {
        Size {
            w: self.width as u32,
            h: self.height as u32,
        }
    }
    /// Draws a point of the specified color
    pub fn point(&self, x: i32, y: i32, color: Rgba) {
        let x = x + self.offs.x;
        let y = y + self.offs.y;
        if x < 0 || y < 0 || x as usize >= self.width || y as usize >= self.height {
            return;
        }
        let [r, g, b, a] = color.into();
        let (x, y) = (x as usize, y as usize);
        let pixel_range =
            &mut self.data.borrow_mut()[(x + y * self.width) * 3..(x + y * self.width) * 3 + 3];
        if a == 255 {
            // Quick optimization
            pixel_range.copy_from_slice(&[r, g, b]);
        } else {
            // Alpha blending. SRC * A / 255 + DST * (255-A) / 255 = (SRC - DST) * A / 255 + DST
            let (r, g, b, a) = (r as i32, g as i32, b as i32, a as i32);
            pixel_range[0] = ((r - pixel_range[0] as i32) * a / 255 + pixel_range[0] as i32) as u8;
            pixel_range[1] = ((g - pixel_range[1] as i32) * a / 255 + pixel_range[1] as i32) as u8;
            pixel_range[2] = ((b - pixel_range[2] as i32) * a / 255 + pixel_range[2] as i32) as u8;
        }
    }
    /// Fills a rectangle, clipped to the buffer's size
    pub fn fill_rect(&self, rect: Rect, color: Rgba) {
        let rect = rect + self.offs;
        let rect = rect.clamp(Size::from((self.width as u32, self.height as u32)).into());
        let p1 = rect.offset();
        let p2 = rect.offset_2();
        let [r, g, b, a] = color.into();

        let (x1, y1, x2, y2) = (p1.x as usize, p1.y as usize, p2.x as usize, p2.y as usize);
        for row in y1..y2 {
            let pixel_range = &mut self.data.borrow_mut()
                [(x1 + row * self.width) * 3..(x2 + row * self.width) * 3];
            if a == 255 {
                pixel_range.copy_from_slice(&[r, g, b].repeat(x2 - x1));
            } else {
                for i in x1..x2 {
                    let (r, g, b, a) = (r as i32, g as i32, b as i32, a as i32);
                    let r_i = (i - x1) * 3;
                    let g_i = r_i + 1;
                    let b_i = r_i + 2;
                    pixel_range[r_i] =
                        ((r - pixel_range[r_i] as i32) * a / 255 + pixel_range[r_i] as i32) as u8;
                    pixel_range[g_i] =
                        ((g - pixel_range[g_i] as i32) * a / 255 + pixel_range[g_i] as i32) as u8;
                    pixel_range[b_i] =
                        ((b - pixel_range[b_i] as i32) * a / 255 + pixel_range[b_i] as i32) as u8;
                }
            }
        }
    }
    pub fn line(&self, mut p1: Offset, mut p2: Offset, color: Rgba) {
        let steep = if p1.x.abs_diff(p2.x) < p1.y.abs_diff(p2.y) {
            mem::swap(&mut p1.x, &mut p1.y);
            mem::swap(&mut p2.x, &mut p2.y);
            true
        } else {
            false
        };
        if p1.x > p2.x {
            mem::swap(&mut p1, &mut p2);
        }
        let dx = p2.x - p1.x;
        let mut dy = p2.y - p1.y;
        let mut yi = 1;

        if dy < 0 {
            yi = -1;
            dy = -dy;
        }
        let mut d = (2 * dy) - dx;
        let mut y = p1.y;

        for x in p1.x..=p2.x {
            if steep {
                self.point(y, x, color);
            } else {
                self.point(x, y, color);
            }
            if d > 0 {
                y += yi;
                d += 2 * (dy - dx);
            } else {
                d += 2 * dy;
            }
        }
    }
    pub fn line_aa(&self, mut p1: Offset, mut p2: Offset, color: Rgba) {
        let steep = if p1.x.abs_diff(p2.x) < p1.y.abs_diff(p2.y) {
            mem::swap(&mut p1.x, &mut p1.y);
            mem::swap(&mut p2.x, &mut p2.y);
            true
        } else {
            false
        };
        if p1.x > p2.x {
            mem::swap(&mut p1, &mut p2);
        }

        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;

        let gradient = if dx == 0 { 1.0 } else { dy as f32 / dx as f32 };

        // handle first endpoint
        let xend = p1.x as f32;
        let yend = p1.y as f32;
        let xpxl1 = xend;
        let ypxl1 = yend;
        if steep {
            self.point(ypxl1 as i32, xpxl1 as i32, color);
        } else {
            self.point(xpxl1 as i32, ypxl1 as i32, color);
        }
        let mut intery = yend + gradient;

        // handle second endpoint
        let xend = p2.x as f32;
        let yend = p2.y as f32;
        let xpxl2 = xend;
        let ypxl2 = yend;
        if steep {
            self.point(ypxl2 as i32, xpxl2 as i32, color);
        } else {
            self.point(xpxl2 as i32, ypxl2 as i32, color);
        }

        // main loop
        if steep {
            for x in xpxl1 as i32 + 1..xpxl2 as i32 {
                self.point(
                    intery as i32,
                    x,
                    color.set_a(255 - (intery.fract() * 255.0) as u8),
                );
                self.point(
                    intery as i32 + 1,
                    x,
                    color.set_a((intery.fract() * 255.0) as u8),
                );
                intery += gradient;
            }
        } else {
            for x in xpxl1 as i32 + 1..xpxl2 as i32 {
                self.point(
                    x,
                    intery as i32,
                    color.set_a(255 - (intery.fract() * 255.0) as u8),
                );
                self.point(
                    x,
                    intery as i32 + 1,
                    color.set_a((intery.fract() * 255.0) as u8),
                );
                intery += gradient;
            }
        }
    }
    pub fn line_h(&self, p1: Offset, length: i32, color: Rgba) {
        let size = Size {
            w: length as _,
            h: 1,
        };
        let rect = Rect::from((p1 + self.offs, size))
            .clamp(Size::from((self.width as u32, self.height as u32)).into());
        let [r, g, b, a] = color.into();

        let (x1, y1) = (rect.x as usize, rect.y as usize);
        let x2 = rect.offset_2().x as usize;
        let pixel_range =
            &mut self.data.borrow_mut()[(x1 + y1 * self.width) * 3..(x2 + y1 * self.width) * 3];
        if a == 255 {
            pixel_range.copy_from_slice(&[r, g, b].repeat(x2 - x1));
        } else {
            for i in x1..x2 {
                let (r, g, b, a) = (r as i32, g as i32, b as i32, a as i32);
                let r_i = (i - x1) * 3;
                let g_i = r_i + 1;
                let b_i = r_i + 2;
                pixel_range[r_i] =
                    ((r - pixel_range[r_i] as i32) * a / 255 + pixel_range[r_i] as i32) as u8;
                pixel_range[g_i] =
                    ((g - pixel_range[g_i] as i32) * a / 255 + pixel_range[g_i] as i32) as u8;
                pixel_range[b_i] =
                    ((b - pixel_range[b_i] as i32) * a / 255 + pixel_range[b_i] as i32) as u8;
            }
        }
    }
    pub fn line_v(&self, p1: Offset, length: i32, color: Rgba) {
        let size = Size {
            w: 1,
            h: length as _,
        };
        let rect = Rect::from((p1 + self.offs, size))
            .clamp(Size::from((self.width as u32, self.height as u32)).into());
        let [r, g, b, a] = color.into();

        let (x1, y1) = (rect.x as usize, rect.y as usize);
        let y2 = rect.offset_2().y as usize;
        for row in y1..y2 {
            let pixel_range = &mut self.data.borrow_mut()
                [(x1 + row * self.width) * 3..(x1 + 1 + row * self.width) * 3];
            if a == 255 {
                pixel_range.copy_from_slice(&[r, g, b]);
            } else {
                let (r, g, b, a) = (r as i32, g as i32, b as i32, a as i32);
                pixel_range[0] =
                    ((r - pixel_range[0] as i32) * a / 255 + pixel_range[0] as i32) as u8;
                pixel_range[1] =
                    ((g - pixel_range[1] as i32) * a / 255 + pixel_range[1] as i32) as u8;
                pixel_range[2] =
                    ((b - pixel_range[2] as i32) * a / 255 + pixel_range[2] as i32) as u8;
            }
        }
    }
    /// NOTE: this isn't a perfect circle, but it's very efficient.
    pub fn circle(&self, center: Offset, radius: u32, color: Rgba) {
        let mut e = (1 - radius as i32) / 2;
        let mut x = radius as i32;
        let mut y = 0;
        while x >= y {
            if y != 0 {
                if x != y {
                    self.point(center.x - y, center.y + x, color);
                    self.point(center.x + y, center.y - x, color);
                }
                self.point(center.x - x, center.y + y, color);
                self.point(center.x + x, center.y - y, color);
            }
            if x != y {
                self.point(center.x + y, center.y + x, color);
                self.point(center.x - y, center.y - x, color);
            }
            self.point(center.x + x, center.y + y, color);
            self.point(center.x - x, center.y - y, color);
            y += 1;
            if e >= 0 {
                x -= 1;
                e -= x;
            }
            e += y;
        }
    }
    pub fn circle_aa(&self, center: Offset, radius: u32, color: Rgba) {
        let rmin = (radius * (radius - 2)) as i32;
        let rmax = (radius * (radius + 2)) as i32;
        for y in 0..=radius as i32 {
            let sqy = y * y;
            for x in 0..=radius as i32 {
                let sqd = x * x + sqy;
                if sqd < rmax && sqd >= (radius * radius) as i32 {
                    let mut c = rmax - sqd;
                    c *= 256;
                    c /= 2 * radius as i32;
                    if c > 255 {
                        c = 255
                    };
                    quadrant!(self.point(center.x, center.y, x, y, color.set_a(c as u8)));
                } else if sqd < (radius * radius) as i32 && sqd >= rmin {
                    let mut c = sqd - rmin;
                    c *= 256;
                    c /= 2 * radius as i32;
                    if c > 255 {
                        c = 255
                    };
                    quadrant!(self.point(center.x, center.y, x, y, color.set_a(c as u8)));
                }
            }
        }
    }
    pub fn fill_circle(&self, center: Offset, radius: u32, color: Rgba) {
        for y in 0..=radius as i32 {
            let sqy = y * y;
            for x in 0..=radius as i32 {
                let sqd = x * x + sqy;
                if sqd <= (radius * (1 + radius)) as i32 {
                    quadrant!(self.point(center.x, center.y, x, y, color));
                }
            }
        }
    }
    pub fn fill_circle_aa(&self, center: Offset, radius: u32, color: Rgba) {
        let rmin = (radius * radius) as i32;
        let rmax = (radius * (radius + 2)) as i32;
        for y in 0..=radius as i32 {
            let sqy = y * y;
            for x in 0..=radius as i32 {
                let sqd = x * x + sqy;
                if sqd < rmin {
                    quadrant!(self.point(center.x, center.y, x, y, color));
                } else if sqd < rmax {
                    let mut c = rmax - sqd;
                    c *= 256;
                    c /= 2 * radius as i32;
                    if c > 255 {
                        c = 255
                    };
                    quadrant!(self.point(center.x, center.y, x, y, color.set_a(c as u8)));
                }
            }
        }
    }
    pub fn rect(&self, rect: Rect, color: Rgba) {
        let p1 = rect.offset();
        let p3 = rect.offset_2();

        for x in p1.x..=p3.x {
            self.point(x, p1.y, color);
            self.point(x, p3.y, color);
        }
        for y in p1.y + 1..p3.y {
            self.point(p1.x, y, color);
            self.point(p3.x, y, color);
        }
    }
    pub fn round_rect(&self, rect: Rect, radius: u32, color: Rgba) {
        let p1 = rect.offset();
        let p3 = rect.offset_2();

        let p1_c = Offset {
            x: p1.x + radius as i32,
            y: p1.y + radius as i32,
        };
        let p3_c = Offset {
            x: p3.x - radius as i32,
            y: p3.y - radius as i32,
        };

        for x in p1_c.x + 1..p3_c.x {
            self.point(x, p1.y, color);
            self.point(x, p3.y, color);
        }
        for y in p1_c.y + 1..p3_c.y {
            self.point(p1.x, y, color);
            self.point(p3.x, y, color);
        }

        let mut e = (1 - radius as i32) / 2;
        let mut x = radius as i32;
        let mut y = 0;
        while x >= y {
            if x != y {
                self.point(p3_c.x + y, p3_c.y + x, color);
                self.point(p1_c.x - y, p3_c.y + x, color);
                self.point(p3_c.x + y, p1_c.y - x, color);
                self.point(p1_c.x - y, p1_c.y - x, color);
            }
            self.point(p3_c.x + x, p3_c.y + y, color);
            self.point(p1_c.x - x, p3_c.y + y, color);
            self.point(p3_c.x + x, p1_c.y - y, color);
            self.point(p1_c.x - x, p1_c.y - y, color);
            y += 1;
            if e >= 0 {
                x -= 1;
                e -= x;
            }
            e += y;
        }
    }
    pub fn round_rect_aa(&self, rect: Rect, radius: u32, color: Rgba) {
        let rmin = (radius * (radius - 2)) as i32;
        let rmax = if radius == 0 {
            1
        } else {
            radius * (radius + 2)
        } as i32;

        let p1 = rect.offset();
        let p3 = rect.offset_2();

        let p1_c = Offset {
            x: p1.x + radius as i32,
            y: p1.y + radius as i32,
        };
        let p3_c = Offset {
            x: p3.x - radius as i32,
            y: p3.y - radius as i32,
        };

        for x in p1_c.x + 1..p3_c.x {
            self.point(x, p1.y, color);
            self.point(x, p3.y, color);
        }
        for y in p1_c.y + 1..p3_c.y {
            self.point(p1.x, y, color);
            self.point(p3.x, y, color);
        }

        for y in 0..=radius as i32 {
            let sqy = y * y;
            for x in 0..=radius as i32 {
                let sqd = x * x + sqy;
                if sqd < rmax && sqd >= (radius * radius) as i32 {
                    let mut c = rmax - sqd;
                    c *= 256;
                    c /= 2 * radius as i32;
                    if c > 255 {
                        c = 255
                    };
                    self.point(p1_c.x - x, p1_c.y - y, color.set_a(c as u8));
                    self.point(p3_c.x + x, p1_c.y - y, color.set_a(c as u8));
                    self.point(p1_c.x - x, p3_c.y + y, color.set_a(c as u8));
                    self.point(p3_c.x + x, p3_c.y + y, color.set_a(c as u8));
                } else if sqd < (radius * radius) as i32 && sqd >= rmin {
                    let mut c = sqd - rmin;
                    c *= 256;
                    c /= 2 * radius as i32;
                    if c > 255 {
                        c = 255
                    };
                    self.point(p1_c.x - x, p1_c.y - y, color.set_a(c as u8));
                    self.point(p3_c.x + x, p1_c.y - y, color.set_a(c as u8));
                    self.point(p1_c.x - x, p3_c.y + y, color.set_a(c as u8));
                    self.point(p3_c.x + x, p3_c.y + y, color.set_a(c as u8));
                }
            }
        }
    }
    pub fn fill_round_rect(&self, rect: Rect, radius: u32, color: Rgba) {
        let p1 = rect.offset();
        let p3 = rect.offset_2();

        let p1_c = Offset {
            x: p1.x + radius as i32,
            y: p1.y + radius as i32,
        };
        let p3_c = Offset {
            x: p3.x - radius as i32,
            y: p3.y - radius as i32,
        };

        for y in p1_c.y + 1..p3_c.y {
            for x in p1.x..=p3.x {
                self.point(x, y, color);
            }
        }
        for x in p1_c.x + 1..p3_c.x {
            for y in p1.y..=p1_c.y {
                self.point(x, y, color);
            }
            for y in p3_c.y..=p3.y {
                self.point(x, y, color);
            }
        }

        for y in 0..=radius as i32 {
            let sqy = y * y;
            for x in 0..=radius as i32 {
                let sqd = x * x + sqy;
                if sqd <= (radius * (2 + radius)) as i32 {
                    self.point(p1_c.x - x, p1_c.y - y, color);
                    self.point(p3_c.x + x, p1_c.y - y, color);
                    self.point(p1_c.x - x, p3_c.y + y, color);
                    self.point(p3_c.x + x, p3_c.y + y, color);
                }
            }
        }
    }
    pub fn fill_round_rect_aa(&self, rect: Rect, radius: u32, color: Rgba) {
        let rmin = if radius == 0 { 1 } else { radius * radius } as i32;
        let rmax = (radius * (radius + 2)) as i32;

        let p1 = rect.offset();
        let p3 = rect.offset_2();

        let p1_c = Offset {
            x: p1.x + radius as i32,
            y: p1.y + radius as i32,
        };
        let p3_c = Offset {
            x: p3.x - radius as i32,
            y: p3.y - radius as i32,
        };

        for y in p1_c.y + 1..p3_c.y {
            for x in p1.x..=p3.x {
                self.point(x, y, color);
            }
        }
        for x in p1_c.x + 1..p3_c.x {
            for y in p1.y..=p1_c.y {
                self.point(x, y, color);
            }
            for y in p3_c.y..=p3.y {
                self.point(x, y, color);
            }
        }
        for y in 0..=radius as i32 {
            let sqy = y * y;
            for x in 0..=radius as i32 {
                let sqd = x * x + sqy;
                if sqd < rmin {
                    self.point(p1_c.x - x, p1_c.y - y, color);
                    self.point(p3_c.x + x, p1_c.y - y, color);
                    self.point(p1_c.x - x, p3_c.y + y, color);
                    self.point(p3_c.x + x, p3_c.y + y, color);
                } else if sqd < rmax {
                    let mut c = rmax - sqd;
                    c *= 256;
                    c /= 2 * radius as i32;
                    if c > 255 {
                        c = 255
                    };
                    self.point(p1_c.x - x, p1_c.y - y, color.set_a(c as u8));
                    self.point(p3_c.x + x, p1_c.y - y, color.set_a(c as u8));
                    self.point(p1_c.x - x, p3_c.y + y, color.set_a(c as u8));
                    self.point(p3_c.x + x, p3_c.y + y, color.set_a(c as u8));
                }
            }
        }
    }
}
