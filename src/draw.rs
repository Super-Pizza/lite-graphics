use core::f32::consts::{FRAC_PI_2 as PI_2_32, PI as PI32, TAU as TAU32};
use std::{cell::RefCell, mem, rc::Rc};

use crate::{Offset, Rect, Size};

use crate::color::Color;

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
    pub fn point(&self, x: i32, y: i32, color: &impl Color) {
        let x = x + self.offs.x;
        let y = y + self.offs.y;
        if x < 0 || y < 0 || x as usize >= self.width || y as usize >= self.height {
            return;
        }
        let [r, g, b, a] = color.get(Offset { x, y }).into();
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
    pub fn fill_rect(&self, rect: Rect, color: impl Color) {
        let rect = rect + self.offs;
        let rect = rect.clamp(Size::from((self.width as u32, self.height as u32)).into());
        let p1 = rect.offset();
        let p2 = rect.offset_2();

        let (x1, y1, x2, y2) = (p1.x as usize, p1.y as usize, p2.x as usize, p2.y as usize);
        for row in y1..y2 {
            let pixel_range = &mut self.data.borrow_mut()
                [(x1 + row * self.width) * 3..(x2 + row * self.width) * 3];
            for i in x1..x2 {
                let [r, g, b, a] = color
                    .get(Offset {
                        x: i as i32,
                        y: row as i32,
                    })
                    .into();
                let r_i = (i - x1) * 3;
                let g_i = r_i + 1;
                let b_i = r_i + 2;
                if a == 255 {
                    pixel_range[r_i..r_i + 3].copy_from_slice(&[r, g, b]);
                } else {
                    let (r, g, b, a) = (r as i32, g as i32, b as i32, a as i32);
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
    pub fn line(&self, mut p1: Offset, mut p2: Offset, color: impl Color) {
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
                self.point(y, x, &color);
            } else {
                self.point(x, y, &color);
            }
            if d > 0 {
                y += yi;
                d += 2 * (dy - dx);
            } else {
                d += 2 * dy;
            }
        }
    }
    pub fn line_aa(&self, mut p1: Offset, mut p2: Offset, color: impl Color) {
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
            self.point(ypxl1 as i32, xpxl1 as i32, &color);
        } else {
            self.point(xpxl1 as i32, ypxl1 as i32, &color);
        }
        let mut intery = yend + gradient;

        // handle second endpoint
        let xend = p2.x as f32;
        let yend = p2.y as f32;
        let xpxl2 = xend;
        let ypxl2 = yend;
        if steep {
            self.point(ypxl2 as i32, xpxl2 as i32, &color);
        } else {
            self.point(xpxl2 as i32, ypxl2 as i32, &color);
        }

        // main loop
        if steep {
            for x in xpxl1 as i32 + 1..xpxl2 as i32 {
                self.point(
                    intery as i32,
                    x,
                    &color.set_a(255 - (intery.fract() * 255.0) as u8),
                );
                self.point(
                    intery as i32 + 1,
                    x,
                    &color.set_a((intery.fract() * 255.0) as u8),
                );
                intery += gradient;
            }
        } else {
            for x in xpxl1 as i32 + 1..xpxl2 as i32 {
                self.point(
                    x,
                    intery as i32,
                    &color.set_a(255 - (intery.fract() * 255.0) as u8),
                );
                self.point(
                    x,
                    intery as i32 + 1,
                    &color.set_a((intery.fract() * 255.0) as u8),
                );
                intery += gradient;
            }
        }
    }
    pub fn line_h(&self, p1: Offset, length: i32, color: impl Color) {
        let size = Size {
            w: length as _,
            h: 1,
        };
        let rect = Rect::from((p1 + self.offs, size))
            .clamp(Size::from((self.width as u32, self.height as u32)).into());

        let (x1, y1) = (rect.x as usize, rect.y as usize);
        let x2 = rect.offset_2().x as usize;
        let pixel_range =
            &mut self.data.borrow_mut()[(x1 + y1 * self.width) * 3..(x2 + y1 * self.width) * 3];
        for i in x1..x2 {
            let [r, g, b, a] = color
                .get(Offset {
                    x: i as i32,
                    y: y1 as i32,
                })
                .into();
            let r_i = (i - x1) * 3;
            let g_i = r_i + 1;
            let b_i = r_i + 2;
            if a == 255 {
                pixel_range[r_i..r_i + 3].copy_from_slice(&[r, g, b]);
            } else {
                let (r, g, b, a) = (r as i32, g as i32, b as i32, a as i32);
                pixel_range[r_i] =
                    ((r - pixel_range[r_i] as i32) * a / 255 + pixel_range[r_i] as i32) as u8;
                pixel_range[g_i] =
                    ((g - pixel_range[g_i] as i32) * a / 255 + pixel_range[g_i] as i32) as u8;
                pixel_range[b_i] =
                    ((b - pixel_range[b_i] as i32) * a / 255 + pixel_range[b_i] as i32) as u8;
            }
        }
    }
    pub fn line_v(&self, p1: Offset, length: i32, color: impl Color) {
        let size = Size {
            w: 1,
            h: length as _,
        };
        let rect = Rect::from((p1 + self.offs, size))
            .clamp(Size::from((self.width as u32, self.height as u32)).into());

        let (x1, y1) = (rect.x as usize, rect.y as usize);
        let y2 = rect.offset_2().y as usize;
        for row in y1..y2 {
            let [r, g, b, a] = color
                .get(Offset {
                    x: x1 as i32,
                    y: row as i32,
                })
                .into();
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
    pub fn circle(&self, center: Offset, radius: u32, color: impl Color) {
        let mut e = (1 - radius as i32) / 2;
        let mut x = radius as i32;
        let mut y = 0;
        while x >= y {
            if y != 0 {
                if x != y {
                    self.point(center.x - y, center.y + x, &color);
                    self.point(center.x + y, center.y - x, &color);
                }
                self.point(center.x - x, center.y + y, &color);
                self.point(center.x + x, center.y - y, &color);
            }
            if x != y {
                self.point(center.x + y, center.y + x, &color);
                self.point(center.x - y, center.y - x, &color);
            }
            self.point(center.x + x, center.y + y, &color);
            self.point(center.x - x, center.y - y, &color);
            y += 1;
            if e >= 0 {
                x -= 1;
                e -= x;
            }
            e += y;
        }
    }
    pub fn circle_aa(&self, center: Offset, radius: u32, color: impl Color) {
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
                    quadrant!(self.point(center.x, center.y, x, y, &color.set_a(c as u8)));
                } else if sqd < (radius * radius) as i32 && sqd >= rmin {
                    let mut c = sqd - rmin;
                    c *= 256;
                    c /= 2 * radius as i32;
                    if c > 255 {
                        c = 255
                    };
                    quadrant!(self.point(center.x, center.y, x, y, &color.set_a(c as u8)));
                }
            }
        }
    }
    /// Draws a circle arc from angle1 to angle2 in radians, with positive angles measured counterclockwise from positive x axis.
    pub fn circle_arc(
        &self,
        center: Offset,
        radius: u32,
        angle1: f32,
        angle2: f32,
        color: impl Color,
    ) {
        let (angle1, angle2) = if angle2 < angle1 % TAU32 {
            // Corner case where the arc overlaps angle 0.
            self.circle_arc(center, radius, angle1, TAU32, color.clone());
            self.circle_arc(center, radius, 0., angle2, color);
            return;
        } else if angle2 - angle1 >= TAU32 {
            (0., TAU32)
        } else {
            // The subtractions make zero and TAU become TAU, not zero.
            (angle1 % TAU32, TAU32 - (TAU32 - angle2) % TAU32)
        };

        let mut e = (1 - radius as i32) / 2;
        let mut x = radius as i32;
        let mut y = 0;
        while x >= y {
            let angle = (y as f32).atan2(x as f32);
            if y != 0 {
                if x != y {
                    if angle1 < 3. * PI_2_32 - angle && 3. * PI_2_32 - angle <= angle2 {
                        self.point(center.x - y, center.y + x, &color);
                    }
                    if angle1 < PI_2_32 - angle && PI_2_32 - angle <= angle2 {
                        self.point(center.x + y, center.y - x, &color);
                    }
                }
                if angle1 < angle + PI32 && angle + PI32 <= angle2 {
                    self.point(center.x - x, center.y + y, &color);
                }
                if angle1 < angle && angle <= angle2 {
                    self.point(center.x + x, center.y - y, &color);
                }
            }
            if x != y {
                if angle1 < angle + 3. * PI_2_32 && angle + 3. * PI_2_32 <= angle2 {
                    self.point(center.x + y, center.y + x, &color);
                }
                if angle1 < angle + PI_2_32 && angle + PI_2_32 <= angle2 {
                    self.point(center.x - y, center.y - x, &color);
                }
            }
            if angle1 < TAU32 - angle && TAU32 - angle <= angle2 {
                self.point(center.x + x, center.y + y, &color);
            }
            if angle1 < PI32 - angle && PI32 - angle <= angle2 {
                self.point(center.x - x, center.y - y, &color);
            }

            y += 1;
            if e >= 0 {
                x -= 1;
                e -= x;
            }
            e += y;
        }
    }
    /// Draws a circle arc from angle1 to angle2 in radians, with positive angles measured counterclockwise from positive x axis.
    pub fn circle_arc_aa(
        &self,
        center: Offset,
        radius: u32,
        angle1: f32,
        angle2: f32,
        color: impl Color,
    ) {
        let (angle1, angle2) = if angle2 < angle1 % TAU32 {
            // Corner case where the arc overlaps angle 0.
            self.circle_arc_aa(center, radius, angle1, TAU32, color.clone());
            self.circle_arc_aa(center, radius, 0., angle2, color);
            return;
        } else if angle2 - angle1 >= TAU32 {
            (0., TAU32)
        } else {
            // The subtractions make zero and TAU become TAU, not zero.
            (angle1 % TAU32, TAU32 - (TAU32 - angle2) % TAU32)
        };

        let rmin = (radius * (radius - 2)) as i32;
        let rmax = (radius * (radius + 2)) as i32;
        for y in 0..=radius as i32 {
            let sqy = y * y;
            for x in 0..=radius as i32 {
                let angle = (y as f32).atan2(x as f32);

                let sqd = x * x + sqy;
                let mut c = if sqd < rmax && sqd >= (radius * radius) as i32 {
                    rmax - sqd
                } else if sqd < (radius * radius) as i32 && sqd >= rmin {
                    sqd - rmin
                } else {
                    continue;
                };
                c *= 256;
                c /= 2 * radius as i32;
                if c > 255 {
                    c = 255
                };
                if x != 0 {
                    if y != 0 && angle1 < PI32 - angle && PI32 - angle <= angle2 {
                        self.point(center.x - x, center.y - y, &color.set_a(c as u8));
                    }
                    if angle1 < angle + PI32 && angle + PI32 <= angle2 {
                        self.point(center.x - x, center.y + y, &color.set_a(c as u8));
                    }
                }
                if y != 0 && angle1 < angle && angle <= angle2 {
                    self.point(center.x + x, center.y - y, &color.set_a(c as u8));
                }
                if angle1 < TAU32 - angle && TAU32 - angle <= angle2 {
                    self.point(center.x + x, center.y + y, &color.set_a(c as u8));
                }
            }
        }
    }
    pub fn fill_circle(&self, center: Offset, radius: u32, color: impl Color) {
        for y in 0..=radius as i32 {
            let sqy = y * y;
            for x in 0..=radius as i32 {
                let sqd = x * x + sqy;
                if sqd <= (radius * (1 + radius)) as i32 {
                    quadrant!(self.point(center.x, center.y, x, y, &color));
                }
            }
        }
    }
    pub fn fill_circle_aa(&self, center: Offset, radius: u32, color: impl Color) {
        let rmin = (radius * radius) as i32;
        let rmax = (radius * (radius + 2)) as i32;
        for y in 0..=radius as i32 {
            let sqy = y * y;
            for x in 0..=radius as i32 {
                let sqd = x * x + sqy;
                if sqd < rmin {
                    quadrant!(self.point(center.x, center.y, x, y, &color));
                } else if sqd < rmax {
                    let mut c = rmax - sqd;
                    c *= 256;
                    c /= 2 * radius as i32;
                    if c > 255 {
                        c = 255
                    };
                    quadrant!(self.point(center.x, center.y, x, y, &color.set_a(c as u8)));
                }
            }
        }
    }
    pub fn circle_pie(
        &self,
        center: Offset,
        radius: u32,
        angle1: f32,
        angle2: f32,
        color: impl Color,
    ) {
        let (angle1, angle2) = if angle2 < angle1 % TAU32 {
            // Corner case where the arc overlaps angle 0.
            self.circle_pie(center, radius, angle1, TAU32, color.clone());
            self.circle_pie(center, radius, 0., angle2, color);
            return;
        } else if angle2 - angle1 >= TAU32 {
            (0., TAU32)
        } else {
            // The subtractions make zero and TAU become TAU, not zero.
            (angle1 % TAU32, TAU32 - (TAU32 - angle2) % TAU32)
        };
        for y in 0..=radius as i32 {
            let sqy = y * y;
            for x in 0..=radius as i32 {
                let angle = (y as f32).atan2(x as f32);

                let sqd = x * x + sqy;
                if sqd > (radius * (1 + radius)) as i32 {
                    continue;
                }
                if x != 0 {
                    if y != 0 && angle1 < PI32 - angle && PI32 - angle <= angle2 {
                        self.point(center.x - x, center.y - y, &color);
                    }
                    if angle1 < angle + PI32 && angle + PI32 <= angle2 {
                        self.point(center.x - x, center.y + y, &color);
                    }
                }
                if y != 0 && angle1 < angle && angle <= angle2 {
                    self.point(center.x + x, center.y - y, &color);
                }
                if angle1 < TAU32 - angle && TAU32 - angle <= angle2 {
                    self.point(center.x + x, center.y + y, &color);
                }
            }
        }
    }
    pub fn circle_pie_aa(
        &self,
        center: Offset,
        radius: u32,
        angle1: f32,
        angle2: f32,
        color: impl Color,
    ) {
        let (angle1, angle2) = if angle2 < angle1 % TAU32 {
            // Corner case where the arc overlaps angle 0.
            self.circle_pie_aa(center, radius, angle1, TAU32, color.clone());
            self.circle_pie_aa(center, radius, 0., angle2, color);
            return;
        } else if angle2 - angle1 >= TAU32 {
            (0., TAU32)
        } else {
            // The subtractions make zero and TAU become TAU, not zero.
            (angle1 % TAU32, TAU32 - (TAU32 - angle2) % TAU32)
        };

        let rmin = (radius * radius) as i32;
        let rmax = (radius * (radius + 2)) as i32;
        for y in 0..=radius as i32 {
            let sqy = y * y;
            for x in 0..=radius as i32 {
                let angle = (y as f32).atan2(x as f32);

                let sqd = x * x + sqy;
                let c = if sqd < rmin {
                    255
                } else if sqd < rmax {
                    255.min((rmax - sqd) * 256 / (2 * radius as i32))
                } else {
                    continue;
                };

                let spacing = 1.0 / (sqd as f32).sqrt();

                let angle1_min = angle1 - spacing;
                let angle1_max = angle1 + spacing;
                let angle2_min = angle2 - spacing;
                let angle2_max = angle2 + spacing;

                let angle_ = if angle2_max > TAU32 {
                    angle + TAU32
                } else {
                    angle
                };

                let angle_grad = |angle: f32| {
                    let c1 = if angle1_max < angle && angle <= angle2_min {
                        1.
                    } else if angle > angle1_min && angle <= angle1_max {
                        (angle - angle1_min) / (2.0 * spacing)
                    } else if angle > angle2_min && angle <= angle2_max {
                        (angle2_max - angle) / (2.0 * spacing)
                    } else {
                        0.
                    };
                    c as f32 * c1
                };

                if x != 0 {
                    if y != 0 && angle1_min < PI32 - angle && PI32 - angle <= angle2_max {
                        self.point(
                            center.x - x,
                            center.y - y,
                            &color.set_a(angle_grad(PI32 - angle) as u8),
                        );
                    }
                    if angle1_min < angle + PI32 && angle + PI32 <= angle2_max {
                        self.point(
                            center.x - x,
                            center.y + y,
                            &color.set_a(angle_grad(PI32 + angle) as u8),
                        );
                    }
                }
                if y != 0 && angle1_min < TAU32 - angle && TAU32 - angle <= angle2_max {
                    self.point(
                        center.x + x,
                        center.y + y,
                        &color.set_a(angle_grad(TAU32 - angle) as u8),
                    );
                }
                if angle1_min < angle_ && angle_ <= angle2_max {
                    self.point(
                        center.x + x,
                        center.y - y,
                        &color.set_a(angle_grad(angle) as u8),
                    );
                }
            }
        }
    }
    pub fn rect(&self, rect: Rect, color: impl Color) {
        let p1 = rect.offset();
        let p3 = rect.offset_2();

        for x in p1.x..=p3.x {
            self.point(x, p1.y, &color);
            self.point(x, p3.y, &color);
        }
        for y in p1.y + 1..p3.y {
            self.point(p1.x, y, &color);
            self.point(p3.x, y, &color);
        }
    }
    pub fn round_rect(&self, rect: Rect, radius: u32, color: impl Color) {
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
            self.point(x, p1.y, &color);
            self.point(x, p3.y, &color);
        }
        for y in p1_c.y + 1..p3_c.y {
            self.point(p1.x, y, &color);
            self.point(p3.x, y, &color);
        }

        let mut e = (1 - radius as i32) / 2;
        let mut x = radius as i32;
        let mut y = 0;
        while x >= y {
            if x != y {
                self.point(p3_c.x + y, p3_c.y + x, &color);
                self.point(p1_c.x - y, p3_c.y + x, &color);
                self.point(p3_c.x + y, p1_c.y - x, &color);
                self.point(p1_c.x - y, p1_c.y - x, &color);
            }
            self.point(p3_c.x + x, p3_c.y + y, &color);
            self.point(p1_c.x - x, p3_c.y + y, &color);
            self.point(p3_c.x + x, p1_c.y - y, &color);
            self.point(p1_c.x - x, p1_c.y - y, &color);
            y += 1;
            if e >= 0 {
                x -= 1;
                e -= x;
            }
            e += y;
        }
    }
    pub fn round_rect_aa(&self, rect: Rect, radius: u32, color: impl Color) {
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
            self.point(x, p1.y, &color);
            self.point(x, p3.y, &color);
        }
        for y in p1_c.y + 1..p3_c.y {
            self.point(p1.x, y, &color);
            self.point(p3.x, y, &color);
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
                    self.point(p1_c.x - x, p1_c.y - y, &color.set_a(c as u8));
                    self.point(p3_c.x + x, p1_c.y - y, &color.set_a(c as u8));
                    self.point(p1_c.x - x, p3_c.y + y, &color.set_a(c as u8));
                    self.point(p3_c.x + x, p3_c.y + y, &color.set_a(c as u8));
                } else if sqd < (radius * radius) as i32 && sqd >= rmin {
                    let mut c = sqd - rmin;
                    c *= 256;
                    c /= 2 * radius as i32;
                    if c > 255 {
                        c = 255
                    };
                    self.point(p1_c.x - x, p1_c.y - y, &color.set_a(c as u8));
                    self.point(p3_c.x + x, p1_c.y - y, &color.set_a(c as u8));
                    self.point(p1_c.x - x, p3_c.y + y, &color.set_a(c as u8));
                    self.point(p3_c.x + x, p3_c.y + y, &color.set_a(c as u8));
                }
            }
        }
    }
    pub fn fill_round_rect(&self, rect: Rect, radius: u32, color: impl Color) {
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
                self.point(x, y, &color);
            }
        }
        for x in p1_c.x + 1..p3_c.x {
            for y in p1.y..=p1_c.y {
                self.point(x, y, &color);
            }
            for y in p3_c.y..=p3.y {
                self.point(x, y, &color);
            }
        }

        for y in 0..=radius as i32 {
            let sqy = y * y;
            for x in 0..=radius as i32 {
                let sqd = x * x + sqy;
                if sqd <= (radius * (2 + radius)) as i32 {
                    self.point(p1_c.x - x, p1_c.y - y, &color);
                    self.point(p3_c.x + x, p1_c.y - y, &color);
                    self.point(p1_c.x - x, p3_c.y + y, &color);
                    self.point(p3_c.x + x, p3_c.y + y, &color);
                }
            }
        }
    }
    pub fn fill_round_rect_aa(&self, rect: Rect, radius: u32, color: impl Color) {
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
                self.point(x, y, &color);
            }
        }
        for x in p1_c.x + 1..p3_c.x {
            for y in p1.y..=p1_c.y {
                self.point(x, y, &color);
            }
            for y in p3_c.y..=p3.y {
                self.point(x, y, &color);
            }
        }
        for y in 0..=radius as i32 {
            let sqy = y * y;
            for x in 0..=radius as i32 {
                let sqd = x * x + sqy;
                if sqd < rmin {
                    self.point(p1_c.x - x, p1_c.y - y, &color);
                    self.point(p3_c.x + x, p1_c.y - y, &color);
                    self.point(p1_c.x - x, p3_c.y + y, &color);
                    self.point(p3_c.x + x, p3_c.y + y, &color);
                } else if sqd < rmax {
                    let mut c = rmax - sqd;
                    c *= 256;
                    c /= 2 * radius as i32;
                    if c > 255 {
                        c = 255
                    };
                    self.point(p1_c.x - x, p1_c.y - y, &color.set_a(c as u8));
                    self.point(p3_c.x + x, p1_c.y - y, &color.set_a(c as u8));
                    self.point(p1_c.x - x, p3_c.y + y, &color.set_a(c as u8));
                    self.point(p3_c.x + x, p3_c.y + y, &color.set_a(c as u8));
                }
            }
        }
    }
}
