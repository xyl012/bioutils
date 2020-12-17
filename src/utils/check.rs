// Copyright 2020 Christopher Sugai

//! Trait for checking specific criteria for a u8 of biological file origin. Types include sequence (nucleotide/amino acid) and quality (phred33/64/solexa, phred33 being all printable ascii).
//! See below examples for included functions. Note that quality alphabets overlap, and one quality line may be valid phred33,64, solexa, or any combination. Check with the manufacturer for which quality encoding your data has. If unsure, most data generated beyond 2016 is kept in phred33 encoding.
//! Additional functionality for common checks including has_n, has_gap, is_homopolymer, etc.
//! # Examples
//! ```
//! use bioutils::charsets::*;
//! use bioutils::utils::*;
//! use bioutils::utils::check::Check;
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
//!
//!
//! ```

use crate::charsets::ascii::*;
/// Trait for checking specific criteria for a u8 of biological file origin. Types include sequence (nucleotide/amino acid) and quality (phred33/64/solexa, phred33 being all printable ascii).
/// These should be used with closely with the is_ascii/make/to_ascii_lowercase/make/to_ascii_uppercase functions in standard rust.
/// Additional functionality for common checks including has_n, has_gap, is_homopolymer, is_palindrome, etc.
use crate::charsets::iupac::*;
use crate::charsets::quality::*;

pub trait Check<T> {
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

    /// Checks if the sequence is a homopolymer (no distance difference). Possible to use with Rust's window function for checking homopolymer sequences of arbitrary length.
    fn is_homopolymer(&self) -> bool;
    /// Checks if the sequence is a N homopolymer (no distance difference). Possible to use with Rust's window function for checking homopolymer sequences of arbitrary length.
    fn is_homopolymer_n(&self) -> bool;
    /// Checks if the sequence is any homopolymer comprised of any character other than N or n (no distance difference). Possible to use with Rust's window function for checking homopolymer sequences of arbitrary length.
    fn is_homopolymer_not_n(&self) -> bool;

    /// Checks if u8 is ascii letters.
    fn is_ascii_letters(&self) -> bool;
    /// Checks if u8 is ascii letters uppercase only.
    fn is_ascii_letters_uppercase(&self) -> bool;
    /// Checks if u8 is ascii letters lowercase only.
    fn is_ascii_letters_lowercase(&self) -> bool;
}

impl<T> Check<T> for T
where
    for<'a> &'a T: IntoIterator<Item = &'a u8>,
{
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
