// // Copyright 2020 Christopher Sugai

// //! Checks and classifications returning booleans for an input sequence and basic user input. For example, dna.is_homopolymer(90) will return boolean true if dna is comprised of >90% a single nucleotide. 
// //! Useful external crates to use in combination with these functions: std ascii, especially is_ascii(), rust regex to check for punctuation
// //! # Examples
// //! ```
// //! ```


// trait CheckSequence <T: From<u8>> {

//     // sequence checks whether valid IUPAC sequences
//     fn is_iupac_nucleotide_sequence(&self) -> Self;
//     fn is_iupac_amino_acid_sequence(&self) -> Self;
//     fn is_basic_nucleotide_sequence(&self) -> Self;
//     fn is_basic_amino_acid_sequence(&self) -> Self;
//     fn has_gap(&self) -> Self;
//     fn has_n(&self) -> Self;
//     fn has_mixed_case(&self) -> Self;
//     fn has_seq(&self) -> Self;

//     // quality checks whether valid sequences of quality characters and filtering utilities
//     fn is_quality_sequence(&self) -> Self; // the same as is_phred33_quality_sequence
//     fn is_phred33_quality_sequence(&self) -> Self;
//     fn is_phred64_quality_sequence(&self) -> Self;
//     fn is_solexa_quality_sequence(&self) -> Self;

//     fn is_quality(&self) -> Self;
//     fn is_quality(&self) -> Self;

//     //sequence classification
//     fn is_homopolymer(&self) -> Self;
//     fn is_palindrome(&self) -> Self;
// }

