
//! Example to make nucleotide-level visualizations quickly and easily with image and character functions
//! This example colors each pixel by a vec<u8>, but we assume that the vector is a raw vector of u8s. We mean that the quality string is just read literally from the file. Check if the IO crate you use converts the quality u8s to scores, or if you have to convert them.
//! We convert the u8s to their actual quality score (we assume phred33, e.g. u8-33).
//! We then take the vector and color a png with it.

use bioutils::utils::image::color::*;
use bioutils::charsets::quality::*;
use bioutils::utils::new::random::random_quality;
use bioutils::utils::mutate::item::MutCodeItemU8;

use image::{ImageBuffer, Rgb};
use image::RgbImage;
const WIDTH: u32 = 50;
const HEIGHT: u32 = 50;

fn main() {
    let mut img = RgbImage::new(HEIGHT, WIDTH); // Make a new image with height and width
    let rng = rand::thread_rng(); // Create a random number generator
    let mut qual = random_quality(50,rng); // Get a random phred33 quality vector of length 50, make it mutable so we can get our quality scores from the raw encoding
    println!("Quality integers are: {:?}", qual); // Print our unsigned integers for reference
    // Get what the actual qualities are by lookup with a hashmap (simply with values shifted 33 assuming phred33 encoding)
    let scores = qual.mut_decode_qual().to_owned(); // get our quality scores from raw encoding and create an owned copy. We don't have to create an owned copy, this is just for demonstration.
    println!("Quality scores are: {:?}", scores); // Print our unsigned integers for reference
    let y = 30; // Since we have a single quality let's just choose a y coordinate
    // Color each pixel by the score with increasing rgb. Add 100 so it's easier to see
    for (x, c) in scores.into_iter().enumerate() {
    img.put_pixel(x as u32, y, Rgb([c+100,c+100,c+100]))
    }
    // write it out to a file
    img.save("output3.png").unwrap();
}