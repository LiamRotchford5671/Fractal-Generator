// Copyright © 2019 Liam Rotchford, Simon Barton

#![allow(dead_code)]

use crate::util::Color::*;
use image::*;
use std::io;
use std::str::FromStr;

/// Main mechanism for user interaction
/// Allows user to generate fractal in three ways
pub fn user_menu(mut scheme: &mut Scheme) {
    let mut input = String::new();
    println!("normal, custom, or random fractal generation?");
    io::stdin()
        .read_line(&mut input)
        .ok()
        .expect("Expected good input");

    let trimmed: &str = input.trim();

    match trimmed {
        "normal" => normal_menu(&mut scheme),
        "custom" => {
            println!("We're glad you chose customization message\n\n");
            normal_menu(&mut scheme);
            custom_menu(&mut scheme)
        }
        "random" => _randomize(&mut scheme),
        _ => println!("Unrecognized input... running default."),
    }
}

/// The normal option allows a user to select only the color
/// of the fractal they will generate.
pub fn normal_menu(mut scheme: &mut Scheme) {
    let mut input = String::new();
    if scheme.fractal == "barnsley".to_string() {
        println!("What color fractal? (ROYGBIV)");
    } else {
        println!("What color fractal? red, green, blue, or white?");
    }
    io::stdin().read_line(&mut input).ok();
    scheme.color = str_to_color(input.trim());
}

/// The custom option allows a user to fine tune properties
/// of the fractal art image.
pub fn custom_menu(mut scheme: &mut Scheme) {
    let mut buffer = String::new();
    let std = io::stdin();

    let mut finished: bool = false;
    while !finished {
        println!("Select an item to customize by its ID number:");
        println!("    1. Background color (solid)\n    2. Background color (transition)\n    'quit' to quit\n");
        std.read_line(&mut buffer).ok();

        match buffer.trim() {
            "1" => {
                println!("What color background would you like? ");
                io::stdin().read_line(&mut buffer).ok();
                scheme.bg_color = str_to_color(buffer.trim());
            },
            "2" => {
                scheme.fancy_background = true;
                println!("Choose your first color: red, green, blue: ");
                io::stdin().read_line(&mut buffer).ok();
                scheme.bg_color = str_to_color(buffer.trim());
                buffer.clear();
                println!("Choose one of the remaining two: red, green, blue: ");
                io::stdin().read_line(&mut buffer).ok();
                scheme.bg_color_2 = str_to_color(buffer.trim());
                println!("Assummed good input...");
            }
            "quit" => finished = true,
            _ => println!("Invalid input: {:?}. Enter a number (1, 2, ..)", buffer),
        }
        buffer.clear();
    }
}

pub fn _randomize(_scheme: &mut Scheme) {}

/// Supported colors for user input
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    Red,
    Orange,
    Yellow,
    Blue,
    Green,
    Violet,
    Black,
    White,
}

/// Container for properties of fractal being built
pub struct Scheme {
    pub fractal: String,
    /// Actual color of the fractal
    pub color: Color,
    pub fancy_background: bool,
    pub bg_color: Color,
    pub bg_color_2: Color,
    // enum Transformation
}

impl Default for Scheme {
    fn default() -> Scheme {
        Scheme {
            fractal: "mandelbrot".to_string(),
            color: Green,
            fancy_background: false,
            bg_color: Black,
            bg_color_2: Red,
        }
    }
}

/// Helper to return three u8s based on parsed color
/// u8s function as RGB data
pub fn color_to_rgb(color: &Color) -> [u8; 3] {
    match color {
        Red => [255, 0, 0],
        Orange => [255, 165, 0],
        Yellow => [255, 255, 0],
        Blue => [0, 0, 255],
        Green => [0, 128, 0],
        Violet => [238, 130, 238],
        Black => [0, 0, 0],
        White => [255, 255, 255],
    }
}

/// Convenient conversion from String to a Color
/// Defaults to Blue for invalid input colors
pub fn str_to_color(color: &str) -> Color {
    match color {
        "red" => Red,
        "orange" => Orange,
        "yellow" => Yellow,
        "blue" => Blue,
        "green" => Green,
        "violet" => Violet,
        "black" => Black,
        "white" => White,
        &_ => Blue,
    }
}

/// Iterate over the pixels of the image and apply a cool
/// background, which will depend on scheme.
/// Either transitioning from one color to another
/// or just a solid background.
pub fn apply_background(imgbuf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, scheme: &Scheme) {
    let color: [u8; 3] = color_to_rgb(&scheme.bg_color);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        if scheme.fancy_background {
            match scheme.bg_color {
                Red => match scheme.bg_color_2 {
                    Blue => *pixel = Rgb([((0.3 * x as f32) as u8), 0, ((0.3 * y as f32) as u8)]),
                    Green => *pixel = Rgb([((0.3 * x as f32) as u8), ((0.3 * y as f32) as u8), 0]),
                    _ => println!("Unsupported bg_color_2"),
                },
                Green => match scheme.bg_color_2 {
                    Blue => *pixel = Rgb([0, ((0.3 * x as f32) as u8), ((0.3 * y as f32) as u8)]),
                    Red => *pixel = Rgb([((0.3 * x as f32) as u8), ((0.3 * y as f32) as u8), 0]),
                    _ => println!("Unsupported bg_color_2"),
                },
                Blue => match scheme.bg_color_2 {
                    Red => *pixel = Rgb([((0.3 * x as f32) as u8), 0, ((0.3 * y as f32) as u8)]),
                    Green => *pixel = Rgb([0, ((0.3 * x as f32) as u8), ((0.3 * y as f32) as u8)]),
                    _ => println!("Unsupported bg_color_2"),
                },
                _ => println!("Unsupported bg_color"),
            }
        } else {
            *pixel = Rgb([color[0], color[1], color[2]]);
        }
    }
}

/// Helper to parse a string as a pair of values separated
/// by a separator char.
pub fn parse_pair<T: FromStr>(s: &str, sep: char) -> Option<(T, T)> {
    let fields: Vec<&str> = s.split(sep).collect();
    if fields.len() != 2 {
        return None;
    }
    match (T::from_str(fields[0]), T::from_str(fields[1])) {
        (Ok(f0), Ok(f1)) => Some((f0, f1)),
        _ => None,
    }
}