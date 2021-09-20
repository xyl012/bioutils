//! Trait to random characters with pseudorandom bases (Nn->{AC{TU}G}, IUPAC R to {AG}).
//! Clean up (replace) non-target characters in a slice with target random characters.
//! # Examples
//! ```
//! extern crate rand;
//! use rand::rngs::ThreadRng;
//! use std::str;
//! use rand::seq::SliceRandom;
//! use crate::bioutils::utils::replace::CleanAsMutSlice;
//! use crate::bioutils::utils::replace::XnaCleanAsMutSlice;
//! 
//! let mut rng1 = rand::thread_rng(); //create a random number generator
//! let mut rng2 = rand::thread_rng(); //create a random number generator
//! let mut seq = b"acugnnnqqq".to_owned(); // or by *: let mut seq = *b"acugnnnqqq";
//! seq.mut_xna_clean("RNA", rng1).unwrap().mut_xna_clean_n("RNA", rng2).unwrap().mut_to_upper_basic().unwrap();
//! let printseq = str::from_utf8(&seq); // turn into utf8
//! println!("{:?}", printseq);
//! ```

use super::*;

/// Takes a BioUtilsCharSet and a ThreadRng and replaces any character not in the charset with a random character from the characterset.
pub trait CleanAsMutSlice<T> {
    /// Takes a BioUtilsCharSet and a ThreadRng and replaces any character not in the charset with a random character from the characterset.
    fn mut_clean(&mut self, charset: BioUtilsCharSet, rng: ThreadRng) -> Result<&mut Self>;
    /// Takes a character slice and a ThreadRng and replaces any character not in the charset with a random character from the characterset.
    fn mut_clean_with(&mut self, charset: &[u8], rng: ThreadRng) -> Result<&mut Self>;
}

/// Takes a BioUtilsCharSet and a ThreadRng and replaces any character not in the charset with a random character from the characterset.
impl<T> CleanAsMutSlice<T> for T where 
T: AsMut<[u8]>,
{
    /// Takes a BioUtilsCharSet and a ThreadRng and replaces any character not in the charset with a random character from the characterset.
    fn mut_clean(&mut self, charset: BioUtilsCharSet, mut rng: ThreadRng) -> Result<&mut Self> {
        self.as_mut().iter_mut().for_each(|c| if charset.value().contains(c) {} else {*c = *charset.value().choose(&mut rng).expect("Could not choose character")});
        Ok(self)
    }
    /// Takes a character slice and a ThreadRng and replaces any character not in the charset with a random character from the characterset.
    fn mut_clean_with(&mut self, charset: &[u8], mut rng: ThreadRng) -> Result<&mut Self> {
        self.as_mut().iter_mut().for_each(|c| if charset.contains(c) {} else {*c = *charset.choose(&mut rng).expect("Could not choose character")});
        Ok(self)
    }
}

pub trait XnaCleanAsMutSlice {
    /// Random all other than ACG(T/U)acg(t/u) with pseudorandom nucleotides ACG(T/U). Should be used last after other functions or for cleanup of unknown characters.
    fn mut_xna_clean(&mut self, xna: &str, rng: ThreadRng) -> Result<&mut Self>;
    /// Fill {N,n} with pseudorandom nucleotides ACUG if xna is "RNA" or ACTG for all other xna.
    fn mut_xna_clean_n(&mut self, xna: &str, rng: ThreadRng) -> Result<&mut Self>;
    /// Fill gaps {.,-} with pseudorandom nucleotides ACUG if xna is "RNA" or ACTG for all other xna.
    fn mut_xna_clean_gap(&mut self, xna: &str, rng: ThreadRng) -> Result<&mut Self>;
    /// Specifically make actgu into ACTGU
    fn mut_to_upper_basic(&mut self) -> Result<&mut Self>;
    /// Specifically make ACTGU into actgu
    fn mut_to_lower_basic(&mut self) -> Result<&mut Self>;
    /// Pseudorandom nucleotide randomments within IUPAC specifications, e.g. R: either A or G. Case specific, r: either a or g.
    fn mut_xna_clean_iupac(&mut self, xna: &str, rng: ThreadRng) -> Result<&mut Self>;
}

impl<T> XnaCleanAsMutSlice for T where 
T: AsMut<[u8]>,
{
    /// random all other than ACGTUactgu with pseudorandom nucleotides ACTGU. Should be used last after other functions or for cleanup of unknown characters.
    fn mut_xna_clean(&mut self, xna: &str, mut rng: ThreadRng) -> Result<&mut Self> {
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
                    _ => *c = *RNA.choose(&mut rng).expect("Could not choose base"),
                }
            )
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
                    _ => *c = *DNA.choose(&mut rng).expect("Could not choose base"),
                })
        }
        Ok(self)
    }

    /// Fill {N,n} with pseudorandom nucleotides ACUG if xna is "RNA" or ACTG for all other xna.
    fn mut_xna_clean_n(&mut self, xna: &str, mut rng: ThreadRng) -> Result<&mut Self> {
        if xna == "RNA" {
            self.as_mut().iter_mut().for_each(|c| match c {
                b'N' => *c = *RNA.choose(&mut rng).expect("Could not choose base"),
                b'n' => *c = *RNA_LOWERCASE.choose(&mut rng).expect("Could not choose base"),
                    _ => {}
                })
        } else if xna == "DNA" {
            self.as_mut().iter_mut().for_each(|c| match c {
                b'N' => *c = *DNA.choose(&mut rng).expect("Could not choose base"),
                b'n' => *c = *DNA_LOWERCASE.choose(&mut rng).expect("Could not choose base"),
                    _ => {}
                })
        }
        Ok(self)
    }

    /// Fill gaps {.,-} with pseudorandom nucleotides ACUG if xna is "RNA" or ACTG for all other xna.
    fn mut_xna_clean_gap(&mut self, xna: &str, mut rng: ThreadRng) -> Result<&mut Self> {
        if xna == "RNA" {
            self.as_mut().iter_mut().for_each(|c| match c {
                    b'.' => *c = *RNA.choose(&mut rng).expect("Could not choose base"),
                    b'-' => *c = *RNA.choose(&mut rng).expect("Could not choose base"),
                    _ => {}
                })
        } else if xna == "DNA" {
            self.as_mut().iter_mut().for_each(|c| match c {
                    b'.' => *c = *DNA.choose(&mut rng).expect("Could not choose base"),
                    b'-' => *c = *DNA.choose(&mut rng).expect("Could not choose base"),
                    _ => {}
                })
        }
        Ok(self)
    }

    /// Specifically make actgu into ACTGU, leave all others intact
    fn mut_to_upper_basic(&mut self) -> Result<&mut Self> {
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
        Ok(self)
    }

    /// Specifically make ACTGU into actgu, leave all others intact
    fn mut_to_lower_basic(&mut self) -> Result<&mut Self> {
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
        Ok(self)
    }
    /// Pseudorandom nucleotide randomments within IUPAC specifications, e.g. R: either A or G. Case specific, r: either a or g.
    fn mut_xna_clean_iupac(&mut self, xna: &str, mut rng: ThreadRng) -> Result<&mut Self> {
        if xna == "RNA" {
            self.as_mut().iter_mut().for_each(|c| match c {
                    b'R' => *c = *R_BASES.choose(&mut rng).expect("Could not choose base"),
                    b'r' => *c = *R_BASES_LOWERCASE.choose(&mut rng).expect("Could not choose base"),
                    b'Y' => *c = *Y_BASES_RNA.choose(&mut rng).expect("Could not choose base"),
                    b'y' => *c = *Y_BASES_RNA_LOWERCASE.choose(&mut rng).expect("Could not choose base"),
                    b'S' => *c = *S_BASES.choose(&mut rng).expect("Could not choose base"),
                    b's' => *c = *S_BASES_LOWERCASE.choose(&mut rng).expect("Could not choose base"),
                    b'W' => *c = *W_BASES_RNA.choose(&mut rng).expect("Could not choose base"),
                    b'w' => *c = *W_BASES_RNA_LOWERCASE.choose(&mut rng).expect("Could not choose base"),
                    b'K' => *c = *K_BASES_RNA.choose(&mut rng).expect("Could not choose base"),
                    b'k' => *c = *K_BASES_RNA_LOWERCASE.choose(&mut rng).expect("Could not choose base"),
                    b'M' => *c = *M_BASES.choose(&mut rng).expect("Could not choose base"),
                    b'm' => *c = *M_BASES_LOWERCASE.choose(&mut rng).expect("Could not choose base"),
                    b'B' => *c = *B_BASES_RNA.choose(&mut rng).expect("Could not choose base"),
                    b'b' => *c = *B_BASES_RNA_LOWERCASE.choose(&mut rng).expect("Could not choose base"),
                    b'D' => *c = *D_BASES_RNA.choose(&mut rng).expect("Could not choose base"),
                    b'd' => *c = *D_BASES_RNA_LOWERCASE.choose(&mut rng).expect("Could not choose base"),
                    b'H' => *c = *H_BASES_RNA.choose(&mut rng).expect("Could not choose base"),
                    b'h' => *c = *H_BASES_RNA_LOWERCASE.choose(&mut rng).expect("Could not choose base"),
                    b'V' => *c = *V_BASES.choose(&mut rng).expect("Could not choose base"),
                    b'v' => *c = *V_BASES_LOWERCASE.choose(&mut rng).expect("Could not choose base"),
                    _ => {}
                })
        } else if xna == "DNA" {
            self.as_mut().iter_mut().for_each(|c| match c {
                    b'R' => *c = *R_BASES.choose(&mut rng).expect("Could not choose base"),
                    b'r' => *c = *R_BASES_LOWERCASE.choose(&mut rng).expect("Could not choose base"),
                    b'Y' => *c = *Y_BASES.choose(&mut rng).expect("Could not choose base"),
                    b'y' => *c = *Y_BASES_LOWERCASE.choose(&mut rng).expect("Could not choose base"),
                    b'S' => *c = *S_BASES.choose(&mut rng).expect("Could not choose base"),
                    b's' => *c = *S_BASES_LOWERCASE.choose(&mut rng).expect("Could not choose base"),
                    b'W' => *c = *W_BASES.choose(&mut rng).expect("Could not choose base"),
                    b'w' => *c = *W_BASES_LOWERCASE.choose(&mut rng).expect("Could not choose base"),
                    b'K' => *c = *K_BASES.choose(&mut rng).expect("Could not choose base"),
                    b'k' => *c = *K_BASES_LOWERCASE.choose(&mut rng).expect("Could not choose base"),
                    b'M' => *c = *M_BASES.choose(&mut rng).expect("Could not choose base"),
                    b'm' => *c = *M_BASES_LOWERCASE.choose(&mut rng).expect("Could not choose base"),
                    b'B' => *c = *B_BASES.choose(&mut rng).expect("Could not choose base"),
                    b'b' => *c = *B_BASES_LOWERCASE.choose(&mut rng).expect("Could not choose base"),
                    b'D' => *c = *D_BASES.choose(&mut rng).expect("Could not choose base"),
                    b'd' => *c = *D_BASES_LOWERCASE.choose(&mut rng).expect("Could not choose base"),
                    b'H' => *c = *H_BASES.choose(&mut rng).expect("Could not choose base"),
                    b'h' => *c = *H_BASES_LOWERCASE.choose(&mut rng).expect("Could not choose base"),
                    b'V' => *c = *V_BASES.choose(&mut rng).expect("Could not choose base"),
                    b'v' => *c = *V_BASES_LOWERCASE.choose(&mut rng).expect("Could not choose base"),
                    _ => {}
                })
        }
        Ok(self)
    }
}

pub trait CleanIntoIteratorSlice<T> {
    /// Random all other than ACG(T/U)acg(t/u) with pseudorandom nucleotides ACG(T/U). Should be used last after other functions or for cleanup of unknown characters.
    fn vec_xna_clean(&self, xna: &str, rng: ThreadRng) -> Vec<u8> ;
    /// Fill {N,n} with pseudorandom nucleotides ACUG if xna is "RNA" or ACTG for all other xna.
    fn vec_xna_clean_n(&self, xna: &str, rng: ThreadRng) -> Vec<u8> ;
    /// Fill gaps {.,-} with pseudorandom nucleotides ACUG if xna is "RNA" or ACTG for all other xna.
    fn vec_xna_clean_gap(&self, xna: &str, rng: ThreadRng) -> Vec<u8> ;
}

impl<T> CleanIntoIteratorSlice<T> for T
where
    T: IntoIterator<Item = u8>,
    T: Copy,
{
    /// Random all other than ACG(T/U)acg(t/u) with pseudorandom nucleotides ACG(T/U). Should be used last after other functions or for cleanup of unknown characters.
    fn vec_xna_clean(&self, xna: &str, mut rng: ThreadRng) -> Vec<u8> {
        if xna == "RNA" {
            self.into_iter()
                .map(|base| match base {
                    b'A' => b'A',
                    b'a' => b'a',
                    b'C' => b'C',
                    b'c' => b'c',
                    b'G' => b'G',
                    b'g' => b'g',
                    b'U' => b'U',
                    b'u' => b'u',
                    _ => *RNA.choose(&mut rng).expect("Could not choose base"),
                }).collect::<Vec<u8>>()
        } else {
            self.into_iter()
            .map(|base| match base {
                b'A' => b'A',
                b'a' => b'a',
                b'C' => b'C',
                b'c' => b'c',
                b'G' => b'G',
                b'g' => b'g',
                b'U' => b'U',
                b'u' => b'u',
                _ => *DNA.choose(&mut rng).expect("Could not choose base"),
            }).collect::<Vec<u8>>()
        }
    }

    /// Fill {N,n} with pseudorandom nucleotides ACUG if xna is "RNA" or ACTG for all other xna.
    fn vec_xna_clean_n(&self, xna: &str, mut rng: ThreadRng) -> Vec<u8> {
        if xna == "RNA" {
            self.into_iter()
                .map(|base| match base {
                    b'N' => *RNA.choose(&mut rng).expect("Could not choose base"),
                    b'n' => *RNA_LOWERCASE.choose(&mut rng).expect("Could not choose base"),
                    _ => base,
                }).collect::<Vec<u8>>()
        } else {
            self.into_iter()
            .map(|base| match base {
                b'N' => *DNA.choose(&mut rng).expect("Could not choose base"),
                b'n' => *DNA_LOWERCASE.choose(&mut rng).expect("Could not choose base"),
                _ => base,
            }).collect::<Vec<u8>>()
        }
    }

    /// Fill gaps {.,-} with pseudorandom nucleotides ACUG if xna is "RNA" or ACTG for all other xna.
    fn vec_xna_clean_gap(&self, xna: &str, mut rng: ThreadRng) -> Vec<u8> {
        if xna == "RNA" {
            self.into_iter()
                .map(|base| match base {
                    b'.' => *RNA.choose(&mut rng).expect("Could not choose base"),
                    b'-' => *RNA.choose(&mut rng).expect("Could not choose base"),
                    _ => base,
                }).collect::<Vec<u8>>()
        } else {
            self.into_iter()
                .map(|base| match base {
                    b'.' => *DNA.choose(&mut rng).expect("Could not choose base"),
                    b'-' => *DNA.choose(&mut rng).expect("Could not choose base"),
                    _ => base,
                }).collect::<Vec<u8>>()
        }
    }
}

