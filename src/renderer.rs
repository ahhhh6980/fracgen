// fracgen
// Renderer type
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

#![allow(incomplete_features, unused_imports, dead_code)]
use clap::Parser;
use image::{DynamicImage, ImageBuffer, Rgba};
use linya::{Bar, Progress};
use num::complex::Complex;
use rand::Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{
    f64::consts::PI,
    path,
    sync::Mutex,
    time::{Instant, SystemTime},
};
type Cf64 = Complex<f64>;
type Img8 = ImageBuffer<Rgba<u8>, Vec<u8>>;
use crate::color::{Color, ColorType};

pub struct Functions;

impl Functions {
    #[allow(dead_code, unused_variables)]
    pub fn default_bail(rend: &Renderer, z: Cf64, der: Cf64, der_sum: Cf64) -> bool {
        z.norm_sqr() < rend.args.bail
    }

    #[allow(dead_code, unused_variables)]
    pub fn sqrt_bail(rend: &Renderer, z: Cf64, der: Cf64, der_sum: Cf64) -> bool {
        z.norm() < rend.args.bail
    }

    #[allow(dead_code, unused_variables)]
    pub fn der_bail(rend: &Renderer, z: Cf64, der: Cf64, der_sum: Cf64) -> bool {
        (der_sum * der_sum).norm_sqr() < rend.args.derbail
            && z.norm_sqr() * z.norm_sqr() < rend.args.bail
    }

    #[allow(dead_code, unused_variables)]
    pub fn coloring(rend: &Renderer, i: f64, s: f64, z: Cf64, der: Cf64) -> Color {
        let hue = ((1.0 - (s / rend.args.limit)) * 360.0)
            .powf(rend.args.cexp)
            .powf(1.5);
        Color::new([hue, 1.0, 1.0, 1.0], ColorType::Hsva).to_Rgba()
    }

    #[allow(dead_code, unused_variables)]
    pub fn miles_coloring(rend: &Renderer, i: f64, s: f64, z: Cf64, der: Cf64) -> Color {
        let iter_count = s;
        // let iter_count = s;

        let sat = (4096.0 / 360.0 * PI * iter_count).cos() / 2.0 + 0.5;
        let val = 1.0 - (2048.0 / 360.0 * PI * iter_count).sin() / 2.0 - 0.5;

        // # convert u into rgb of hue cycle
        let mut r = (((1.0 - 2.0 * (iter_count).cos()) / 2.0).max(0.0)).min(1.0);
        let mut g = (((1.0 - 2.0 * (iter_count + PI * 2.0 / 3.0).cos()) / 2.0).max(0.0)).min(1.0);
        let mut b = (((1.0 - 2.0 * (iter_count + PI * 4.0 / 3.0).cos()) / 2.0).max(0.0)).min(1.0);

        // # apply saturation and brightness to the rgb
        r = ((1.0 + r * sat - sat) * val).sqrt();
        g = ((1.0 + g * sat - sat) * val).sqrt();
        b = ((1.0 + b * sat - sat) * val).sqrt();

        let light_deg = 270f64;
        let norm_height = 1.5;
        let light_vec = Cf64::new(
            ((light_deg * PI) / 180.0).cos(),
            ((light_deg * PI) / 180.0).sin(),
        );
        let normal_vec = z / der;
        let normal_vec = normal_vec / normal_vec.norm(); // abs norm_vec
        let mut value =
            ((normal_vec.re * light_vec.re) + (normal_vec.im * light_vec.im) + norm_height)
                / (1.0 + norm_height);
        if value < 0.0 {
            value = 0.0;
        }
        if value > 1.0 {
            value = 1.0;
        }

        // let hue = (((s / limit).powf(cexp)) * 360.0).powf(1.5);
        let mut color = Color::new([r, g, b, 1.0], ColorType::Rgba).to_Rgba();
        color.ch[2] *= value;
        color
    }

    #[allow(dead_code, unused_variables)]
    pub fn miles_coloring2(rend: &Renderer, i: f64, s: f64, z: Cf64, der: Cf64) -> Color {
        let iter_count = s.sqrt().powf(rend.args.cexp);
        // let iter_count = s;

        let sat = (4096.0 / 360.0 * PI * iter_count).cos() / 2.0 + 0.5;
        let sat = 1.0;
        let val = 1.0 - (2048.0 / 360.0 * PI * iter_count).sin() / 2.0 - 0.5;
        let val = 1.0;
        // # convert u into rgb of hue cycle
        let mut r = (((1.0 - 2.0 * (iter_count).cos()) / 2.0).max(0.0)).min(1.0);
        let mut g = (((1.0 - 2.0 * (iter_count + PI * 2.0 / 3.0).cos()) / 2.0).max(0.0)).min(1.0);
        let mut b = (((1.0 - 2.0 * (iter_count + PI * 4.0 / 3.0).cos()) / 2.0).max(0.0)).min(1.0);

        // # apply saturation and brightness to the rgb
        r = ((1.0 + r * sat - sat) * val).sqrt();
        g = ((1.0 + g * sat - sat) * val).sqrt();
        b = ((1.0 + b * sat - sat) * val).sqrt();

        let light_deg = 270f64;
        let norm_height = 1.5;
        let light_vec = Cf64::new(
            ((light_deg * PI) / 180.0).cos(),
            ((light_deg * PI) / 180.0).sin(),
        );
        let normal_vec = z / der;
        let normal_vec = normal_vec / normal_vec.norm(); // abs norm_vec
        let mut value =
            ((normal_vec.re * light_vec.re) + (normal_vec.im * light_vec.im) + norm_height)
                / (1.0 + norm_height);
        if value < 0.0 {
            value = 0.0;
        }
        if value > 1.0 {
            value = 1.0;
        }

        // let hue = (((s / limit).powf(cexp)) * 360.0).powf(1.5);
        let mut color = Color::new([r, g, b, 1.0], ColorType::Rgba).to_Rgba();
        color.ch[2] *= value;
        color
    }

    #[allow(dead_code, unused_variables)]
    pub fn normal_map(rend: &Renderer, i: f64, s: f64, z: Cf64, der: Cf64) -> Color {
        let light_deg = 270f64;
        let norm_height = 1.5;
        let light_vec = Cf64::new(
            ((light_deg * PI) / 180.0).cos(),
            ((light_deg * PI) / 180.0).sin(),
        );
        let normal_vec = z / der;
        let normal_vec = normal_vec / normal_vec.norm(); // abs norm_vec
        let mut value =
            ((normal_vec.re * light_vec.re) + (normal_vec.im * light_vec.im) + norm_height)
                / (1.0 + norm_height);
        if value < 0.0 {
            value = 0.0;
        }
        if value > 1.0 {
            value = 1.0;
        }
        let hue = (((s / rend.args.limit).powf(rend.args.cexp)) * 360.0).powf(1.5);
        Color::new([hue, 1.0, value, 1.0], ColorType::Hsva).to_Rgba()
    }

    #[allow(dead_code, unused_variables)]
    pub fn image_mapping(rend: &Renderer, i: f64, s: f64, z: Cf64, der: Cf64) -> Color {
        let (w, h) = (rend.texture.width(), rend.texture.height());
        let width = ((z.im.atan2(z.re) + PI) / (PI * 2.0) * w as f64).round() as u32 % w;
        let height = (h as f64 - 1.0f64)
            - ((z.norm() / rend.args.bail).log(rend.args.bail) * (h as f64 - 1.0f64)).floor();
        let mut height = ((height as u32) * 2) % h;
        if i as u32 % 2 == 1 {
            height = (h - 1) - height;
        }
        let mut color = Color::new(
            rend.texture
                .get_pixel(width, height)
                .0
                .map(|x| x as f64 / u8::MAX as f64),
            ColorType::Rgba,
        )
        .to_Hsva();

        let light_deg = 270f64;
        let norm_height = 1.5;
        let light_vec = Cf64::new(
            ((light_deg * PI) / 180.0).cos(),
            ((light_deg * PI) / 180.0).sin(),
        );
        let normal_vec = z / der;
        let normal_vec = normal_vec / normal_vec.norm(); // abs norm_vec
        let mut value =
            ((normal_vec.re * light_vec.re) + (normal_vec.im * light_vec.im) + norm_height)
                / (1.0 + norm_height);
        if value < 0.0 {
            value = 0.0;
        }
        if value > 1.0 {
            value = 1.0;
        }

        color.ch[2] *= value;
        color.to_Rgba()
    }

    #[allow(dead_code, unused_variables)]
    pub fn map_complex(c: Cf64) -> Cf64 {
        let nc = Cf64::new(c.im, c.re);
        (nc + 1.0) / ((-nc / 1.25) + 1.0)
    }

    #[allow(dead_code, unused_variables)]
    pub fn map_complex2(c: Cf64) -> Cf64 {
        let nc = Cf64::new(c.im, c.re);
        (c - 1.0) / (c + 1.0)
    }
    #[allow(dead_code, unused_variables)]
    pub fn map_circle(c: Cf64) -> Cf64 {
        1.0 / c
    }

    #[allow(dead_code, unused_variables)]
    pub fn identity(c: Cf64) -> Cf64 {
        c
    }

    #[allow(dead_code, unused_variables)]
    pub fn mandelbrot(z: Cf64, c: Cf64, j: Cf64) -> Cf64 {
        z * z + c
    }
}

#[derive(Debug, Clone, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(long, default_value = "1920")]
    pub width: i32,

    #[clap(long, default_value = "1680")]
    pub height: i32,

    #[clap(long, default_value = "mandel")]
    pub name: String,

    #[clap(long, default_value_t=((num_cpus::get() as f64) * 0.70).ceil() as usize)]
    pub threads: usize,

    #[clap(long, default_value_t=Complex::<f64>::new(-0.75,0.0))]
    pub origin: Cf64,

    #[clap(long, default_value_t=Complex::<f64>::new(0.0,0.0))]
    pub z_init: Cf64,

    #[clap(long, default_value_t=Complex::<f64>::new(0.0,0.0))]
    pub julia: Cf64,

    #[clap(long, default_value = "0.7")]
    pub zoom: f64,

    #[clap(long, default_value = "4")]
    pub samples: usize,

    #[clap(long, default_value = "20")]
    pub cycles: usize,

    #[clap(long, default_value = "2.0")]
    pub sampled: f64,

    #[clap(long, default_value = "1024.0")]
    pub limit: f64,

    #[clap(long, default_value = "64.0")]
    pub bail: f64,

    #[clap(long, default_value = "16384.0")]
    pub derbail: f64,

    #[clap(long, default_value = "1.0")]
    pub cexp: f64,

    #[clap(long, default_value = "0,0,0,255")]
    pub set_color: Color,

    #[clap(short)]
    pub is_julia: bool,

    #[clap(long, default_value = "0")]
    pub fractal_mode: usize,

    #[clap(long, default_value = "0")]
    pub color_mode: usize,

    #[clap(long, default_value = "0")]
    pub bail_mode: usize,
}

impl Args {
    pub fn new() -> Args {
        Args {
            width: 1920,
            height: 1680,
            name: String::from("mandel"),
            threads: ((num_cpus::get() as f64) * 0.75).ceil() as usize,
            origin: Complex::<f64>::new(-0.75, 0.0),
            z_init: Complex::<f64>::new(0.0, 0.0),
            julia: Complex::<f64>::new(0.0, 0.0),
            cycles: 20,
            zoom: 0.7,
            samples: 4,
            sampled: 2.0,
            limit: 1024.0,
            bail: 256.0,
            derbail: 16384.0,
            cexp: 1.0,
            is_julia: false,
            fractal_mode: 0,
            color_mode: 0,
            bail_mode: 0,
            set_color: Color::new([0.0, 0.0, 0.0, 1.0], ColorType::Rgba),
        }
    }
}

impl Default for Args {
    fn default() -> Self {
        Self::new()
    }
}

fn abs(z: Cf64) -> f64 {
    z.re * z.re + z.im * z.im
}

pub fn normalize_coords(x: i32, y: i32, w: i32, h: i32, z: f64) -> Cf64 {
    let nx = 2.0 * (x as f64 / w as f64) - 1.0;
    let ny = 2.0 * (y as f64 / h as f64) - 1.0;
    Complex::new(nx / z, ny * (h as f64 / w as f64) / z)
}

#[derive(Clone)]
pub struct Functs {
    pub iter_funct: fn(Cf64, Cf64, Cf64) -> Cf64,
    pub init_funct: fn(Cf64, Cf64) -> Cf64,
    pub cmap_funct: fn(Cf64) -> Cf64,
    pub color_funct: fn(&Renderer, f64, f64, Cf64, Cf64) -> Color,
    pub conditional: fn(&Renderer, Cf64, Cf64, Cf64) -> bool,
}

impl Functs {
    pub fn new(
        a: fn(Cf64, Cf64, Cf64) -> Cf64,
        b: fn(Cf64, Cf64) -> Cf64,
        c: fn(Cf64) -> Cf64,
        d: fn(&Renderer, f64, f64, Cf64, Cf64) -> Color,
        e: fn(&Renderer, Cf64, Cf64, Cf64) -> bool,
    ) -> Functs {
        Functs {
            iter_funct: a,
            init_funct: b,
            cmap_funct: c,
            color_funct: d,
            conditional: e,
        }
    }
}
pub struct Renderer {
    pub args: Args,
    pub width: i32,
    pub height: i32,
    pub functs: Functs,
    pub image: Img8,
    pub raw: Vec<Vec<Color>>,
    pub rendered_samples: usize,
    pub not_rendering: bool,
    pub texture: Img8,
}

impl Renderer {
    pub fn new(args: Args, functs: Functs) -> Renderer {
        Renderer {
            args: args.clone(),
            width: args.width,
            height: args.height,
            functs,
            image: Img8::new(args.width as u32, args.height as u32),
            raw: vec![
                vec![Color::new([0f64; 4], ColorType::SRgba); args.width as usize];
                args.height as usize
            ],
            rendered_samples: 0,
            not_rendering: true,
            texture: Img8::new(0, 0),
        }
    }

    pub fn resize(&mut self, w: usize, h: usize) {
        self.args.width = w as i32;
        self.args.height = h as i32;
        self.width = w as i32;
        self.height = h as i32;
        self.image = Img8::new(w as u32, h as u32);
        self.raw = vec![vec![Color::new([0f64; 4], ColorType::SRgba); w]; h];
        self.rendered_samples = 0;
    }

    pub fn pixel(&self, i: i32, samples: usize) -> Color {
        let mut out = Color::new([0.0; 4], ColorType::SRgba);
        let d: Cf64 = normalize_coords(1, 1, self.width, self.height, self.args.zoom)
            - normalize_coords(0, 0, self.width, self.height, self.args.zoom);
        let mut rng = rand::thread_rng();
        for _ in 0..samples {
            let mut c = normalize_coords(
                i / self.height,
                i % self.height,
                self.width,
                self.height,
                self.args.zoom,
            ) + self.args.origin;
            c.re += d.re * (rng.gen_range(-1.0..1.0) / self.args.sampled);
            c.im += d.im * (rng.gen_range(-1.0..1.0) / self.args.sampled);
            c = (self.functs.cmap_funct)(c);
            let mut z = (self.functs.init_funct)(self.args.z_init, c);
            let mut i = 0.0;
            let mut s = 0.0;
            let mut der = Cf64::new(1.0, 0.0);
            let mut tot_der = Cf64::new(1.0, 0.0);
            let dc = Cf64::new(1.0, 0.0);
            let mut test = z;
            let mut old = z;
            let chk = d.re.min(d.im) * 0.5;

            let mut period = 1;
            while (self.functs.conditional)(self, z, der, tot_der) && i < self.args.limit {
                tot_der += der;
                der = (der * 2.0 * z) + dc;
                z = (self.functs.iter_funct)(z, c, (self.functs.cmap_funct)(self.args.julia));
                i += 1.0;
                s += (-(abs(z + 1.0))).exp();

                let dif = z - old;
                if dif.re.abs() < chk && dif.im.abs() < chk {
                    i = self.args.limit;
                    s = self.args.limit;
                }

                period += 1;
                if period > self.args.cycles {
                    period = 0;
                    old = z;
                }
                test += z;
            }

            let mut color = (self.functs.color_funct)(self, i, s, z, der);

            color = color.to_sRgba();
            if i < self.args.limit {
                out = out + (color * color);
            } else {
                out = out + (self.args.set_color * self.args.set_color);
            }
        }
        out
    }
    #[allow(clippy::needless_late_init)]
    pub fn render_samples(&mut self, samples: usize, progress: bool) {
        let now = SystemTime::now();
        self.not_rendering = false;
        let out: Vec<Color>;
        if progress {
            let progress = Mutex::new(Progress::new());
            let bar: Bar = progress
                .lock()
                .unwrap()
                .bar((self.width * self.height) as usize, "");
            out = (0..(self.width * self.height))
                .into_par_iter()
                .map(|i| {
                    progress.lock().unwrap().inc_and_draw(&bar, 1);
                    Renderer::pixel(self, i as i32, samples)
                })
                .collect();
        } else {
            out = (0..(self.width * self.height))
                .into_par_iter()
                .map(|i| Renderer::pixel(self, i as i32, samples))
                .collect();
        }

        for (i, e) in out.iter().enumerate() {
            let (x, y) = (
                (i as i32 / (self.height)) as u32,
                (i as i32 % (self.height)) as u32,
            );
            if (y as i32) < self.height {
                if self.rendered_samples > 0 {
                    self.raw[y as usize][x as usize] = self.raw[y as usize][x as usize] + *e;
                } else {
                    self.raw[y as usize][x as usize] = *e;
                }
            }
        }
        println!("{:4.4}", now.elapsed().unwrap().as_secs_f32());
        self.rendered_samples += samples;
        self.not_rendering = true;
    }

    pub fn process_image(&mut self) {
        for i in 0..(self.width * self.height) {
            let (x, y) = (
                (i as i32 / (self.height)) as u32,
                (i as i32 % (self.height)) as u32,
            );
            if (y as i32) < self.height {
                let e = self.raw[y as usize][x as usize] / self.rendered_samples as f64;
                self.image.put_pixel(
                    x,
                    y,
                    Rgba::from(
                        e.to_Rgba()
                            .to_arr()
                            .map(|v| (v.sqrt() * u8::MAX as f64) as u8),
                    ),
                );
            }
        }
    }

    pub fn update_args(&mut self, args: Args) {
        self.args = args.clone();
        self.width = args.width;
        self.height = args.height;
    }

    pub fn update_functs(&mut self, functs: Functs) {
        self.functs = functs;
    }
}
