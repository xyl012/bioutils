// Copyright 2020 Christopher Sugai

//! Other functions that do not fall into the general categories of checking, replacing, or generating a new sequence. 
//! # Examples
//! # Check the input sequence for symbols other than ACTG and Replace with pseudorandom bases. 
//! # Generate all sequences with hamming distance 1 using the bases ACTG.
//! 
//! 
//! 

use crate::charsets::iupac::*;
use crate::charsets::quality::*;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

// Encodes a u8 vector of bytes with information from the sequence as well as quality. Each byte being phred33-33 or {0..40}+{128..255}, which is the phred33 score plus ascii

pub fn qs_bytes<'a>(bytes_1: &'a mut Vec<u8>,bytes_2: &'a mut Vec<u8>)-> &'a mut Vec<u8> {

    let mut bytes_1_i = bytes_1.len() - 1;
    let mut bytes_2_i = bytes_2.len() - 1;

    while bytes_1_i > 0 {
        bytes_1[bytes_1_i] = bytes_1[bytes_1_i] + bytes_2[bytes_1_i] - 33 as u8;
        if bytes_1_i == 0 { break; }
        bytes_1_i -= 1;
    }
    bytes_1
}

// let x = b"ABCD";
// let y = b"1010";
// println!(b"BBDD");
// println!(b"BBDD");

// /// Take in a sequence string and create a vector of sequence strings with hamming distance 1 using the bases ACTG. Requires the sequence to be ACTGs, use replace if N.- or other symbols present.
// // Example: AAAA -> CAAA GAAA TAAA ACAA AGAA ATAA etc.
// pub fn nucleotide_set_hamming(nucl: String) -> Vec<String>  {
//     let mut rng = rand::thread_rng();
//     for base in 0 .. nucl.len() + 1 {
//     let results: Vec<String> = Vec::new();
//         for symbol in BASIC_DNA_U8.iter() {
//             let (first, last) = symbol.split_at(base);
//             let mut buffer = [0; 1];
//             let result = symbol.encode_utf8(&mut buffer);
//             results.push([first, result, last].concat());
//         }
//     results
//     }
// }
// //-> Vec<String> 
// // /// A function that returns the correction for the specified word.
// // pub fn correct(&mut self, word: &str) -> String {
// //     // A word in our word frequency map is already correct.
// //     if self.n_words.contains_key(word) {
// //         return word.to_string();
// //     }

