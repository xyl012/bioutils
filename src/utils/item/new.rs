
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

use super::*;

pub trait NewRandomBio {
    /// Create new random DNA sequence with specified number of bases.
    fn random_dna(nbases: &usize) -> Vec<u8>;
    /// Create new random RNA sequence with specified number of bases.
    fn random_rna(nbases: &usize) -> Vec<u8>;
    /// Create new random AA sequence with specified number of bases.
    fn random_aa(nbases: &usize) -> Vec<u8>;
    /// Create new random PHRED33 sequence with specified number of bases.
    fn random_quality(nbases: &usize) -> Vec<u8>;
    /// Create new random PHRED64 sequence with specified number of bases.
    fn random_quality_phred64(nbases: &usize) -> Vec<u8>;
}

impl NewRandomBio for Vec<u8>
{
    /// Create new random DNA sequence with specified number of bases.
    fn random_dna(nbases: &usize) -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let mut vec = Vec::with_capacity(*nbases);
        for _base in 0..*nbases {
            vec.push(DNA.choose(&mut rng).expect("Could not make random DNA sequence").to_owned())
        };
        vec
    }
    /// Create new random DNA sequence with specified number of bases.
    fn random_rna(nbases: &usize) -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let mut vec = Vec::with_capacity(*nbases);
        for _base in 0..*nbases {
            vec.push(RNA.choose(&mut rng).expect("Could not make random RNA sequence").to_owned())
        };
        vec
    }
    /// Create new random amino acid sequence with specified number of amino acids.
    fn random_aa(nbases: &usize) -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let mut vec = Vec::with_capacity(*nbases);
        for _base in 0..*nbases {
            vec.push(AMINO_ACID.choose(&mut rng).expect("Could not make random amino acid sequence").to_owned())
        };
        vec
    }
    /// Create new random amino acid sequence with specified number of amino acids.
    fn random_quality(nbases: &usize) -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let mut vec = Vec::with_capacity(*nbases);
        for _base in 0..*nbases {
            vec.push(PHRED33.choose(&mut rng).expect("Could not make random PHRED33 sequence").to_owned())
        };
        vec
    }
    /// Create new random amino acid sequence with specified number of amino acids.
    fn random_quality_phred64(nbases: &usize) -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let mut vec = Vec::with_capacity(*nbases);
        for _base in 0..*nbases {
            vec.push(PHRED64.choose(&mut rng).expect("Could not make random PHRED64 sequence").to_owned())
        };
        vec
    }
}

/// Create new random DNA sequence with specified number of bases.
pub fn random_dna(nbases: &usize, mut rng: ThreadRng) -> Vec<u8> {
    let mut vec = Vec::with_capacity(*nbases);
    for _base in 0..*nbases {
        vec.push(DNA.choose(&mut rng).expect("Could not make random dna").to_owned())
    };
    vec
}
/// Create new random RNA sequence with specified number of bases.
pub fn random_rna(nbases: usize, mut rng: ThreadRng) -> Vec<u8> {
    let mut vec = Vec::with_capacity(nbases);
    for _base in 0..nbases {
        vec.push(RNA.choose(&mut rng).expect("Could not make random RNA sequence").to_owned())
    }
    vec
}
/// Create new random amino acid sequence with specified number of amino acids.
pub fn random_aa(nbases: usize, mut rng: ThreadRng) -> Vec<u8> {
    let mut vec = Vec::with_capacity(nbases);
    for _base in 0..nbases {
        vec.push(AMINO_ACID.choose(&mut rng).expect("Could not make random amino acid sequence").to_owned())
    }
    vec
}
/// Create new random quality sequence (phred33) with specified number of characters.
pub fn random_quality(nbases: usize, mut rng: ThreadRng) -> Vec<u8> {
    let mut vec = Vec::with_capacity(nbases);
    for _base in 0..nbases {
        vec.push(PHRED33.choose(&mut rng).expect("Could not make random PHRED33 sequence").to_owned())
    };
    vec
}
/// Create new random quality sequence (phred64) with specified number of characters.
pub fn random_phred64_quality(nbases: usize, mut rng: ThreadRng) -> Vec<u8> {
    let mut vec = Vec::with_capacity(nbases);
    for _base in 0..nbases {
        vec.push(PHRED64.choose(&mut rng).expect("Could not make random PHRED64 sequence").to_owned())
    };
    vec
}
