// Copyright 2020 Christopher Sugai
//! Functions to make new random u8 biological sequences. Includes dna,rna,aa,quality functions to create u8 vectors.
//! # Examples
//! ```
//! 
//! ```
use rand::seq::SliceRandom;
use rand::rngs::ThreadRng;
use crate::charsets::iupac::*;
use crate::charsets::quality::*;

pub trait New<T> {
    /// Create new random DNA sequence with specified number of bases.
    fn random_dna(nbases: usize, rng: ThreadRng) -> Vec<u8>;
    /// Create new random RNA sequence with specified number of bases.
    fn random_rna(nbases: usize, rng: ThreadRng) -> Vec<u8>;
    /// Create new random amino acid sequence with specified number of amino acids.
    fn random_aa(nbases: usize, rng: ThreadRng) -> Vec<u8>;
    /// Create new random quality sequence with specified number of characters.
    fn random_quality(nbases: usize, rng: ThreadRng) -> Vec<u8>;
}

impl<T> New<T> for T where for<'a> &'a T: IntoIterator<Item = &'a u8> {
    /// Create new random DNA sequence with specified number of bases.
    fn random_dna(nbases: usize, mut rng: ThreadRng) -> Vec<u8> {
        let mut vec = Vec::with_capacity(nbases);
        for base in 0..nbases {
            vec.push(*BASIC_DNA_U8.choose(&mut rng).unwrap())
        };
        vec
    }
    /// Create new random RNA sequence with specified number of bases.
    fn random_rna(nbases: usize, mut rng: ThreadRng) -> Vec<u8> {
        let mut vec = Vec::with_capacity(nbases);
        for base in 0..nbases {
            vec.push(*BASIC_RNA_U8.choose(&mut rng).unwrap())
        };
        vec
    }
    /// Create new random amino acid sequence with specified number of amino acids.
    fn random_aa(nbases: usize, mut rng: ThreadRng) -> Vec<u8> {
        let mut vec = Vec::with_capacity(nbases);
        for base in 0..nbases {
            vec.push(*BASIC_AMINO_ACID_U8.choose(&mut rng).unwrap())
        };
        vec
    }
    /// Create new random quality sequence with specified number of characters.
    fn random_quality(nbases: usize, mut rng: ThreadRng) -> Vec<u8> {
        let mut vec = Vec::with_capacity(nbases);
        for base in 0..nbases {
            vec.push(*PHRED33_U8.choose(&mut rng).unwrap())
        };
        vec
    }
}