
//! Trait to random characters with pseudorandom bases (Nn->{AC{TU}G}, IUPAC R to {AG}).
//! # Examples
//! ```
//! extern crate rand;
//!
//! use crate::bioutils::utils::mutate::random::AsMutRandomU8;
//! use rand::rngs::ThreadRng;
//! use std::string::String;
//! use std::str;
//! use rand::seq::SliceRandom;
//!
//! let mut rng1 = rand::thread_rng(); //create a random number generator
//! let mut rng2 = rand::thread_rng(); //create a random number generator
//! let mut seq = b"acugnnnqqq".to_owned(); // or by *: let mut seq = *b"acugnnnqqq";
//! let mut seq = seq.mut_random_replace_non_basic("RNA", rng1).mut_random_replace_n("RNA", rng2).mut_to_upper_basic();
//! let printseq = str::from_utf8(seq).unwrap(); // turn into utf8
//! println!("{:?}", printseq);
//! ```

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use crate::charsets::iupac::*;


pub trait AsMutRandomU8 {
    fn mut_random_replace_non_basic(&mut self, xna: &str, rng: ThreadRng) -> &mut Self ;
    fn mut_random_replace_n(&mut self, xna: &str, rng: ThreadRng) -> &mut Self ;
    fn mut_random_replace_gap(&mut self, xna: &str, rng: ThreadRng) -> &mut Self ;
    fn mut_random_replace_iupac(&mut self, xna: &str, rng: ThreadRng) -> &mut Self ;
    fn mut_to_upper_basic(&mut self) -> &mut Self ;
    fn mut_to_lower_basic(&mut self) -> &mut Self ;
}

impl<T> AsMutRandomU8 for T
where T: AsMut<[u8]>,
{
    /// Fill {N,n} with pseudorandom nucleotides ACUG if xna is "RNA" or ACTG for all other xna.
    fn mut_random_replace_n(&mut self, xna: &str, mut rng: ThreadRng) -> &mut Self {
        if xna == "RNA" {
            self.as_mut().iter_mut().for_each(|c| match c {
                b'N' => *c = *BASIC_RNA_U8.choose(&mut rng).unwrap(),
                b'n' => *c = *BASIC_LOWERCASE_RNA_U8.choose(&mut rng).unwrap(),
                    _ => {}
                })
        } else if xna == "DNA" {
            self.as_mut().iter_mut().for_each(|c| match c {
                b'N' => *c = *BASIC_DNA_U8.choose(&mut rng).unwrap(),
                b'n' => *c = *BASIC_LOWERCASE_DNA_U8.choose(&mut rng).unwrap(),
                    _ => {}
                })
        }
        self
    }

    /// Fill gaps {.,-} with pseudorandom nucleotides ACUG if xna is "RNA" or ACTG for all other xna.
    fn mut_random_replace_gap(&mut self, xna: &str, mut rng: ThreadRng) -> &mut Self {
        if xna == "RNA" {
            self.as_mut().iter_mut().for_each(|c| match c {
                    b'.' => *c = *BASIC_RNA_U8.choose(&mut rng).unwrap(),
                    b'-' => *c = *BASIC_RNA_U8.choose(&mut rng).unwrap(),
                    _ => {}
                })
        } else if xna == "DNA" {
            self.as_mut().iter_mut().for_each(|c| match c {
                    b'.' => *c = *BASIC_DNA_U8.choose(&mut rng).unwrap(),
                    b'-' => *c = *BASIC_DNA_U8.choose(&mut rng).unwrap(),
                    _ => {}
                })
        }
        self
    }

    /// random all other than ACGTUactgu with pseudorandom nucleotides ACTGU. Should be used last after other functions or for cleanup of unknown characters.
    fn mut_random_replace_non_basic(&mut self, xna: &str, mut rng: ThreadRng) -> &mut Self {
        if xna == "RNA" {
            self.as_mut().iter_mut().for_each(|c| match c {
                    b'A' => {},
                    b'a' => {},
                    b'C' => {},
                    b'c' => {},
                    b'G' => {},
                    b'g' => {},
                    b'U' => {},
                    b'u' => {},
                    _ => *c = *BASIC_RNA_U8.choose(&mut rng).unwrap()
                })
        } else if xna == "DNA" {
            self.as_mut().iter_mut().for_each(|c| match c {
                    b'A' => {},
                    b'a' => {},
                    b'C' => {},
                    b'c' => {},
                    b'T' => {},
                    b't' => {},
                    b'G' => {},
                    b'g' => {},
                    _ => *c = *BASIC_DNA_U8.choose(&mut rng).unwrap()
                })
        }
        self
    }

    /// Specifically make actgu into ACTGU
    fn mut_to_upper_basic(&mut self) -> &mut Self {
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
            });
        self
    }

    /// Specifically make ACTGU into actgu
    fn mut_to_lower_basic(&mut self) -> &mut Self {
        self.as_mut().iter_mut().for_each(|c| match c {
                b'A' => *c = b'a',
                b'a' => {},
                b'C' => *c = b'c',
                b'c' => {},
                b'T' => *c = b't',
                b't' => {},
                b'G' => *c = b'g',
                b'g' => {},
                b'U' => *c = b'u',
                b'u' => {},
                _ => {}
            });
        self
    }
    /// Pseudorandom nucleotide randomments within IUPAC specifications, e.g. R: either A or G. Case specific, r: either a or g.
    fn mut_random_replace_iupac(&mut self, xna: &str, mut rng: ThreadRng) -> &mut Self {
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
        } else if xna == "DNA" {
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
        self
    }
}

pub trait CopyRandomNucleotide<T> {
    fn copy_random_replace_n(&self, xna: &str, rng: ThreadRng) -> Vec<u8>  ;
    fn copy_random_replace_gap(&self, xna: &str, rng: ThreadRng) -> Vec<u8> ;
    // fn mut_random_replace_non_basic(&mut self, xna: &str, rng: ThreadRng) ;
    // fn mut_random_replace_iupac(&mut self, xna: &str, rng: ThreadRng) ;
    // fn mut_to_upper_basic(&mut self) ;
    // fn mut_to_lower_basic(&mut self) ;
}

impl<T> CopyRandomNucleotide<T> for T
where
    for<'a> &'a T: IntoIterator<Item = &'a u8>,
{
    /// Fill {N,n} with pseudorandom nucleotides ACUG if xna is "RNA" or ACTG for all other xna.
    fn copy_random_replace_n(&self, xna: &str, mut rng: ThreadRng) -> Vec<u8> {
        if xna == "RNA" {
            self.into_iter()
                .map(|ch| match ch {
                    b'N' => *BASIC_RNA_U8.choose(&mut rng).unwrap(),
                    b'n' => *BASIC_LOWERCASE_RNA_U8.choose(&mut rng).unwrap(),
                    _ => *ch,
                }).collect::<Vec<u8>>()
        } else {
            self.into_iter()
            .map(|ch| match ch {
                b'N' => *BASIC_DNA_U8.choose(&mut rng).unwrap(),
                b'n' => *BASIC_LOWERCASE_DNA_U8.choose(&mut rng).unwrap(),
                _ => *ch,
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
                    _ => *ch,
                }).collect::<Vec<u8>>()
        } else {
            self.into_iter()
                .map(|ch| match ch {
                    b'.' => *BASIC_DNA_U8.choose(&mut rng).unwrap(),
                    b'-' => *BASIC_DNA_U8.choose(&mut rng).unwrap(),
                    _ => *ch,
                }).collect::<Vec<u8>>()
        }
    }
}



// if non-ascii found, will replace with !, a q score of 0 by default. May also return error.
