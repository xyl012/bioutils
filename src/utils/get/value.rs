// Copyright (c) 2021 Kana LLC

//! Other functions that do not fall into the general categories of checking, replacing, or generating a new sequence. Generally includes statistics and transformations.
//! # Examples
//! # Generate all sequences with hamming distance 1 using the bases ACTG.
//! 
//! use crate::bioutils::get::value;
//! use crate::bioutils::utils::new::random_dna;
//! use rand::rngs::ThreadRng;
//! use std::string::String;
//! use rand::seq::SliceRandom;
//! 
//! let mut rng = rand::thread_rng(); //create a random number generator
//! let dna = random_dna(4,rng);
//! let distance = dna.hamming_distance(b"AAAA");
//! println!("{:?}", distance);


use crate::charsets::PERCENTAGE_RANGE;
use std::collections::HashMap;
use crate::charsets::iupac::*;
use crate::charsets::quality::*;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use std::convert::TryInto;

pub trait ValueU8<T> {
    /// Returns the percent (0-100) of the quality u8 in bases (rounded) above the quality score supplied. Should be used when mapq scores are required.
    fn quality_percent_passing(&self, quality_score: &u8) -> usize;

    /// Returns the hamming distance of self and another seq.
    fn hamming_distance(&self, seq2: &T) -> u64;

    /// Returns the number of iterators greater than criteria. Used for calculating percents/numerators
    fn iters_greater_than(&self, criteria:&u8)-> usize;

    /// Returns the number of occurrences of the mode
    fn mode_count(&self) -> usize;

    /// Returns the mode
    fn mode(&self) -> Option<&u8>;
}

impl<T> ValueU8<T> for T
where
    for<'a> &'a T: IntoIterator<Item = &'a u8>, 
    // T: std::ops::BitXor<Output = u8>
{
    /// Checks each quality u8 and returns the percent above (passing) the given u8
    fn quality_percent_passing(&self, quality_score: &u8)-> usize {
        percentage(self.iters_greater_than(&quality_score), self.into_iter().count())
    }

    /// Checks the hamming distance between our item and a supplied item
    fn hamming_distance(&self, seq2: &T) -> u64 {
        let seq1=self;
        seq1.into_iter().zip(seq2).fold(0, |seq1, (seq2, c)| seq1 + (*seq2 ^ *c).count_ones() as u64)
    }

    /// Returns the iterations greater than the criteria
    fn iters_greater_than(&self, criteria:&u8)-> usize {
        self.into_iter().filter_map(|s| Some(&s>=&criteria)).count()
    }

    /// Returns the number of occurrences of the mode
    fn mode_count(&self) -> usize {
        let mode = self.mode().unwrap();
        self.into_iter().filter_map(|s| Some(s == mode)).count()
    }

    /// Returns the mode
    fn mode(&self) -> Option<&u8> {
        let mut counts = HashMap::new();
        self.into_iter().max_by_key(|&s| {
            let count = counts.entry(s).or_insert(0);
            *count += 1;
            *count})
    }
}

/// Calculates percentage with usizes
pub fn percentage(numerator: usize, denominator: usize) -> usize {
    (100 * numerator + denominator / 2) / denominator
}

/// Validate a u8 is 0 to 100 and return a wrapped boolean
pub fn validate_percentage_u8(percent: &u8) -> Result<bool, &'static str> {
    match PERCENTAGE_RANGE.contains(percent){    
    true => Ok(true),
    false => Err("Please supply a percent (0-100, not fractional) as u8"),
    }
}

// {
//     None => Err("No percent supplied"),
//     Some(true) => Ok(true),
//     Some(false) => Ok(false),
//     Some(_) => Err("Please supply a percent (0-100, not fractional) as u8"),
// }

// /// Validate if a quality score is phred33, phred64, etc. Example: validate_quality_score_u8(quality_score,"phred33")
// pub fn validate_quality_score_u8(quality_score: , quality_charset: &str){};
// /// Returns the number of occurrences of the mode
// pub fn mode_count(numbers: &[u8]) -> Option<&u64> {
//     let mut counts = HashMap::new();
//     let mode = numbers.into_iter().max_by_key(|&s| {
//         let count = counts.entry(s).or_insert(0);
//         *count += 1;
//         *count}).unwrap();
//     counts.get(&mode)
// }

// /// Returns the mode
// pub fn mode(numbers: &[u8]) -> Option<&u8> {
//     let mut counts = HashMap::new();
//     numbers.into_iter().max_by_key(|&s| {
//         let count = counts.entry(s).or_insert(0);
//         *count += 1;
//         *count})
// }

// pub fn hamming_distance(seq1: &[u8], seq2: &[u8]) -> u64 {
//     seq1.iter().zip(seq2).fold(0, |seq1, (seq2, c)| seq1 + (*seq2 ^ *c).count_ones() as u64)
// }
