use std::{cell::RefCell, mem, rc::Rc};

use crate::{Offset, Rect};

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
    pub const RED: Self = Self {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const GREEN: Self = Self {
        r: 0,
        g: 255,
        b: 0,
        a: 255,
    };
    pub const BLUE: Self = Self {
        r: 0,
        g: 0,
        b: 255,
        a: 255,
    };
    pub const BLACK: Self = Self {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const WHITE: Self = Self {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };
    pub fn set_a(self, a: u8) -> Self {
        Self {
            r: self.r,
            g: self.g,
            b: self.b,
            a: (a as u16 * self.a as u16 / 255) as u8,
        }
    }
}

pub struct Buffer {
    pub(crate) data: RefCell<Vec<u8>>,
    pub(crate) width: usize,
    pub(crate) height: usize,
}

impl Buffer {
    /// Creates a buffer, filled with black
    pub fn new(width: usize, height: usize) -> Rc<Self> {
        Rc::new(Self {
            data: RefCell::new(vec![255; width * height * 3]),
            width,
            height,
        })
    }
    /// Draws a point of the specified color
    pub fn point(&self, x: i32, y: i32, color: Rgba) {
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
        let p1 = Offset {
            x: rect.x.max(0),
            y: rect.y.max(0),
        };
        let p2 = Offset {
            x: (self.width as i32).min(rect.w + rect.x),
            y: (self.height as i32).min(rect.h + rect.y),
        };
        let [r, g, b, a] = color.into();

        let (x1, y1, x2, y2) = (p1.x as usize, p1.y as usize, p2.x as usize, p2.y as usize);
        for row in y1..y2 {
            let pixel_range = &mut self.data.borrow_mut()
                [(x1 + row * self.width) * 3..(x2 + row * self.width) * 3];
            if a == 255 {
                pixel_range.copy_from_slice(&[r, g, b].repeat(x2 - x1));
            } else {
                for i in x1..x2 {
                    let (r, g, b, a) = (r as i16, g as i16, b as i16, a as i16);
                    let r_i = (i - x1) * 3;
                    let g_i = r_i + 1;
                    let b_i = r_i + 2;
                    pixel_range[r_i] =
                        ((r - pixel_range[r_i] as i16) * a / 255 + pixel_range[r_i] as i16) as u8;
                    pixel_range[g_i] =
                        ((g - pixel_range[g_i] as i16) * a / 255 + pixel_range[g_i] as i16) as u8;
                    pixel_range[b_i] =
                        ((b - pixel_range[b_i] as i16) * a / 255 + pixel_range[b_i] as i16) as u8;
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
    /// NOTE: this isn't a perfect circle, but it's very efficient.
    pub fn circle(&self, center: Offset, radius: i32, color: Rgba) {
        let mut e = (1 - radius) / 2;
        let mut x = radius;
        let mut y = 0;
        while x >= y {
            self.point(center.x + x, center.y + y, color);
            self.point(center.x + y, center.y + x, color);
            self.point(center.x - x, center.y + y, color);
            self.point(center.x - y, center.y + x, color);
            self.point(center.x + x, center.y - y, color);
            self.point(center.x + y, center.y - x, color);
            self.point(center.x - x, center.y - y, color);
            self.point(center.x - y, center.y - x, color);
            y += 1;
            if e >= 0 {
                x -= 1;
                e -= x;
            }
            e += y;
        }
    }
    pub fn circle_aa(&self, center: Offset, radius: i32, color: Rgba) {
        let rmin = radius * (radius - 2);
        let rmax = radius * (radius + 2);
        for y in center.y - radius..=center.y + radius {
            let sqy = (y - center.y) * (y - center.y);
            for x in center.x - radius..=center.x + radius {
                let sqd = (x - center.x) * (x - center.x) + sqy;
                if sqd < rmax && sqd >= radius * radius {
                    let mut c = rmax - sqd;
                    c *= 256;
                    c /= 2 * radius;
                    if c > 255 {
                        c = 255
                    };
                    self.point(x, y, color.set_a(c as u8));
                } else if sqd < radius * radius && sqd >= rmin {
                    let mut c = sqd - rmin;
                    c *= 256;
                    c /= 2 * radius;
                    if c > 255 {
                        c = 255
                    };
                    self.point(x, y, color.set_a(c as u8));
                }
            }
        }
    }
    pub fn fill_circle(&self, center: Offset, radius: i32, color: Rgba) {
        for y in center.y - radius..=center.y + radius {
            let sqy = (y - center.y) * (y - center.y);
            for x in center.x - radius..=center.x + radius {
                let sqd = (x - center.x) * (x - center.x) + sqy;
                if sqd <= radius * radius {
                    self.point(x, y, color);
                }
            }
        }
    }
    pub fn fill_circle_aa(&self, center: Offset, radius: i32, color: Rgba) {
        let rmin = radius * (radius - 1);
        let rmax = radius * (radius + 1);
        for y in center.y - radius..=center.y + radius {
            let sqy = (y - center.y) * (y - center.y);
            for x in center.x - radius..=center.x + radius {
                let sqd = (x - center.x) * (x - center.x) + sqy;
                if sqd < rmin {
                    self.point(x, y, color);
                } else if sqd < rmax {
                    let mut c = rmax - sqd;
                    c *= 256;
                    c /= 2 * radius;
                    if c > 255 {
                        c = 255
                    };
                    self.point(x, y, color.set_a(c as u8));
                }
            }
        }
    }
    pub fn rect(&self, rect: Rect, color: Rgba) {
        let p1 = Offset {
            x: rect.x,
            y: rect.y,
        };
        let p3 = Offset {
            x: rect.w + rect.x,
            y: rect.h + rect.y,
        };

        for x in p1.x..=p3.x {
            self.point(x, p1.y, color);
            self.point(x, p3.y, color);
        }
        for y in p1.y + 1..p3.y {
            self.point(p1.x, y, color);
            self.point(p3.x, y, color);
        }
    }
    pub fn round_rect(&self, rect: Rect, radius: i32, color: Rgba) {
        let p1 = Offset {
            x: rect.x,
            y: rect.y,
        };
        let p3 = Offset {
            x: rect.w + rect.x,
            y: rect.h + rect.y,
        };
        for x in p1.x + radius + 1..p3.x - radius {
            self.point(x, p1.y, color);
            self.point(x, p3.y, color);
        }
        for y in p1.y + radius + 1..p3.y - radius {
            self.point(p1.x, y, color);
            self.point(p3.x, y, color);
        }
        let p1_c = Offset {
            x: p1.x + radius,
            y: p1.y + radius,
        };
        let p3_c = Offset {
            x: p3.x - radius,
            y: p3.y - radius,
        };
        let mut e = (1 - radius) / 2;
        let mut x = radius;
        let mut y = 0;
        while x >= y {
            self.point(p3_c.x + x, p3_c.y + y, color);
            self.point(p3_c.x + y, p3_c.y + x, color);
            self.point(p1_c.x - x, p3_c.y + y, color);
            self.point(p1_c.x - y, p3_c.y + x, color);
            self.point(p3_c.x + x, p1_c.y - y, color);
            self.point(p3_c.x + y, p1_c.y - x, color);
            self.point(p1_c.x - x, p1_c.y - y, color);
            self.point(p1_c.x - y, p1_c.y - x, color);
            y += 1;
            if e >= 0 {
                x -= 1;
                e -= x;
            }
            e += y;
        }
    }
    pub fn round_rect_aa(&self, rect: Rect, radius: i32, color: Rgba) {
        let rmin = radius * (radius - 2);
        let rmax = radius * (radius + 2);
        let plot = |x: i32, y: i32, sqd: i32| {
            if sqd < rmax && sqd >= radius * radius {
                let mut c = rmax - sqd;
                c *= 256;
                c /= 2 * radius;
                if c > 255 {
                    c = 255
                };
                self.point(x, y, color.set_a(c as u8));
            } else if sqd < radius * radius && sqd >= rmin {
                let mut c = sqd - rmin;
                c *= 256;
                c /= 2 * radius;
                if c > 255 {
                    c = 255
                };
                self.point(x, y, color.set_a(c as u8));
            }
        };
        let p1 = Offset {
            x: rect.x,
            y: rect.y,
        };
        let p3 = Offset {
            x: rect.w + rect.x,
            y: rect.h + rect.y,
        };
        for x in p1.x + radius + 1..p3.x - radius {
            self.point(x, p1.y, color);
            self.point(x, p3.y, color);
        }
        for y in p1.y + radius + 1..p3.y - radius {
            self.point(p1.x, y, color);
            self.point(p3.x, y, color);
        }
        let p1_c = Offset {
            x: p1.x + radius,
            y: p1.y + radius,
        };
        let p3_c = Offset {
            x: p3.x - radius,
            y: p3.y - radius,
        };
        for y in p1_c.y - radius..=p1_c.y {
            let sqy = (y - p1_c.y) * (y - p1_c.y);
            for x in p1_c.x - radius..=p1_c.x {
                let sqd = (x - p1_c.x) * (x - p1_c.x) + sqy;
                plot(x, y, sqd)
            }
        }
        for y in p3_c.y..=p3_c.y + radius {
            let sqy = (y - p3_c.y) * (y - p3_c.y);
            for x in p1_c.x - radius..=p1_c.x {
                let sqd = (x - p1_c.x) * (x - p1_c.x) + sqy;
                plot(x, y, sqd)
            }
        }
        for y in p1_c.y - radius..=p1_c.y {
            let sqy = (y - p1_c.y) * (y - p1_c.y);
            for x in p3_c.x..=p3_c.x + radius {
                let sqd = (x - p3_c.x) * (x - p3_c.x) + sqy;
                plot(x, y, sqd)
            }
        }
        for y in p3_c.y..=p3_c.y + radius {
            let sqy = (y - p3_c.y) * (y - p3_c.y);
            for x in p3_c.x..=p3_c.x + radius {
                let sqd = (x - p3_c.x) * (x - p3_c.x) + sqy;
                plot(x, y, sqd)
            }
        }
    }
    pub fn fill_round_rect(&self, rect: Rect, radius: i32, color: Rgba) {
        let plot = |x: i32, y: i32, sqd: i32| {
            if sqd <= radius * (2 + radius) {
                self.point(x, y, color);
            }
        };
        let p1 = Offset {
            x: rect.x,
            y: rect.y,
        };
        let p3 = Offset {
            x: rect.w + rect.x,
            y: rect.h + rect.y,
        };

        let p1_c = Offset {
            x: p1.x + radius,
            y: p1.y + radius,
        };
        let p3_c = Offset {
            x: p3.x - radius,
            y: p3.y - radius,
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
        for y in p1_c.y - radius..=p1_c.y {
            let sqy = (y - p1_c.y) * (y - p1_c.y);
            for x in p1_c.x - radius..=p1_c.x {
                let sqd = (x - p1_c.x) * (x - p1_c.x) + sqy;
                plot(x, y, sqd)
            }
        }
        for y in p3_c.y..=p3_c.y + radius {
            let sqy = (y - p3_c.y) * (y - p3_c.y);
            for x in p1_c.x - radius..=p1_c.x {
                let sqd = (x - p1_c.x) * (x - p1_c.x) + sqy;
                plot(x, y, sqd)
            }
        }
        for y in p1_c.y - radius..=p1_c.y {
            let sqy = (y - p1_c.y) * (y - p1_c.y);
            for x in p3_c.x..=p3_c.x + radius {
                let sqd = (x - p3_c.x) * (x - p3_c.x) + sqy;
                plot(x, y, sqd)
            }
        }
        for y in p3_c.y..=p3_c.y + radius {
            let sqy = (y - p3_c.y) * (y - p3_c.y);
            for x in p3_c.x..=p3_c.x + radius {
                let sqd = (x - p3_c.x) * (x - p3_c.x) + sqy;
                plot(x, y, sqd)
            }
        }
    }
    pub fn fill_round_rect_aa(&self, rect: Rect, radius: i32, color: Rgba) {
        let rmin = radius * (radius);
        let rmax = radius * (radius + 2);
        let plot = |x: i32, y: i32, sqd: i32| {
            if sqd < rmin {
                self.point(x, y, color);
            } else if sqd < rmax {
                let mut c = rmax - sqd;
                c *= 256;
                c /= 2 * radius;
                if c > 255 {
                    c = 255
                };
                self.point(x, y, color.set_a(c as u8));
            }
        };
        let p1 = Offset {
            x: rect.x,
            y: rect.y,
        };
        let p3 = Offset {
            x: rect.w + rect.x,
            y: rect.h + rect.y,
        };

        let p1_c = Offset {
            x: p1.x + radius,
            y: p1.y + radius,
        };
        let p3_c = Offset {
            x: p3.x - radius,
            y: p3.y - radius,
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
        for y in p1_c.y - radius..=p1_c.y {
            let sqy = (y - p1_c.y) * (y - p1_c.y);
            for x in p1_c.x - radius..=p1_c.x {
                let sqd = (x - p1_c.x) * (x - p1_c.x) + sqy;
                plot(x, y, sqd)
            }
        }
        for y in p3_c.y..=p3_c.y + radius {
            let sqy = (y - p3_c.y) * (y - p3_c.y);
            for x in p1_c.x - radius..=p1_c.x {
                let sqd = (x - p1_c.x) * (x - p1_c.x) + sqy;
                plot(x, y, sqd)
            }
        }
        for y in p1_c.y - radius..=p1_c.y {
            let sqy = (y - p1_c.y) * (y - p1_c.y);
            for x in p3_c.x..=p3_c.x + radius {
                let sqd = (x - p3_c.x) * (x - p3_c.x) + sqy;
                plot(x, y, sqd)
            }
        }
        for y in p3_c.y..=p3_c.y + radius {
            let sqy = (y - p3_c.y) * (y - p3_c.y);
            for x in p3_c.x..=p3_c.x + radius {
                let sqd = (x - p3_c.x) * (x - p3_c.x) + sqy;
                plot(x, y, sqd)
            }
        }
    }
}
