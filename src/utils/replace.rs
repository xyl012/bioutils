// Copyright 2020 Christopher Sugai

//! Trait to remove or replace characters (u<->t,Nn->{ACTG}, etc.) with pseudorandom bases.
//! # Examples
//! ```
//! 
//! extern crate rand;
//! 
//! use rand::rngs::ThreadRng;
//! use std::string::String;
//! use rand::seq::SliceRandom;
//! 
//! let mut rng = rand::thread_rng(); //create a random number generator
//! let x1 = "ACTGuyUNn.-@<^>".to_string(); //string
//! let mut simple = "ACTG".to_string(); //string
//! let mut simpleedit = simple.replace_u_with_t(); //string
//! 
//! assert_eq!("ACUG", simpleedit);
//!
//! let y1 = x1.replace_gap(rng); //replace gap {.-} using the random number generator
//!
//! let x2 = "ACTGuyUNn.-@<^>"; //stringslice
//! let y2 = x2.to_string(); //convert stringslice to String
//! let y2 = y2.replace_n(rng); //replace ambiguous/uncalled base {nN}
//!
//! let y3 = std::str::from_utf8(x3).expect("Not valid UTF8").to_string(); //convert textslice to either Text or String but must be owned
//! let y3 = y3; //replace all other text other than ACTGUactgu, does not take rng
//! 
//! let y4 = x4.replace_all(rng); //replace u with t
//! let y4 = std::string::String::from_utf8(y4).expect("Not valid UTF8");
//!
//! assert_eq!(y4.len(), "ACTGuyUNn.-@<^>".to_string().len());
//!
//!    // println!("{}", y1);
//!    // println!("{}", y2);
//!    // println!("{}", y3);
//!    // println!("{}", y4);
//! ```

use rand::rngs::ThreadRng;
use std::string::String;
use rand::seq::SliceRandom;
use core::iter::FromIterator;

use crate::charsets::iupac::*;
use crate::charsets::quality::*;
use crate::charsets::ascii::*;

trait ReplaceNucleotide <T> {
    fn to_upper_basic(&self) -> Vec<u8>;
    fn to_lower_basic(&self) -> Vec<u8>;
    fn replace_gap(&self, xna: &str, rng: ThreadRng) -> Vec<u8>;
    fn replace_n(&self, xna: &str, rng: ThreadRng) -> Vec<u8>;

    // fn replace_gap_lowercase(&self, rng: ThreadRng) -> Self;
    fn replace_iupac(&self, rng: ThreadRng) -> Vec<u8>;
    // fn replace_lowercase_with_uppercase(&self, rng: ThreadRng) -> Self;
    fn replace_non_basic_with_uppercase(&self, xna: &str, rng: ThreadRng) -> Vec<u8>;
    // fn replace_all_other_with_lowercase(&self, rng: ThreadRng) -> Self;
}

impl<T> ReplaceNucleotide<T> for T where for<'a> &'a T: IntoIterator<Item = &'a u8> {

    /// Fill {N,n} with pseudorandom nucleotides ACUG if xna is "RNA" or ACTG for all other xna.
    fn replace_n(&self, xna: &str, mut rng: ThreadRng) -> Vec<u8> {
        if xna == "RNA" {
        self.into_iter()
            .map(|&ch| match ch {
                b'N' => *BASIC_RNA_U8.choose(&mut rng).unwrap(),
                b'n' => *BASIC_LOWERCASE_RNA_U8.choose(&mut rng).unwrap(),
                _ => ch,
            })
            .collect::<Vec<u8>>()}
        else {
            self.into_iter()
            .map(|&ch| match ch {
                b'N' => *BASIC_DNA_U8.choose(&mut rng).unwrap(),
                b'n' => *BASIC_LOWERCASE_DNA_U8.choose(&mut rng).unwrap(),
                _ => ch,
            })
            .collect::<Vec<u8>>()}
    }

    /// Fill gaps {.,-} with pseudorandom nucleotides ACUG if xna is "RNA" or ACTG for all other xna.
    fn replace_gap(&self, xna: &str, mut rng: ThreadRng) -> Vec<u8> {
        if xna == "RNA" {
            self.into_iter()
            .map(|&ch| match ch {
                b'.' => *BASIC_RNA_U8.choose(&mut rng).unwrap(),
                b'-' => *BASIC_RNA_U8.choose(&mut rng).unwrap(),
                _ => ch,
            }).collect::<Vec<u8>>()
        } else {
            self.into_iter()
            .map(|&ch| match ch {
                b'.' => *BASIC_DNA_U8.choose(&mut rng).unwrap(),
                b'-' => *BASIC_DNA_U8.choose(&mut rng).unwrap(),
                _ => ch,
            }).collect::<Vec<u8>>()
        }
    }

    /// Specifically make actgu into ACTGU
    fn to_upper_basic(&self) -> Vec<u8> {
        self.into_iter()
            .map(|&ch| match ch {
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
            .map(|&ch| match ch {
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

    /// Replace all other than ACGTUactgu with pseudorandom nucleotides ACTGU. Should be used last after other functions or for cleanup of unknown characters.
    fn replace_all_other_with_uppercase(&self, xna = &str, mut rng: ThreadRng) -> Vec<u8> {
        self.into_iter()
            .map(|&ch| match ch {
                'A' => ch,
                'a' => ch,
                'C' => ch,
                'c' => ch,
                'T' => ch,
                't' => ch,
                'G' => ch,
                'g' => ch,
                'U' => ch,
                'u' => ch,
                _ => *bases.choose(&mut rng).unwrap(),
            })
            .collect::<Vec<u8>>()
    }

    /// Pseudorandom nucleotide replacements within IUPAC specifications, e.g. R: either A or G. Case specific, r: either a or g.
    fn replace_iupac(&self, xna: &str, mut rng: ThreadRng) -> Vec<u8> {
        if xna == "RNA" {
            self.into_iter()
                .map(|ch| match ch {
                    'R' => *r_bases.choose(&mut rng).unwrap(),
                    'r' => *r_bases_lowercase.choose(&mut rng).unwrap(),
                    'Y' => *y_bases.choose(&mut rng).unwrap(),
                    'y' => *y_bases_lowercase.choose(&mut rng).unwrap(),
                    'S' => *s_bases.choose(&mut rng).unwrap(),
                    's' => *s_bases_lowercase.choose(&mut rng).unwrap(),
                    'W' => *w_bases.choose(&mut rng).unwrap(),
                    'w' => *w_bases_lowercase.choose(&mut rng).unwrap(),
                    'K' => *k_bases.choose(&mut rng).unwrap(),
                    'k' => *k_bases_lowercase.choose(&mut rng).unwrap(),
                    'M' => *m_bases.choose(&mut rng).unwrap(),
                    'm' => *m_bases_lowercase.choose(&mut rng).unwrap(),
                    'B' => *b_bases.choose(&mut rng).unwrap(),
                    'b' => *b_bases_lowercase.choose(&mut rng).unwrap(),
                    'D' => *d_bases.choose(&mut rng).unwrap(),
                    'd' => *d_bases_lowercase.choose(&mut rng).unwrap(),
                    'H' => *h_bases.choose(&mut rng).unwrap(),
                    'h' => *h_bases_lowercase.choose(&mut rng).unwrap(),
                    'V' => *v_bases.choose(&mut rng).unwrap(),
                    'v' => *v_bases_lowercase.choose(&mut rng).unwrap(),
                    _ => ch,
                })
                .collect::<Vec<u8>>()}
        } else {
            self.into_iter()
                .map(|ch| match ch {
                    'R' => *r_bases.choose(&mut rng).unwrap(),
                    'r' => *r_bases_lowercase.choose(&mut rng).unwrap(),
                    'Y' => *y_bases.choose(&mut rng).unwrap(),
                    'y' => *y_bases_lowercase.choose(&mut rng).unwrap(),
                    'S' => *s_bases.choose(&mut rng).unwrap(),
                    's' => *s_bases_lowercase.choose(&mut rng).unwrap(),
                    'W' => *w_bases.choose(&mut rng).unwrap(),
                    'w' => *w_bases_lowercase.choose(&mut rng).unwrap(),
                    'K' => *k_bases.choose(&mut rng).unwrap(),
                    'k' => *k_bases_lowercase.choose(&mut rng).unwrap(),
                    'M' => *m_bases.choose(&mut rng).unwrap(),
                    'm' => *m_bases_lowercase.choose(&mut rng).unwrap(),
                    'B' => *b_bases.choose(&mut rng).unwrap(),
                    'b' => *b_bases_lowercase.choose(&mut rng).unwrap(),
                    'D' => *d_bases.choose(&mut rng).unwrap(),
                    'd' => *d_bases_lowercase.choose(&mut rng).unwrap(),
                    'H' => *h_bases.choose(&mut rng).unwrap(),
                    'h' => *h_bases_lowercase.choose(&mut rng).unwrap(),
                    'V' => *v_bases.choose(&mut rng).unwrap(),
                    'v' => *v_bases_lowercase.choose(&mut rng).unwrap(),
                    _ => ch,
                })
                .collect::<Vec<u8>>()}
    }
}

const r_bases = ['A', 'G'];
const r_bases_lowercase = ['a', 'g'];
const y_bases = ['C', 'U'];
const y_bases_lowercase = ['c', 'u'];
const s_bases = ['C', 'G'];
const s_bases_lowercase = ['c', 'g'];
const w_bases = ['A', 'U'];
const w_bases_lowercase = ['a', 'u'];
const k_bases = ['U', 'G'];
const k_bases_lowercase = ['u', 'g'];
const m_bases = ['A', 'C'];
const m_bases_lowercase = ['a', 'c'];
const b_bases = ['C', 'U', 'G'];
const b_bases_lowercase = ['c', 'u', 'g'];
const d_bases = ['A', 'U', 'G'];
const d_bases_lowercase = ['a', 'u', 'g'];
const h_bases = ['A', 'C', 'U'];
const h_bases_lowercase = ['a', 'c', 'u'];
const v_bases = ['A', 'C', 'G'];
const v_bases_lowercase = ['a', 'c', 'g'];

const r_bases = ['A', 'G'];
const r_bases_lowercase = ['a', 'g'];
const y_bases = ['C', 'T'];
const y_bases_lowercase = ['c', 't'];
const s_bases = ['C', 'G'];
const s_bases_lowercase = ['c', 'g'];
const w_bases = ['A', 'T'];
const w_bases_lowercase = ['a', 't'];
const k_bases = ['T', 'G'];
const k_bases_lowercase = ['t', 'g'];
const m_bases = ['A', 'C'];
const m_bases_lowercase = ['a', 'c'];
const b_bases = ['C', 'T', 'G'];
const b_bases_lowercase = ['c', 't', 'g'];
const d_bases = ['A', 'T', 'G'];
const d_bases_lowercase = ['a', 't', 'g'];
const h_bases = ['A', 'C', 'T'];
const h_bases_lowercase = ['a', 'c', 't'];
const v_bases = ['A', 'C', 'G'];
const v_bases_lowercase = ['a', 'c', 'g'];

    // /// Fill gaps {.,-} with pseudorandom nucleotides actg
    // fn replace_gap_lowercase(&self, mut rng: ThreadRng) -> Self {
    //     let bases = ['a', 'c', 't', 'g'];
    //     self.chars()
    //         .map(|ch| match ch {
    //             b'.' => *bases.choose(&mut rng).unwrap(),
    //             b'-' => *bases.choose(&mut rng).unwrap(),
    //             _ => ch,
    //         })
    //         .collect()
    // }





    // /// fill all other than ACGTUactgu with pseudorandom nucleotides actgu. Should be used last after other functions or for cleanup of unknown characters.
    // fn replace_all_other_with_lowercase(&self, mut rng: ThreadRng) -> Self {
    //     let bases = ['a', 'c', 't', 'g'];
    //     self.chars()
    //         .map(|ch| match ch {
    //             'A' => ch,
    //             'a' => ch,
    //             'C' => ch,
    //             'c' => ch,
    //             'T' => ch,
    //             't' => ch,
    //             'G' => ch,
    //             'g' => ch,
    //             'U' => ch,
    //             'u' => ch,
    //             _ => *bases.choose(&mut rng).unwrap(),
    //         })
    //         .collect()
    // }







// impl ReplaceNucleotide<Text> for Text {
//     /// Specifically replace Uu with Tt, lowercase to lowercase, uppercase to uppercase. Leaves all other characters in place.
//     fn replace_u_with_t(&self) -> Self {
//         std::string::String::from_utf8(self.to_owned()).expect("Not valid UTF8").chars()
//             .map(|ch| match ch {
//                 'U' => 'T',
//                 'u' => 't',
//                 _ => ch,
//             })
//             .collect::<String>().into_bytes()
//     }

//     /// Specifically fill lowercase actgu with ACTGU but leaves all other characters in place.
//     fn replace_lowercase_with_uppercase(&self) -> Self {
//         std::string::String::from_utf8(self.to_owned()).expect("Not valid UTF8").chars()
//             .map(|ch| match ch {
//                 'A' => ch,
//                 'a' => 'A',
//                 'C' => ch,
//                 'c' => 'C',
//                 'T' => ch,
//                 't' => 'T',
//                 'G' => ch,
//                 'g' => 'G',
//                 'U' => ch,
//                 'u' => 'U',
//                 _ => ch,
//             }
//     }

//     /// Fill gaps {.,-} with pseudorandom nucleotides ACTG
//     fn replace_gap(&self, mut rng: ThreadRng) -> Self {
//         let bases = ['A', 'C', 'T', 'G'];
//         std::string::String::from_utf8(self.to_owned()).expect("Not valid UTF8").chars()
//             .map(|ch| match ch {
//                 '.' => *bases.choose(&mut rng).unwrap(),
//                 '-' => *bases.choose(&mut rng).unwrap(),
//                 _ => ch,
//             })
//             .collect::<String>().into_bytes()
//     }
//     /// Fill gaps {.,-} with pseudorandom nucleotides actg
//     fn replace_gap_lowercase(&self, mut rng: ThreadRng) -> Self {
//         let bases = ['a', 'c', 't', 'g'];
//         std::string::String::from_utf8(self.to_owned()).expect("Not valid UTF8").chars()
//             .map(|ch| match ch {
//                 '.' => *bases.choose(&mut rng).unwrap(),
//                 '-' => *bases.choose(&mut rng).unwrap(),
//                 _ => ch,
//             })
//             .collect::<String>().into_bytes()
//     }
//     /// Fill N with pseudorandom nucleotides ACTG and n with actg
//     fn replace_n(&self, mut rng: ThreadRng) -> Self {
//         let bases = ['A', 'C', 'T', 'G'];
//         let lowercase_bases = ['a', 'c', 't', 'g'];
//         std::string::String::from_utf8(self.to_owned()).expect("Not valid UTF8").chars()
//             .map(|ch| match ch {
//                 'N' => *bases.choose(&mut rng).unwrap(),
//                 'n' => *lowercase_bases.choose(&mut rng).unwrap(),
//                 _ => ch,
//             })
//             .collect::<String>().into_bytes()
//     }
//     /// Pseudorandom nucleotide replacements within IUPAC specifications, e.g. R: either A or G. Case specific, r: either a or g.    
//     fn replace_iupac(&self, mut rng: ThreadRng) -> Self {
//         let r_bases = ['A', 'G'];
//         let r_bases_lowercase = ['a', 'g'];
//         let y_bases = ['C', 'T'];
//         let y_bases_lowercase = ['c', 't'];
//         let s_bases = ['C', 'G'];
//         let s_bases_lowercase = ['c', 'g'];
//         let w_bases = ['A', 'T'];
//         let w_bases_lowercase = ['a', 't'];
//         let k_bases = ['T', 'G'];
//         let k_bases_lowercase = ['t', 'g'];
//         let m_bases = ['A', 'C'];
//         let m_bases_lowercase = ['a', 'c'];
//         let b_bases = ['C', 'T', 'G'];
//         let b_bases_lowercase = ['c', 't', 'g'];
//         let d_bases = ['A', 'T', 'G'];
//         let d_bases_lowercase = ['a', 't', 'g'];
//         let h_bases = ['A', 'C', 'T'];
//         let h_bases_lowercase = ['a', 'c', 't'];
//         let v_bases = ['A', 'C', 'G'];
//         let v_bases_lowercase = ['a', 'c', 'g'];
//         std::string::String::from_utf8(self.to_owned()).expect("Not valid UTF8").chars()
//             .map(|ch| match ch {
//                 'R' => *r_bases.choose(&mut rng).unwrap(),
//                 'r' => *r_bases_lowercase.choose(&mut rng).unwrap(),
//                 'Y' => *y_bases.choose(&mut rng).unwrap(),
//                 'y' => *y_bases_lowercase.choose(&mut rng).unwrap(),
//                 'S' => *s_bases.choose(&mut rng).unwrap(),
//                 's' => *s_bases_lowercase.choose(&mut rng).unwrap(),
//                 'W' => *w_bases.choose(&mut rng).unwrap(),
//                 'w' => *w_bases_lowercase.choose(&mut rng).unwrap(),
//                 'K' => *k_bases.choose(&mut rng).unwrap(),
//                 'k' => *k_bases_lowercase.choose(&mut rng).unwrap(),
//                 'M' => *m_bases.choose(&mut rng).unwrap(),
//                 'm' => *m_bases_lowercase.choose(&mut rng).unwrap(),
//                 'B' => *b_bases.choose(&mut rng).unwrap(),
//                 'b' => *b_bases_lowercase.choose(&mut rng).unwrap(),
//                 'D' => *d_bases.choose(&mut rng).unwrap(),
//                 'd' => *d_bases_lowercase.choose(&mut rng).unwrap(),
//                 'H' => *h_bases.choose(&mut rng).unwrap(),
//                 'h' => *h_bases_lowercase.choose(&mut rng).unwrap(),
//                 'V' => *v_bases.choose(&mut rng).unwrap(),
//                 'v' => *v_bases_lowercase.choose(&mut rng).unwrap(),
//                 _ => ch,
//             })
//             .collect::<String>().into_bytes()
//     }
//     /// fill all other than ACGTUactgu with pseudorandom nucleotides ACTGU. Should be used last after other functions or for cleanup of unknown characters.
//     fn replace_all_other_with_uppercase(&self, mut rng: ThreadRng) -> Self {
//         let bases = ['A', 'C', 'T', 'G'];
//         std::string::String::from_utf8(self.to_owned()).expect("Not valid UTF8").chars()
//             .map(|ch| match ch {
//                 'A' => ch,
//                 'a' => ch,
//                 'C' => ch,
//                 'c' => ch,
//                 'T' => ch,
//                 't' => ch,
//                 'G' => ch,
//                 'g' => ch,
//                 'U' => ch,
//                 'u' => ch,
//                 _ => *bases.choose(&mut rng).unwrap(),
//             })
//             .collect::<String>().into_bytes()
//     }
//     /// fill all other than ACGTUactgu with pseudorandom nucleotides actgu. Should be used last after other functions or for cleanup of unknown characters.
//     fn replace_all_other_with_lowercase(&self, mut rng: ThreadRng) -> Self {
//         let bases = ['a', 'c', 't', 'g'];
//         std::string::String::from_utf8(self.to_owned()).expect("Not valid UTF8").chars()
//             .map(|ch| match ch {
//                 'A' => ch,
//                 'a' => ch,
//                 'C' => ch,
//                 'c' => ch,
//                 'T' => ch,
//                 't' => ch,
//                 'G' => ch,
//                 'g' => ch,
//                 'U' => ch,
//                 'u' => ch,
//                 _ => *bases.choose(&mut rng).unwrap(),
//             })
//             .collect::<String>().into_bytes()
//     }
// }




#[cfg(test)]
mod tests {
    use crate::utils::replace::ReplaceNucleotide;
    #[test]
    fn test_replace_u_with_t() {
    let mut rng = rand::thread_rng();
    let mut test = *b"ACTG";
    test.replace_u_with_t();
    assert_eq!(test, b"ACUG");
}
fn test_replace_x_with_y() {
    let mut rng = rand::thread_rng();
    let mut test = *b"ACTG";
    test.replace_x_with_y(b"T", b"U");
    assert_eq!(test, b"ACUG");
}
}

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
//     }
// }