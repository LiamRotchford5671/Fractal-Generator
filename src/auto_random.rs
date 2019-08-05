// Copyright © 2019 Liam Rotchford, Simon Barton

use crate::barnsley::*;
use crate::julia_sets::*;
use crate::mandelbrot::*;
use crate::util::*;
use rand::Rng;

const FRACTALS: [&str; 4] = ["barnsley", "mandelbrot", "julia", "multi-julia"];

pub fn auto_random(num_to_make: usize, filename: &str) { 
    for i in 0..num_to_make+1 {
        let fractal_index = rand::thread_rng().gen_range(0, 4);
        let fractal = FRACTALS[fractal_index];
        let filename = filename.to_owned() + &i.to_string() + ".png";

        let mut scheme = Scheme {
             fractal: fractal.to_string(),
             ..Default::default()
        };

        randomize(&mut scheme);
        match fractal {
            "barnsley" => barnsley_fern(700, 700, &filename, &mut scheme),
            "julia" | "multi-julia" => julia_fractal(fractal, 700, 700, &filename, &scheme),
            "mandelbrot" => mandelbrot_fractal(700, 700, &filename, &mut scheme),
            _ => println!("Unsupported fractal type"),
        }

        random_transforms(&scheme, &filename);
    }
}