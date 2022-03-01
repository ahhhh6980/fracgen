// fracgen
// Color type
// (C) 2022 by Jacob (ahhhh6980@gmail.com)

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::{num::ParseIntError, str::FromStr};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorType {
    RGBA,
    SRGBA,
    HSVA
}

#[allow(non_snake_case)]
#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
    pub mode: ColorType,
}

#[allow(non_snake_case, dead_code)]
impl Color {
    pub fn new_color_alpha(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color {
            r,
            g,
            b,
            a,
            mode: ColorType::RGBA,
        }
    }
    pub fn new_color(r: f32, g: f32, b: f32) -> Color {
        Color {
            r,
            g,
            b,
            a: 1.0,
            mode: ColorType::RGBA,
        }
    }
    pub fn new_alpha(v: f32, a: f32) -> Color {
        Color {
            r: v,
            g: v,
            b: v,
            a,
            mode: ColorType::RGBA,
        }
    }
    pub fn new(v: f32) -> Color {
        Color {
            r: v,
            g: v,
            b: v,
            a: 1.0,
            mode: ColorType::RGBA,
        }
    }

    pub fn to(self, ctype: ColorType) -> Color {
        if self.mode != ctype {
            match ctype {
                ColorType::HSVA => {
                    match self.mode {
                        ColorType::RGBA => self.to_HSVA(),
                        ColorType::SRGBA => self.to_RGBA().to_HSVA(),
                        _ => self,
                    };
                },
                ColorType::RGBA => {
                    match self.mode {
                        ColorType::SRGBA => self.to_RGBA(),
                        ColorType::HSVA => self.to_RGBA(),
                        _ => self,
                    };
                },
                ColorType::SRGBA => {
                    match self.mode {
                        ColorType::RGBA => self.to_sRGBA(),
                        ColorType::HSVA => self.to_RGBA().to_sRGBA(),
                        _ => self,
                    };
                },
            }
        }
        self
    }

    fn sRGB(value: f32, inverse: bool) -> f32 {
        if inverse {
            if value <= 0.04045 {
                (25.0 * value) / 323.0
            } else {
                (((200.0 * value) + 11.0) / 211.0).powf(12.0 / 5.0)
            }
        } else {
            if value <= 0.0031308 {
                (323.0 * value) / 25.0
            } else {
                (211.0 * value.powf(5.0 / 12.0) - 11.0) / 200.0
            }
        }
    }

    pub fn f_hsv(h: f32, s: f32, v: f32, n: f32) -> f32 {
        let k = (n + (h / 60.0)) % 6.0;
        v - (v * s * (0.0f32).max((k).min((4.0 - k).min(1.0))))
    }
    pub fn from_hsv(h: f32, s: f32, v: f32, a: f32) -> Color {
        let h = h % 360.0;
        Color {
            r: Color::f_hsv(h, s, v, 5.0),
            g: Color::f_hsv(h, s, v, 3.0),
            b: Color::f_hsv(h, s, v, 1.0),
            a: a,
            mode: ColorType::RGBA,
        }
    }

    pub fn to_HSVA(&self) -> Color {
        let v = self.r.max(self.g.max(self.b));
        let min = self.r.min(self.g.min(self.b));
        let c = v - min;
        // let l = v - (c / 2.0);
        let mut h = 0.0;
        let (r, g, b) = (self.r, self.g, self.b);
        if c != 0.0 {
            h = 60.0;
            if v == r {
                h *= 0.0 + ((g - b) / c);
            }
            if v == g {
                h *= 2.0 + ((b - r) / c);
            }
            if v == b {
                h *= 4.0 + ((r - g) / c);
            }
        }
        let mut s = 0.0;
        if v != 0.0 {
            s = c / v;
        }
        Color {
            r: h,
            g: s,
            b: v,
            a: self.a,
            mode: ColorType::HSVA,
        }
    }

    pub fn to_sRGBA(&self) -> Color {
        Color {
            r: Color::sRGB(self.r, false),
            g: Color::sRGB(self.g, false),
            b: Color::sRGB(self.b, false),
            a: Color::sRGB(self.a, true),
            mode: ColorType::SRGBA,
        }
    }
    pub fn to_RGBA(&self) -> Color {
        if self.mode == ColorType::HSVA {
            Color {
                r: Color::f_hsv(self.r, self.g, self.b, 5.0),
                g: Color::f_hsv(self.r, self.g, self.b, 3.0),
                b: Color::f_hsv(self.r, self.g, self.b, 1.0),
                a: self.a,
                mode: ColorType::RGBA,
            }
        } else {
            Color {
                r: Color::sRGB(self.r, true),
                g: Color::sRGB(self.g, true),
                b: Color::sRGB(self.b, true),
                a: Color::sRGB(self.a, true),
                mode: ColorType::RGBA,
            }
        }
    }
    pub fn to_arr(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
    pub fn to_arr16(&self) -> [u16; 4] {
        [
            (self.r * u16::MAX as f32) as u16,
            (self.g * u16::MAX as f32) as u16,
            (self.b * u16::MAX as f32) as u16,
            (self.a * u16::MAX as f32) as u16,
        ]
    }
    pub fn to_arr8(&self) -> [u8; 4] {
        [
            (self.r * u8::MAX as f32) as u8,
            (self.g * u8::MAX as f32) as u8,
            (self.b * u8::MAX as f32) as u8,
            (self.a * u8::MAX as f32) as u8,
        ]
    }
}

impl FromStr for Color {
    type Err = ParseIntError;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let cols: Vec<f32> = string
            .split(',')
            .map(|x| x.parse::<f32>().unwrap())
            .collect();
        Ok(Color {
            r: cols[0] / 255.0,
            g: cols[1] / 255.0,
            b: cols[2] / 255.0,
            a: cols[3] / 255.0,
            mode: ColorType::RGBA,
        })
    }
}
// mode: ColorType::RGBA
// Color operators
impl std::ops::Add<Color> for Color {
    type Output = Color;
    fn add(self, mut _rhs: Color) -> Color {
        _rhs = _rhs.to(self.mode);
        Color {
            r: self.r + _rhs.r,
            g: self.g + _rhs.g,
            b: self.b + _rhs.b,
            a: self.a + _rhs.a,
            mode: self.mode,
        }
    }
}
impl std::ops::Sub<Color> for Color {
    type Output = Color;
    fn sub(self, mut _rhs: Color) -> Color {
        _rhs = _rhs.to(self.mode);
        Color {
            r: self.r - _rhs.r,
            g: self.g - _rhs.g,
            b: self.b - _rhs.b,
            a: self.a - _rhs.a,
            mode: self.mode,
        }
    }
}
impl std::ops::Mul<Color> for Color {
    type Output = Color;
    fn mul(self, mut _rhs: Color) -> Color {
        _rhs = _rhs.to(self.mode);
        Color {
            r: self.r * _rhs.r,
            g: self.g * _rhs.g,
            b: self.b * _rhs.b,
            a: self.a * _rhs.a,
            mode: self.mode,
        }
    }
}

// f32 operators
impl std::ops::Add<f32> for Color {
    type Output = Color;
    fn add(self, _rhs: f32) -> Color {
        Color {
            r: self.r + _rhs,
            g: self.g + _rhs,
            b: self.b + _rhs,
            a: self.a + _rhs,
            mode: self.mode,
        }
    }
}
impl std::ops::Sub<f32> for Color {
    type Output = Color;
    fn sub(self, _rhs: f32) -> Color {
        Color {
            r: self.r - _rhs,
            g: self.g - _rhs,
            b: self.b - _rhs,
            a: self.a - _rhs,
            mode: self.mode,
        }
    }
}
impl std::ops::Mul<f32> for Color {
    type Output = Color;
    fn mul(self, _rhs: f32) -> Color {
        Color {
            r: self.r * _rhs,
            g: self.g * _rhs,
            b: self.b * _rhs,
            a: self.a * _rhs,
            mode: self.mode,
        }
    }
}
impl std::ops::Div<f32> for Color {
    type Output = Color;
    fn div(self, _rhs: f32) -> Color {
        Color {
            r: self.r / _rhs,
            g: self.g / _rhs,
            b: self.b / _rhs,
            a: self.a / _rhs,
            mode: self.mode,
        }
    }
}
