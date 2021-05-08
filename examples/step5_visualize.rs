use image::{ImageBuffer, Rgb};
use image::RgbImage;
const WIDTH: u32 = 10;
const HEIGHT: u32 = 10;

fn main() {

    let mut img = RgbImage::new(HEIGHT, WIDTH);
    
    for x in 15..=17 {
        for y in 8..24 {
            img.put_pixel(x, y, Rgb([255, 0, 0]));
            img.put_pixel(y, x, Rgb([255, 0, 0]));
        }
    }
    // // set a central pixel to white
    // image.get_pixel_mut(5, 5).data = [255, 255, 255];

    // // write it out to a file
    img.save("output.png").unwrap();
}