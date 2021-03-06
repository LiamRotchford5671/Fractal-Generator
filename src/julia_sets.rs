// Copyright © 2019 Liam Rotchford, Simon Barton
//! Julia Sets and Multi-Julia Set Implementations

//Generate a randomly chosen julia set fractal as a .png image
//base code credited to: https://crates.io/crates/image
//resource on julia_set fractals: https://en.wikipedia.org/wiki/Julia_set#Pseudocode_for_normal_Julia_sets

use crate::julias::*;

use crate::util::Color::*;
use crate::util::*;
use image::Rgba;
use rand::Rng;

///Julia_fractal is a middle man function for both the julia sets fractal and the multi julia sets fractal. This fuction handles
///the generation of the intial background image and then cycles through each pixel in the image. Sending the pixel to the appropriate
///function in julias.rs based on the fractal type to run through the correct formula to alter the pixel and draw the fractal.

pub fn julia_fractal(julia_type: &str, imgy: u32, imgx: u32, filename: &str, scheme: &Scheme) {
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy); // Create a new ImgBuf and apply our dimensions to it.
    let scaleset = ((3.0 / imgx as f32), (3.0 / imgy as f32));

    apply_background(&mut imgbuf, &scheme); //set the intial background of the image based on the users choice

    let mut rng = rand::thread_rng();
    let randjulia = match julia_type {
        //determine random value that will choose which julia set will be generated
        "julia" => rng.gen_range(1, 11),
        "multi-julia" => rng.gen_range(2, 8),
        _ => rng.gen_range(1, 11),
    };

    //cycle through every pixel, send to fractal formula function pixel_setter, set the pixel based on result of that function
    for x in 0..imgx {
        for y in 0..imgy {
            let complex_pos = ((y as f32 * scaleset.0 - 1.5), (x as f32 * scaleset.1 - 1.5)); //determines position in frame

            let result = match julia_type {
                "julia" => pixel_setter(complex_pos, 0, randjulia), //run pixel through fractal formula in Julias.rs
                "multi-julia" => pixel_set_multi(complex_pos, 0, randjulia),
                _ => pixel_setter(complex_pos, 0, randjulia), //default is normal julia set
            };

            let pixel = imgbuf.get_pixel_mut(x, y); //pull out pixel data
            let Rgba(data) = *pixel; //set pixel data onto the rgb array

            match scheme.color {
                //apply the pixel shade on the result from pixel setter
                Red => *pixel = Rgba([result as u8, data[1], data[2], 255]), //apply it to the channel the user chose
                Green => *pixel = Rgba([data[0], result as u8, data[2], 255]),
                Blue => *pixel = Rgba([data[0], data[1], result as u8, 255]),
                White => *pixel = Rgba([result as u8, result as u8, result as u8, 255]),
                _ => panic!("Unsupported color"),
            }
        }
    }

    // Save the image
    imgbuf.save(filename).unwrap();
}
