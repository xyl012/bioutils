
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
//! ```

// use crate::utils::check_percentage_u8;
use std::borrow::Borrow;
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

pub trait Check<K> {
        /// Checks if the sequence or quality u8 is less than or equal to the given length. Used to cut read to minimum length.
        fn is_at_least_length(&self, length: &usize) -> bool;
        /// Checks if the sequence or quality u8 is greater than or equal to the given length. Used to cut read to maximum length.
        fn is_at_most_length(&self, length: &usize) -> bool;
        /// Checks if the sequence or quality u8 is equal to the given length.
        fn is_length(&self, length: &usize) -> bool;
        /// Checks if u8 is completely comprised of the same character. Does not use a character set, so could be all gaps, etc. Use has_mixed_case and to_uppercase/to_lowercase prior if mixed case.
        fn is_homopolymer(&self) -> bool;
}

impl<T, K> Check<K> for T
where
    T: AsRef<[K]>,
    K: PartialEq,
{
        /// Checks if the sequence or quality u8 is less than or equal to the given length. Used to cut read to minimum length.
        fn is_at_least_length(&self, length: &usize) -> bool {
            self.as_ref().len() >= *length
        }
    
        /// Checks if the sequence or quality u8 is greater than or equal to the given length. Used to cut read to maximum length.
        fn is_at_most_length(&self, length: &usize) -> bool {
            self.as_ref().len() <= *length
        }
    
        /// Checks if equal to the given length.
        fn is_length(&self, length: &usize) -> bool {
            self.as_ref().len() == *length
        }

        /// Checks if completely comprised of the same character. Does not use a character set, so could be all gaps, etc. Use has_mixed_case and to_uppercase/to_lowercase prior if mixed case.
        fn is_homopolymer(&self) -> bool {
            self.as_ref().iter().fold((true, None), {
                |x, y| {
                    if let Some(p) = x.1 {
                        (x.0 && (p == y), Some(y))
                    } else {
                        (true, Some(y))
                    }
                }
            })
            .0
        }
}

    // /// Validates whether is a valid something based on the boolean is_x smaller functions in this trait and returns a wrapped boolean. Example: check_u8(b"ACTG","is_basic_dna") returns a wrapped "true". Options for is_what are the names of the charset boolean functions:
    // /// is_basic_dna, is_phred33, is_basic_rna,  
    // /// The goal of this function would be to set is_what to a constant in the program, for example a program focused on illumina data might set a constant to phred33 and input as is_what, rather than having to call is_phred33 each time. This means we can easily make our program take phred33 or 64 by just changing the constant.
    // fn check_u8(&self, is_what: &str) -> Result<bool, &str>;

// /// Validates whether is a valid something based on the boolean is_x smaller functions in this trait and returns a wrapped boolean. Example: check_u8(b"ACTG","is_basic_dna") returns a wrapped "true". Options for is_what are the names of the charset boolean functions:
    // /// "is_iupac_nucleotide", "is_iupac_amino_acid", "is_iupac",
    // /// "is_phred33", "is_phred64", "is_solexa",  
    // /// "is_basic_dna", "is_basic_rna", "is_basic_amino_acid",
    // /// "is_homopolymer", "is_homopolymer_n", "is_homopolymer_not_n",
    // /// "has_n", "has_gap",
    // /// "is_ascii_letters", "is_ascii_letters_uppercase", "is_ascii_letters_lowercase" 
    // /// The goal of this function would be to set is_what to a constant in the program, for example a program focused on dna data might set a constant to is_basic_dna and input as is_what rather than having to call is_basic_dna each time. Later, if we want to focus on rna, we can easily change our constant to is_basic_rna just by changing the constant.
    // fn check_u8(&self, is_what: &str) -> Result<bool, &str>{
    //     if validate_is_what(&is_what).unwrap() {
    //         match is_what {
    //             "is_phred33" => Ok(self.is_phred33()),
    //             "is_phred64" => Ok(self.is_phred64()),
    //             "is_solexa" => Ok(self.is_solexa()),
    //             "is_iupac_nucleotide" => Ok(self.is_iupac_nucleotide()),
    //             "is_iupac_amino_acid" => Ok(self.is_iupac_amino_acid()),
    //             "is_iupac" => Ok(self.is_iupac()),
    //             "is_basic_dna" => Ok(self.is_basic_dna()),
    //             "is_basic_rna" => Ok(self.is_basic_rna()),
    //             "is_basic_amino_acid" => Ok(self.is_basic_amino_acid()),
    //             "is_homopolymer" => Ok(CheckU8::is_homopolymer(&self)),
    //             "is_homopolymer_n" => Ok(self.is_homopolymer_n()),
    //             "is_homopolymer_not_n" => Ok(self.is_homopolymer_not_n()),
    //             "has_n" => Ok(self.has_n()),
    //             "has_gap" => Ok(self.has_gap()),
    //             "is_ascii_letters" => Ok(self.is_ascii_letters()),
    //             "is_ascii_letters_uppercase" => Ok(self.is_ascii_letters_uppercase()),
    //             "is_ascii_letters_lowercase" => Ok(self.is_ascii_letters_lowercase()),
    //             _ => Err("Invalid is_what parameter, please choose a valid option")
    //         }
    //     } else {validate_is_what(&is_what)}
    // }



pub trait CheckU8<T> {
    /// Checks the sequence has the percent bases (rounded) above the quality score
    fn is_qual_passing(&self, quality_score: &u8, percent: &u8) -> Result<bool, &str>;

    /// Checks if the sequence is a homopolymer with percentage cutoff.
    fn is_percent_homopolymer(&self, percent: &u8) -> Result<bool, &str>;
    /// Checks if the sequence is a x homopolymer with percentage cutoff.
    fn is_percent_homopolymer_x(&self, x: &u8, percent: &u8) -> Result<bool, &str>;

    /// Checks if the u8 sequence is homopolymer. Possible to use with Rust's window function for checking homopolymer sequences of arbitrary length.
    fn is_homopolymer(&self) -> bool;
    /// Checks if the u8 sequence is a N homopolymer. Possible to use with Rust's window function for checking homopolymer sequences of arbitrary length.
    fn is_homopolymer_n(&self) -> bool;
    /// Checks if the u8 sequence is any homopolymer comprised of any character other than N or n. Possible to use with Rust's window function for checking homopolymer sequences of arbitrary length.
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
    T: AsRef<[u8]>,
{
    /// Checks the sequence has a number of bases (percent rounded) greater than or equal to the supplied quality score
    fn is_qual_passing(&self, quality_score: &u8, percent: &u8) -> Result<bool, &str> {
        if validate_percentage_u8(percent).unwrap() {
            if self.quality_percent_passing(&quality_score) >= (*percent).into() {
                Ok(true)
            } else { Ok(false) }
        } else { validate_percentage_u8(percent) }
    }

    /// Checks if the sequence is a homopolymer with percentage cutoff
    fn is_percent_homopolymer(&self, percent: &u8) -> Result<bool, &str> {
        if validate_percentage_u8(&percent).unwrap() {
            if percentage(self.count_mode(), self.as_ref().len()) >= (*percent).into() {
                Ok(true)
            } else {Ok(false)}
        } else {validate_percentage_u8(&percent)}
    }
    /// Checks if the sequence is comprised of 'x' base greater than 'percent' cutoff. Primary use is for filtering for reads with >90% percent N's or A's
    fn is_percent_homopolymer_x(&self, x: &u8, percent: &u8) -> Result<bool, &str> {
        if validate_percentage_u8(&percent).unwrap() {
            if percentage(self.count_xu8(x), self.as_ref().len()) >= (*percent).into() {
                Ok(true)
            } else {Ok(false)}
        } else {validate_percentage_u8(&percent)}
    }

    /// Checks if u8 comprised completely of the iupac including nucleotide, amino acid, punctuation.
    fn is_iupac(&self) -> bool {
        self.as_ref().iter().all(|x| IUPAC_U8.contains(&x))
    }

    /// Checks if u8 comprised completely of iupac including nucleotide, punctuation.
    fn is_iupac_nucleotide(&self) -> bool {
        self.as_ref().iter().all(|x| IUPAC_NUCLEOTIDE_U8.contains(&x))
    }

    /// Checks if u8 comprised completely of iupac amino acids.
    fn is_iupac_amino_acid(&self) -> bool {
        self.as_ref().iter().all(|x| IUPAC_AMINO_ACID_U8.contains(&x))
    }

    /// Checks if u8 comprised completely of the 4 basic dna bases.
    fn is_basic_dna(&self) -> bool {
        self.as_ref().iter().all(|x| BASIC_DNA_U8.contains(&x))
    }

    /// Checks if u8 comprised completely of the 4 basic rna bases.
    fn is_basic_rna(&self) -> bool {
        self.as_ref().iter().all(|x| BASIC_RNA_U8.contains(&x))
    }

    /// Checks if u8 is comprised completely of the basic aa bases.
    fn is_basic_amino_acid(&self) -> bool {
        self.as_ref().iter().all(|x| BASIC_AMINO_ACID_U8.contains(&x))
    }

    /// Checks if u8 contains gap punctuation.
    fn has_gap(&self) -> bool {
        self.as_ref().iter().any(|x| GAP_U8.contains(&x))
    }

    /// Checks if u8 contains N or n.
    fn has_n(&self) -> bool {
        self.as_ref().iter().any(|x| N_U8.contains(&x))
    }

    /// Checks if u8 is completely comprised of the same character. Does not use a character set, so could be all gaps, etc. Use has_mixed_case and to_uppercase/to_lowercase prior if mixed case.
    fn is_homopolymer(&self) -> bool {
        self.as_ref().iter().min() == self.as_ref().iter().max()
    }
    
    /// Checks if u8 is completely comprised of N or n's.
    fn is_homopolymer_n(&self) -> bool {
        self.as_ref().iter().all(|x| N_U8.contains(&x))
    }

    /// Checks if u8 is completely comprised of non-N or non-n's. Use has_mixed_case and to_uppercase or lowercase prior if mixed case.
    fn is_homopolymer_not_n(&self) -> bool {
        if self.has_n() {
            false
        } else {
            CheckU8::is_homopolymer(&self)
        }
    }
    
    /// Checks if u8 is completely comprised of phred33 characters (all printable ascii). Incorporates other character sets.
    fn is_phred33(&self) -> bool {
        self.as_ref().iter().all(|x| PHRED33_U8.contains(&x))
    }

    /// Checks if u8 is completely comprised of phred64 characters.
    fn is_phred64(&self) -> bool {
        self.as_ref().iter().all(|x| PHRED64_U8.contains(&x))
    }

    /// Checks if u8 is completely comprised of solexa characters.
    fn is_solexa(&self) -> bool {
        self.as_ref().iter().all(|x| SOLEXA_U8.contains(&x))
    }

    /// check if u8 is comprised completely of ascii letters Aa-Zz
    fn is_ascii_letters(&self) -> bool {
        self.as_ref().iter().all(|x| ASCII_LETTERS_U8.contains(&x))
    }

    /// check if u8 is comprised completely of ascii letters A-Z
    fn is_ascii_letters_uppercase(&self) -> bool {
        self.as_ref().iter()
            .all(|x| ASCII_LETTERS_UPPERCASE_U8.contains(&x))
    }

    /// check if u8 is comprised completely of ascii lowercase letters a-z
    fn is_ascii_letters_lowercase(&self) -> bool {
        self.as_ref().iter()
            .all(|x| ASCII_LETTERS_LOWERCASE_U8.contains(&x))
    }
}


pub trait CheckEqItems<K>{
    /// Checks if the sequence and quality u8 vectors are the same length. Generally checks two u8 items for length against each other
    fn is_seq_qual_length_equal(&self, quality: &K) -> bool;
}

impl<T, K> CheckEqItems<K> for T
where
    T: AsRef<[K]>,
    T: PartialEq,
    K: AsRef<[T]>,
    K: PartialEq,
{
    /// Checks if two items are the same length. 
    fn is_seq_qual_length_equal(&self, quality: &K)-> bool {
        self.as_ref().len() == quality.as_ref().len()
    }
}



// /// Validates that 'is_what' parameter of the check_u8() function is a valid option.
// pub fn validate_is_what<'a>(is_what: &str) -> Result<bool, &'a str> {
//         match IS_WHAT_OPTIONS.contains(&is_what) {
//             true => Ok(true),
//             false => Err("Not a valid option for is_what parameter, please check valid options"),
//         }
// }

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