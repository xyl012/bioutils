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
/// Hex Code: #ADFF2F
pub const GREEN_YELLOW_RGB: image::Rgb<u8> = Rgb([173,255,47]);
/// Hex Code: #006400
pub const DARK_GREEN_RGB: image::Rgb<u8> = Rgb([0,100,0]);
/// Hex Code: #228B22
pub const FOREST_GREEN_RGB: image::Rgb<u8> = Rgb([34,139,34]);
/// Hex Code: #32CD32
pub const LIME_GREEN_RGB: image::Rgb<u8> = Rgb([50,205,50]);
/// Hex Code: #90EE90
pub const LIGHT_GREEN_RGB: image::Rgb<u8> = Rgb([144,238,144]);
/// Hex Code: #98FB98
pub const PALE_GREEN_RGB: image::Rgb<u8> = Rgb([152,251,152]);
/// Hex Code: #8FBC8F
pub const DARK_SEA_GREEN_RGB: image::Rgb<u8> = Rgb([143,188,143]);
/// Hex Code: #00FA9A
pub const MEDIUM_SPRING_RGB: image::Rgb<u8> = Rgb([0,250,154]);
/// Hex Code: #00FF7F
pub const SPRING_GREEN_RGB: image::Rgb<u8> = Rgb([0,255,127]);
/// Hex Code: #2E8B57
pub const SEA_GREEN_RGB: image::Rgb<u8> = Rgb([46,139,87]);
/// Hex Code: #66CDAA
pub const MEDIUM_AQUAMARINE_RGB: image::Rgb<u8> = Rgb([102,205,170]);
/// Hex Code: #3CB371
pub const MEDIUM_SEA_GREEN_RGB: image::Rgb<u8> = Rgb([60,179,113]);
/// Hex Code: #20B2AA
pub const LIGHT_SEA_GREEN_RGB: image::Rgb<u8> = Rgb([32,178,170]);
/// Hex Code: #2F4F4F
pub const DARK_SLATE_GRAY_RGB: image::Rgb<u8> = Rgb([47,79,79]);
/// Hex Code: #008B8B
pub const DARK_CYAN_RGB: image::Rgb<u8> = Rgb([0,139,139]);
/// Hex Code: #E0FFFF
pub const LIGHT_CYAN_RGB: image::Rgb<u8> = Rgb([224,255,255]);
/// Hex Code: #00CED1
pub const DARK_TURQOISE_RGB: image::Rgb<u8> = Rgb([0,206,209]);
/// Hex Code: #40E0D0
pub const TURQOISE_RGB: image::Rgb<u8> = Rgb([64,224,208]);
/// Hex Code: #48D1CC
pub const MEDIUM_TURQOISE_RGB: image::Rgb<u8> = Rgb([72,209,204]);
/// Hex Code: #AFEEEE
pub const PALE_TURQOISE_RGB: image::Rgb<u8> = Rgb([178,238,238]);
/// Hex Code: #7FFFD4
pub const AQUAMARINE_RGB: image::Rgb<u8> = Rgb([127,255,212]);
/// Hex Code: #B0E0E6
pub const POWDER_BLUE_RGB: image::Rgb<u8> = Rgb([176,224,230]);
/// Hex Code: #5F9EA0
pub const CADET_BLUE_RGB: image::Rgb<u8> = Rgb([95,158,160]);
/// Hex Code: #4682B4
pub const STEEL_BLUE_RGB: image::Rgb<u8> = Rgb([70,130,180]);
/// Hex Code: #6495ED
pub const CORN_FLOWER_BLUE_RGB: image::Rgb<u8> = Rgb([100,149,237]);
/// Hex Code: #00BFFF
pub const DEEP_SKY_RGB: image::Rgb<u8> = Rgb([0,191,255]);
/// Hex Code: #1E90FF
pub const DODGER_BLUE_RGB: image::Rgb<u8> = Rgb([30,144,255]);
/// Hex Code: #ADD8E6
pub const LIGHT_BLUE_RGB: image::Rgb<u8> = Rgb([173,216,230]);
/// Hex Code: #87CEEB
pub const SKY_BLUE_RGB: image::Rgb<u8> = Rgb([135,206,235]);
/// Hex Code: #87CEFA
pub const LIGHT_SKY_BLUE_RGB: image::Rgb<u8> = Rgb([135,206,250]);
/// Hex Code: #191970
pub const MIDNIGHT_BLUE_RGB: image::Rgb<u8> = Rgb([25,25,112]);
/// Hex Code: #00008B
pub const DARK_BLUE_RGB: image::Rgb<u8> = Rgb([0,0,139]);
/// Hex Code: #0000CD
pub const MEDIUM_BLUE_RGB: image::Rgb<u8> = Rgb([0,0,205]);
/// Hex Code: #4169E1
pub const ROYAL_BLUE_RGB: image::Rgb<u8> = Rgb([65,105,225]);
/// Hex Code: #8A2BE2
pub const BLUE_VIOLET_RGB: image::Rgb<u8> = Rgb([138,43,226]);
/// Hex Code: #4B0082
pub const INDIGO_RGB: image::Rgb<u8> = Rgb([75,0,130]);
/// Hex Code: #483D8B
pub const DARK_SLATE_BLUE_RGB: image::Rgb<u8> = Rgb([72,61,139]);
/// Hex Code: #6A5ACD
pub const SLATE_BLUE_RGB: image::Rgb<u8> = Rgb([106,90,205]);
/// Hex Code: #7B68EE
pub const MEDIUM_SLATE_BLUE_RGB: image::Rgb<u8> = Rgb([123,104,238]);
/// Hex Code: #9370DB
pub const MEDIUM_PURPLE_RGB: image::Rgb<u8> = Rgb([147,112,219]);
/// Hex Code: #8B008B
pub const DARK_MAGENTA_RGB: image::Rgb<u8> = Rgb([139,0,139]);
/// Hex Code: #9400D3
pub const DARK_VIOLET_RGB: image::Rgb<u8> = Rgb([148,0,211]);
/// Hex Code: #9932CC
pub const DARK_ORCHID_RGB: image::Rgb<u8> = Rgb([153,50,204]);
/// Hex Code: #BA55D3
pub const MEDIUM_ORCHID_RGB: image::Rgb<u8> = Rgb([186,85,211]);
/// Hex Code: #D8BFD8
pub const THISTLE_RGB: image::Rgb<u8> = Rgb([216,191,216]);
/// Hex Code: #DDA0DD
pub const PLUM_RGB: image::Rgb<u8> = Rgb([221,160,221]);
/// Hex Code: #EE82EE
pub const VIOLET_RGB: image::Rgb<u8> = Rgb([238,130,238]);
/// Hex Code: #DA70D6
pub const ORCHID_RGB: image::Rgb<u8> = Rgb([218,12,214]);
/// Hex Code: #C71585
pub const MEDIUM_VIOLET_RED_RGB: image::Rgb<u8> = Rgb([199,21,133]);
/// Hex Code: #DB7093
pub const PALE_VIOLET_RED_RGB: image::Rgb<u8> = Rgb([219,112,147]);
/// Hex Code: #FF1493
pub const DEEP_PINK_RGB: image::Rgb<u8> = Rgb([255,20,147]);
/// Hex Code: #FF69B4
pub const HOT_PINK_RGB: image::Rgb<u8> = Rgb([255,105,180]);
/// Hex Code: #FFB6C1
pub const LIGHT_PINK_RGB: image::Rgb<u8> = Rgb([255,182,193]);
/// Hex Code: #FFC0CB
pub const PINK_RGB: image::Rgb<u8> = Rgb([255,192,203]);
/// Hex Code: #FAEBD7
pub const ANTIQUE_WHITE_RGB: image::Rgb<u8> = Rgb([250,235,215]);
/// Hex Code: #F5F5DC
pub const BEIGE_RGB: image::Rgb<u8> = Rgb([245,245,220]);
/// Hex Code: #FFE4C4
pub const BISQUE_RGB: image::Rgb<u8> = Rgb([255,225,196]);
/// Hex Code: #FFEBCD
pub const BLANCHED_ALMOND_RGB: image::Rgb<u8> = Rgb([255,235,205]);
/// Hex Code: #F5DEB3
pub const WHEAT_RGB: image::Rgb<u8> = Rgb([245,222,179]);
/// Hex Code: #FFF8DC
pub const CORN_SILK_RGB: image::Rgb<u8> = Rgb([255,248,220]);
/// Hex Code: #FFFACD
pub const LEMON_CHIFFON_RGB: image::Rgb<u8> = Rgb([255,250,205]);
/// Hex Code: #FAFAD2
pub const LIGHT_GOLDENROD_YELLOW_RGB: image::Rgb<u8> = Rgb([250,250,210]);
/// Hex Code: #FFFFE0
pub const LIGHT_YELLOW_RGB: image::Rgb<u8> = Rgb([255,255,224]);
/// Hex Code: #8B4513
pub const SADDLE_BROWN_RGB: image::Rgb<u8> = Rgb([139,69,19]);
/// Hex Code: #000000
pub const _RGB: image::Rgb<u8> = Rgb([0,0,0]);
/// Hex Code: #000000
pub const _RGB: image::Rgb<u8> = Rgb([0,0,0]);
/// Hex Code: #000000
pub const _RGB: image::Rgb<u8> = Rgb([0,0,0]);



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
