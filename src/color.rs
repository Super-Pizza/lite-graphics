use crate::Offset;

pub trait Color: Clone {
    fn get(&self, pos: Offset) -> Rgba;
    fn set_a(&self, a: u8) -> Self;
}

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
}

impl Color for Rgba {
    fn get(&self, _pos: Offset) -> Rgba {
        *self
    }
    fn set_a(&self, a: u8) -> Self {
        Rgba::set_a(*self, a)
    }
}
