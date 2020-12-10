// Copyright 2020 Christopher Sugai

//! Checks and classifications returning booleans for an input sequence and basic user input. For example, dna.is_homopolymer(90) returns boolean true if dna is comprised of >90% a single nucleotide. 
//! # Examples
//! ```
//! 
//! ```

use crate::charsets::iupac::*;
use crate::charsets::quality::*;

/// Trait for checking specific criteria for a [u8] of biological file origin. Types include sequence (nucleotide/amino acid) and quality (phred33/64/solexa, solexa being all printable ascii).
/// Additional functionality for common checks including has_n, has_gap, is_homopolymer, is_palindrome, etc.
pub trait Check<T> {

    // Checks if [u8] is valid iupac, etc.
    fn is_iupac(&self) -> bool;
    fn is_iupac_nucleotide(&self) -> bool;
    fn is_iupac_amino_acid(&self) -> bool;
    fn is_basic_dna(&self) -> bool;
    fn is_basic_rna(&self) -> bool;
    fn is_basic_amino_acid(&self) -> bool;

    // Checks for combining with validity checks
    fn has_gap(&self) -> bool;
    fn has_n(&self) -> bool;
    // fn has_mixed_case(&self) -> bool;
    // fn has_seq(&self) -> bool;

    // // quality checks whether valid sequences of phred33, phred64, or solexa quality characters
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

    fn is_iupac(&self) -> bool {
        self.into_iter().all(|x| IUPAC_U8.contains(&x))
    }

    fn is_iupac_nucleotide(&self) -> bool {
    self.into_iter().all(|x| IUPAC_NUCLEOTIDE_U8.contains(&x))
    }
 
    fn is_iupac_amino_acid(&self) -> bool {
        self.into_iter().all(|x| IUPAC_AMINO_ACID_U8.contains(&x))
    }

    fn is_basic_dna(&self) -> bool {
        self.into_iter().all(|x| BASIC_DNA_U8.contains(&x))
    }

    fn is_basic_rna(&self) -> bool {
        self.into_iter().all(|x| BASIC_RNA_U8.contains(&x))
    }

    fn is_basic_amino_acid(&self) -> bool {
        self.into_iter().all(|x| BASIC_AMINO_ACID_U8.contains(&x))
    }

}