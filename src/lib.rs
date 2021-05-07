// Copyright (c) 2021 Kana LLC

//! # Bioutils: Simple Biological Utilities in Rust
//! Bioutils provides simple biological utilities including:
//! Functions to check sequence validity and content (palindromes too!) 
//! Functions to create new random IUPAC sequences
//! Functions to download human and mouse Gencode reference files
//! Functions to download fastq files
//! Functions to replace N or gaps with pseudorandom nucleotides
//! Complete iupac and quality character sets with shifted value matches.
//! 
//! ## Quick Start
//! ```
//!
//! Examples for using checks:
//! 
//! use bioutils::charsets;
//! use bioutils::utils;
//! use bioutils::utils::check::CheckU8;
//! use bioutils::utils::new::random::random_dna;
//! use bioutils::utils::new::random::random_quality;
//! use rand::rngs::ThreadRng;
//! use rand::seq::SliceRandom;
//! use std::string::String;
//! use std::str;
//! 
//! let dna = b"ACTG";
//! let rna = b"ACUG";
//! let homopolymerN = b"NNNN";
//! let homopolymerA = b"AAAA";
//! let gapna = b"AC-G";
//! let nna = b"ACnG";
//! let quality = b"@ABC";
//!
//! assert!(homopolymerN.is_homopolymer());
//! assert!(homopolymerA.is_homopolymer_not_n());
//! assert!(homopolymerN.is_homopolymer_n());
//!
//! assert!(gapna.has_gap());
//! assert!(nna.has_n());
//! assert!(dna.is_iupac());
//! assert!(rna.is_basic_rna());
//!
//! assert!(quality.is_phred33());
//! assert!(quality.is_phred64());
//! assert!(quality.is_solexa());
//! 
//! // We can also do checks this way:
//! assert!(quality.check_u8("is_phred33").unwrap());
//! assert!(dna.check_u8("is_basic_dna").unwrap());
//!
//! Examples for creating a new random sequence and quality 
//!
//! let mut rng1 = rand::thread_rng(); // Create a random number generator
//! let dna = random_dna(4,rng1); // Create a random dna sequence
//! let mut rng2 = rand::thread_rng();
//! let quality = random_quality(4,rng2); // Create a random quality string
//! println!("{:?}", dna.to_owned());
//! println!("{:?}", quality.to_owned());
//!
//! Example for replacing nucleotides
//!
//! let mut rng3 = rand::thread_rng(); //create a random number generator
//! let mut rng4 = rand::thread_rng(); //create a random number generator
//! let mut seq = b"acugnnnqqq".to_owned(); // or by *: let mut seq = *b"acugnnnqqq";
//! let mut seq = seq.mut_random_replace_non_basic("RNA", rng4).mut_random_replace_n("RNA", rng3).mut_to_upper_basic();
//! let printseq = str::from_utf8(seq).unwrap();
//! println!("{:?}", printseq);
//! ```

pub mod charsets;
pub mod files;
pub mod utils;
pub mod references;

#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
