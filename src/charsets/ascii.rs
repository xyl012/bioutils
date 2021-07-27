//! English/ASCII letter sets provided as u8, str arrays, and hashsets.

use super::*;

// Upper and lower case
pub const ASCII_LETTERS_U8: [u8; 52] = [
    b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
    b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd', b'e', b'f',
    b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v',
    b'w', b'x', b'y', b'z',
];
pub const ASCII_LETTERS_STR: [&str; 52] = [
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S",
    "T", "U", "V", "W", "X", "Y", "Z", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l",
    "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z",
];

// pub const ASCII_LETTERS_U8_RANGE = Range {ASCII_LETTERS_U8};

lazy_static! {
    pub static ref ASCII_LETTERS_HASHSET_U8: HashSet<u8> = new_hashset(&ASCII_LETTERS_U8);
}
lazy_static! {
    pub static ref ASCII_LETTERS_HASHSET_STR: HashSet<&'static str> =
        new_str_hashset(&ASCII_LETTERS_STR);
}

// Upper case
pub const ASCII_LETTERS_UPPERCASE_U8: [u8; 26] = [
    b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O', b'P',
    b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z',
];
pub const ASCII_LETTERS_UPPERCASE_STR: [&str; 26] = [
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S",
    "T", "U", "V", "W", "X", "Y", "Z",
];
lazy_static! {
    pub static ref ASCII_LETTERS_UPPERCASE_HASHSET_U8: HashSet<u8> =
        new_hashset(&ASCII_LETTERS_UPPERCASE_U8);
}
lazy_static! {
    pub static ref ASCII_LETTERS_UPPERCASE_HASHSET_STR: HashSet<&'static str> =
        new_str_hashset(&ASCII_LETTERS_UPPERCASE_STR);
}

// Lower case
pub const ASCII_LETTERS_LOWERCASE_U8: [u8; 26] = [
    b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p',
    b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z',
];
pub const ASCII_LETTERS_LOWERCASE_STR: [&str; 26] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s",
    "t", "u", "v", "w", "x", "y", "z",
];
lazy_static! {
    pub static ref ASCII_LETTERS_LOWERCASE_HASHSET_U8: HashSet<u8> =
        new_hashset(&ASCII_LETTERS_LOWERCASE_U8);
}
lazy_static! {
    pub static ref ASCII_LETTERS_LOWERCASE_HASHSET_STR: HashSet<&'static str> =
        new_str_hashset(&ASCII_LETTERS_LOWERCASE_STR);
}
