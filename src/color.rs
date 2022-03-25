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
    Rgba,
    SRgba,
    Hsva,
}

#[allow(non_snake_case)]
#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub ch: [f64; 4],
    pub mode: ColorType,
}

#[allow(non_snake_case, dead_code)]
impl Color {
    pub fn new(ch: [f64; 4], mode: ColorType) -> Color {
        Color { ch, mode }
    }

    pub fn to(self, ctype: ColorType) -> Color {
        let mut out = self;
        if self.mode != ctype {
            match ctype {
                ColorType::Hsva => match self.mode {
                    ColorType::Rgba => out = self.to_Hsva(),
                    ColorType::SRgba => out = self.to_Rgba().to_Hsva(),
                    _ => out = self,
                },
                ColorType::Rgba => match self.mode {
                    ColorType::SRgba => out = self.to_Rgba(),
                    ColorType::Hsva => out = self.to_Rgba(),
                    _ => out = self,
                },
                ColorType::SRgba => match self.mode {
                    ColorType::Rgba => out = self.to_sRgba(),
                    ColorType::Hsva => out = self.to_Rgba().to_sRgba(),
                    _ => out = self,
                },
            }
        }
        out
    }

    fn sRGB(value: f64, inverse: bool) -> f64 {
        if inverse {
            if value <= 0.04045 {
                (25.0 * value) / 323.0
            } else {
                (((200.0 * value) + 11.0) / 211.0).powf(12.0 / 5.0)
            }
        } else if value <= 0.0031308 {
            (323.0 * value) / 25.0
        } else {
            (211.0 * value.powf(5.0 / 12.0) - 11.0) / 200.0
        }
    }

    pub fn f_hsv(h: f64, s: f64, v: f64, n: f64) -> f64 {
        let k = (n + (h / 60.0)) % 6.0;
        v - (v * s * (0.0f64).max((k).min((4.0 - k).min(1.0))))
    }
    pub fn from_hsv(h: f64, s: f64, v: f64, a: f64) -> Color {
        let h = h % 360.0;
        Color {
            ch: [
                Color::f_hsv(h, s, v, 5.0),
                Color::f_hsv(h, s, v, 3.0),
                Color::f_hsv(h, s, v, 1.0),
                a,
            ],
            mode: ColorType::Rgba,
        }
    }

    pub fn to_Hsva(&self) -> Color {
        let v = self.ch[0].max(self.ch[1]).max(self.ch[2]);
        let min = self.ch[0].min(self.ch[1]).min(self.ch[2]);
        let c = v - min;
        // let l = v - (c / 2.0);
        let mut h = 0.0;
        let (r, g, b) = (self.ch[0], self.ch[1], self.ch[2]);
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
            ch: [h, s, v, self.ch[3]],
            mode: ColorType::Hsva,
        }
    }

    pub fn to_sRgba(&self) -> Color {
        Color {
            ch: self.ch.map(|v| Color::sRGB(v, false)),
            mode: ColorType::SRgba,
        }
    }
    pub fn to_Rgba(&self) -> Color {
        if self.mode == ColorType::Hsva {
            Color {
                ch: [
                    Color::f_hsv(self.ch[0], self.ch[1], self.ch[2], 5.0),
                    Color::f_hsv(self.ch[0], self.ch[1], self.ch[2], 3.0),
                    Color::f_hsv(self.ch[0], self.ch[1], self.ch[2], 1.0),
                    self.ch[3],
                ],
                mode: ColorType::Rgba,
            }
        } else {
            Color {
                ch: self.ch.map(|v| Color::sRGB(v, true)),
                mode: ColorType::Rgba,
            }
        }
    }
    pub fn to_arr(&self) -> [f64; 4] {
        self.ch
    }
    pub fn to_arr16(&self) -> [u16; 4] {
        self.ch.map(|x| (x * u16::MAX as f64) as u16)
    }
    pub fn to_arr8(&self) -> [u8; 4] {
        self.ch.map(|x| (x * u8::MAX as f64) as u8)
    }
}

impl FromStr for Color {
    type Err = ParseIntError;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let cols: Vec<f64> = string
            .split(',')
            .map(|x| x.parse::<f64>().unwrap())
            .collect();
        let cols: [f64; 4] = cols.try_into().unwrap();
        Ok(Color {
            ch: cols.map(|x| x / 255.0),
            mode: ColorType::Rgba,
        })
    }
}
// mode: ColorType::Rgba
// Color operators
impl std::ops::Add<Color> for Color {
    type Output = Color;
    fn add(self, mut _rhs: Color) -> Color {
        _rhs = _rhs.to(self.mode);
        let cols: Vec<f64> = self
            .ch
            .iter()
            .enumerate()
            .map(|(i, v)| v + _rhs.ch[i])
            .collect();
        Color {
            ch: cols.try_into().unwrap(),
            mode: self.mode,
        }
    }
}
impl std::ops::Sub<Color> for Color {
    type Output = Color;
    fn sub(self, mut _rhs: Color) -> Color {
        _rhs = _rhs.to(self.mode);
        let cols: Vec<f64> = self
            .ch
            .iter()
            .enumerate()
            .map(|(i, v)| v - _rhs.ch[i])
            .collect();
        Color {
            ch: cols.try_into().unwrap(),
            mode: self.mode,
        }
    }
}
impl std::ops::Mul<Color> for Color {
    type Output = Color;
    fn mul(self, mut _rhs: Color) -> Color {
        _rhs = _rhs.to(self.mode);
        let cols: Vec<f64> = self
            .ch
            .iter()
            .enumerate()
            .map(|(i, v)| v * _rhs.ch[i])
            .collect();
        Color {
            ch: cols.try_into().unwrap(),
            mode: self.mode,
        }
    }
}

// f64 operators
impl std::ops::Add<f64> for Color {
    type Output = Color;
    fn add(self, _rhs: f64) -> Color {
        let cols: Vec<f64> = self.ch.iter().map(|v| v + _rhs).collect();
        Color {
            ch: cols.try_into().unwrap(),
            mode: self.mode,
        }
    }
}
impl std::ops::Sub<f64> for Color {
    type Output = Color;
    fn sub(self, _rhs: f64) -> Color {
        let cols: Vec<f64> = self.ch.iter().map(|v| v - _rhs).collect();
        Color {
            ch: cols.try_into().unwrap(),
            mode: self.mode,
        }
    }
}
impl std::ops::Mul<f64> for Color {
    type Output = Color;
    fn mul(self, _rhs: f64) -> Color {
        let cols: Vec<f64> = self.ch.iter().map(|v| v * _rhs).collect();
        Color {
            ch: cols.try_into().unwrap(),
            mode: self.mode,
        }
    }
}
impl std::ops::Div<f64> for Color {
    type Output = Color;
    fn div(self, _rhs: f64) -> Color {
        let cols: Vec<f64> = self.ch.iter().map(|v| v / _rhs).collect();
        Color {
            ch: cols.try_into().unwrap(),
            mode: self.mode,
        }
    }
}
