// Copyright 2020 Christopher Sugai

//! Trait to random characters with pseudorandom bases (Nn->{AC{TU}G}, IUPAC R to {AG}).
//! # Examples
//! ```
//! extern crate rand;
//!
//! use rand::rngs::ThreadRng;
//! use std::string::String;
//! use rand::seq::SliceRandom;
//!
//! let mut rng = rand::thread_rng(); //create a random number generator
//! let mut seq = *b"acugqqq";
//! let mut seq = seq.to_upper_basic();
//! assert_eq!(b"ACUGqqq", seq);
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




// impl<T> randomNucleotide<T> for T
// where
//     for<'a> &'a T: IntoIterator<Item = &'a u8>,
// {
//     /// Fill {N,n} with pseudorandom nucleotides ACUG if xna is "RNA" or ACTG for all other xna.
//     fn random_n(&self, xna: &str, mut rng: ThreadRng)  {
//         if xna == "RNA" {
//             self.into_iter()
//                 .map(|ch| match ch {
//                     b'N' => *BASIC_RNA_U8.choose(&mut rng).unwrap(),
//                     b'n' => *BASIC_LOWERCASE_RNA_U8.choose(&mut rng).unwrap(),
//                     _ => *ch,
//                 })
                
//         } else {
//             self.into_iter()
//                 .map(|ch| match ch {
//                     b'N' => *BASIC_DNA_U8.choose(&mut rng).unwrap(),
//                     b'n' => *BASIC_LOWERCASE_DNA_U8.choose(&mut rng).unwrap(),
//                     _ => *ch,
//                 })
                
//         }
//     }

//     /// Fill gaps {.,-} with pseudorandom nucleotides ACUG if xna is "RNA" or ACTG for all other xna.
//     fn random_gap(&self, xna: &str, mut rng: ThreadRng)  {
//         if xna == "RNA" {
//             self.into_iter()
//                 .map(|ch| match ch {
//                     b'.' => *BASIC_RNA_U8.choose(&mut rng).unwrap(),
//                     b'-' => *BASIC_RNA_U8.choose(&mut rng).unwrap(),
//                     _ => *ch,
//                 })
                
//         } else {
//             self.into_iter()
//                 .map(|ch| match ch {
//                     b'.' => *BASIC_DNA_U8.choose(&mut rng).unwrap(),
//                     b'-' => *BASIC_DNA_U8.choose(&mut rng).unwrap(),
//                     _ => *ch,
//                 })
                
//         }
//     }

//     /// random all other than ACGTUactgu with pseudorandom nucleotides ACTGU. Should be used last after other functions or for cleanup of unknown characters.
//     fn random_non_basic(&self, xna: &str, mut rng: ThreadRng)  {
//         if xna == "RNA" {
//             self.into_iter()
//                 .map(|ch| match ch {
//                     b'A' => *ch,
//                     b'a' => *ch,
//                     b'C' => *ch,
//                     b'c' => *ch,
//                     b'T' => *ch,
//                     b't' => *ch,
//                     b'G' => *ch,
//                     b'g' => *ch,
//                     b'U' => *ch,
//                     b'u' => *ch,
//                     _ => *BASIC_RNA_U8.choose(&mut rng).unwrap(),
//                 })
                
//         } else {
//             self.into_iter()
//                 .map(|ch| match ch {
//                     b'A' => *ch,
//                     b'a' => *ch,
//                     b'C' => *ch,
//                     b'c' => *ch,
//                     b'T' => *ch,
//                     b't' => *ch,
//                     b'G' => *ch,
//                     b'g' => *ch,
//                     b'U' => *ch,
//                     b'u' => *ch,
//                     _ => *BASIC_DNA_U8.choose(&mut rng).unwrap(),
//                 })
                
//         }
//     }

//     /// Specifically make actgu into ACTGU
//     fn to_upper_basic(&self)  {
//         self.into_iter()
//             .map(|ch| match ch {
//                 b'A' => *ch,
//                 b'a' => b'A',
//                 b'C' => *ch,
//                 b'c' => b'C',
//                 b'T' => *ch,
//                 b't' => b'T',
//                 b'G' => *ch,
//                 b'g' => b'G',
//                 b'U' => *ch,
//                 b'u' => b'U',
//                 _ => *ch,
//             })
            
//     }

//     /// Specifically make ACTGU into actgu
//     fn to_lower_basic(&self)  {
//         self.into_iter()
//             .map(|ch| match ch {
//                 b'A' => *ch,
//                 b'a' => b'A',
//                 b'C' => *ch,
//                 b'c' => b'C',
//                 b'T' => *ch,
//                 b't' => b'T',
//                 b'G' => *ch,
//                 b'g' => b'G',
//                 b'U' => *ch,
//                 b'u' => b'U',
//                 _ => *ch,
//             })
            
//     }

//     /// Pseudorandom nucleotide randomments within IUPAC specifications, e.g. R: either A or G. Case specific, r: either a or g.
//     fn random_iupac(&self, xna: &str, mut rng: ThreadRng)  {
//         if xna == "RNA" {
//             self.into_iter()
//                 .map(|ch| match ch {
//                     b'R' => *R_BASES.choose(&mut rng).unwrap(),
//                     b'r' => *R_BASES_LOWERCASE.choose(&mut rng).unwrap(),
//                     b'Y' => *Y_BASES_RNA.choose(&mut rng).unwrap(),
//                     b'y' => *Y_BASES_LOWERCASE_RNA.choose(&mut rng).unwrap(),
//                     b'S' => *S_BASES.choose(&mut rng).unwrap(),
//                     b's' => *S_BASES_LOWERCASE.choose(&mut rng).unwrap(),
//                     b'W' => *W_BASES_RNA.choose(&mut rng).unwrap(),
//                     b'w' => *W_BASES_LOWERCASE_RNA.choose(&mut rng).unwrap(),
//                     b'K' => *K_BASES_RNA.choose(&mut rng).unwrap(),
//                     b'k' => *K_BASES_LOWERCASE_RNA.choose(&mut rng).unwrap(),
//                     b'M' => *M_BASES.choose(&mut rng).unwrap(),
//                     b'm' => *M_BASES_LOWERCASE.choose(&mut rng).unwrap(),
//                     b'B' => *B_BASES_RNA.choose(&mut rng).unwrap(),
//                     b'b' => *B_BASES_LOWERCASE_RNA.choose(&mut rng).unwrap(),
//                     b'D' => *D_BASES_RNA.choose(&mut rng).unwrap(),
//                     b'd' => *D_BASES_LOWERCASE_RNA.choose(&mut rng).unwrap(),
//                     b'H' => *H_BASES_RNA.choose(&mut rng).unwrap(),
//                     b'h' => *H_BASES_LOWERCASE_RNA.choose(&mut rng).unwrap(),
//                     b'V' => *V_BASES.choose(&mut rng).unwrap(),
//                     b'v' => *V_BASES_LOWERCASE.choose(&mut rng).unwrap(),
//                     _ => *ch,
//                 })
                
//         } else {
//             self.into_iter()
//                 .map(|ch| match ch {
//                     b'R' => *R_BASES.choose(&mut rng).unwrap(),
//                     b'r' => *R_BASES_LOWERCASE.choose(&mut rng).unwrap(),
//                     b'Y' => *Y_BASES.choose(&mut rng).unwrap(),
//                     b'y' => *Y_BASES_LOWERCASE.choose(&mut rng).unwrap(),
//                     b'S' => *S_BASES.choose(&mut rng).unwrap(),
//                     b's' => *S_BASES_LOWERCASE.choose(&mut rng).unwrap(),
//                     b'W' => *W_BASES.choose(&mut rng).unwrap(),
//                     b'w' => *W_BASES_LOWERCASE.choose(&mut rng).unwrap(),
//                     b'K' => *K_BASES.choose(&mut rng).unwrap(),
//                     b'k' => *K_BASES_LOWERCASE.choose(&mut rng).unwrap(),
//                     b'M' => *M_BASES.choose(&mut rng).unwrap(),
//                     b'm' => *M_BASES_LOWERCASE.choose(&mut rng).unwrap(),
//                     b'B' => *B_BASES.choose(&mut rng).unwrap(),
//                     b'b' => *B_BASES_LOWERCASE.choose(&mut rng).unwrap(),
//                     b'D' => *D_BASES.choose(&mut rng).unwrap(),
//                     b'd' => *D_BASES_LOWERCASE.choose(&mut rng).unwrap(),
//                     b'H' => *H_BASES.choose(&mut rng).unwrap(),
//                     b'h' => *H_BASES_LOWERCASE.choose(&mut rng).unwrap(),
//                     b'V' => *V_BASES.choose(&mut rng).unwrap(),
//                     b'v' => *V_BASES_LOWERCASE.choose(&mut rng).unwrap(),
//                     _ => *ch,
//                 })
                
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use crate::utils::random::randomNucleotide;
//     #[test]
//     fn test_random_u_with_t() {
//     let mut rng = rand::thread_rng();
//     let mut test = b"ACTG";
//     test.random_u_with_t();
//     assert_eq!(test, b"ACUG");
// }
// fn test_random_x_with_y() {
//     let mut rng = rand::thread_rng();
//     let mut test = b"ACTG";
//     test.random_x_with_y(b"T", b"U");
//     assert_eq!(test, b"ACUG");
// }
// }

// #[test]
// fn test_random_gap() {
//     let mut rng = rand::thread_rng();
//     let test = "ACTGuyUNn.-@<^>".to_string().random_gap(rng);
//     assert_eq!(test.len(), "ACTGuyUNn.-@<^>".to_string().len());
// }

// #[test]
// fn test_random_gap_lowercase() {
//     let mut rng = rand::thread_rng();
//     let test = "ACTGuyUNn.-@<^>".to_string().random_gap_lowercase(rng);
//     assert_eq!(test.len(), "ACTGuyUNn.-@<^>".to_string().len());
// }

// #[test]
// fn test_random_n() {
//     let mut rng = rand::thread_rng();
//     let test = "ACTGuyUNn.-@<^>".to_string().random_n(rng);
//     assert_eq!(test.len(), "ACTGuyUNn.-@<^>".to_string().len());
// }

// #[test]
// fn test_random_iupac() {
//     let mut rng = rand::thread_rng();
//     let test = "ACTGuyUNn.-@<^>".to_string().random_iupac(rng);
//     assert_eq!(test.len(), "ACTGuyUNn.-@<^>".to_string().len());
// }

// #[test]
// fn test_random_all_other_with_uppercase() {
//     let mut rng = rand::thread_rng();
//     let test = "ACTGuyUNn.-@<^>".to_string().random_all_other_with_uppercase(rng);
//     assert_eq!(test.len(), "ACTGuyUNn.-@<^>".to_string().len());
// }

// #[test]
// fn test_rrandom_all_other_with_lowercase() {
//     let mut rng = rand::thread_rng();
//     let test = "ACTGuyUNn.-@<^>".to_string().random_all_other_with_lowercase(rng);
//     assert_eq!(test.len(), "ACTGuyUNn.-@<^>".to_string().len());
// }

// // fn random_grapheme_at(s: &str, c: &str) -> String {
// //     let mut r = String::with_capacity(s.len());
// //     for (i, g) in s.grapheme_indices(true) {
// //         r.push_str(if i == idx { c } else { g });
// //     }
// //     r
// // }

// // fn get_extension_from_filename(file: &PathBuf) -> Option<&str> {
// //     Path::new(file)
// //         .extension()
// //         .and_then(OsStr::to_str)
// // }

// // fn check_gz(filestr: Option<&str>) -> bool {
// //     match filestr.unwrap() {
// //         "gz" => true,
// //         _ => false
// //     }
// // }

// // fn read_fastq(fastq_path: &std::path::PathBuf) -> seq_io::fastq::Reader<std::fs::File> {
// //     let fq = File::open(fastq_path).expect("Could not open Fastq");
// //     let ext = get_extension_from_filename(fastq_path);
// //     if check_gz(ext) {
// //         let fq = flate2::read::GzDecoder::new(fq).into_inner();
// //         let mut reader = seq_io::fastq::Reader::new(fq);
// //         reader
// //     } else {
// //         let mut reader = seq_io::fastq::Reader::new(fq);
// //         reader
// //     }
// // }

// impl<T> randomNucleotide<T> for T where T: AsMut<[u8]> {
// // impl randomNucleotide<String> for String {
//     /// Specifically random Uu with Tt, lowercase to lowercase, uppercase to uppercase. Leaves all other characters in place.
//     fn random_u_with_t(&mut self) {
//         self.as_mut().iter_mut().for_each(|c| match c {
//             b'U' => *c = b'T',
//             b'u' => *c = b'T',
//             _ => {}
//         })
//     }

//     fn random_x_with_y(&mut self, rx: u8, ry: u8) {
//         self.as_mut().iter_mut().for_each(|c| match rx {
//             rx => *c = ry,
//             _ => {}
//         })
// //     }
