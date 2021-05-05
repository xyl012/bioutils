// Copyright (c) 2021 Kana LLC

//! IUPAC character (sub-)sets including basic and full sets. Provided as u8 and str arrays.

use super::*;

// Full IUPAC charset including nucleotides and amino acids
pub const IUPAC_U8: [u8; 46] = [
    b'A', b'a', b'C', b'c', b'G', b'g', b'T', b't', b'U', b'u', b'R', b'r', b'Y', b'y', b'S', b's',
    b'W', b'w', b'K', b'k', b'M', b'm', b'B', b'b', b'D', b'd', b'H', b'h', b'V', b'v', b'N', b'n',
    b'-', b'.', b'F', b'f', b'G', b'g', b'I', b'i', b'L', b'l', b'P', b'p', b'Q', b'q',
];

pub const IUPAC_STR: [&str; 46] = [
    "A", "a", "C", "c", "G", "g", "T", "t", "U", "u", "R", "r", "Y", "y", "S", "s", "W", "w", "K",
    "k", "M", "m", "B", "b", "D", "d", "H", "h", "V", "v", "N", "n", r#"-"#, r#"."#, "F", "f", "G",
    "g", "I", "i", "L", "l", "P", "p", "Q", "q",
];
lazy_static! {
    pub static ref IUPAC_HASHSET_U8: HashSet<u8> = new_u8_hashset(&IUPAC_U8);
}
lazy_static! {
    pub static ref IUPAC_HASHSET_STR: HashSet<&'static str> = new_str_hashset(&IUPAC_STR);
}

// Full IUPAC nucleotide charset including N,n,-,.
pub const IUPAC_NUCLEOTIDE_U8: [u8; 34] = [
    b'A', b'a', b'C', b'c', b'G', b'g', b'T', b't', b'U', b'u', b'R', b'r', b'Y', b'y', b'S', b's',
    b'W', b'w', b'K', b'k', b'M', b'm', b'B', b'b', b'D', b'd', b'H', b'h', b'V', b'v', b'N', b'n',
    b'-', b'.',
];
pub const IUPAC_NUCLEOTIDE_STR: [&str; 34] = [
    "A", "a", "C", "c", "G", "g", "T", "t", "U", "u", "R", "r", "Y", "y", "S", "s", "W", "w", "K",
    "k", "M", "m", "B", "b", "D", "d", "H", "h", "V", "v", "N", "n", r#"-"#, r#"."#,
];
lazy_static! {
    pub static ref IUPAC_NUCLEOTIDE_HASHSET_U8: HashSet<u8> = new_u8_hashset(&IUPAC_NUCLEOTIDE_U8);
}
lazy_static! {
    pub static ref IUPAC_NUCLEOTIDE_HASHSET_STR: HashSet<&'static str> =
        new_str_hashset(&IUPAC_NUCLEOTIDE_STR);
}

// Full IUPAC amino acid charset
pub const IUPAC_AMINO_ACID_U8: [u8; 42] = [
    b'A', b'a', b'C', b'c', b'D', b'd', b'E', b'e', b'F', b'f', b'G', b'g', b'H', b'h', b'I', b'i',
    b'K', b'k', b'L', b'l', b'M', b'm', b'N', b'n', b'P', b'p', b'Q', b'q', b'R', b'r', b'S', b's',
    b'T', b't', b'U', b'u', b'V', b'v', b'W', b'w', b'Y', b'y',
];
pub const IUPAC_AMINO_ACID_STR: [&str; 42] = [
    "A", "a", "C", "c", "D", "d", "E", "e", "F", "f", "G", "g", "H", "h", "I", "i", "K", "k", "L",
    "l", "M", "m", "N", "n", "P", "p", "Q", "q", "R", "r", "S", "s", "T", "t", "U", "u", "V", "v",
    "W", "w", "Y", "y",
];
lazy_static! {
    pub static ref IUPAC_AMINO_ACID_HASHSET_U8: HashSet<u8> = new_u8_hashset(&IUPAC_AMINO_ACID_U8);
}
lazy_static! {
    pub static ref IUPAC_AMINO_ACID_HASHSET_STR: HashSet<&'static str> =
        new_str_hashset(&IUPAC_AMINO_ACID_STR);
}

// Basic DNA charset
pub const BASIC_DNA_U8: [u8; 4] = [b'A', b'C', b'G', b'T'];
pub const BASIC_DNA_STR: [&str; 4] = ["A", "C", "G", "T"];
lazy_static! {
    pub static ref BASIC_DNA_HASHSET_U8: HashSet<u8> = new_u8_hashset(&BASIC_DNA_U8);
}
lazy_static! {
    pub static ref BASIC_DNA_HASHSET_STR: HashSet<&'static str> = new_str_hashset(&BASIC_DNA_STR);
}

pub const BASIC_LOWERCASE_DNA_U8: [u8; 4] = [b'a', b'c', b'g', b't'];
pub const BASIC_LOWERCASE_DNA_STR: [&str; 4] = ["a", "c", "g", "t"];
lazy_static! {
    pub static ref BASIC_LOWERCASE_DNA_HASHSET_U8: HashSet<u8> =
        new_u8_hashset(&BASIC_LOWERCASE_DNA_U8);
}
lazy_static! {
    pub static ref BASIC_LOWERCASE_DNA_HASHSET_STR: HashSet<&'static str> =
        new_str_hashset(&BASIC_LOWERCASE_DNA_STR);
}

// Basic RNA charset
pub const BASIC_RNA_U8: [u8; 4] = [b'A', b'C', b'G', b'U'];
pub const BASIC_RNA_STR: [&str; 4] = ["A", "C", "G", "U"];
lazy_static! {
    pub static ref BASIC_RNA_HASHSET_U8: HashSet<u8> = new_u8_hashset(&BASIC_RNA_U8);
}
lazy_static! {
    pub static ref BASIC_RNA_HASHSET_STR: HashSet<&'static str> = new_str_hashset(&BASIC_RNA_STR);
}

pub const BASIC_LOWERCASE_RNA_U8: [u8; 4] = [b'a', b'c', b'g', b'u'];
pub const BASIC_LOWERCASE_RNA_STR: [&str; 4] = ["a", "c", "g", "u"];
lazy_static! {
    pub static ref BASIC_LOWERCASE_RNA_HASHSET_U8: HashSet<u8> =
        new_u8_hashset(&BASIC_LOWERCASE_RNA_U8);
}
lazy_static! {
    pub static ref BASIC_LOWERCASE_RNA_HASHSET_STR: HashSet<&'static str> =
        new_str_hashset(&BASIC_LOWERCASE_RNA_STR);
}

// Basic AA charset
pub const BASIC_AMINO_ACID_U8: [u8; 21] = [
    b'A', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'K', b'L', b'M', b'N', b'P', b'Q', b'R', b'S',
    b'T', b'U', b'V', b'W', b'Y',
];
pub const BASIC_AMINO_ACID_STR: [&str; 21] = [
    "A", "C", "D", "E", "F", "G", "H", "I", "K", "L", "M", "N", "P", "Q", "R", "S", "T", "U", "V",
    "W", "Y",
];
lazy_static! {
    pub static ref BASIC_AMINO_ACID_HASHSET_U8: HashSet<u8> = new_u8_hashset(&BASIC_AMINO_ACID_U8);
}
lazy_static! {
    pub static ref BASIC_AMINO_ACID_HASHSET_STR: HashSet<&'static str> =
        new_str_hashset(&BASIC_AMINO_ACID_STR);
}

// Only gap charset
pub const GAP_U8: [u8; 2] = [b'.', b'-'];
pub const GAP_STR: [&str; 2] = [".", "-"];

// Only N charset
pub const N_U8: [u8; 2] = [b'N', b'n'];
pub const N_STR: [&str; 2] = ["N", "n"];

// IUPAC equivalent charsets
pub const Y_BASES_RNA: [u8; 2] = [b'C', b'U'];
pub const Y_BASES_LOWERCASE_RNA: [u8; 2] = [b'c', b'u'];
pub const W_BASES_RNA: [u8; 2] = [b'A', b'U'];
pub const W_BASES_LOWERCASE_RNA: [u8; 2] = [b'a', b'u'];
pub const K_BASES_RNA: [u8; 2] = [b'U', b'G'];
pub const K_BASES_LOWERCASE_RNA: [u8; 2] = [b'u', b'g'];
pub const B_BASES_RNA: [u8; 3] = [b'C', b'U', b'G'];
pub const B_BASES_LOWERCASE_RNA: [u8; 3] = [b'c', b'u', b'g'];
pub const D_BASES_RNA: [u8; 3] = [b'A', b'U', b'G'];
pub const D_BASES_LOWERCASE_RNA: [u8; 3] = [b'a', b'u', b'g'];
pub const H_BASES_RNA: [u8; 3] = [b'A', b'C', b'U'];
pub const H_BASES_LOWERCASE_RNA: [u8; 3] = [b'a', b'c', b'u'];

pub const R_BASES: [u8; 2] = [b'A', b'G'];
pub const R_BASES_LOWERCASE: [u8; 2] = [b'a', b'g'];
pub const Y_BASES: [u8; 2] = [b'C', b'T'];
pub const Y_BASES_LOWERCASE: [u8; 2] = [b'c', b't'];
pub const S_BASES: [u8; 2] = [b'C', b'G'];
pub const S_BASES_LOWERCASE: [u8; 2] = [b'c', b'g'];
pub const W_BASES: [u8; 2] = [b'A', b'T'];
pub const W_BASES_LOWERCASE: [u8; 2] = [b'a', b't'];
pub const K_BASES: [u8; 2] = [b'T', b'G'];
pub const K_BASES_LOWERCASE: [u8; 2] = [b't', b'g'];
pub const M_BASES: [u8; 2] = [b'A', b'C'];
pub const M_BASES_LOWERCASE: [u8; 2] = [b'a', b'c'];
pub const B_BASES: [u8; 3] = [b'C', b'T', b'G'];
pub const B_BASES_LOWERCASE: [u8; 3] = [b'c', b't', b'g'];
pub const D_BASES: [u8; 3] = [b'A', b'T', b'G'];
pub const D_BASES_LOWERCASE: [u8; 3] = [b'a', b't', b'g'];
pub const H_BASES: [u8; 3] = [b'A', b'C', b'T'];
pub const H_BASES_LOWERCASE: [u8; 3] = [b'a', b'c', b't'];
pub const V_BASES: [u8; 3] = [b'A', b'C', b'G'];
pub const V_BASES_LOWERCASE: [u8; 3] = [b'a', b'c', b'g'];

// #[cfg(test)]
// mod tests {
//     use super::{
//         BASIC_AMINO_ACID_U8, BASIC_DNA_U8, BASIC_RNA_U8, IUPAC_AMINO_ACID_U8, IUPAC_NUCLEOTIDE_U8,
//         IUPAC_U8,
//     };
//     #[test]
//     fn test_iupac() {
//         let dec: [u8; 46] = [
//             65, 97, 67, 99, 71, 103, 84, 116, 85, 117, 82, 114, 89, 121, 83, 115, 87, 119, 75, 107,
//             77, 109, 66, 98, 68, 100, 72, 104, 86, 118, 78, 110, 45, 46, 70, 102, 71, 103, 73, 105,
//             76, 108, 80, 112, 81, 113,
//         ];
//         assert_eq!(dec, IUPAC_U8);
//     }
//     #[test]
//     fn test_iupac_nucleotide() {
//         let dec: [u8; 34] = [
//             65, 97, 67, 99, 71, 103, 84, 116, 85, 117, 82, 114, 89, 121, 83, 115, 87, 119, 75, 107,
//             77, 109, 66, 98, 68, 100, 72, 104, 86, 118, 78, 110, 45, 46,
//         ];
//         assert_eq!(dec, IUPAC_NUCLEOTIDE_U8);
//     }
//     #[test]
//     fn test_iupac_amino_acid() {
//         let dec: [u8; 42] = [
//             65, 97, 67, 99, 68, 100, 69, 101, 70, 102, 71, 103, 72, 104, 73, 105, 75, 107, 76, 108,
//             77, 109, 78, 110, 80, 112, 81, 113, 82, 114, 83, 115, 84, 116, 85, 117, 86, 118, 87,
//             119, 89, 121,
//         ];
//         assert_eq!(dec, IUPAC_AMINO_ACID_U8);
//     }
//     #[test]
//     fn test_basic_dna() {
//         let dec: [u8; 4] = [65, 67, 71, 84];
//         assert_eq!(dec, BASIC_DNA_U8);
//     }
//     #[test]
//     fn test_basic_rna() {
//         let dec: [u8; 4] = [65, 67, 71, 85];
//         assert_eq!(dec, BASIC_RNA_U8);
//     }
//     #[test]
//     fn test_basic_amino_acid() {
//         let dec: [u8; 21] = [
//             65, 67, 68, 69, 70, 71, 72, 73, 75, 76, 77, 78, 80, 81, 82, 83, 84, 85, 86, 87, 89,
//         ];
//         assert_eq!(dec, BASIC_AMINO_ACID_U8);
//     }
// }
