// Copyright (c) 2021 Kana LLC

//! Trait for checking specific criteria for a u8 of biological file origin. Types include sequence (nucleotide/amino acid) and quality (phred33/64/solexa, phred33 being all printable ascii).
//! See below examples for included functions. Note that quality alphabets overlap, and one quality line may be valid phred33,64, solexa, or any combination. Check with the manufacturer for which quality encoding your data has. If unsure, most data generated beyond 2016 is kept in phred33 encoding.
//! Additional functionality for common checks including has_n, has_gap, is_homopolymer, etc.
//! # Examples
//! ```
//! use crate::bioutils::charsets;
//! use crate::bioutils::utils::check::value::CheckU8;
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
//! ```

// use crate::utils::check_percentage_u8;
use crate::utils::get::value::percentage;
use crate::utils::get::value::validate_percentage_u8;
use std::collections::HashMap;
use crate::charsets::PERCENTAGE_RANGE;

use crate::charsets::ascii::*;
use crate::utils::get::value;
use crate::utils::get::value::ValueU8;

/// Trait for checking specific criteria for a u8 of biological file origin. Types include sequence (nucleotide/amino acid) and quality (phred33/64/solexa, phred33 being all printable ascii).
/// These should be used with closely with the is_ascii/make/to_ascii_lowercase/make/to_ascii_uppercase functions in standard rust.
/// Additional functionality for common checks including has_n, has_gap, is_homopolymer, is_palindrome, etc.
use crate::charsets::iupac::*;
use crate::charsets::quality::*;

pub const IS_WHAT_OPTIONS: [&str; 17] = 
["is_iupac_nucleotide", "is_iupac_amino_acid", "is_iupac",
"is_phred33", "is_phred64", "is_solexa",  
"is_basic_dna", "is_basic_rna", "is_basic_amino_acid",
"is_homopolymer", "is_homopolymer_n", "is_homopolymer_not_n",
"has_n", "has_gap",
"is_ascii_letters", "is_ascii_letters_uppercase", "is_ascii_letters_lowercase"
];

pub trait CheckU8<T> {
    /// Validates whether is a valid something based on the boolean is_x smaller functions in this trait and returns a wrapped boolean. Example: check_u8(b"ACTG","is_basic_dna") returns a wrapped "true". Options for is_what are the names of the charset boolean functions:
    /// is_basic_dna, is_phred33, is_basic_rna,  
    /// The goal of this function would be to set is_what to a constant in the program, for example a program focused on illumina data might set a constant to phred33 and input as is_what, rather than having to call is_phred33 each time. This means we can easily make our program take phred33 or 64 by just changing the constant.
    fn check_u8(&self, is_what: &str) -> Result<bool, &str>;

    /// Checks the sequence has the percent bases (rounded) above the quality score
    fn is_qual_passing(&self, quality_score: &u8, percent: &u8) -> Result<bool, &str>;
    /// Checks if the sequence and quality u8 vectors are the same length. Generally checks two u8 items for length against each other
    fn is_seq_qual_length_equal(&self, quality: &T) -> bool;

    /// Checks if the sequence is a homopolymer with percentage cutoff.
    fn is_percent_homopolymer(&self, percent: &u8) -> Result<bool, &str>;
    /// Checks if the sequence is a x homopolymer with percentage cutoff.
    fn is_percent_homopolymer_x(&self, x: &u8, percent: &u8) -> Result<bool, &str>;
    // /// Checks if the sequence is any homopolymer comprised of any character other than N or n with percentage cutoff. Possible to use with Rust's window function for checking homopolymer sequences of arbitrary length.
    // fn is_percent_homopolymer_not_n(&self, percent: &u8) -> bool;
    /// Checks if the sequence is a homopolymer. Possible to use with Rust's window function for checking homopolymer sequences within sequences of arbitrary length.
    fn is_homopolymer(&self) -> bool;
    /// Checks if the sequence is a N homopolymer. Possible to use with Rust's window function for checking homopolymer sequences of arbitrary length.
    fn is_homopolymer_n(&self) -> bool;
    /// Checks if the sequence is any homopolymer comprised of any character other than N or n. Possible to use with Rust's window function for checking homopolymer sequences of arbitrary length.
    fn is_homopolymer_not_n(&self) -> bool;

    /// Checks if u8 comprised completely of the iupac including nucleotide, amino acid, punctuation.
    fn is_iupac(&self) -> bool;
    /// Checks if u8 comprised completely of the iupac including nucleotide, punctuation.
    fn is_iupac_nucleotide(&self) -> bool;
    /// Checks if u8 comprised completely of the iupac amino acids.
    fn is_iupac_amino_acid(&self) -> bool;
    /// Checks if u8 comprised completely of the 4 basic dna bases.
    fn is_basic_dna(&self) -> bool;
    /// Checks if u8 comprised completely of the 4 basic rna bases.
    fn is_basic_rna(&self) -> bool;
    /// Checks if u8 comprised completely of the 4 basic aa bases.
    fn is_basic_amino_acid(&self) -> bool;

    /// Checks if u8 contains gap punctuation.
    fn has_gap(&self) -> bool;
    /// Checks if u8 contains nN.
    fn has_n(&self) -> bool;

    /// Checks if u8 is completely comprised of phred33 characters.
    fn is_phred33(&self) -> bool;
    /// Checks if u8 is completely comprised of phred64 characters.
    fn is_phred64(&self) -> bool;
    /// Checks if u8 is completely comprised of solexa characters (all printable ascii). Incorporates other character sets.
    fn is_solexa(&self) -> bool;

    /// Checks if u8 is ascii letters.
    fn is_ascii_letters(&self) -> bool;
    /// Checks if u8 is ascii letters uppercase only.
    fn is_ascii_letters_uppercase(&self) -> bool;
    /// Checks if u8 is ascii letters lowercase only.
    fn is_ascii_letters_lowercase(&self) -> bool;
}

impl<T> CheckU8<T> for T
where
    for<'a> &'a T: IntoIterator<Item = &'a u8>,
{
    /// Validates whether is a valid something based on the boolean is_x smaller functions in this trait and returns a wrapped boolean. Example: check_u8(b"ACTG","is_basic_dna") returns a wrapped "true". Options for is_what are the names of the charset boolean functions:
    /// "is_iupac_nucleotide", "is_iupac_amino_acid", "is_iupac",
    /// "is_phred33", "is_phred64", "is_solexa",  
    /// "is_basic_dna", "is_basic_rna", "is_basic_amino_acid",
    /// "is_homopolymer", "is_homopolymer_n", "is_homopolymer_not_n",
    /// "has_n", "has_gap",
    /// "is_ascii_letters", "is_ascii_letters_uppercase", "is_ascii_letters_lowercase" 
    /// The goal of this function would be to set is_what to a constant in the program, for example a program focused on dna data might set a constant to is_basic_dna and input as is_what rather than having to call is_basic_dna each time. Later, if we want to focus on rna, we can easily change our constant to is_basic_rna just by changing the constant.
    fn check_u8(&self, is_what: &str) -> Result<bool, &str>{
        if validate_is_what(&is_what).unwrap() {
            match is_what {
                "is_phred33" => Ok(self.is_phred33()),
                "is_phred64" => Ok(self.is_phred64()),
                "is_solexa" => Ok(self.is_solexa()),
                "is_iupac_nucleotide" => Ok(self.is_iupac_nucleotide()),
                "is_iupac_amino_acid" => Ok(self.is_iupac_amino_acid()),
                "is_iupac" => Ok(self.is_iupac()),
                "is_basic_dna" => Ok(self.is_basic_dna()),
                "is_basic_rna" => Ok(self.is_basic_rna()),
                "is_basic_amino_acid" => Ok(self.is_basic_amino_acid()),
                "is_homopolymer" => Ok(self.is_homopolymer()),
                "is_homopolymer_n" => Ok(self.is_homopolymer()),
                "is_homopolymer_not_n" => Ok(self.is_homopolymer()),
                "has_n" => Ok(self.has_n()),
                "has_gap" => Ok(self.has_gap()),
                "is_ascii_letters" => Ok(self.is_ascii_letters()),
                "is_ascii_letters_uppercase" => Ok(self.is_ascii_letters_uppercase()),
                "is_ascii_letters_lowercase" => Ok(self.is_ascii_letters_lowercase()),
                _ => Err("Invalid is_what parameter, please choose a valid option")
            }
        } else {validate_is_what(&is_what)}
    }

    /// Checks the sequence has a number of bases (percent rounded) greater than or equal to the supplied quality score
    fn is_qual_passing(&self, quality_score: &u8, percent: &u8) -> Result<bool, &str> {
        if validate_percentage_u8(percent).unwrap() {
            if self.quality_percent_passing(&quality_score) >= (*percent).into() {
                Ok(true)
            } else { Ok(false) }
        } else { validate_percentage_u8(percent) }
    }

    /// Checks if the sequence and quality u8 vectors are the same length. Generally checks two u8 items for length against each other
    fn is_seq_qual_length_equal(&self, quality: &T)-> bool {
        self.into_iter().count() == quality.into_iter().count()
    }

    /// Checks if u8 comprised completely of the iupac including nucleotide, amino acid, punctuation.
    fn is_iupac(&self) -> bool {
        self.into_iter().all(|x| IUPAC_U8.contains(&x))
    }

    /// Checks if u8 comprised completely of iupac including nucleotide, punctuation.
    fn is_iupac_nucleotide(&self) -> bool {
        self.into_iter().all(|x| IUPAC_NUCLEOTIDE_U8.contains(&x))
    }

    /// Checks if u8 comprised completely of iupac amino acids.
    fn is_iupac_amino_acid(&self) -> bool {
        self.into_iter().all(|x| IUPAC_AMINO_ACID_U8.contains(&x))
    }

    /// Checks if u8 comprised completely of the 4 basic dna bases.
    fn is_basic_dna(&self) -> bool {
        self.into_iter().all(|x| BASIC_DNA_U8.contains(&x))
    }

    /// Checks if u8 comprised completely of the 4 basic rna bases.
    fn is_basic_rna(&self) -> bool {
        self.into_iter().all(|x| BASIC_RNA_U8.contains(&x))
    }

    /// Checks if u8 is comprised completely of the basic aa bases.
    fn is_basic_amino_acid(&self) -> bool {
        self.into_iter().all(|x| BASIC_AMINO_ACID_U8.contains(&x))
    }

    /// Checks if u8 contains gap punctuation.
    fn has_gap(&self) -> bool {
        self.into_iter().any(|x| GAP_U8.contains(&x))
    }

    /// Checks if u8 contains N or n.
    fn has_n(&self) -> bool {
        self.into_iter().any(|x| N_U8.contains(&x))
    }

    /// Checks if u8 is completely comprised of the same character. Does not use a character set, so could be all gaps, etc. Use has_mixed_case and to_uppercase/to_lowercase prior if mixed case.
    fn is_homopolymer(&self) -> bool {
        self.into_iter().min() == self.into_iter().max()
    }

    /// Checks if u8 is completely comprised of N or n's.
    fn is_homopolymer_n(&self) -> bool {
        self.into_iter().all(|x| N_U8.contains(&x))
    }

    /// Checks if u8 is completely comprised of non-N or non-n's. Use has_mixed_case and to_uppercase or lowercase prior if mixed case.
    fn is_homopolymer_not_n(&self) -> bool {
        if self.has_n() {
            false
        } else {
            self.is_homopolymer()
        }
    }

    /// Checks if the sequence is a homopolymer with percentage cutoff
    fn is_percent_homopolymer(&self, percent: &u8) -> Result<bool, &str> {
        if validate_percentage_u8(&percent).unwrap() {
            if percentage(self.count_mode(), self.into_iter().count()) >= (*percent).into() {
                Ok(true)
            } else {Ok(false)}
        } else {validate_percentage_u8(&percent)}
    }

    /// Checks if the sequence is comprised of 'x' base greater than 'percent' cutoff. Primary use is for filtering for reads with >90% percent N's or A's
    fn is_percent_homopolymer_x(&self, x: &u8, percent: &u8) -> Result<bool, &str> {
        if validate_percentage_u8(&percent).unwrap() {
            if percentage(self.count_xu8(x), self.into_iter().count()) >= (*percent).into() {
                Ok(true)
            } else {Ok(false)}
        } else {validate_percentage_u8(&percent)}
    }
    
    /// Checks if u8 is completely comprised of phred33 characters (all printable ascii). Incorporates other character sets.
    fn is_phred33(&self) -> bool {
        self.into_iter().any(|x| PHRED33_U8.contains(&x))
    }

    /// Checks if u8 is completely comprised of phred64 characters.
    fn is_phred64(&self) -> bool {
        self.into_iter().any(|x| PHRED64_U8.contains(&x))
    }

    /// Checks if u8 is completely comprised of solexa characters.
    fn is_solexa(&self) -> bool {
        self.into_iter().any(|x| SOLEXA_U8.contains(&x))
    }

    /// check if u8 is comprised completely of ascii letters Aa-Zz
    fn is_ascii_letters(&self) -> bool {
        self.into_iter().all(|x| ASCII_LETTERS_U8.contains(&x))
    }

    /// check if u8 is comprised completely of ascii letters A-Z
    fn is_ascii_letters_uppercase(&self) -> bool {
        self.into_iter()
            .all(|x| ASCII_LETTERS_UPPERCASE_U8.contains(&x))
    }

    /// check if u8 is comprised completely of ascii lowercase letters a-z
    fn is_ascii_letters_lowercase(&self) -> bool {
        self.into_iter()
            .all(|x| ASCII_LETTERS_LOWERCASE_U8.contains(&x))
    }
}

/// Validates that 'is_what' parameter of the check_u8() function is a valid option.
pub fn validate_is_what<'a>(is_what: &str) -> Result<bool, &'a str> {
        match IS_WHAT_OPTIONS.contains(&is_what) {
            true => Ok(true),
            false => Err("Not a valid option for is_what parameter, please check valid options"),
        }
}




// pub trait WindowsU8<T> {
//     fn cg_pos(&self) -> Vec<usize> ;
// }

// impl<T> WindowsU8<T> for T
// where
//     for<'a> &'a T: IntoIterator<Item = &'a u8>,
// {
//     fn cg_pos(&self)-> Vec<usize> {
//         self.windows(2).enumerate()
//             .filter(move |(_, x)| x == b"CG")
//             .map(|(idx, _)| idx).collect::<Vec<usize>>()
//     }
// }

//    /// Checks if two nucleotides are cg. Used with window function to check sliding windows across sequence.
//    fn is_cg(&self) -> bool;
//     /// Checks if two nucleotides are cg. Used with window function to check sliding windows across sequence.
//     fn is_cg(&self) -> bool {
//         self.windows()
//     }

// #[cfg(test)]
// mod tests {
//     use super::{IUPAC_U8, IUPAC_NUCLEOTIDE_U8, IUPAC_AMINO_ACID_U8, BASIC_DNA_U8, BASIC_RNA_U8, BASIC_AMINO_ACID_U8};
//     #[test]
//     fn test_iupac() {
//         let dec: [u8; 46] = [65, 97, 67, 99, 71, 103, 84, 116, 85, 117, 82, 114, 89, 121, 83, 115, 87, 119, 75, 107, 77, 109, 66, 98, 68, 100, 72, 104, 86, 118, 78, 110, 45, 46, 70, 102, 71, 103, 73, 105, 76, 108, 80, 112, 81, 113];
//         assert_eq!(dec, IUPAC_U8);
//     }
//     #[test]
//     fn test_iupac_nucleotide() {
//         let dec: [u8; 34] = [65, 97, 67, 99, 71, 103, 84, 116, 85, 117, 82, 114, 89, 121, 83, 115, 87, 119, 75, 107, 77, 109, 66, 98, 68, 100, 72, 104, 86, 118, 78, 110, 45, 46];
//         assert_eq!(dec, IUPAC_NUCLEOTIDE_U8);
//     }