// fracgen
// Main
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
use fracgen::{Args, Functions, Functs, Renderer};
use num::Complex;
use std::{path, time::Instant};
type Cf64 = Complex<f64>;
fn main() {
    let args = Args::parse();
    let name = format!(
        "out{}{}_{}x{}-{}_s{}-{}-f{}-c{}-d{}.png",
        path::MAIN_SEPARATOR,
        args.name,
        args.width,
        args.height,
        args.zoom,
        args.samples,
        args.sampled,
        args.fractal_mode,
        args.color_mode,
        args.bail_mode,
    );
    println!("Now processing {} with {} threads...", name, args.threads);
    rayon::ThreadPoolBuilder::new()
        .num_threads(args.threads)
        .build_global()
        .unwrap();
    let now = Instant::now();
    let mut mandelbrot = Renderer::new(
        args.clone(),
        Functs::new(
            match args.fractal_mode {
                0 => match args.is_julia {
                    true => |z, _, j| z * z + j,
                    false => |z, c, _| z * z + c,
                },
                1 => match args.is_julia {
                    true => |z, _, j| {
                        Cf64::new(z.re.abs(), z.im.abs()) * Cf64::new(z.re.abs(), z.im.abs()) + j
                    },
                    false => |z, c, _| {
                        Cf64::new(z.re.abs(), z.im.abs()) * Cf64::new(z.re.abs(), z.im.abs()) + c
                    },
                },
                2 => match args.is_julia {
                    true => |z, _, j| {
                        let mut tz = z * z + j;
                        tz = Cf64::new(tz.re.abs(), tz.im.abs())
                            * Cf64::new(tz.re.abs(), tz.im.abs())
                            + j;
                        tz = tz * tz + j;
                        tz = tz * tz + j;
                        tz
                    },
                    false => |z, c, _| {
                        let mut tz = z * z + c;
                        tz = Cf64::new(tz.re.abs(), tz.im.abs())
                            * Cf64::new(tz.re.abs(), tz.im.abs())
                            + c;
                        tz = tz * tz + c;
                        tz = tz * tz + c;
                        tz
                    },
                },
                3 => match args.is_julia {
                    true => |z, _, j| (z * j).powc(z / j) + (z / j),
                    false => |z, c, _| (z * c).powc(z / c) + (z / c),
                },
                _ => match args.is_julia {
                    true => |z, _, j| z * z + j,
                    false => |z, c, _| z * z + c,
                },
            },
            move |z, _| z,
            Functions::identity,
            match args.color_mode {
                0 => Functions::coloring,
                1 => Functions::normal_map,
                2 => Functions::miles_coloring,
                3 => Functions::miles_coloring2,
                _ => Functions::coloring,
            },
            match args.bail_mode {
                0 => Functions::default_bail,
                1 => Functions::sqrt_bail,
                2 => Functions::der_bail,
                _ => Functions::default_bail,
            },
        ),
    );
    mandelbrot.render_samples(args.samples, true);
    mandelbrot.process_image();
    mandelbrot.image.save(&name).unwrap();
    // output.save(&name).unwrap();
    let notif = format!("Finished in: {}ms!", now.elapsed().as_millis());
    println!(
        "{},{},{},{}",
        &args.set_color.ch[0], &args.set_color.ch[1], &args.set_color.ch[2], &args.set_color.ch[3]
    );

    println!("{}", notif);
}
