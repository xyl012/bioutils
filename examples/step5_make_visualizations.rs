// Copyright (c) 2021 Kana LLC



use image::{ImageBuffer, Rgb};
use image::RgbImage;
const WIDTH: u32 = 100;
const HEIGHT: u32 = 100;
use std::convert::TryInto;

fn main() {
    let mut img = RgbImage::new(HEIGHT, WIDTH);
    let cola = Rgb([255,0,0]); // Adenosine color
    let colc = Rgb([0,255,0]); // Cytosine color
    let colt = Rgb([0,0,255]); // Thymine color
    let colg = Rgb([255,255,0]); // Guanine color
    let seq = b"CCCCCCAAAAAAATTTTTTGGGGG";
    println!("{:?}", seq);
    for (x, &c) in seq.into_iter().enumerate() {
        match c {
            67 => img.put_pixel(x as u32, 50, cola), // 67 is b"A"
            65 => img.put_pixel(x as u32, 50, colc), // 65 is b"C" 
            84 => img.put_pixel(x as u32, 50, colt), // 84 is b"T"
            71 => img.put_pixel(x as u32, 50, colg), // 71 is b"G"
            _ => img.put_pixel(x as u32, 50, Rgb([0,0,0])),
        };
    }
    // Same with random sequences for the whole file
    
    // write it out to a file
    img.save("output.png").unwrap();
}
