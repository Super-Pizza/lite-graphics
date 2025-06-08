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
        const fn u8_from_nibs(n1: &u8, n2: &u8) -> Option<u8> {
            #[rustfmt::skip]
            const TABLE: [u8; 128] = [
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0x0,  0x1,  0x2,  0x3,  0x4,  0x5,  0x6,  0x7,  0x8,  0x9,  0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xa,  0xb,  0xc,  0xd,  0xe,  0xf,  0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xa,  0xb,  0xc,  0xd,  0xe,  0xf,  0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            ];
            if TABLE[*n1 as usize] == 0xff || TABLE[*n2 as usize] == 0xff {
                return None;
            };
            Some(TABLE[*n1 as usize] * 16 + TABLE[*n2 as usize])
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
        let (Some(r1), Some(r2), Some(g1)) = (get(iter, 1), get(iter, 2), get(iter, 3)) else {
            return None;
        };
        let g2 = get(iter, 4);
        let b1 = get(iter, 5);
        let b2 = get(iter, 6);
        let a1 = get(iter, 7);
        let a2 = get(iter, 8);

        let (r, g, b, a) = if b1.is_none() {
            let a = if let Some(a) = g2 {
                u8_from_nibs(a, a)
            } else {
                Some(255)
            };
            // 12/16 bits
            (
                u8_from_nibs(r1, r1),
                u8_from_nibs(r2, r2),
                u8_from_nibs(g1, g1),
                a,
            )
        } else if b2.is_none() || a1.is_some() && a2.is_none() {
            return None;
        } else {
            let a = if let (Some(a1), Some(a2)) = (a1, a2) {
                u8_from_nibs(a1, a2)
            } else {
                Some(255)
            };
            let (Some(g2), Some(b1), Some(b2)) = (g2, b1, b2) else {
                return None;
            };
            // 24/32 bits
            (
                u8_from_nibs(r1, r2),
                u8_from_nibs(g1, g2),
                u8_from_nibs(b1, b2),
                a,
            )
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
