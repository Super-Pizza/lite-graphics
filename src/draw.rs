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
            let (r, g, b, a) = (r as i16, g as i16, b as i16, a as i16);
            pixel_range[0] = ((r - pixel_range[0] as i16) * a / 255 + pixel_range[0] as i16) as u8;
            pixel_range[1] = ((g - pixel_range[1] as i16) * a / 255 + pixel_range[1] as i16) as u8;
            pixel_range[2] = ((b - pixel_range[2] as i16) * a / 255 + pixel_range[2] as i16) as u8;
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
}
