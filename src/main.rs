// fracmd
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

use argh::FromArgs;
use image::{ImageBuffer, Rgba};
use num::complex::Complex;
use rand::Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{path, time::Instant};

mod rgbaf;
use rgbaf::RgbaF;

#[derive(FromArgs, Clone)]
#[argh(description = "args")]
pub struct Args {
    #[argh(
        option,
        short = 'o',
        from_str_fn(complex_from_str),
        description = "coordinate at center of image",
        default = "Complex::<f32>::new(-0.75,0.0)"
    )]
    origin: Complex<f32>,

    #[argh(option, short = 'z', description = "zoom factor", default = "0.7")]
    zoom: f32,

    #[argh(option, description = "samples per pixel", default = "4")]
    samples: usize,

    #[argh(
        option,
        description = "divisor for sample distance, smaller is blurrier",
        default = "2.0"
    )]
    sample_d: f32,

    #[argh(
        option,
        short = 'l',
        description = "iteration limit",
        default = "256.0"
    )]
    limit: f32,

    #[argh(
        option,
        short = 'b',
        description = "bailout value for z",
        default = "16.0"
    )]
    bail: f32,

    #[argh(
        option,
        description = "color exponent to nudge where colors appear",
        default = "1.0"
    )]
    c_exp: f32,

    #[argh(
        option,
        from_str_fn(color_from_str),
        description = "color of the set",
        default = "RgbaF::new_alpha(0.0,1.0).to_sRGB()"
    )]
    set_color: RgbaF,

    #[argh(option, description = "width", default = "1920")]
    w: i32,

    #[argh(option, description = "height", default = "1680")]
    h: i32,

    #[argh(
        option,
        short = 'n',
        description = "file name",
        default = "String::from(\"mandelbrot\")"
    )]
    name: String,

    #[argh(
        option,
        short = 't',
        description = "threads",
        default = "((num_cpus::get() as f32) * 0.5).ceil() as usize"
    )]
    threads: usize,
}

fn complex_from_str(string: &str) -> Result<Complex<f32>, String> {
    let cols: Vec<f32> = string
        .split(',')
        .map(|x| x.parse::<f32>().unwrap())
        .collect();
    Ok(Complex::<f32>::new(cols[0], cols[1]))
}
fn color_from_str(string: &str) -> Result<RgbaF, String> {
    let cols: Vec<f32> = string
        .split(',')
        .map(|x| x.parse::<f32>().unwrap())
        .collect();
    Ok(RgbaF {
        r: cols[0] / 255.0,
        g: cols[1] / 255.0,
        b: cols[2] / 255.0,
        a: cols[3] / 255.0,
        sRGB: false,
    })
}

pub struct Renderer<F>
where
    F: Fn(Complex<f32>, Complex<f32>) -> Complex<f32> + Send + Sync + 'static,
{
    args: Args,
    w: i32,
    h: i32,
    funct: F,
}

impl<F> Renderer<F>
where
    F: Fn(Complex<f32>, Complex<f32>) -> Complex<f32> + Send + Sync + 'static,
{
    pub fn new(args: Args, funct: F) -> Renderer<F> {
        Renderer {
            args: args.clone(),
            w: args.w,
            h: args.h,
            funct: funct,
        }
    }

    fn abs(z: Complex<f32>) -> f32 {
        z.re * z.re + z.im * z.im
    }

    fn normalize_coords(x: i32, y: i32, w: i32, h: i32, z: f32) -> Complex<f32> {
        let nx = 2.0 * (x as f32 / w as f32) - 1.0;
        let ny = 2.0 * (y as f32 / h as f32) - 1.0;
        Complex::new(nx / z, ny * (h as f32 / w as f32) / z)
    }

    pub fn pixel(&self, i: i32) -> Rgba<u16> {
        let mut out = RgbaF::new(0.0);
        let d: Complex<f32> = Renderer::<F>::normalize_coords(1, 1, self.w, self.h, self.args.zoom)
            - Renderer::<F>::normalize_coords(0, 0, self.w, self.h, self.args.zoom);
        let mut rng = rand::thread_rng();
        for _ in 0..self.args.samples {
            let mut c = Renderer::<F>::normalize_coords(
                i / self.h,
                i % self.h,
                self.w,
                self.h,
                self.args.zoom,
            ) + self.args.origin;
            c.re += d.re * (rng.gen_range(-1.0..1.0) / self.args.sample_d);
            c.im += d.im * (rng.gen_range(-1.0..1.0) / self.args.sample_d);
            let mut z = c.clone();
            let mut i = 0.0;
            let mut s = 0.0;
            while (Renderer::<F>::abs(z) < self.args.bail) && i < self.args.limit {
                z = (self.funct)(z, c);
                i += 1.0;
                s = s + (-(Renderer::<F>::abs(z))).exp();
            }
            let hue = ((1.0 - (s / self.args.limit)) * 360.0)
                .powf(self.args.c_exp)
                .powf(1.5);
            let mut color = RgbaF::from_hsv(hue, 0.5, 1.0, 1.0);
            color.a = 1.0;
            color = color.to_sRGB();
            if i < self.args.limit {
                out = out + (color * color);
            } else {
                out = out + (self.args.set_color * self.args.set_color);
            }
        }
        out = out / self.args.samples as f32;
        Rgba::from(
            out.to_RGB()
                .to_arr()
                .map(|v| (v.sqrt() * u16::MAX as f32) as u16),
        )
    }

    pub fn render(&self) -> ImageBuffer<Rgba<u16>, Vec<u16>> {
        let mut output = ImageBuffer::<Rgba<u16>, Vec<u16>>::new(self.w as u32, self.h as u32);
        let out: Vec<Rgba<u16>> = (0..(self.w * self.h))
            .into_par_iter()
            .map(|i| Renderer::pixel(self, i as i32))
            .collect();
        for (i, e) in out.iter().enumerate() {
            //println!(e);
            let (x, y) = ((i as i32 / (self.h)) as u32, (i as i32 % (self.h)) as u32);
            if (y as i32) < self.h {
                output.put_pixel(x, y, *e);
            }
        }
        output
    }
}

fn main() {
    let args: Args = argh::from_env();
    let name = format!(
        "out{}{}_{}x{}-{}_s{}-{}.png",
        path::MAIN_SEPARATOR,
        args.name,
        args.w,
        args.h,
        args.zoom,
        args.samples,
        args.sample_d
    );
    println!("Now processing {} with {} threads...", name, args.threads);
    rayon::ThreadPoolBuilder::new()
        .num_threads(args.threads)
        .build_global()
        .unwrap();
    let now = Instant::now();
    let mandelbrot = Renderer::new(args.clone(), |z, c| z * z + c);
    let output = mandelbrot.render();
    output.save(name).unwrap();
    println!("Finished in: {}ms!", now.elapsed().as_millis());
}
