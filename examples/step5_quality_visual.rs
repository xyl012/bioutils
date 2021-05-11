// Copyright (c) 2021 Kana LLC

//! Example to make nucleotide-level visualizations quickly and easily with image and character functions
//! This example colors each pixel by a vec<u8>, but we assume that the vector is a raw vector of u8s. We mean that the quality string is just read literally from the file. Check if the IO crate you use converts the quality u8s to scores, or if you have to convert them.
//! We convert the u8s to their actual quality score (we assume phred33, e.g. u8-33).
//! We then take the vector and color a png with it.



use bioutils::files::image::color::*;
use image::RgbImage;
const WIDTH: u32 = 50;
const HEIGHT: u32 = 50;

use bioutils::utils::new::random::random_dna;


fn main() {
    let mut img = RgbImage::new(HEIGHT, WIDTH); // Make a new image with height and width
    
    println!("{:?}", qual); // Print our unsigned integers for reference

}