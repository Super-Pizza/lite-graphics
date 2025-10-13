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

pub trait Drawable {
    /// Get the buffer's size
    fn size(&self) -> Size;

    /// Limit the drawing to a subregion. Any further operations are relative to the subregion, including further subregions.
    fn subregion(&mut self, rect: Rect);

    /// Pop a subregion, keeping the one before as the new subregion.
    fn end_subregion(&mut self);

    fn get_subregion(&self) -> Rect;

    /// Draw one pixel.
    fn point(&self, x: i32, y: i32, color: &Color);

    /// Draws a filled rectangle.
    fn fill_rect(&self, rect: Rect, color: Color) {
        let p1 = rect.offset();
        let p2 = rect.offset_2();

        let (x1, y1, x2, y2) = (p1.x, p1.y, p2.x, p2.y);
        for y in y1..y2 {
            for x in x1..x2 {
                self.point(x, y, &color);
            }
        }
    }

    /// Draws an aliased line
    fn line(&self, mut p1: Offset, mut p2: Offset, color: Color) {
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

    /// Draws an antialiased line
    fn line_aa(&self, mut p1: Offset, mut p2: Offset, color: Color) {
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

    /// Draws a horizontal line
    fn line_h(&self, p1: Offset, length: i32, color: Color) {
        let size = Size {
            w: length as _,
            h: 1,
        };
        let rect = Rect::new(p1, size);

        let (x1, y) = (rect.x, rect.y);
        let x2 = rect.offset_2().x - 1;
        for x in x1..x2 {
            self.point(x, y, &color);
        }
    }

    /// Draws a vertical line
    fn line_v(&self, p1: Offset, length: i32, color: Color) {
        let size = Size {
            w: 1,
            h: length as _,
        };
        let rect = Rect::new(p1, size);

        let (x, y1) = (rect.x, rect.y);
        let y2 = rect.offset_2().y - 1;
        for y in y1..y2 {
            self.point(x, y, &color);
        }
    }

    /// Draws an aliased circle. Imperfect, but very fast.
    fn circle(&self, center: Offset, radius: u32, color: Color) {
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

    /// Draws an antialiased circle. Doesn't use square roots for efficiency.
    fn circle_aa(&self, center: Offset, radius: u32, color: Color) {
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

    /// Draws an aliased circle arc from angle1 to angle2 in radians, with positive angles measured counterclockwise from positive x axis.
    fn circle_arc(&self, center: Offset, radius: u32, angle1: f32, angle2: f32, color: Color) {
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
    /// Draws an antialiased circle arc from angle1 to angle2 in radians, with positive angles measured counterclockwise from positive x axis.
    fn circle_arc_aa(&self, center: Offset, radius: u32, angle1: f32, angle2: f32, color: Color) {
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

    /// Draws an aliased, filled circle.
    fn fill_circle(&self, center: Offset, radius: u32, color: Color) {
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

    /// Draws an antialiased, filled circle.
    fn fill_circle_aa(&self, center: Offset, radius: u32, color: Color) {
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

    /// Draws an aliased, filled circle pie.
    fn circle_pie(&self, center: Offset, radius: u32, angle1: f32, angle2: f32, color: Color) {
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

    /// Draws an antialiased, filled circle pie.
    fn circle_pie_aa(&self, center: Offset, radius: u32, angle1: f32, angle2: f32, color: Color) {
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

    /// Draws a rectangle border.
    fn rect(&self, rect: Rect, color: Color) {
        let p1 = rect.offset();
        let p3 = rect.offset_2() - Offset::new(1, 1);

        for x in p1.x..=p3.x {
            self.point(x, p1.y, &color);
            self.point(x, p3.y, &color);
        }
        for y in p1.y + 1..p3.y {
            self.point(p1.x, y, &color);
            self.point(p3.x, y, &color);
        }
    }

    /// Draws an aliased rounded rectangle border.
    fn round_rect(&self, rect: Rect, radius: u32, color: Color) {
        let p1 = rect.offset();
        let p3 = rect.offset_2() - Offset::new(1, 1);

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

    /// Draws an antialiased rounded rectangle border.
    fn round_rect_aa(&self, rect: Rect, radius: u32, color: Color) {
        let rmin = (radius * (radius - 2)) as i32;
        let rmax = if radius == 0 {
            1
        } else {
            radius * (radius + 2)
        } as i32;

        let p1 = rect.offset();
        let p3 = rect.offset_2() - Offset::new(1, 1);

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

    /// Draws a filled aliased rounded rectangle.
    fn fill_round_rect(&self, rect: Rect, radius: u32, color: Color) {
        let p1 = rect.offset();
        let p3 = rect.offset_2() - Offset::new(1, 1);

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

    /// Draws a filled antialiased rounded rectangle.
    fn fill_round_rect_aa(&self, rect: Rect, radius: u32, color: Color) {
        let rmin = if radius == 0 { 1 } else { radius * radius } as i32;
        let rmax = (radius * (radius + 2)) as i32;

        let p1 = rect.offset();
        let p3 = rect.offset_2() - Offset::new(1, 1);

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

pub struct Buffer {
    pub(crate) data: Rc<RefCell<Vec<u8>>>,
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) subregions: Vec<Rect>,
}

impl Buffer {
    /// Creates a buffer, filled with black
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            data: Rc::new(RefCell::new(vec![255; width * height * 3])),
            width,
            height,
            subregions: vec![Size::new(width as _, height as _).into()],
        }
    }
    pub fn data(&self) -> std::cell::Ref<'_, Vec<u8>> {
        self.data.borrow()
    }
}

impl Drawable for Buffer {
    fn subregion(&mut self, rect: Rect) {
        let rect = rect.clamp(self.subregions.last().unwrap().size().into());
        self.subregions
            .push(rect + self.subregions.last().unwrap().offset());
    }

    fn end_subregion(&mut self) {
        if self.subregions.len() == 1 {
            return;
        }
        self.subregions.pop();
    }

    fn get_subregion(&self) -> Rect {
        *self.subregions.last().unwrap()
    }

    fn size(&self) -> Size {
        Size {
            w: self.width as u32,
            h: self.height as u32,
        }
    }
    fn point(&self, x: i32, y: i32, color: &Color) {
        let subregion = self.subregions.last().unwrap();
        let x_o = x + subregion.x;
        let y_o = y + subregion.y;
        if x < 0 || y < 0 || x as u32 >= subregion.w || y as u32 >= subregion.h {
            return;
        }
        let [r, g, b, a] = color.get(Offset { x: x_o, y: y_o }).into();
        let (x, y) = (x_o as usize, y_o as usize);
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
}

pub struct Overlay {
    // RGB
    base: Rc<RefCell<Vec<u8>>>,
    base_width: usize,
    base_height: usize,
    // Premultiplied RGB + Alpha
    overlay_data: Rc<RefCell<Vec<u8>>>,
    dst_rect: Rect,
    subregions: Vec<Rect>,
}

impl Overlay {
    pub fn new(base: Buffer, rect: Rect) -> Self {
        let overlay = Rc::new(RefCell::new(vec![0; rect.w as usize * rect.h as usize * 4]));
        Self {
            base: base.data,
            base_width: base.width,
            base_height: base.height,
            overlay_data: overlay,
            dst_rect: rect,
            subregions: vec![rect.size().into()],
        }
    }
    pub fn offset(&mut self, offset: Offset) {
        self.dst_rect.x = offset.x;
        self.dst_rect.y = offset.y;
    }

    /// Writes to the underlying buffer, and returns a copy.
    pub fn write(&self) -> Buffer {
        let mut base = self.base.borrow_mut();
        let overlay = self.overlay_data.borrow();
        let src_offs = -self.dst_rect.offset().min(Offset::default());
        let dst_rect = self
            .dst_rect
            .clamp(Size::new(self.base_width as _, self.base_height as _).into());
        for j in 0..self.base_height {
            for i in 0..self.base_width {
                let offs = 3 * (i + j * self.base_width);
                if i < dst_rect.x as _
                    || i >= dst_rect.offset_2().x as _
                    || j < dst_rect.y as _
                    || j >= dst_rect.offset_2().y as _
                {
                    continue;
                }

                let overlay_offs = src_offs + (Offset::new(i as _, j as _) - dst_rect.offset());
                let overlay_array_offs =
                    4 * (overlay_offs.x + overlay_offs.y * dst_rect.w as i32) as usize;

                let [r, g, b, a] = overlay[overlay_array_offs..overlay_array_offs + 4] else {
                    unreachable!()
                };
                if a == 255 {
                    // Quick optimization
                    base[offs..offs + 3].copy_from_slice(&[r, g, b]);
                    continue;
                }
                let (r, g, b, a) = (r as i32, g as i32, b as i32, a as i32);
                base[offs] = ((255 - a) * base[offs] as i32 / 255 + r) as u8;
                base[offs + 1] = ((255 - a) * base[offs + 1] as i32 / 255 + g) as u8;
                base[offs + 2] = ((255 - a) * base[offs + 2] as i32 / 255 + b) as u8;
            }
        }
        mem::drop(base);
        Buffer {
            data: self.base.clone(),
            width: self.base_width,
            height: self.base_height,
            subregions: vec![Size::new(self.base_width as _, self.base_height as _).into()],
        }
    }
}

impl Drawable for Overlay {
    fn subregion(&mut self, rect: Rect) {
        let rect = rect.clamp(self.subregions.last().unwrap().size().into());
        self.subregions
            .push(rect + self.subregions.last().unwrap().offset());
    }

    fn end_subregion(&mut self) {
        if self.subregions.len() == 1 {
            return;
        }
        self.subregions.pop();
    }

    fn get_subregion(&self) -> Rect {
        *self.subregions.last().unwrap()
    }

    fn size(&self) -> Size {
        Size {
            w: self.base_width as u32,
            h: self.base_height as u32,
        }
    }
    fn point(&self, x: i32, y: i32, color: &Color) {
        let subregion = self.subregions.last().unwrap();
        let x_o = x + subregion.x;
        let y_o = y + subregion.y;
        if x < 0 || y < 0 || x as u32 >= subregion.w || y as u32 >= subregion.h {
            return;
        }
        let [r, g, b, a] = color.get(Offset { x: x_o, y: y_o }).into();
        let (x, y) = (x_o as usize, y_o as usize);
        let pixel_range = &mut self.overlay_data.borrow_mut()
            [(x + y * self.dst_rect.w as usize) * 4..(x + y * self.dst_rect.w as usize) * 4 + 4];
        if a == 255 {
            // Quick optimization
            pixel_range.copy_from_slice(&[r, g, b, a]);
        } else {
            // Alpha blending. SRC * A / 255 + DST * (255-A) / 255 = (SRC - DST) * A / 255 + DST
            let (r, g, b, a) = (r as i32, g as i32, b as i32, a as i32);
            pixel_range[0] = ((r - pixel_range[0] as i32) * a / 255 + pixel_range[0] as i32) as u8;
            pixel_range[1] = ((g - pixel_range[1] as i32) * a / 255 + pixel_range[1] as i32) as u8;
            pixel_range[2] = ((b - pixel_range[2] as i32) * a / 255 + pixel_range[2] as i32) as u8;
            pixel_range[3] = ((255 - a) * pixel_range[3] as i32 / 255 + a) as u8;
        }
    }
}
