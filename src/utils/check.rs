// Copyright 2020 Christopher Sugai

//! Checks and classifications returning booleans for an input sequence and basic user input. For example, dna.is_homopolymer(90) returns boolean true if dna is comprised of >90% a single nucleotide. 
//! # Examples
//! ```
//! ```

use crate::charsets::iupac::*;
use crate::charsets::quality::*;
use std::str::from_utf8;

trait CheckSequence<T: From<u8>> {

    // // sequence checks
    // fn has_gap(&self) -> bool;
    // fn has_n(&self) -> bool;
    // fn has_mixed_case(&self) -> bool;
    // fn has_seq(&self) -> bool;
    fn is_iupac_nucleotide(&self) -> bool;
    // fn is_iupac_amino_acid(&self) -> bool;
    // fn is_basic_nucleotide(&self) -> bool;
    // fn is_basic_amino_acid(&self) -> bool;
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

impl<T> dyn CheckSequence<T> where T: From<u8> {
    fn is_iupac_nucleotide(&self) -> bool {
        if IUPAC_NUCLEOTIDE_U8.contains(self as u8) {true} else{false}
        // if self.iter().any(|&IUPAC_NUCLwEOTIDE_U8| IUPAC_NUCLEOTIDE_U8==self) {
        //     true
        }
    // fn is_iupac_amino_acid(&self) -> bool {
    //     true
    // }
    // fn is_basic_nucleotide(&self) -> bool {
    //     true
    // }
    // fn is_basic_amino_acid(&self) -> bool {
    //     true
    // }
}