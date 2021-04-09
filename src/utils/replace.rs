// Copyright 2020 Christopher Sugai

//! Trait to random characters with pseudorandom bases (Nn->{AC{TU}G}, IUPAC R to {AG}).
//! # Examples
//! ```
//! extern crate rand;
//!
//! use crate::bioutils::utils::replace::AsMutRandomNucleotide;
//! use rand::rngs::ThreadRng;
//! use std::string::String;
//! use rand::seq::SliceRandom;
//!
//! let mut rng = rand::thread_rng(); //create a random number generator
//! let mut seq = *b"acugqqq";
//! let mut seq = seq.mut_to_upper_basic();
//! ```

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

use crate::charsets::iupac::*;

pub trait AsMutRandomNucleotide {
    fn mut_random_replace_n(&mut self, xna: &str, rng: ThreadRng) ;
    fn mut_random_replace_gap(&mut self, xna: &str, rng: ThreadRng) ;
    fn mut_random_replace_non_basic(&mut self, xna: &str, rng: ThreadRng) ;
    fn mut_random_replace_iupac(&mut self, xna: &str, rng: ThreadRng) ;
    fn mut_to_upper_basic(&mut self) ;
    fn mut_to_lower_basic(&mut self) ;
}

impl<T> AsMutRandomNucleotide for T
where T: AsMut<[u8]>,
{
    /// Fill {N,n} with pseudorandom nucleotides ACUG if xna is "RNA" or ACTG for all other xna.
    fn mut_random_replace_n(&mut self, xna: &str, mut rng: ThreadRng) {
        if xna == "RNA" {
            self.as_mut().iter_mut().for_each(|c| match c {
                b'N' => *c = *BASIC_RNA_U8.choose(&mut rng).unwrap(),
                b'n' => *c = *BASIC_LOWERCASE_RNA_U8.choose(&mut rng).unwrap(),
                    _ => {}
                })
        } else {
            self.as_mut().iter_mut().for_each(|c| match c {
                b'N' => *c = *BASIC_DNA_U8.choose(&mut rng).unwrap(),
                b'n' => *c = *BASIC_LOWERCASE_DNA_U8.choose(&mut rng).unwrap(),
                    _ => {}
                })
        }
    }

    /// Fill gaps {.,-} with pseudorandom nucleotides ACUG if xna is "RNA" or ACTG for all other xna.
    fn mut_random_replace_gap(&mut self, xna: &str, mut rng: ThreadRng)  {
        if xna == "RNA" {
            self.as_mut().iter_mut().for_each(|c| match c {
                    b'.' => *c = *BASIC_RNA_U8.choose(&mut rng).unwrap(),
                    b'-' => *c = *BASIC_RNA_U8.choose(&mut rng).unwrap(),
                    _ => {}
                })
        } else {
            self.as_mut().iter_mut().for_each(|c| match c {
                    b'.' => *c = *BASIC_DNA_U8.choose(&mut rng).unwrap(),
                    b'-' => *c = *BASIC_DNA_U8.choose(&mut rng).unwrap(),
                    _ => {}
                })
        }
    }

    /// random all other than ACGTUactgu with pseudorandom nucleotides ACTGU. Should be used last after other functions or for cleanup of unknown characters.
    fn mut_random_replace_non_basic(&mut self, xna: &str, mut rng: ThreadRng)  {
        if xna == "RNA" {
            self.as_mut().iter_mut().for_each(|c| match c {
                    b'A' => {},
                    b'a' => {},
                    b'C' => {},
                    b'c' => {},
                    b'T' => {},
                    b't' => {},
                    b'G' => {},
                    b'g' => {},
                    b'U' => {},
                    b'u' => {},
                    _ => *c = *BASIC_RNA_U8.choose(&mut rng).unwrap()
                })
        } else {
            self.as_mut().iter_mut().for_each(|c| match c {
                    b'A' => {},
                    b'a' => {},
                    b'C' => {},
                    b'c' => {},
                    b'T' => {},
                    b't' => {},
                    b'G' => {},
                    b'g' => {},
                    b'U' => {},
                    b'u' => {},
                    _ => *c = *BASIC_DNA_U8.choose(&mut rng).unwrap()
                })
        }
    }

    /// Specifically make actgu into ACTGU
    fn mut_to_upper_basic(&mut self)  {
        self.as_mut().iter_mut().for_each(|c| match c {
                b'A' => {},
                b'a' => *c = b'A',
                b'C' => {},
                b'c' => *c = b'C',
                b'T' => {},
                b't' => *c = b'T',
                b'G' => {},
                b'g' => *c = b'G',
                b'U' => {},
                b'u' => *c = b'U',
                _ => {}
            })
    }

    /// Specifically make ACTGU into actgu
    fn mut_to_lower_basic(&mut self)  {
        self.as_mut().iter_mut().for_each(|c| match c {
                b'A' => {},
                b'a' => *c = b'A',
                b'C' => {},
                b'c' => *c = b'C',
                b'T' => {},
                b't' => *c = b'T',
                b'G' => {},
                b'g' => *c = b'G',
                b'U' => {},
                b'u' => *c = b'U',
                _ => {}
            })
    }

    /// Pseudorandom nucleotide randomments within IUPAC specifications, e.g. R: either A or G. Case specific, r: either a or g.
    fn mut_random_replace_iupac(&mut self, xna: &str, mut rng: ThreadRng)  {
        if xna == "RNA" {
            self.as_mut().iter_mut().for_each(|c| match c {
                    b'R' => *c = *R_BASES.choose(&mut rng).unwrap(),
                    b'r' => *c = *R_BASES_LOWERCASE.choose(&mut rng).unwrap(),
                    b'Y' => *c = *Y_BASES_RNA.choose(&mut rng).unwrap(),
                    b'y' => *c = *Y_BASES_LOWERCASE_RNA.choose(&mut rng).unwrap(),
                    b'S' => *c = *S_BASES.choose(&mut rng).unwrap(),
                    b's' => *c = *S_BASES_LOWERCASE.choose(&mut rng).unwrap(),
                    b'W' => *c = *W_BASES_RNA.choose(&mut rng).unwrap(),
                    b'w' => *c = *W_BASES_LOWERCASE_RNA.choose(&mut rng).unwrap(),
                    b'K' => *c = *K_BASES_RNA.choose(&mut rng).unwrap(),
                    b'k' => *c = *K_BASES_LOWERCASE_RNA.choose(&mut rng).unwrap(),
                    b'M' => *c = *M_BASES.choose(&mut rng).unwrap(),
                    b'm' => *c = *M_BASES_LOWERCASE.choose(&mut rng).unwrap(),
                    b'B' => *c = *B_BASES_RNA.choose(&mut rng).unwrap(),
                    b'b' => *c = *B_BASES_LOWERCASE_RNA.choose(&mut rng).unwrap(),
                    b'D' => *c = *D_BASES_RNA.choose(&mut rng).unwrap(),
                    b'd' => *c = *D_BASES_LOWERCASE_RNA.choose(&mut rng).unwrap(),
                    b'H' => *c = *H_BASES_RNA.choose(&mut rng).unwrap(),
                    b'h' => *c = *H_BASES_LOWERCASE_RNA.choose(&mut rng).unwrap(),
                    b'V' => *c = *V_BASES.choose(&mut rng).unwrap(),
                    b'v' => *c = *V_BASES_LOWERCASE.choose(&mut rng).unwrap(),
                    _ => {}
                })
        } else {
            self.as_mut().iter_mut().for_each(|c| match c {
                    b'R' => *c = *R_BASES.choose(&mut rng).unwrap(),
                    b'r' => *c = *R_BASES_LOWERCASE.choose(&mut rng).unwrap(),
                    b'Y' => *c = *Y_BASES.choose(&mut rng).unwrap(),
                    b'y' => *c = *Y_BASES_LOWERCASE.choose(&mut rng).unwrap(),
                    b'S' => *c = *S_BASES.choose(&mut rng).unwrap(),
                    b's' => *c = *S_BASES_LOWERCASE.choose(&mut rng).unwrap(),
                    b'W' => *c = *W_BASES.choose(&mut rng).unwrap(),
                    b'w' => *c = *W_BASES_LOWERCASE.choose(&mut rng).unwrap(),
                    b'K' => *c = *K_BASES.choose(&mut rng).unwrap(),
                    b'k' => *c = *K_BASES_LOWERCASE.choose(&mut rng).unwrap(),
                    b'M' => *c = *M_BASES.choose(&mut rng).unwrap(),
                    b'm' => *c = *M_BASES_LOWERCASE.choose(&mut rng).unwrap(),
                    b'B' => *c = *B_BASES.choose(&mut rng).unwrap(),
                    b'b' => *c = *B_BASES_LOWERCASE.choose(&mut rng).unwrap(),
                    b'D' => *c = *D_BASES.choose(&mut rng).unwrap(),
                    b'd' => *c = *D_BASES_LOWERCASE.choose(&mut rng).unwrap(),
                    b'H' => *c = *H_BASES.choose(&mut rng).unwrap(),
                    b'h' => *c = *H_BASES_LOWERCASE.choose(&mut rng).unwrap(),
                    b'V' => *c = *V_BASES.choose(&mut rng).unwrap(),
                    b'v' => *c = *V_BASES_LOWERCASE.choose(&mut rng).unwrap(),
                    _ => {}
                })
        }
    }
}

pub trait CopyRandomNucleotide {
    fn copy_random_replace_n(&self, xna: &str, rng: ThreadRng) -> Vec<u8>  ;
    fn copy_random_replace_gap(&self, xna: &str, rng: ThreadRng) -> Vec<u8> ;
    // fn mut_random_replace_non_basic(&mut self, xna: &str, rng: ThreadRng) ;
    // fn mut_random_replace_iupac(&mut self, xna: &str, rng: ThreadRng) ;
    // fn mut_to_upper_basic(&mut self) ;
    // fn mut_to_lower_basic(&mut self) ;
}

impl<T> CopyRandomNucleotide for T
where T: IntoIterator<Item = u8> + Copy,
//     for<'a> &'a T: IntoIterator<Item = &'a u8> = Copy,
{
    /// Fill {N,n} with pseudorandom nucleotides ACUG if xna is "RNA" or ACTG for all other xna.
    fn copy_random_replace_n(&self, xna: &str, mut rng: ThreadRng) -> Vec<u8> {
        if xna == "RNA" {
            self.into_iter()
                .map(|ch| match ch {
                    b'N' => *BASIC_RNA_U8.choose(&mut rng).unwrap(),
                    b'n' => *BASIC_LOWERCASE_RNA_U8.choose(&mut rng).unwrap(),
                    _ => ch,
                }).collect::<Vec<u8>>()
        } else {
            self.into_iter()
            .map(|ch| match ch {
                b'N' => *BASIC_DNA_U8.choose(&mut rng).unwrap(),
                b'n' => *BASIC_LOWERCASE_DNA_U8.choose(&mut rng).unwrap(),
                _ => ch,
            }).collect::<Vec<u8>>()
        }
    }

    /// Fill gaps {.,-} with pseudorandom nucleotides ACUG if xna is "RNA" or ACTG for all other xna.
    fn copy_random_replace_gap(&self, xna: &str, mut rng: ThreadRng) -> Vec<u8> {
        if xna == "RNA" {
            self.into_iter()
                .map(|ch| match ch {
                    b'.' => *BASIC_RNA_U8.choose(&mut rng).unwrap(),
                    b'-' => *BASIC_RNA_U8.choose(&mut rng).unwrap(),
                    _ => ch,
                }).collect::<Vec<u8>>()
        } else {
            self.into_iter()
                .map(|ch| match ch {
                    b'.' => *BASIC_DNA_U8.choose(&mut rng).unwrap(),
                    b'-' => *BASIC_DNA_U8.choose(&mut rng).unwrap(),
                    _ => ch,
                }).collect::<Vec<u8>>()
        }
    }
}



// if non-ascii found, will replace with !, a q score of 0 by default. May also return error.
