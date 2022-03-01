// fracgen
// Main File
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

use clap::Parser;
use image::{ImageBuffer, Rgba};
use num::complex::Complex;
use rand::Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{path, time::Instant};

mod color;
use color::Color;

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, default_value = "1920")]
    width: i32,

    #[clap(short, long, default_value = "1680")]
    height: i32,

    #[clap(short, long, default_value = "mandelbrot")]
    name: String,

    #[clap(short, long, default_value_t=((num_cpus::get() as f32) * 0.5).ceil() as usize)]
    threads: usize,

    #[clap(short, long, default_value_t=Complex::<f32>::new(-0.75,0.0))]
    origin: Complex<f32>,

    #[clap(short, long, default_value = ".7")]
    zoom: f32,

    #[clap(short, long, default_value = "4")]
    samples: usize,

    #[clap(short, long, default_value = "2.0")]
    sampled: f32,

    #[clap(short, long, default_value = "256.0")]
    limit: f32,

    #[clap(short, long, default_value = "16.0")]
    bail: f32,

    #[clap(short, long, default_value = "1.0")]
    cexp: f32,

    #[clap(short, long, default_value = "0,0,0,255")]
    set_color: Color,
}

fn abs(z: Complex<f32>) -> f32 {
    z.re * z.re + z.im * z.im
}

fn normalize_coords(x: i32, y: i32, w: i32, h: i32, z: f32) -> Complex<f32> {
    let nx = 2.0 * (x as f32 / w as f32) - 1.0;
    let ny = 2.0 * (y as f32 / h as f32) - 1.0;
    Complex::new(nx / z, ny * (h as f32 / w as f32) / z)
}

pub struct Functs {
    iter_funct: fn(Complex<f32>, Complex<f32>) -> Complex<f32>,
    init_funct: fn(Complex<f32>) -> Complex<f32>,
    cmap_funct: fn(z: Complex<f32>) -> Complex<f32>,
    color_funct: fn(f32, f32, Complex<f32>, f32, f32) -> Color,
}

pub struct Renderer {
    args: Args,
    width: i32,
    height: i32,
    functs: Functs,
}

impl Renderer {
    pub fn new(args: Args, functs: Functs) -> Renderer {
        Renderer {
            args: args.clone(),
            width: args.width,
            height: args.height,
            functs: functs,
        }
    }

    pub fn pixel(&self, i: i32) -> Rgba<u16> {
        let mut out = Color::new(0.0);
        let d: Complex<f32> = normalize_coords(1, 1, self.width, self.height, self.args.zoom)
            - normalize_coords(0, 0, self.width, self.height, self.args.zoom);
        let mut rng = rand::thread_rng();
        for _ in 0..self.args.samples {
            let mut c = normalize_coords(
                i / self.height,
                i % self.height,
                self.width,
                self.height,
                self.args.zoom,
            ) + self.args.origin;
            c.re += d.re * (rng.gen_range(-1.0..1.0) / self.args.sampled);
            c.im += d.im * (rng.gen_range(-1.0..1.0) / self.args.sampled);
            let c = (self.functs.cmap_funct)(c);
            let mut z = (self.functs.init_funct)(c);
            let mut i = 0.0;
            let mut s = 0.0;
            while (abs(z) < self.args.bail) && i < self.args.limit {
                z = (self.functs.iter_funct)(z, c);
                i += 1.0;
                s = s + (-(abs(z))).exp();
            }

            let mut color = (self.functs.color_funct)(i, s, z, self.args.limit, self.args.cexp);

            color = color.to_sRGBA();
            if i < self.args.limit {
                out = out + (color * color);
            } else {
                out = out + (self.args.set_color * self.args.set_color);
            }
        }
        out = out / self.args.samples as f32;
        Rgba::from(
            out.to_RGBA()
                .to_arr()
                .map(|v| (v.sqrt() * u16::MAX as f32) as u16),
        )
    }

    pub fn render(&self) -> ImageBuffer<Rgba<u16>, Vec<u16>> {
        let mut output =
            ImageBuffer::<Rgba<u16>, Vec<u16>>::new(self.width as u32, self.height as u32);
        let out: Vec<Rgba<u16>> = (0..(self.width * self.height))
            .into_par_iter()
            .map(|i| Renderer::pixel(self, i as i32))
            .collect();
        for (i, e) in out.iter().enumerate() {
            //println!(e);
            let (x, y) = (
                (i as i32 / (self.height)) as u32,
                (i as i32 % (self.height)) as u32,
            );
            if (y as i32) < self.height {
                output.put_pixel(x, y, *e);
            }
        }
        output
    }

    pub fn update_args(mut self, args: Args) {
        self.args = args.clone();
        self.width = args.width;
        self.height = args.height;
    }

    pub fn update_functs(mut self, functs: Functs) {
        self.functs = functs;
    }
}

fn coloring(i: f32, s: f32, z: Complex<f32>, limit: f32, cexp: f32) -> Color {
    let hue = ((1.0 - (s / limit)) * 360.0).powf(cexp).powf(1.5);
    let mut color = Color::from_hsv(hue, 0.5, 1.0, 1.0);
    color.ch[3] = 1.0;
    color
}

fn map_complex(z: Complex<f32>) -> Complex<f32> {
    z
}

// fn open_frac<P: AsRef<Path>>(n: P) {
//     open::that(n).unwrap();
// }

fn main() {
    let args = Args::parse();
    let name = format!(
        "out{}{}_{}x{}-{}_s{}-{}.png",
        path::MAIN_SEPARATOR,
        args.name,
        args.width,
        args.height,
        args.zoom,
        args.samples,
        args.sampled
    );
    println!("Now processing {} with {} threads...", name, args.threads);
    rayon::ThreadPoolBuilder::new()
        .num_threads(args.threads)
        .build_global()
        .unwrap();
    // (z / (z-c).sin()).powc(z / c) + c
    // SPADE: (z * c).powc(z / c) + (z / c)
    let now = Instant::now();

    let functs = Functs {
        iter_funct: |z, c| z * z + c,
        init_funct: |c| c,
        cmap_funct: map_complex,
        color_funct: coloring,
    };
    let mandelbrot = Renderer::new(args.clone(), functs);
    let output = mandelbrot.render();
    output.save(&name).unwrap();
    let notif = format!("Finished in: {}ms!", now.elapsed().as_millis());
    println!("{},{},{},{}", &args.set_color.ch[0], &args.set_color.ch[1], &args.set_color.ch[2], &args.set_color.ch[3]);

    println!("{}", notif);
}
