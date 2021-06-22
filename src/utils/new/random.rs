
//! Functions to make new random u8 biological sequences. Includes dna,rna,aa,quality functions to create u8 vectors.
//! # Examples
//! ```
//! use crate::bioutils::utils::mutate::random::AsMutRandomU8;
//! use crate::bioutils::utils::new::random::random_dna;
//! use crate::bioutils::utils::new::random::random_quality;
//! use rand::rngs::ThreadRng;
//! use std::string::String;
//! use rand::seq::SliceRandom;
//!
//! let mut rng = rand::thread_rng(); //create a random number generator
//! let dna = random_dna(4,rng);
//! let mut rng2 = rand::thread_rng(); //create a random number generator
//! let quality = random_quality(4,rng2);
//! println!("{:?}", dna.to_owned());
//! println!("{:?}", quality.to_owned());
//! ```

use crate::charsets::iupac::*;
use crate::charsets::quality::*;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

/// Create new random DNA sequence with specified number of bases.
pub fn random_dna(nbases: usize, mut rng: ThreadRng) -> Vec<u8> {
    let mut vec = Vec::with_capacity(nbases);
    for _base in 0..nbases {
        vec.push(*BASIC_DNA_U8.choose(&mut rng).unwrap())
    }
    vec
}
/// Create new random RNA sequence with specified number of bases.
pub fn random_rna(nbases: usize, mut rng: ThreadRng) -> Vec<u8> {
    let mut vec = Vec::with_capacity(nbases);
    for _base in 0..nbases {
        vec.push(*BASIC_RNA_U8.choose(&mut rng).unwrap())
    }
    vec
}
/// Create new random amino acid sequence with specified number of amino acids.
pub fn random_aa(nbases: usize, mut rng: ThreadRng) -> Vec<u8> {
    let mut vec = Vec::with_capacity(nbases);
    for _base in 0..nbases {
        vec.push(*BASIC_AMINO_ACID_U8.choose(&mut rng).unwrap())
    }
    vec
}
/// Create new random quality sequence (phred33) with specified number of characters.
pub fn random_quality(nbases: usize, mut rng: ThreadRng) -> Vec<u8> {
    let mut vec = Vec::with_capacity(nbases);
    for _base in 0..nbases {
        vec.push(*PHRED33_U8.choose(&mut rng).unwrap())}
    vec
}
/// Create new random quality sequence (phred64) with specified number of characters.
pub fn random_phred64_quality(nbases: usize, mut rng: ThreadRng) -> Vec<u8> {
    let mut vec = Vec::with_capacity(nbases);
    for _base in 0..nbases {
        vec.push(*PHRED64_U8.choose(&mut rng).unwrap())}
    vec
}
