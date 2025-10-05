use crate::Offset;

#[derive(Clone)]
pub enum Color {
    Rgba(Rgba),
    DirectionalGradient(DirectionalGradient),
}

impl Color {
    pub const RED: Self = Self::Rgba(Rgba::RED);
    pub const GREEN: Self = Self::Rgba(Rgba::GREEN);
    pub const BLUE: Self = Self::Rgba(Rgba::BLUE);
    pub const YELLOW: Self = Self::Rgba(Rgba::YELLOW);
    pub const CYAN: Self = Self::Rgba(Rgba::CYAN);
    pub const MAGENTA: Self = Self::Rgba(Rgba::MAGENTA);
    pub const DARK_RED: Self = Self::Rgba(Rgba::DARK_RED);
    pub const DARK_GREEN: Self = Self::Rgba(Rgba::DARK_GREEN);
    pub const DARK_BLUE: Self = Self::Rgba(Rgba::DARK_BLUE);
    pub const DARK_YELLOW: Self = Self::Rgba(Rgba::DARK_YELLOW);
    pub const DARK_CYAN: Self = Self::Rgba(Rgba::DARK_CYAN);
    pub const DARK_MAGENTA: Self = Self::Rgba(Rgba::DARK_MAGENTA);
    pub const ORANGE: Self = Self::Rgba(Rgba::ORANGE);
    pub const PINK: Self = Self::Rgba(Rgba::PINK);
    pub const BROWN: Self = Self::Rgba(Rgba::BROWN);
    pub const GRAY: Self = Self::Rgba(Rgba::GRAY);
    pub const SILVER: Self = Self::Rgba(Rgba::SILVER);
    pub const BLACK: Self = Self::Rgba(Rgba::BLACK);
    pub const WHITE: Self = Self::Rgba(Rgba::WHITE);
    pub const TRANSPARENT: Self = Self::Rgba(Rgba::TRANSPARENT);

    /// Parse hex string
    pub const fn hex(val: &'static str) -> Option<Self> {
        match Rgba::hex(val) {
            Some(x) => Some(Self::Rgba(x)),
            None => None,
        }
    }

    pub fn get(&self, pos: Offset) -> Rgba {
        match self {
            Self::Rgba(rgba) => rgba.get(pos),
            Self::DirectionalGradient(grad) => grad.get(pos),
        }
    }
    pub fn set_a(&self, a: u8) -> Self {
        match self {
            Self::Rgba(rgba) => Self::Rgba(rgba.set_a(a)),
            Self::DirectionalGradient(grad) => Self::DirectionalGradient(grad.set_a(a)),
        }
    }

    /// Maps an Rgba value if one.
    pub fn map<F: Fn(Rgba) -> Rgba>(self, f: F) -> Self {
        match self {
            Self::Rgba(rgba) => Self::Rgba(f(rgba)),
            x => x,
        }
    }

    pub fn get_rgba(&self) -> Option<Rgba> {
        if let Self::Rgba(rgba) = self {
            Some(*rgba)
        } else {
            None
        }
    }
}

impl From<Rgba> for Color {
    fn from(value: Rgba) -> Self {
        Self::Rgba(value)
    }
}

impl From<DirectionalGradient> for Color {
    fn from(value: DirectionalGradient) -> Self {
        Self::DirectionalGradient(value)
    }
}

#[derive(Clone, Copy)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
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
    pub const DARK_RED: Self = Self::hex("#800000").unwrap();
    pub const DARK_GREEN: Self = Self::hex("#008000").unwrap();
    pub const DARK_BLUE: Self = Self::hex("#000080").unwrap();
    pub const DARK_YELLOW: Self = Self::hex("#808000").unwrap();
    pub const DARK_CYAN: Self = Self::hex("#008080").unwrap();
    pub const DARK_MAGENTA: Self = Self::hex("#800080").unwrap();
    pub const ORANGE: Self = Self::hex("#ffa500").unwrap();
    pub const PINK: Self = Self::hex("#ff80ff").unwrap();
    pub const BROWN: Self = Self::hex("#804000").unwrap();
    pub const GRAY: Self = Self::hex("#808080").unwrap();
    pub const SILVER: Self = Self::hex("#C0C0C0").unwrap();
    pub const BLACK: Self = Self::hex("#000").unwrap();
    pub const WHITE: Self = Self::hex("#fff").unwrap();
    pub const TRANSPARENT: Self = Self::hex("#0000").unwrap();
    // Parse hex string (const fn).
    pub const fn hex(val: &'static str) -> Option<Self> {
        const fn hex_decode(n1: u8, n2: u8) -> Option<u8> {
            if n1 < 0x30 || n2 < 0x30 {
                return None;
            }
            let n1 = (n1 & 0xdf) - 0x10;
            let n2 = (n2 & 0xdf) - 0x10;
            if (n1 > 0x9 && n1 < 0x31) || (n2 > 0x9 && n2 < 0x31) || n1 > 0x36 || n2 > 0x36 {
                return None;
            }
            Some((n1 - 0x27 * (n1 >> 5)) * 16 + (n2 - 0x27 * (n2 >> 5)))
        }
        const fn get<T: Copy>(s: &[T], n: usize) -> Option<T> {
            if s.len() > n {
                Some(s[n])
            } else {
                None
            }
        }

        let bytes = val.as_bytes();
        if bytes[0] != 35 {
            return None;
        }

        let (Some(r1), Some(r2), Some(g1)) = (get(bytes, 1), get(bytes, 2), get(bytes, 3)) else {
            return None;
        };
        let g2 = get(bytes, 4);
        let b1 = get(bytes, 5);
        let b2 = get(bytes, 6);
        let mut a1 = get(bytes, 7);
        let mut a2 = get(bytes, 8);

        let (r, g, b) = if b1.is_none() {
            a1 = g2;
            a2 = g2;
            (hex_decode(r1, r1), hex_decode(r2, r2), hex_decode(g1, g1))
        } else if let (Some(g2), Some(b1), Some(b2)) = (g2, b1, b2) {
            (hex_decode(r1, r2), hex_decode(g1, g2), hex_decode(b1, b2))
        } else {
            return None;
        };

        let a = if let (Some(a1), Some(a2)) = (a1, a2) {
            hex_decode(a1, a2)
        } else if a1.is_some() {
            return None;
        } else {
            Some(255)
        };

        match (r, g, b, a) {
            (Some(r), Some(g), Some(b), Some(a)) => Some(Self { r, g, b, a }),
            _ => None,
        }
    }
    pub const fn set_a(self, a: u8) -> Self {
        Self {
            r: self.r,
            g: self.g,
            b: self.b,
            a: (a as u16 * self.a as u16 / 255) as u8,
        }
    }
    /// Linear intERPolation.
    ///
    /// `r`,`g`,`b`, `a`: `(other - self) * t / 255 + self`
    pub const fn lerp(self, other: Self, t: u8) -> Self {
        Self {
            r: ((other.r as i32 - self.r as i32) * t as i32 / 255 + self.r as i32) as u8,
            g: ((other.g as i32 - self.g as i32) * t as i32 / 255 + self.g as i32) as u8,
            b: ((other.b as i32 - self.b as i32) * t as i32 / 255 + self.b as i32) as u8,
            a: ((other.a as i32 - self.a as i32) * t as i32 / 255 + self.a as i32) as u8,
        }
    }
    /// Linear intERPolation with approximate gamma correction.
    ///
    /// `r`,`g`,`b`: `sqrt((other^2 - self^2) * t / 255 + self^2)`
    ///
    /// `a`: `(other - self) * t / 255 + self`
    pub const fn gamma_lerp(self, other: Self, t: u8) -> Self {
        Self {
            r: (((other.r as i32).pow(2) - (self.r as i32).pow(2)) * t as i32 / 255
                + (self.r as i32).pow(2))
            .isqrt() as u8,
            g: (((other.g as i32).pow(2) - (self.g as i32).pow(2)) * t as i32 / 255
                + (self.g as i32).pow(2))
            .isqrt() as u8,
            b: (((other.b as i32).pow(2) - (self.b as i32).pow(2)) * t as i32 / 255
                + (self.b as i32).pow(2))
            .isqrt() as u8,
            a: ((other.a as i32 - self.a as i32) * t as i32 / 255 + self.a as i32) as u8,
        }
    }

    /// Get intensity (avg. of R,G,B)
    pub const fn intensity(&self) -> u8 {
        ((self.r as i32 + self.g as i32 + self.b as i32) / 3) as u8
    }
    fn get(&self, _pos: Offset) -> Rgba {
        *self
    }
}

#[derive(Clone)]
pub struct GradientBase {
    steps: Vec<(u16, Rgba)>,
    repeating: bool,
}

impl GradientBase {
    /// Color values are sorted, and ads both end values if missing.
    fn new(colors: &[(u16, Rgba)], repeating: bool) -> Self {
        let mut steps = colors.to_vec();
        if steps.len() == 1 {
            return Self {
                repeating,
                steps: vec![(0, steps[0].1), (u16::MAX, steps[0].1)],
            };
        } else if steps.is_empty() {
            return Self {
                repeating,
                steps: vec![(0, Rgba::BLACK), (u16::MAX, Rgba::BLACK)],
            };
        }
        steps.sort_by_key(|k| k.0);

        let max = steps.last().unwrap().0;
        if max < u16::MAX {
            steps.push((u16::MAX, steps.last().unwrap().1))
        }

        if steps[0].0 != 0 {
            steps.insert(0, (0, steps[0].1));
        }
        Self { steps, repeating }
    }

    /// Get a value from the gradient
    fn get(&self, v: f32) -> Rgba {
        if !v.is_finite() {
            return Rgba::BLACK;
        }

        let v_c = if self.repeating {
            v.rem_euclid(1.0)
        } else {
            v.clamp(0., 1.)
        } * u16::MAX as f32;

        let v_i = v_c as u16;

        match self.steps.binary_search_by_key(&v_i, |k| k.0) {
            Ok(mut i) => {
                while i < self.steps.len() && self.steps[i].0 == v_i {
                    i += 1
                }
                i -= 1;
                self.steps[i].1
            }
            Err(i) => {
                let over = self.steps[i];
                let under = self.steps[i - 1];
                let lerp_t =
                    ((v_c - under.0 as f32) / (over.0 as f32 - under.0 as f32) * 255.0) as u8;
                under.1.lerp(over.1, lerp_t)
            }
        }
    }
}

#[derive(Clone)]
pub struct DirectionalGradient {
    base: GradientBase,
    angle: f32,
    scale: f32,
    offset: Offset,
}

impl DirectionalGradient {
    /// Angle in radians, Color values are sorted and clamped to 0-1. Scale defines total width in pixels, and offset defines starting point.
    pub fn new(
        colors: &[(f32, Rgba)],
        repeating: bool,
        angle: f32,
        scale: f32,
        offset: Offset,
    ) -> Self {
        let colors = colors
            .iter()
            .filter_map(|(v, c)| {
                if !v.is_finite() || *v < 0.0 || *v > 1.0 {
                    None
                } else {
                    Some(((v * u16::MAX as f32) as u16, *c))
                }
            })
            .collect::<Vec<_>>();

        let base = GradientBase::new(&colors, repeating);
        Self {
            base,
            angle: angle.rem_euclid(std::f32::consts::TAU),
            scale,
            offset,
        }
    }
    fn get(&self, pos: Offset) -> Rgba {
        let (sin, cos) = self.angle.sin_cos();
        let real_x = (pos.x - self.offset.x) as f32 * cos - (pos.y - self.offset.y) as f32 * sin;
        let value = real_x / self.scale;
        self.base.get(value)
    }
    fn set_a(&self, a: u8) -> Self {
        let steps = self
            .base
            .steps
            .iter()
            .map(|c| (c.0, c.1.set_a(a)))
            .collect();
        DirectionalGradient {
            base: GradientBase { steps, ..self.base },
            ..self.clone()
        }
    }
}
