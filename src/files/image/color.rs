// Copyright (c) 2021 Kana LLC

//! Common colloqially named colors for use with rust image to create visualizations of nucleotide-level data
//! We recommend using the colorous crate if you want to use a gradient as it also is simple to convert the color struct to an Rgb struct in the image crate
//! Colors are released here in case the user wants to specify their colors or need to add colors

use image::{ImageBuffer, Rgb};

/// General colors as rgb structs with common names

/// Basic Colors

/// Hex Code: #000000
pub const BLACK_RGB: image::Rgb<u8> = Rgb([0,0,0]);
/// Hex Code: #FFFFFF
pub const WHITE_RGB: image::Rgb<u8> = Rgb([255,255,255]); 
/// Hex Code: #FF0000
pub const RED_RGB: image::Rgb<u8> = Rgb([255,0,0]); 
/// Hex Code: #00FF00
pub const LIME_RGB: image::Rgb<u8> = Rgb([0,255,0]);
/// Hex Code: #0000FF
pub const BLUE_RGB: image::Rgb<u8> = Rgb([0,0,255]);
/// Hex Code: #FFFF00
pub const YELLOW_RGB: image::Rgb<u8> = Rgb([255,255,0]);
/// Hex Code: #00FFFF
pub const CYAN_RGB: image::Rgb<u8> = Rgb([0,255,255]);
/// Hex Code: #FF00FF
pub const MAGENTA_RGB: image::Rgb<u8> = Rgb([255,0,255]);
/// Hex Code: #C0C0C0
pub const SILVER_RGB: image::Rgb<u8> = Rgb([192,192,192]);
/// Hex Code: #808080
pub const GRAY_RGB: image::Rgb<u8> = Rgb([128,128,128]);
/// Hex Code: #800000
pub const MAROON_RGB: image::Rgb<u8> = Rgb([128,0,0]);
/// Hex Code: #808000
pub const OLIVE_RGB: image::Rgb<u8> = Rgb([128,128,0]);
/// Hex Code: #008000
pub const GREEN_RGB: image::Rgb<u8> = Rgb([0,128,0]);
/// Hex Code: #800080
pub const PURPLE_RGB: image::Rgb<u8> = Rgb([128,0,128]);
/// Hex Code: #008080
pub const TEAL_RGB: image::Rgb<u8> = Rgb([0,128,128]);
/// Hex Code: #000080
pub const NAVY_RGB: image::Rgb<u8> = Rgb([0,0,128]);

/// Expanded colors
/// Hex Code: #8B0000
pub const DARK_RED_RGB: image::Rgb<u8> = Rgb([139,0,0]);
/// Hex Code: #A52A2A
pub const BROWN_RGB: image::Rgb<u8> = Rgb([165,42,42]);
/// Hex Code: #B22222
pub const FIREBRICK_RGB: image::Rgb<u8> = Rgb([178,34,34]);
/// Hex Code: #DC143C
pub const CRIMSON_RGB: image::Rgb<u8> = Rgb([220,20,60]);
/// Hex Code: #FF6347
pub const TOMATO_RGB: image::Rgb<u8> = Rgb([255,99,71]);
/// Hex Code: #FF7F50
pub const CORAL_RGB: image::Rgb<u8> = Rgb([255,127,80]);
/// Hex Code: #CD5C5C
pub const INDIAN_RGB: image::Rgb<u8> = Rgb([205,92,92]);
/// Hex Code: #F08080
pub const LIGHT_CORAL_RGB: image::Rgb<u8> = Rgb([240,128,128]);
/// Hex Code: #E9967A
pub const DARK_SALMON_RGB: image::Rgb<u8> = Rgb([233,150,122]);
/// Hex Code: #FA8072
pub const SALMON_RGB: image::Rgb<u8> = Rgb([250,128,114]);
/// Hex Code: #FFA07A
pub const LIGHT_SALMON_RGB: image::Rgb<u8> = Rgb([250,160,122]);
/// Hex Code: #FF4500
pub const ORANGE_RED_RGB: image::Rgb<u8> = Rgb([255,69,0]);
/// Hex Code: #FF8C00
pub const DARK_ORANGE_RGB: image::Rgb<u8> = Rgb([255,140,0]);
/// Hex Code: #FFA500
pub const ORANGE_RGB: image::Rgb<u8> = Rgb([255,165,0]);
/// Hex Code: #FFD700
pub const GOLD_RGB: image::Rgb<u8> = Rgb([255,215,0]);
/// Hex Code: #B8860B
pub const DARK_GOLDENROD_RGB: image::Rgb<u8> = Rgb([184,134,11]);
/// Hex Code: #DAA520
pub const GOLDENROD_RGB: image::Rgb<u8> = Rgb([218,165,32]);
/// Hex Code: #EEE8AA
pub const PALE_GOLDENROD_RGB: image::Rgb<u8> = Rgb([238,232,170]);
/// Hex Code: #BDB76B
pub const DARK_KHAKI_RGB: image::Rgb<u8> = Rgb([189,183,107]);
/// Hex Code: #F0E68C
pub const KHAKI_RGB: image::Rgb<u8> = Rgb([240,230,140]);
/// Hex Code: #9ACD32
pub const YELLOW_GREEN_RGB: image::Rgb<u8> = Rgb([154,205,50]);
/// Hex Code: #556B2F
pub const DARK_OLIVE_GREEN_RGB: image::Rgb<u8> = Rgb([85,107,47]);
/// Hex Code: #6B8E23
pub const OLIVE_DRAB_RGB: image::Rgb<u8> = Rgb([107,142,35]);
/// Hex Code: #7CFC00
pub const LAWN_GREEN_RGB: image::Rgb<u8> = Rgb([124,252,0]);
/// Hex Code: #7FFF00
pub const CHARTREUSE_RGB: image::Rgb<u8> = Rgb([127,255,0]);



// for (x, &c) in seq.into_iter().enumerate() {
//     match c {
//         67 => img.put_pixel(x as u32, 50, cola), // 67 is b"A"
//         65 => img.put_pixel(x as u32, 50, colc), // 65 is b"C" 
//         84 => img.put_pixel(x as u32, 50, colt), // 84 is b"T"
//         71 => img.put_pixel(x as u32, 50, colg), // 71 is b"G"
//         _ => img.put_pixel(x as u32, 50, Rgb([0,0,0])),
//     };
// }


// // Basic usage: simple actgn colors

// pub const A_COLOR: image::Rgb<u8> = Rgb([255,0,0]); // A color
// pub const C_COLOR: image::Rgb<u8> = Rgb([0,255,0]); // C color
// pub const T_COLOR: image::Rgb<u8> = Rgb([0,0,255]); // T color
// pub const G_COLOR: image::Rgb<u8> = Rgb([255,255,0]); // G color
// pub const N_COLOR: image::Rgb<u8> = Rgb([0,255,255]); // N color
