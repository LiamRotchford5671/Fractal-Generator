// Copyright © 2019 Liam Rotchford, Simon Barton
//Generate 3 a randomly chosen julia set fractal as a .png image
//base code credited to: https://crates.io/crates/image
//resource on julia_set fractals: https://en.wikipedia.org/wiki/Julia_set#Pseudocode_for_normal_Julia_sets


use ::image::Rgb;
use rand::Rng;


pub fn pixel_setter((complex_x, complex_y): (f32, f32), mut iteration: u64) -> u64 {
    
    let mut rng = rand::thread_rng();
    let randjulia = rng.gen_range(1, 3);

    //determine which julia_set fractal will be generated
    let complex_num = match randjulia {
    	1 => num::Complex::new(-0.4, 0.6),
    	2 => num::Complex::new(0.285, 0.01),
    	3 => num::Complex::new(-0.7269, 0.1889),
    	_ => num::Complex::new(-0.4, 0.6),
    };
	
    let mut value = num::Complex::new(complex_x, complex_y);

    while iteration < 255 && value.norm() <= 2.0 {
        //the julia fractal
        value = value * value + complex_num;

        iteration += 1;
    }

    iteration
}

pub fn julia_fractal(imgy: u32, imgx: u32, filename: &str, scheme: &str) {
    let scaleset = ((3.0 / imgx as f32), (3.0 / imgy as f32));

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
  
    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = Rgb([((0.3 * x as f32) as u8), 0, ((0.3 * y as f32) as u8)]);
    }

    for x in 0..imgx {
        for y in 0..imgy {
            let complex_pos = ((y as f32 * scaleset.0 - 1.5), (x as f32 * scaleset.1 - 1.5)); //determines position in frame

            let result = pixel_setter(complex_pos, 0);

            let pixel = imgbuf.get_pixel_mut(x, y);

            let Rgb(data) = *pixel;

            if scheme == "color" {
                *pixel = Rgb([data[0], result as u8, data[2]]);
            } else {
                *pixel = Rgb([result as u8, result as u8, result as u8]);
            }
        }
    }

    // Save the image
    imgbuf.save(filename).unwrap();
}
