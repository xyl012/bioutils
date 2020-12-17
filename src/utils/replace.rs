// Copyright 2020 Christopher Sugai

//! Trait to replace characters with pseudorandom bases (Nn->{AC{TU}G}, IUPAC R to {AG}).
//! # Examples
//! ```
//! extern crate rand;
//!
//! use rand::rngs::ThreadRng;
//! use std::string::String;
//! use rand::seq::SliceRandom;
//!
//! let mut rng = rand::thread_rng(); //create a random number generator
//! let mut seq = b"acugqqq".to_upper_basic(); 
//!
//! assert_eq!(b"ACUGqqq", seq);
//! ```

// // replace gap {.-} using the random number generator

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

use crate::charsets::iupac::*;

pub trait ReplaceNucleotide<T> {

    fn replace_n(&self, xna: &str, rng: ThreadRng) -> Vec<u8>;
    fn replace_gap(&self, xna: &str, rng: ThreadRng) -> Vec<u8>;
    fn replace_non_basic(&self, xna: &str, rng: ThreadRng) -> Vec<u8>;
    fn replace_iupac(&self, xna: &str, rng: ThreadRng) -> Vec<u8>;
    fn to_upper_basic(&self) -> Vec<u8>;
    fn to_lower_basic(&self) -> Vec<u8>;
}

// impl<T> ReplaceNucleotide<T> for T
// where
//     for<'a> &'a T: IntoIterator<Item = &'a u8> + AsMut<&'a u8>,
// {
impl<T> ReplaceNucleotide<T> for T
where T: IntoIterator<Item = u8> + AsMut<u8> + Copy,
{
    /// Fill {N,n} with pseudorandom nucleotides ACUG if xna is "RNA" or ACTG for all other xna.
    fn replace_n(&self, xna: &str, mut rng: ThreadRng) -> Vec<u8> {
        if xna == "RNA" {
            self.into_iter()
                .map(|ch| match ch {
                    b'N' => *BASIC_RNA_U8.choose(&mut rng).unwrap(),
                    b'n' => *BASIC_LOWERCASE_RNA_U8.choose(&mut rng).unwrap(),
                    _ => ch,
                })
                .collect::<Vec<u8>>()
        } else {
            self.into_iter()
                .map(|ch| match ch {
                    b'N' => *BASIC_DNA_U8.choose(&mut rng).unwrap(),
                    b'n' => *BASIC_LOWERCASE_DNA_U8.choose(&mut rng).unwrap(),
                    _ => ch,
                })
                .collect::<Vec<u8>>()
        }
    }

    /// Fill gaps {.,-} with pseudorandom nucleotides ACUG if xna is "RNA" or ACTG for all other xna.
    fn replace_gap(&self, xna: &str, mut rng: ThreadRng) -> Vec<u8> {
        if xna == "RNA" {
            self.into_iter()
                .map(|ch| match ch {
                    b'.' => *BASIC_RNA_U8.choose(&mut rng).unwrap(),
                    b'-' => *BASIC_RNA_U8.choose(&mut rng).unwrap(),
                    _ => ch,
                })
                .collect::<Vec<u8>>()
        } else {
            self.into_iter()
                .map(|ch| match ch {
                    b'.' => *BASIC_DNA_U8.choose(&mut rng).unwrap(),
                    b'-' => *BASIC_DNA_U8.choose(&mut rng).unwrap(),
                    _ => ch,
                })
                .collect::<Vec<u8>>()
        }
    }

    /// Replace all other than ACGTUactgu with pseudorandom nucleotides ACTGU. Should be used last after other functions or for cleanup of unknown characters.
    fn replace_non_basic(&self, xna: &str, mut rng: ThreadRng) -> Vec<u8> {
        if xna == "RNA" {
            self.into_iter()
                .map(|ch| match ch {
                    b'A' => ch,
                    b'a' => ch,
                    b'C' => ch,
                    b'c' => ch,
                    b'T' => ch,
                    b't' => ch,
                    b'G' => ch,
                    b'g' => ch,
                    b'U' => ch,
                    b'u' => ch,
                    _ => *BASIC_RNA_U8.choose(&mut rng).unwrap(),
                })
                .collect::<Vec<u8>>()
        } else {
            self.into_iter()
                .map(|ch| match ch {
                    b'A' => ch,
                    b'a' => ch,
                    b'C' => ch,
                    b'c' => ch,
                    b'T' => ch,
                    b't' => ch,
                    b'G' => ch,
                    b'g' => ch,
                    b'U' => ch,
                    b'u' => ch,
                    _ => *BASIC_DNA_U8.choose(&mut rng).unwrap(),
                })
                .collect::<Vec<u8>>()
        }
    }

    /// Specifically make actgu into ACTGU
    fn to_upper_basic(&self) -> Vec<u8> {
        self.into_iter()
            .map(|ch| match ch {
                b'A' => ch,
                b'a' => b'A',
                b'C' => ch,
                b'c' => b'C',
                b'T' => ch,
                b't' => b'T',
                b'G' => ch,
                b'g' => b'G',
                b'U' => ch,
                b'u' => b'U',
                _ => ch,
            })
            .collect::<Vec<u8>>()
    }

    /// Specifically make ACTGU into actgu
    fn to_lower_basic(&self) -> Vec<u8> {
        self.into_iter()
            .map(|ch| match ch {
                b'A' => ch,
                b'a' => b'A',
                b'C' => ch,
                b'c' => b'C',
                b'T' => ch,
                b't' => b'T',
                b'G' => ch,
                b'g' => b'G',
                b'U' => ch,
                b'u' => b'U',
                _ => ch,
            })
            .collect::<Vec<u8>>()
    }

    /// Pseudorandom nucleotide replacements within IUPAC specifications, e.g. R: either A or G. Case specific, r: either a or g.
    fn replace_iupac(&self, xna: &str, mut rng: ThreadRng) -> Vec<u8> {
        if xna == "RNA" {
            self.into_iter()
                .map(|ch| match ch {
                    b'R' => *R_BASES.choose(&mut rng).unwrap(),
                    b'r' => *R_BASES_LOWERCASE.choose(&mut rng).unwrap(),
                    b'Y' => *Y_BASES_RNA.choose(&mut rng).unwrap(),
                    b'y' => *Y_BASES_LOWERCASE_RNA.choose(&mut rng).unwrap(),
                    b'S' => *S_BASES.choose(&mut rng).unwrap(),
                    b's' => *S_BASES_LOWERCASE.choose(&mut rng).unwrap(),
                    b'W' => *W_BASES_RNA.choose(&mut rng).unwrap(),
                    b'w' => *W_BASES_LOWERCASE_RNA.choose(&mut rng).unwrap(),
                    b'K' => *K_BASES_RNA.choose(&mut rng).unwrap(),
                    b'k' => *K_BASES_LOWERCASE_RNA.choose(&mut rng).unwrap(),
                    b'M' => *M_BASES.choose(&mut rng).unwrap(),
                    b'm' => *M_BASES_LOWERCASE.choose(&mut rng).unwrap(),
                    b'B' => *B_BASES_RNA.choose(&mut rng).unwrap(),
                    b'b' => *B_BASES_LOWERCASE_RNA.choose(&mut rng).unwrap(),
                    b'D' => *D_BASES_RNA.choose(&mut rng).unwrap(),
                    b'd' => *D_BASES_LOWERCASE_RNA.choose(&mut rng).unwrap(),
                    b'H' => *H_BASES_RNA.choose(&mut rng).unwrap(),
                    b'h' => *H_BASES_LOWERCASE_RNA.choose(&mut rng).unwrap(),
                    b'V' => *V_BASES.choose(&mut rng).unwrap(),
                    b'v' => *V_BASES_LOWERCASE.choose(&mut rng).unwrap(),
                    _ => ch,
                })
                .collect::<Vec<u8>>()
        } else {
            self.into_iter()
                .map(|ch| match ch {
                    b'R' => *R_BASES.choose(&mut rng).unwrap(),
                    b'r' => *R_BASES_LOWERCASE.choose(&mut rng).unwrap(),
                    b'Y' => *Y_BASES.choose(&mut rng).unwrap(),
                    b'y' => *Y_BASES_LOWERCASE.choose(&mut rng).unwrap(),
                    b'S' => *S_BASES.choose(&mut rng).unwrap(),
                    b's' => *S_BASES_LOWERCASE.choose(&mut rng).unwrap(),
                    b'W' => *W_BASES.choose(&mut rng).unwrap(),
                    b'w' => *W_BASES_LOWERCASE.choose(&mut rng).unwrap(),
                    b'K' => *K_BASES.choose(&mut rng).unwrap(),
                    b'k' => *K_BASES_LOWERCASE.choose(&mut rng).unwrap(),
                    b'M' => *M_BASES.choose(&mut rng).unwrap(),
                    b'm' => *M_BASES_LOWERCASE.choose(&mut rng).unwrap(),
                    b'B' => *B_BASES.choose(&mut rng).unwrap(),
                    b'b' => *B_BASES_LOWERCASE.choose(&mut rng).unwrap(),
                    b'D' => *D_BASES.choose(&mut rng).unwrap(),
                    b'd' => *D_BASES_LOWERCASE.choose(&mut rng).unwrap(),
                    b'H' => *H_BASES.choose(&mut rng).unwrap(),
                    b'h' => *H_BASES_LOWERCASE.choose(&mut rng).unwrap(),
                    b'V' => *V_BASES.choose(&mut rng).unwrap(),
                    b'v' => *V_BASES_LOWERCASE.choose(&mut rng).unwrap(),
                    _ => ch,
                })
                .collect::<Vec<u8>>()
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::utils::replace::ReplaceNucleotide;
//     #[test]
//     fn test_replace_u_with_t() {
//     let mut rng = rand::thread_rng();
//     let mut test = b"ACTG";
//     test.replace_u_with_t();
//     assert_eq!(test, b"ACUG");
// }
// fn test_replace_x_with_y() {
//     let mut rng = rand::thread_rng();
//     let mut test = b"ACTG";
//     test.replace_x_with_y(b"T", b"U");
//     assert_eq!(test, b"ACUG");
// }
// }

// #[test]
// fn test_replace_gap() {
//     let mut rng = rand::thread_rng();
//     let test = "ACTGuyUNn.-@<^>".to_string().replace_gap(rng);
//     assert_eq!(test.len(), "ACTGuyUNn.-@<^>".to_string().len());
// }

// #[test]
// fn test_replace_gap_lowercase() {
//     let mut rng = rand::thread_rng();
//     let test = "ACTGuyUNn.-@<^>".to_string().replace_gap_lowercase(rng);
//     assert_eq!(test.len(), "ACTGuyUNn.-@<^>".to_string().len());
// }

// #[test]
// fn test_replace_n() {
//     let mut rng = rand::thread_rng();
//     let test = "ACTGuyUNn.-@<^>".to_string().replace_n(rng);
//     assert_eq!(test.len(), "ACTGuyUNn.-@<^>".to_string().len());
// }

// #[test]
// fn test_replace_iupac() {
//     let mut rng = rand::thread_rng();
//     let test = "ACTGuyUNn.-@<^>".to_string().replace_iupac(rng);
//     assert_eq!(test.len(), "ACTGuyUNn.-@<^>".to_string().len());
// }

// #[test]
// fn test_replace_all_other_with_uppercase() {
//     let mut rng = rand::thread_rng();
//     let test = "ACTGuyUNn.-@<^>".to_string().replace_all_other_with_uppercase(rng);
//     assert_eq!(test.len(), "ACTGuyUNn.-@<^>".to_string().len());
// }

// #[test]
// fn test_rreplace_all_other_with_lowercase() {
//     let mut rng = rand::thread_rng();
//     let test = "ACTGuyUNn.-@<^>".to_string().replace_all_other_with_lowercase(rng);
//     assert_eq!(test.len(), "ACTGuyUNn.-@<^>".to_string().len());
// }

// // fn replace_grapheme_at(s: &str, c: &str) -> String {
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

// impl<T> ReplaceNucleotide<T> for T where T: AsMut<[u8]> {
// // impl ReplaceNucleotide<String> for String {
//     /// Specifically replace Uu with Tt, lowercase to lowercase, uppercase to uppercase. Leaves all other characters in place.
//     fn replace_u_with_t(&mut self) {
//         self.as_mut().iter_mut().for_each(|c| match c {
//             b'U' => *c = b'T',
//             b'u' => *c = b'T',
//             _ => {}
//         })
//     }

//     fn replace_x_with_y(&mut self, rx: u8, ry: u8) {
//         self.as_mut().iter_mut().for_each(|c| match rx {
//             rx => *c = ry,
//             _ => {}
//         })
// //     }
