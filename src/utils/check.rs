// Copyright 2020 Christopher Sugai

//! Trait for checking specific criteria for a [u8] of biological file origin. Types include sequence (nucleotide/amino acid) and quality (phred33/64/solexa, solexa being all printable ascii).
//! Additional functionality for common checks including has_n, has_gap, is_homopolymer, is_palindrome, etc.
//! # Examples
//! ```
//! 
//! ```


/// Trait for checking specific criteria for a [u8] of biological file origin. Types include sequence (nucleotide/amino acid) and quality (phred33/64/solexa, solexa being all printable ascii). 
/// These should be used with closely with the is_ascii/make/to_ascii_lowercase/make/to_ascii_uppercase functions in standard rust.
/// Additional functionality for common checks including has_n, has_gap, is_homopolymer, is_palindrome, etc.

use crate::charsets::iupac::*;
use crate::charsets::quality::*;

pub trait Check<T> {

    /// Checks if [u8] contains only iupac including nucleotide, amino acid, punctuation.
    fn is_iupac(&self) -> bool;
    /// Checks if [u8] contains only iupac including nucleotide, punctuation.
    fn is_iupac_nucleotide(&self) -> bool;
    /// Checks if [u8] contains only iupac amino acids.
    fn is_iupac_amino_acid(&self) -> bool;
    /// Checks if [u8] contains only 4 basic dna bases.
    fn is_basic_dna(&self) -> bool;
    /// Checks if [u8] contains only 4 basic rna bases.
    fn is_basic_rna(&self) -> bool;
    /// Checks if [u8] contains only 4 basic aa bases.
    fn is_basic_amino_acid(&self) -> bool;

    /// Checks if [u8] contains gap punctuation.
    fn has_gap(&self) -> bool;
    /// Checks if [u8] contains nN.
    fn has_n(&self) -> bool;

    // fn has_mixed_case(&self) -> bool;
    // fn has_seq(&self) -> bool;

    // Checks if [u8] is valid quality, etc.
    // fn is_phred33(&self) -> bool;
    // fn is_phred64(&self) -> bool;
    // fn is_solexa(&self) -> bool;

    // // checks whther sequence is within user given bounds
    // fn is_quality(&self) -> bool; 

    // //Exact sequence classification (no distance difference)
    // fn is_homopolymer(&self) -> bool;
    // fn is_homopolymer_n(&self) -> bool;
    // fn is_homopolymer_not_n(&self) -> bool;
    // fn is_palindrome(&self) -> bool;

    // //sequence classification within user given Hamming distance
    // fn is_inexact_homopolymer(&self) -> bool;
    // fn is_inexact_palindrome(&self) -> bool;
}

impl<T> Check<T> for [u8] where for<'a> &'a T: IntoIterator<Item = &'a T> {

    /// Checks if [u8] contains only iupac including nucleotide, amino acid, punctuation.
    fn is_iupac(&self) -> bool {
        self.into_iter().all(|x| IUPAC_U8.contains(&x))
    }

    /// Checks if [u8] contains only iupac including nucleotide, punctuation.
    fn is_iupac_nucleotide(&self) -> bool {
    self.into_iter().all(|x| IUPAC_NUCLEOTIDE_U8.contains(&x))
    }

    /// Checks if [u8] contains only iupac amino acids.
    fn is_iupac_amino_acid(&self) -> bool {
        self.into_iter().all(|x| IUPAC_AMINO_ACID_U8.contains(&x))
    }

    /// Checks if [u8] contains only 4 basic dna bases.
    fn is_basic_dna(&self) -> bool {
        self.into_iter().all(|x| BASIC_DNA_U8.contains(&x))
    }

    /// Checks if [u8] contains only 4 basic rna bases.
    fn is_basic_rna(&self) -> bool {
        self.into_iter().all(|x| BASIC_RNA_U8.contains(&x))
    }

    /// Checks if [u8] contains only 4 basic aa bases.
    fn is_basic_amino_acid(&self) -> bool {
        self.into_iter().all(|x| BASIC_AMINO_ACID_U8.contains(&x))
    }

    /// Checks if [u8] contains gap punctuation.
    fn has_gap(&self) -> bool {
        self.into_iter().all(|x| GAP_U8.contains(&x))
    }

    /// Checks if [u8] contains nN.
    fn has_n(&self) -> bool {
        self.into_iter().all(|x| N_U8.contains(&x))
    }

    // fn is_homopolymer(&self) -> bool{
        
    // }

    // fn is_palindrome(&self) -> bool{
        
    // }

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