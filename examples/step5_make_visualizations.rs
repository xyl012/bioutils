// Copyright (c) 2021 Kana LLC

//! Example to make nucleotide-level visualizations quickly and easily with image and character functions

use bioutils::files::image::color::*;
use image::RgbImage;
const WIDTH: u32 = 50;
const HEIGHT: u32 = 50;

use bioutils::utils::new::random::random_dna;

fn main() {
    let mut img = RgbImage::new(HEIGHT, WIDTH); // Make a new image with height and width
    let seq = b"CCCCAAAAAATTTTTGGGGNNNNN"; // Make a sequence
    let y_pixel_coord=49; // Set our y coordinate to 49 for first example. Since our height is set at 50 and we start at 0, if we set 50 it'll be out of bounds and rust will panic.
    println!("{:?}", seq); // Print our unsigned integers for reference
    for (x, &c) in seq.into_iter().enumerate() { // For each unsigned integer in our sequence, color the pixel corresponding to the x coordinate and y coordinate, color by the unsigned integer present.
        match c {
            67 => img.put_pixel(x as u32, y_pixel_coord, RED_RGB), // 67 is b"A", set Adenosine color
            65 => img.put_pixel(x as u32, y_pixel_coord, GREEN_RGB), // 65 is b"C", set Cytosine color
            84 => img.put_pixel(x as u32, y_pixel_coord, BLUE_RGB), // 84 is b"T", set Thymine color
            71 => img.put_pixel(x as u32, y_pixel_coord, PURPLE_RGB), // 71 is b"G", set Guanine color
            78 => img.put_pixel(x as u32, y_pixel_coord, GRAY_RGB), // 110 is b"N", set Undetermined color
            _ => img.put_pixel(x as u32, y_pixel_coord, BLACK_RGB), // 71 is b"G", set all other as black
        };
    }
    // write it out to a file
    img.save("output1.png").unwrap();

    // Let's do the same thing with random sequences for the whole file
    for y in 0..HEIGHT {
        let rng = rand::thread_rng(); // Create a random number generator
        let dna = random_dna(50,rng); // Get a random dna sequence of length 50
        for (x, c) in dna.into_iter().enumerate() {
            match c {
                67 => img.put_pixel(x as u32, y, RED_RGB), // 67 is b"A", set Adenosine color
                65 => img.put_pixel(x as u32, y, GREEN_RGB), // 65 is b"C", set Cytosine color
                84 => img.put_pixel(x as u32, y, BLUE_RGB), // 84 is b"T", set Thymine color
                71 => img.put_pixel(x as u32, y, CYAN_RGB), // 71 is b"G", set Guanine color
                78 => img.put_pixel(x as u32, y, GRAY_RGB), // 110 is b"N", set Undetermined color
                _ => img.put_pixel(x as u32, y, BLACK_RGB), // 71 is b"G", set all other as black
            };
        }
    }

    // write it out to a file
    img.save("output2.png").unwrap();
}
