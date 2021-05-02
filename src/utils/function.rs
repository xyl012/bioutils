// Copyright 2021 Christopher Sugai

//! Other functions that do not fall into the general categories of checking, replacing, or generating a new sequence. Generally includes statistics and transformations.
//! # Examples
//! # Generate all sequences with hamming distance 1 using the bases ACTG.
//! 
//! 
//! 

use crate::charsets::iupac::*;
use crate::charsets::quality::*;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use std::convert::TryInto;

pub trait PositionU8<T> {
    /// Checks the sequence has the percent bases (rounded) above the quality score
    fn quality_percent_passing(&self, quality_score: &u8) -> usize;
    
/// Get the positions of cgs in a u8 sequence
// pub fn cg_positions(seq:&[u8])-> &[u8] {
//     seq.windows(2).rposition(|&x| x == &seq)
}

// /// Get the positions of sequence in u8 sequence
// pub fn seq_positions(seq:&[u8])-> &[u8] {
    
// }

// }


impl<T> PositionU8<T> for T
where
    for<'a> &'a T: IntoIterator<Item = &'a u8>,
{
    fn quality_percent_passing(&self, quality_score: &u8)-> usize {
        let passing_count = self.into_iter().filter_map(|s| Some(&s>=&quality_score)).count();
        let total_count = self.into_iter().count();
        (100 * passing_count + total_count / 2) / total_count}
    // pub fn cg_positions(seq:&[u8])-> &[u8] {
    //     seq.windows(2).rposition(|&x| x == &seq)
    // }
}


// /// Get the percentage content in a u8 sequence

// /// Get the percentage content of a sequence in a u8 sequence


// // Encodes a u8 vector of bytes with information from the sequence as well as quality. Each byte being phred33-33 or {0..40}+{128..255}, which is the phred33 score plus ascii

pub fn qs_bytes(bytes_1: &mut Vec<u8>,bytes_2: &mut Vec<u8>)-> Vec<u16> {
    let mut bytes_1_i = bytes_1.len() - 1;
    let mut bytes_2_i = bytes_2.len() - 1;
    let mut bytes_1: Vec<u16> = bytes_1.iter().map(|c| *c as u16).collect();
    let mut bytes_2: Vec<u16> = bytes_2.iter().map(|c| *c as u16).collect();
    while bytes_1_i >= 0 {
        bytes_1[bytes_1_i] = (bytes_1[bytes_1_i] - 33) * (bytes_2[bytes_1_i] + 128);
        bytes_1[bytes_1_i];
        if bytes_1_i == 0 { break; }
        bytes_1_i -= 1;
    }
    bytes_1
}

// // pub fn qs_bytes<'a>(bytes_1: &'a mut Vec<u8>,bytes_2: &'a mut Vec<u8>)-> &'a mut Vec<u8> {

// //     let mut bytes_1_i = bytes_1.len() - 1;
// //     let mut bytes_2_i = bytes_2.len() - 1;
// //     while bytes_1_i > 0 {
// //         bytes_1[bytes_1_i] = (bytes_1[bytes_1_i] - 33) * (bytes_2[bytes_1_i] + 128);
// //         bytes_1[bytes_1_i];
// //         if bytes_1_i == 0 { break; }
// //         bytes_1_i -= 1;
// //     }
// //     bytes_1
// // }

// // let x = b"ABCD";
// // let y = b"1010";
// // println!(b"BBDD");
// // println!(b"BBDD");

// // /// Take in a sequence string and create a vector of sequence strings with hamming distance 1 using the bases ACTG. Requires the sequence to be ACTGs, use replace if N.- or other symbols present.
// // // Example: AAAA -> CAAA GAAA TAAA ACAA AGAA ATAA etc.
// // pub fn nucleotide_set_hamming(nucl: String) -> Vec<String>  {
// //     let mut rng = rand::thread_rng();
// //     for base in 0 .. nucl.len() + 1 {
// //     let results: Vec<String> = Vec::new();
// //         for symbol in BASIC_DNA_U8.iter() {
// //             let (first, last) = symbol.split_at(base);
// //             let mut buffer = [0; 1];
// //             let result = symbol.encode_utf8(&mut buffer);
// //             results.push([first, result, last].concat());
// //         }
// //     results
// //     }
// // }
// // //-> Vec<String> 
// // // /// A function that returns the correction for the specified word.
// // // pub fn correct(&mut self, word: &str) -> String {
// // //     // A word in our word frequency map is already correct.
// // //     if self.n_words.contains_key(word) {
// // //         return word.to_string();
// // //     }

