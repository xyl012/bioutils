// Copyright (c) 2021 Kana LLC



use bioutils::files::image::color::*;
use image::{ImageBuffer, Rgb};
use image::RgbImage;
const WIDTH: u32 = 100;
const HEIGHT: u32 = 100;
use std::convert::TryInto;


fn main() {
    let mut img = RgbImage::new(HEIGHT, WIDTH);
    let cola = Rgb([255,0,0]); 
    let colc = Rgb([0,255,0]); 
    let colt = Rgb([0,0,255]);
    let colg = Rgb([255,255,0]);
    let seq = b"CCCCAAAAAATTTTTGGGGNNNNN";
    println!("{:?}", seq);
    for (x, &c) in seq.into_iter().enumerate() {
        match c {
            67 => img.put_pixel(x as u32, 50, RED_RGB), // 67 is b"A", set Adenosine color
            65 => img.put_pixel(x as u32, 50, GREEN_RGB), // 65 is b"C", set Cytosine color
            84 => img.put_pixel(x as u32, 50, BLUE_RGB), // 84 is b"T", set Thymine color
            71 => img.put_pixel(x as u32, 50, PURPLE_RGB), // 71 is b"G", set Guanine color
            110 => img.put_pixel(x as u32, 50, GRAY_RGB), // 110 is b"N", set Undetermined color
            _ => img.put_pixel(x as u32, 50, BLACK_RGB), // 71 is b"G", set all other as black
        };
    }
    // Same with random sequences for the whole file
    
    // write it out to a file
    img.save("output.png").unwrap();
}
