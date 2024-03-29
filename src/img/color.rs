
//! Common colloqially named colors for use with rust image to create visualizations of nucleotide-level data
//! We recommend using the colorous colorous if you want to use a gradient as it also is simple to convert the color struct to an Rgb struct in the image colorous
//! Colors are released here in case the user wants to specify their colors or need to add colors
//! These can simply be used with the image colorous:
//! 
//! ``
//! use bioutils::files::image::color::*;
//! use image::RgbImage;
//! let mut img = RgbImage::new(50, 50); // Make a new image with height and width
//! img.put_pixel(x as u32, y as u32, RED_RGB);
//! ``


use image::{Rgb};

/// General colors as rgb structs with common names

/// Basic Colors
/// Hex Code: #000000
pub const BLACK_RGB: image::Rgb<u8> = Rgb(BLACK_U8);
pub const BLACK_U8: [u8;3] = [0,0,0];
/// Hex Code: #FFFFFF
pub const WHITE_RGB: image::Rgb<u8> = Rgb(WHITE_U8); 
pub const WHITE_U8: [u8;3] = [255,255,255];
/// Hex Code: #FF0000
pub const RED_RGB: image::Rgb<u8> = Rgb(RED_U8); 
pub const RED_U8: [u8;3] = [255,0,0];
/// Hex Code: #00FF00
pub const LIME_RGB: image::Rgb<u8> = Rgb(LIME_U8);
pub const LIME_U8: [u8;3] = [0,255,0];
/// Hex Code: #0000FF
pub const BLUE_RGB: image::Rgb<u8> = Rgb(BLUE_U8);
pub const BLUE_U8: [u8;3] = [0,0,255];
/// Hex Code: #FFFF00
pub const YELLOW_RGB: image::Rgb<u8> = Rgb(YELLOW_U8);
pub const YELLOW_U8: [u8;3] = [255,255,0];
/// Hex Code: #00FFFF
pub const CYAN_RGB: image::Rgb<u8> = Rgb(CYAN_U8);
pub const CYAN_U8: [u8;3] = [0,255,255];
/// Hex Code: #FF00FF
pub const MAGENTA_RGB: image::Rgb<u8> = Rgb(MAGENTA_U8);
pub const MAGENTA_U8: [u8;3] = [255,0,255];
/// Hex Code: #C0C0C0
pub const SILVER_RGB: image::Rgb<u8> = Rgb(SILVER_U8);
pub const SILVER_U8: [u8;3] = [192,192,192];
/// Hex Code: #808080
pub const GRAY_RGB: image::Rgb<u8> = Rgb(GRAY_U8);
pub const GRAY_U8: [u8;3] = [128,128,128];
/// Hex Code: #800000
pub const MAROON_RGB: image::Rgb<u8> = Rgb(MAROON_U8);
pub const MAROON_U8: [u8;3] = [128,0,0];
/// Hex Code: #808000
pub const OLIVE_RGB: image::Rgb<u8> = Rgb(OLIVE_U8);
pub const OLIVE_U8: [u8;3] = [128,128,0];
/// Hex Code: #008000
pub const GREEN_RGB: image::Rgb<u8> = Rgb(GREEN_U8);
pub const GREEN_U8: [u8;3] = [0,128,0];
/// Hex Code: #800080
pub const PURPLE_RGB: image::Rgb<u8> = Rgb(PURPLE_U8);
pub const PURPLE_U8: [u8;3] = [128,0,128];
/// Hex Code: #008080
pub const TEAL_RGB: image::Rgb<u8> = Rgb(TEAL_U8);
pub const TEAL_U8: [u8;3] = [0,128,128];
/// Hex Code: #000080
pub const NAVY_RGB: image::Rgb<u8> = Rgb(NAVY_U8);
pub const NAVY_U8: [u8;3] = [0,0,128];

/// Expanded colors
/// Hex Code: #8B0000
pub const DARK_RED_RGB: image::Rgb<u8> = Rgb(DARK_RED_U8);
pub const DARK_RED_U8: [u8;3] = [139,0,0];
/// Hex Code: #A52A2A
pub const BROWN_RGB: image::Rgb<u8> = Rgb(BROWN_U8);
pub const BROWN_U8: [u8;3] = [165,42,42];
/// Hex Code: #B22222
pub const FIREBRICK_RGB: image::Rgb<u8> = Rgb(FIREBRICK_U8);
pub const FIREBRICK_U8: [u8;3] = [178,34,34];
/// Hex Code: #DC143C
pub const CRIMSON_RGB: image::Rgb<u8> = Rgb(CRIMSON_U8);
pub const CRIMSON_U8: [u8;3] = [220,20,60];
/// Hex Code: #FF6347
pub const TOMATO_RGB: image::Rgb<u8> = Rgb(TOMATO_U8);
pub const TOMATO_U8: [u8;3] = [255,99,71];
/// Hex Code: #FF7F50
pub const CORAL_RGB: image::Rgb<u8> = Rgb(CORAL_U8);
pub const CORAL_U8: [u8;3] = [255,127,80];
/// Hex Code: #CD5C5C
pub const INDIAN_RGB: image::Rgb<u8> = Rgb(INDIAN_U8);
pub const INDIAN_U8: [u8;3] = [205,92,92];
/// Hex Code: #F08080
pub const LIGHT_CORAL_RGB: image::Rgb<u8> = Rgb(LIGHT_CORAL_U8);
pub const LIGHT_CORAL_U8: [u8;3] = [240,128,128];
/// Hex Code: #E9967A
pub const DARK_SALMON_RGB: image::Rgb<u8> = Rgb(DARK_SALMON_U8);
pub const DARK_SALMON_U8: [u8;3] = [233,150,122];
/// Hex Code: #FA8072
pub const SALMON_RGB: image::Rgb<u8> = Rgb(SALMON_U8);
pub const SALMON_U8: [u8;3] = [250,128,114];
/// Hex Code: #FFA07A
pub const LIGHT_SALMON_RGB: image::Rgb<u8> = Rgb(LIGHT_SALMON_U8);
pub const LIGHT_SALMON_U8: [u8;3] = [250,160,122];
/// Hex Code: #FF4500
pub const ORANGE_RED_RGB: image::Rgb<u8> = Rgb(ORANGE_RED_U8);
pub const ORANGE_RED_U8: [u8;3] = [255,69,0];
/// Hex Code: #FF8C00
pub const DARK_ORANGE_RGB: image::Rgb<u8> = Rgb(DARK_ORANGE_U8);
pub const DARK_ORANGE_U8: [u8;3] = [255,140,0];
/// Hex Code: #FFA500
pub const ORANGE_RGB: image::Rgb<u8> = Rgb(ORANGE_U8);
pub const ORANGE_U8: [u8;3] = [255,165,0];
/// Hex Code: #FFD700
pub const GOLD_RGB: image::Rgb<u8> = Rgb(GOLD_U8);
pub const GOLD_U8: [u8;3] = [255,215,0];
/// Hex Code: #B8860B
pub const DARK_GOLDENROD_RGB: image::Rgb<u8> = Rgb(DARK_GOLDENROD_U8);
pub const DARK_GOLDENROD_U8: [u8;3] = [184,134,11];
/// Hex Code: #DAA520
pub const GOLDENROD_RGB: image::Rgb<u8> = Rgb(GOLDENROD_U8);
pub const GOLDENROD_U8: [u8;3] = [218,165,32];
/// Hex Code: #EEE8AA
pub const PALE_GOLDENROD_RGB: image::Rgb<u8> = Rgb(PALE_GOLDENROD_U8);
pub const PALE_GOLDENROD_U8: [u8;3] = [238,232,170];
/// Hex Code: #BDB76B
pub const DARK_KHAKI_RGB: image::Rgb<u8> = Rgb(DARK_KHAKI_U8);
pub const DARK_KHAKI_U8: [u8;3] = [189,183,107];
/// Hex Code: #F0E68C
pub const KHAKI_RGB: image::Rgb<u8> = Rgb(KHAKI_U8);
pub const KHAKI_U8: [u8;3] = [240,230,140];
/// Hex Code: #9ACD32
pub const YELLOW_GREEN_RGB: image::Rgb<u8> = Rgb(YELLOW_GREEN_U8);
pub const YELLOW_GREEN_U8: [u8;3] = [154,205,50];
/// Hex Code: #556B2F
pub const DARK_OLIVE_GREEN_RGB: image::Rgb<u8> = Rgb(DARK_OLIVE_GREEN_U8);
pub const DARK_OLIVE_GREEN_U8: [u8;3] = [85,107,47];
/// Hex Code: #6B8E23
pub const OLIVE_DRAB_RGB: image::Rgb<u8> = Rgb(OLIVE_DRAB_U8);
pub const OLIVE_DRAB_U8: [u8;3] = [107,142,35];
/// Hex Code: #7CFC00
pub const LAWN_GREEN_RGB: image::Rgb<u8> = Rgb(LAWN_GREEN_U8);
pub const LAWN_GREEN_U8: [u8;3] = [124,252,0];
/// Hex Code: #7FFF00
pub const CHARTREUSE_RGB: image::Rgb<u8> = Rgb(CHARTREUSE_U8);
pub const CHARTREUSE_U8: [u8;3] = [127,255,0];
/// Hex Code: #ADFF2F
pub const GREEN_YELLOW_RGB: image::Rgb<u8> = Rgb(GREEN_YELLOW_U8);
pub const GREEN_YELLOW_U8: [u8;3] = [173,255,47];
/// Hex Code: #006400
pub const DARK_GREEN_RGB: image::Rgb<u8> = Rgb(DARK_GREEN_U8);
pub const DARK_GREEN_U8: [u8;3] = [0,100,0];
/// Hex Code: #228B22
pub const FOREST_GREEN_RGB: image::Rgb<u8> = Rgb(FOREST_GREEN_U8);
pub const FOREST_GREEN_U8: [u8;3] = [34,139,34];
/// Hex Code: #32CD32
pub const LIME_GREEN_RGB: image::Rgb<u8> = Rgb(LIME_GREEN_U8);
pub const LIME_GREEN_U8: [u8;3] = [50,205,50];
/// Hex Code: #90EE90
pub const LIGHT_GREEN_RGB: image::Rgb<u8> = Rgb(LIGHT_GREEN_U8);
pub const LIGHT_GREEN_U8: [u8;3] = [152,251,152];
/// Hex Code: #98FB98
pub const PALE_GREEN_RGB: image::Rgb<u8> = Rgb(PALE_GREEN_U8);
pub const PALE_GREEN_U8: [u8;3] = [152,251,152];
/// Hex Code: #8FBC8F
pub const DARK_SEA_GREEN_RGB: image::Rgb<u8> = Rgb(DARK_SEA_GREEN_U8);
pub const DARK_SEA_GREEN_U8: [u8;3] = [143,188,143];
/// Hex Code: #00FA9A
pub const MEDIUM_SPRING_RGB: image::Rgb<u8> = Rgb(MEDIUM_SPRING_U8);
pub const MEDIUM_SPRING_U8: [u8;3] = [0,250,154];
/// Hex Code: #00FF7F
pub const SPRING_GREEN_RGB: image::Rgb<u8> = Rgb(SPRING_GREEN_U8);
pub const SPRING_GREEN_U8: [u8;3] = [0,255,127];
/// Hex Code: #2E8B57
pub const SEA_GREEN_RGB: image::Rgb<u8> = Rgb(SEA_GREEN_U8);
pub const SEA_GREEN_U8: [u8;3] = [46,139,87];
/// Hex Code: #66CDAA
pub const MEDIUM_AQUAMARINE_RGB: image::Rgb<u8> = Rgb(MEDIUM_AQUAMARINE_U8);
pub const MEDIUM_AQUAMARINE_U8: [u8;3] = [102,205,170];
/// Hex Code: #3CB371
pub const MEDIUM_SEA_GREEN_RGB: image::Rgb<u8> = Rgb(MEDIUM_SEA_GREEN_U8);
pub const MEDIUM_SEA_GREEN_U8: [u8;3] = [60,179,113];
/// Hex Code: #20B2AA
pub const LIGHT_SEA_GREEN_RGB: image::Rgb<u8> = Rgb(LIGHT_SEA_GREEN_U8);
pub const LIGHT_SEA_GREEN_U8: [u8;3] = [32,178,170];
/// Hex Code: #2F4F4F
pub const DARK_SLATE_GRAY_RGB: image::Rgb<u8> = Rgb(DARK_SLATE_GRAY_U8);
pub const DARK_SLATE_GRAY_U8: [u8;3] = [47,79,79];
/// Hex Code: #008B8B
pub const DARK_CYAN_RGB: image::Rgb<u8> = Rgb(DARK_CYAN_U8);
pub const DARK_CYAN_U8: [u8;3] = [0,139,139];
/// Hex Code: #E0FFFF
pub const LIGHT_CYAN_RGB: image::Rgb<u8> = Rgb(LIGHT_CYAN_U8);
pub const LIGHT_CYAN_U8: [u8;3] = [224,255,255];
/// Hex Code: #00CED1
pub const DARK_TURQOISE_RGB: image::Rgb<u8> = Rgb(DARK_TURQOISE_U8);
pub const DARK_TURQOISE_U8: [u8;3] = [64,224,208];
/// Hex Code: #40E0D0
pub const TURQOISE_RGB: image::Rgb<u8> = Rgb(TURQOISE_U8);
pub const TURQOISE_U8: [u8;3] = [64,224,208];
/// Hex Code: #48D1CC
pub const MEDIUM_TURQOISE_RGB: image::Rgb<u8> = Rgb(MEDIUM_TURQOISE_U8);
pub const MEDIUM_TURQOISE_U8: [u8;3] = [72,209,204];
/// Hex Code: #AFEEEE
pub const PALE_TURQOISE_RGB: image::Rgb<u8> = Rgb(PALE_TURQOISE_U8);
pub const PALE_TURQOISE_U8: [u8;3] = [178,238,238];
/// Hex Code: #7FFFD4
pub const AQUAMARINE_RGB: image::Rgb<u8> = Rgb(AQUAMARINE_U8);
pub const AQUAMARINE_U8: [u8;3] = [127,255,212];
/// Hex Code: #B0E0E6
pub const POWDER_BLUE_RGB: image::Rgb<u8> = Rgb(POWDER_BLUE_U8);
pub const POWDER_BLUE_U8: [u8;3] = [176,224,230];
/// Hex Code: #5F9EA0
pub const CADET_BLUE_RGB: image::Rgb<u8> = Rgb(CADET_BLUE_U8);
pub const CADET_BLUE_U8: [u8;3] = [95,158,160];
/// Hex Code: #4682B4
pub const STEEL_BLUE_RGB: image::Rgb<u8> = Rgb(STEEL_BLUE_U8);
pub const STEEL_BLUE_U8: [u8;3] = [70,130,180];
/// Hex Code: #6495ED
pub const CORN_FLOWER_BLUE_RGB: image::Rgb<u8> = Rgb(CORN_FLOWER_BLUE_U8);
pub const CORN_FLOWER_BLUE_U8: [u8;3] = [100,149,237];
/// Hex Code: #00BFFF
pub const DEEP_SKY_RGB: image::Rgb<u8> = Rgb(DEEP_SKY_U8);
pub const DEEP_SKY_U8: [u8;3] = [0,191,255];
/// Hex Code: #1E90FF
pub const DODGER_BLUE_RGB: image::Rgb<u8> = Rgb(DODGER_BLUE_U8);
pub const DODGER_BLUE_U8: [u8;3] = [30,144,255];
/// Hex Code: #ADD8E6
pub const LIGHT_BLUE_RGB: image::Rgb<u8> = Rgb(LIGHT_BLUE_U8);
pub const LIGHT_BLUE_U8: [u8;3] = [173,216,230];
/// Hex Code: #87CEEB
pub const SKY_BLUE_RGB: image::Rgb<u8> = Rgb(SKY_BLUE_U8);
pub const SKY_BLUE_U8: [u8;3] = [135,206,235];
/// Hex Code: #87CEFA
pub const LIGHT_SKY_BLUE_RGB: image::Rgb<u8> = Rgb(LIGHT_SKY_BLUE_U8);
pub const LIGHT_SKY_BLUE_U8: [u8;3] = [135,206,250];
/// Hex Code: #191970
pub const MIDNIGHT_BLUE_RGB: image::Rgb<u8> = Rgb(MIDNIGHT_BLUE_U8);
pub const MIDNIGHT_BLUE_U8: [u8;3] = [25,25,112];
/// Hex Code: #00008B
pub const DARK_BLUE_RGB: image::Rgb<u8> = Rgb(DARK_BLUE_U8);
pub const DARK_BLUE_U8: [u8;3] = [0,0,139];
/// Hex Code: #0000CD
pub const MEDIUM_BLUE_RGB: image::Rgb<u8> = Rgb(MEDIUM_BLUE_U8);
pub const MEDIUM_BLUE_U8: [u8;3] = [0,0,205];
/// Hex Code: #4169E1
pub const ROYAL_BLUE_RGB: image::Rgb<u8> = Rgb(ROYAL_BLUE_U8);
pub const ROYAL_BLUE_U8: [u8;3] = [65,105,225];
/// Hex Code: #8A2BE2
pub const BLUE_VIOLET_RGB: image::Rgb<u8> = Rgb(BLUE_VIOLET_U8);
pub const BLUE_VIOLET_U8: [u8;3] = [138,43,226];
/// Hex Code: #4B0082
pub const INDIGO_RGB: image::Rgb<u8> = Rgb(INDIGO_U8);
pub const INDIGO_U8: [u8;3] = [75,0,130];
/// Hex Code: #483D8B
pub const DARK_SLATE_BLUE_RGB: image::Rgb<u8> = Rgb(DARK_SLATE_BLUE_U8);
pub const DARK_SLATE_BLUE_U8: [u8;3] = [72,61,139];
/// Hex Code: #6A5ACD
pub const SLATE_BLUE_RGB: image::Rgb<u8> = Rgb(SLATE_BLUE_U8);
pub const SLATE_BLUE_U8: [u8;3] = [106,90,205];
/// Hex Code: #7B68EE
pub const MEDIUM_SLATE_BLUE_RGB: image::Rgb<u8> = Rgb(MEDIUM_SLATE_BLUE_U8);
pub const MEDIUM_SLATE_BLUE_U8: [u8;3] = [123,104,238];
/// Hex Code: #9370DB
pub const MEDIUM_PURPLE_RGB: image::Rgb<u8> = Rgb(MEDIUM_PURPLE_U8);
pub const MEDIUM_PURPLE_U8: [u8;3] = [147,112,219];
/// Hex Code: #8B008B
pub const DARK_MAGENTA_RGB: image::Rgb<u8> = Rgb(DARK_MAGENTA_U8);
pub const DARK_MAGENTA_U8: [u8;3] = [139,0,139];
/// Hex Code: #9400D3
pub const DARK_VIOLET_RGB: image::Rgb<u8> = Rgb(DARK_VIOLET_U8);
pub const DARK_VIOLET_U8: [u8;3] = [148,0,211];
/// Hex Code: #9932CC
pub const DARK_ORCHID_RGB: image::Rgb<u8> = Rgb(DARK_ORCHID_U8);
pub const DARK_ORCHID_U8: [u8;3] = [153,50,204];
/// Hex Code: #BA55D3
pub const MEDIUM_ORCHID_RGB: image::Rgb<u8> = Rgb(MEDIUM_ORCHID_U8);
pub const MEDIUM_ORCHID_U8: [u8;3] = [186,85,211];
/// Hex Code: #D8BFD8
pub const THISTLE_RGB: image::Rgb<u8> = Rgb(THISTLE_U8);
pub const THISTLE_U8: [u8;3] = [216,191,216];
/// Hex Code: #DDA0DD
pub const PLUM_RGB: image::Rgb<u8> = Rgb(PLUM_U8);
pub const PLUM_U8: [u8;3] = [221,160,221];
/// Hex Code: #EE82EE
pub const VIOLET_RGB: image::Rgb<u8> = Rgb(VIOLET_U8);
pub const VIOLET_U8: [u8;3] = [238,130,238];
/// Hex Code: #DA70D6
pub const ORCHID_RGB: image::Rgb<u8> = Rgb(ORCHID_U8);
pub const ORCHID_U8: [u8;3] = [218,12,214];
/// Hex Code: #C71585
pub const MEDIUM_VIOLET_RED_RGB: image::Rgb<u8> = Rgb(MEDIUM_VIOLET_RED_U8);
pub const MEDIUM_VIOLET_RED_U8: [u8;3] = [199,21,133];
/// Hex Code: #DB7093
pub const PALE_VIOLET_RED_RGB: image::Rgb<u8> = Rgb(PALE_VIOLET_RED_U8);
pub const PALE_VIOLET_RED_U8: [u8;3] = [219,112,147];
/// Hex Code: #FF1493
pub const DEEP_PINK_RGB: image::Rgb<u8> = Rgb(DEEP_PINK_U8);
pub const DEEP_PINK_U8: [u8;3] = [255,20,147];
/// Hex Code: #FF69B4
pub const HOT_PINK_RGB: image::Rgb<u8> = Rgb(HOT_PINK_U8);
pub const HOT_PINK_U8: [u8;3] = [255,105,180];
/// Hex Code: #FFB6C1
pub const LIGHT_PINK_RGB: image::Rgb<u8> = Rgb(LIGHT_PINK_U8);
pub const LIGHT_PINK_U8: [u8;3] = [255,182,193];
/// Hex Code: #FFC0CB
pub const PINK_RGB: image::Rgb<u8> = Rgb(PINK_U8);
pub const PINK_U8: [u8;3] = [255,192,203];
/// Hex Code: #FAEBD7
pub const ANTIQUE_WHITE_RGB: image::Rgb<u8> = Rgb(ANTIQUE_WHITE_U8);
pub const ANTIQUE_WHITE_U8: [u8;3] = [250,235,215];
/// Hex Code: #F5F5DC
pub const BEIGE_RGB: image::Rgb<u8> = Rgb(BEIGE_U8);
pub const BEIGE_U8: [u8;3] = [245,245,220];
/// Hex Code: #FFE4C4
pub const BISQUE_RGB: image::Rgb<u8> = Rgb(BISQUE_U8);
pub const BISQUE_U8: [u8;3] = [255,225,196];
/// Hex Code: #FFEBCD
pub const BLANCHED_ALMOND_RGB: image::Rgb<u8> = Rgb(BLANCHED_ALMOND_U8);
pub const BLANCHED_ALMOND_U8: [u8;3] = [255,235,205];
/// Hex Code: #F5DEB3
pub const WHEAT_RGB: image::Rgb<u8> = Rgb(WHEAT_U8);
pub const WHEAT_U8: [u8;3] = [245,222,179];
/// Hex Code: #FFF8DC
pub const CORN_SILK_RGB: image::Rgb<u8> = Rgb(CORN_SILK_U8);
pub const CORN_SILK_U8: [u8;3] = [255,248,220];
/// Hex Code: #FFFACD
pub const LEMON_CHIFFON_RGB: image::Rgb<u8> = Rgb(LEMON_CHIFFON_U8);
pub const LEMON_CHIFFON_U8: [u8;3] = [255,250,205];
/// Hex Code: #FAFAD2
pub const LIGHT_GOLDENROD_YELLOW_RGB: image::Rgb<u8> = Rgb(LIGHT_GOLDENROD_YELLOW_U8);
pub const LIGHT_GOLDENROD_YELLOW_U8: [u8;3] = [250,250,210];
/// Hex Code: #FFFFE0
pub const LIGHT_YELLOW_RGB: image::Rgb<u8> = Rgb(LIGHT_YELLOW_U8);
pub const LIGHT_YELLOW_U8: [u8;3] = [255,255,224];
/// Hex Code: #8B4513
pub const SADDLE_BROWN_RGB: image::Rgb<u8> = Rgb(SADDLE_BROWN_U8);
pub const SADDLE_BROWN_U8: [u8;3] = [139,69,19];
/// Hex Code: #A0522D
pub const SIENNA_RGB: image::Rgb<u8> = Rgb(SIENNA_U8);
pub const SIENNA_U8: [u8;3] = [160,82,45];
/// Hex Code: #D2691E
pub const CHOCOLATE_RGB: image::Rgb<u8> = Rgb(CHOCOLATE_U8);
pub const CHOCOLATE_U8: [u8;3] = [210,105,30];
/// Hex Code: #CD853F
pub const PERU_RGB: image::Rgb<u8> = Rgb(PERU_U8);
pub const PERU_U8: [u8;3] = [205,133,63];
/// Hex Code: #F4A460
pub const SANDY_BROWN_RGB: image::Rgb<u8> = Rgb(SANDY_BROWN_U8);
pub const SANDY_BROWN_U8: [u8;3] = [244,164,96];
/// Hex Code: #DEB887
pub const BURLY_WOOD_RGB: image::Rgb<u8> = Rgb(BURLY_WOOD_U8);
pub const BURLY_WOOD_U8: [u8;3] = [222,184,135];
/// Hex Code: #D2B48C
pub const TAN_RGB: image::Rgb<u8> = Rgb(TAN_U8);
pub const TAN_U8: [u8;3] = [210,180,140];
/// Hex Code: #BC8F8F
pub const ROSY_BROWN_RGB: image::Rgb<u8> = Rgb(ROSY_BROWN_U8);
pub const ROSY_BROWN_U8: [u8;3] = [188,143,143];
/// Hex Code: #FFE4B5
pub const MOCCASIN_RGB: image::Rgb<u8> = Rgb(MOCCASIN_U8);
pub const MOCCASIN_U8: [u8;3] = [255,228,181];
/// Hex Code: #FFDEAD
pub const NAVAJO_WHITE_RGB: image::Rgb<u8> = Rgb(NAVAJO_WHITE_U8);
pub const NAVAJO_WHITE_U8: [u8;3] = [255,222,173];
/// Hex Code: #FFDAB9
pub const PEACH_PUFF_RGB: image::Rgb<u8> = Rgb(PEACH_PUFF_U8);
pub const PEACH_PUFF_U8: [u8;3] = [255,218,185];
/// Hex Code: #FFE4E1
pub const MISTY_ROSE_RGB: image::Rgb<u8> = Rgb(MISTY_ROSE_U8);
pub const MISTY_ROSE_U8: [u8;3] = [255,228,225];
/// Hex Code: #FFF0F5
pub const LAVENDER_BLUSH_RGB: image::Rgb<u8> = Rgb(LAVENDER_BLUSH_U8);
pub const LAVENDER_BLUSH_U8: [u8;3] = [255,240,245];
/// Hex Code: #FAF0E6
pub const LINEN_RGB: image::Rgb<u8> = Rgb(LINEN_U8);
pub const LINEN_U8: [u8;3] = [250,240,230];
/// Hex Code: #FDF5E6
pub const OLD_LACE_RGB: image::Rgb<u8> = Rgb(OLD_LACE_U8);
pub const OLD_LACE_U8: [u8;3] = [253,245,230];
/// Hex Code: #FFEFD5
pub const PAPAYA_WHIP_RGB: image::Rgb<u8> = Rgb(PAPAYA_WHIP_U8);
pub const PAPAYA_WHIP_U8: [u8;3] = [255,239,213];
/// Hex Code: #FFF5EE
pub const SEA_SHELL_RGB: image::Rgb<u8> = Rgb(SEA_SHELL_U8);
pub const SEA_SHELL_U8: [u8;3] = [255,245,238];
/// Hex Code: #F5FFFA
pub const MINT_CREAM_RGB: image::Rgb<u8> = Rgb(MINT_CREAM_U8);
pub const MINT_CREAM_U8: [u8;3] = [245,255,250];
/// Hex Code: #708090
pub const SLATE_GRAY_RGB: image::Rgb<u8> = Rgb(SLATE_GRAY_U8);
pub const SLATE_GRAY_U8: [u8;3] = [112,128,144];
/// Hex Code: #778899
pub const LIGHT_SLATE_GRAY_RGB: image::Rgb<u8> = Rgb(LIGHT_SLATE_GRAY_U8);
pub const LIGHT_SLATE_GRAY_U8: [u8;3] = [119,136,153];
/// Hex Code: #B0C4DE
pub const LIGHT_STEEL_BLUE_RGB: image::Rgb<u8> = Rgb(LIGHT_STEEL_BLUE_U8);
pub const LIGHT_STEEL_BLUE_U8: [u8;3] = [176,196,222];
/// Hex Code: #E6E6FA
pub const LAVENDER_RGB: image::Rgb<u8> = Rgb(LAVENDER_U8);
pub const LAVENDER_U8: [u8;3] = [230,230,250];
/// Hex Code: #FFFAF0
pub const FLORAL_WHITE_RGB: image::Rgb<u8> = Rgb(FLORAL_WHITE_U8);
pub const FLORAL_WHITE_U8: [u8;3] = [255,250,240];
/// Hex Code: #F0F8FF
pub const ALICE_BLUE_RGB: image::Rgb<u8> = Rgb(ALICE_BLUE_U8);
pub const ALICE_BLUE_U8: [u8;3] = [240,248,255];
/// Hex Code: #F8F8FF
pub const GHOST_WHITE_RGB: image::Rgb<u8> = Rgb(GHOST_WHITE_U8);
pub const GHOST_WHITE_U8: [u8;3] = [248,248,255];
/// Hex Code: #F0FFF0
pub const HONEYDEW_RGB: image::Rgb<u8> = Rgb(HONEYDEW_U8);
pub const HONEYDEW_U8: [u8;3] = [240,255,240];
/// Hex Code: #FFFFF0
pub const IVORY_RGB: image::Rgb<u8> = Rgb(IVORY_U8);
pub const IVORY_U8: [u8;3] = [255,255,240];
/// Hex Code: #F0FFFF
pub const AZURE_RGB: image::Rgb<u8> = Rgb(AZURE_U8);
pub const AZURE_U8: [u8;3] = [240,255,255];
/// Hex Code: #FFFAFA
pub const SNOW_RGB: image::Rgb<u8> = Rgb(SNOW_U8);
pub const SNOW_U8: [u8;3] = [255,250,250];
/// Hex Code: #696969
pub const DIM_GRAY_RGB: image::Rgb<u8> = Rgb(DIM_GRAY_U8);
pub const DIM_GRAY_U8: [u8;3] = [105,105,105];
/// Hex Code: #A9A9A9
pub const DARK_GRAY_RGB: image::Rgb<u8> = Rgb(DARK_GRAY_U8);
pub const DARK_GRAY_U8: [u8;3] = [169,169,169];
/// Hex Code: #D3D3D3
pub const LIGHT_GRAY_RGB: image::Rgb<u8> = Rgb(LIGHT_GRAY_U8);
pub const LIGHT_GRAY_U8: [u8;3] = [211,211,211];
/// Hex Code: #DCDCDC
pub const GAINSBORO_RGB: image::Rgb<u8> = Rgb(GAINSBORO_U8);
pub const GAINSBORO_U8: [u8;3] = [220,220,220];
/// Hex Code: #F5F5F5
pub const WHITE_SMOKE_RGB: image::Rgb<u8> = Rgb(WHITE_SMOKE_U8);
pub const WHITE_SMOKE_U8: [u8;3] = [245,245,245];
