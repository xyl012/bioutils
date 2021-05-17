
//! Functions that do not fall into the general categories of checking, mutating, or generating a new sequence. Generally includes statistics and transformations.
//! # Examples
//! ``
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
// ! ``

use std::convert::TryFrom;
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

    /// Returns the number of iterators greater than criteria. Used for calculating percents/numerators
    fn count_greater_than(&self, criteria:&u8)-> usize;

    /// Returns the number of occurrences of the mode
    fn count_mode(&self) -> usize;

    /// Returns the mean of u8s
    fn mean(&self) -> u64;

    /// Returns the mode
    fn mode(&self) -> Option<&u8>;

    /// Returns the count of a specific u8
    fn count_xu8(&self, x: &u8) -> usize;

}

impl<T> ValueU8<T> for T
where
    T: AsRef<[u8]>,
{
    /// Checks each quality u8 and returns the percent above (passing) the given u8
    fn quality_percent_passing(&self, quality_score: &u8)-> usize {
        percentage(self.count_greater_than(quality_score), self.as_ref().len())
    }

    /// Returns the number of iterations greater than the criteria
    fn count_greater_than(&self, criteria: &u8)-> usize {
        self.as_ref().iter().filter(|&s| s>=criteria).count()
    }

    /// Returns the number of occurrences of the mode
    fn count_mode(&self) -> usize {
        let mode = self.mode().unwrap();
        self.as_ref().iter().filter(|&q| q==mode).count()
    }

    /// Returns the mean of u8s as u64 rounded
    fn mean(&self) -> u64 {
        self.as_ref().iter().map(|x| *x as u64).sum::<u64>() / self.as_ref().len() as u64
    }

    /// Returns the mode of u8s
    fn mode(&self)-> Option<&u8> {
        let mut counts = HashMap::new();
        self.as_ref().iter().max_by_key(|&s| {
            let count = counts.entry(s).or_insert(0);
            *count += 1; *count})
    }
    
    /// Returns the count of a specific u8
    fn count_xu8(&self, x: &u8) -> usize {
        self.as_ref().iter().filter(|&q| q==x).count()
    }

}

pub trait AsMutValueU8<T> {
    /// Returns the percent (0-100) of the quality u8 in bases (rounded) above the quality score supplied. Should be used when mapq scores are required.
    fn mut_quality_percent_passing(&mut self, quality_score: &u8) -> usize;

    /// Returns the number of iterators greater than criteria. Used for calculating percents/numerators
    fn mut_count_greater_than(&mut self, criteria:&u8)-> usize;

    /// Returns the number of occurrences of the mode
    fn mut_count_mode(&mut self) -> usize;

    /// Returns the mean of u8s
    fn mut_mean(&mut self) -> u64;

    /// Returns the mode
    fn mut_mode(&mut self) -> Option<&u8>;

    /// Returns the count of a specific u8
    fn mut_count_xu8(&mut self, x: &u8) -> usize;

}

impl<T> AsMutValueU8<T> for T
where
T: AsMut<[u8]>,
{
    /// Checks each quality u8 and returns the percent above (passing) the given u8
    fn mut_quality_percent_passing(&mut self, quality_score: &u8)-> usize {
        percentage(self.mut_count_greater_than(quality_score), self.as_mut().len())
    }

    /// Returns the number of iterations greater than the criteria
    fn mut_count_greater_than(&mut self, criteria: &u8)-> usize {
        self.as_mut().iter().filter(|&s| s>=criteria).count()
    }

    /// Returns the number of occurrences of the mode
    fn mut_count_mode(&mut self) -> usize {
        let mode = self.mut_mode().unwrap().clone();
        self.as_mut().iter().filter(|&q| q==&mode).count()
    }

    /// Returns the mean of u8s as u64 rounded
    fn mut_mean(&mut self) -> u64 {
        self.as_mut().iter().map(|x| *x as u64).sum::<u64>() / self.as_mut().len() as u64
    }

    /// Returns the mode of u8s
    fn mut_mode(&mut self)-> Option<&u8> {
        let mut counts = HashMap::new();
        self.as_mut().iter().max_by_key(|&s| {
            let count = counts.entry(s).or_insert(0);
            *count += 1; *count})
    }
    
    /// Returns the count of a specific u8
    fn mut_count_xu8(&mut self, x: &u8) -> usize {
        self.as_mut().iter().filter(|&q| q==x).count()
    }

}

fn mean(numbers: &[u8]) -> u64 {
    numbers.iter().sum::<u8>() as u64 / numbers.len() as u64
}



// fn median(numbers: &mut [u8]) -> f32 {
//     if numbers.len() % 2 == 0 {
//         numbers.sort();
//         let mid = numbers.len() / 2;
//     }  else {};
//     numbers.sort();
//     let mid = numbers.len() / 2;
//     numbers[mid]
// }

// need to return a 
// pub trait ValueU8Ord<T>{
//     fn median(&self) -> u8;
// }

// impl<T> ValueU8Ord<T> for T
// where
//     T: AsRef<[u8]>,
//     T: Ord,
// {
//     fn median(&self) -> u8 {
//         self.as_ref().sort();
//         let mid = self.as_ref().len() / 2;
//         self[mid]
//     }
// }



// impl<T, K> ValueU8<K> for T
// where
//     T: AsRef<[u8]>,
// {
    // K: AsRef<[u8]>,
    // K: Eq + PartialEq + Ord + PartialOrd,

    // /// Returns the hamming distance of self and another seq.
    // fn hamming_distance(&self, seq2: &T) -> u64;

    // /// Checks the hamming distance between our item and a supplied item
    // fn hamming_distance(&self, seq2: &K) -> u64 {
    //     let seq1=self;
    //     seq1.as_ref().iter().zip(seq2.as_ref().iter()).fold(0, |seq1, (seq2, c)| seq1 + (seq2 ^ c).count_ones() as u64)
    // }

/// Calculates percentage with usizes
pub fn percentage(numerator: usize, denominator: usize) -> usize {
    (100 * numerator + denominator / 2) / denominator
}

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

