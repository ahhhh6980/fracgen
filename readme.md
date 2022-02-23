# fracmd

## Cpu fractal rendering run from the command prompt!

Currently only supports the mandelbrot set, but formula specification will be the next update.

![](mandelbrot_1920x1680-0.7_s4-2.png)

## To compile
`cargo build --release`

## To use
`./fracmd -w 1920 -h 1680 -name mandelbrot -threads 4 -zoom 0.7 -limit 256.0 -bail 4.0`

## Commands
filename: `-name str`
* example: `-name mandelbrot`
* example: `-n mandelbrot`

threads: `-threads x`
* example: `-threads 4`
* example: `-t 4`

width: `-w x`
* example: `-w 1920`

height: `-h x`
* example: `-h 1680`

origin: `-origin x,y`
* example: `-origin -0.75,0.0`
* example: `-o -0.75,0.0`

set coloring: `-set_color r,g,b,a`
* example: `-set_color 0,0,0,255`

zoom: `-zoom x`
* example: `-zoom 0.7`
* example: `-z 0.7`

samples: `-samples x`
* example: `-samples 4`

sample distance: `-sample_d x`
* example: `-sample_d 2.0`

iteration limit: `-limit x`
* example: `-limit 256.0`
* example: `-l 256.0`

iter bailout: `-bail x`
* example: `-bail 4.0`
* example: `-b 4.0`

color exponent: `-c_exp x`
* example: `-c_exp 1.0`

