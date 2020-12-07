// Copyright 2020 Christopher Sugai
//! IUPAC character (sub-)sets including basic and full sets. Provided as u8 and str arrays.

// Full IUPAC charset including nucleotides and amino acids
pub const IUPAC_U8: [u8; 46] = [b'A', b'a', b'C', b'c', b'G', b'g', b'T', b't', b'U', b'u', b'R', b'r', b'Y', b'y', b'S', b's', b'W', b'w', b'K', b'k', b'M', b'm', b'B', b'b', b'D', b'd', b'H', b'h', b'V', b'v', b'N', b'n', b'-', b'.', b'F', b'f', b'G', b'g', b'I', b'i', b'L', b'l', b'P', b'p', b'Q', b'q'];
pub const IUPAC_STR: [&str; 46] = ["A", "a", "C", "c", "G", "g", "T", "t", "U", "u", "R", "r", "Y", "y", "S", "s", "W", "w", "K", "k", "M", "m", "B", "b", "D", "d", "H", "h", "V", "v", "N", "n", r#"-"#, r#"."#, "F", "f", "G", "g", "I", "i", "L", "l", "P", "p", "Q", "q"];

// Full IUPAC nucleotide charset including N,n,-,.
pub const IUPAC_NUCLEOTIDE_U8: [u8; 34] = [b'A', b'a', b'C', b'c', b'G', b'g', b'T', b't', b'U', b'u', b'R', b'r', b'Y', b'y', b'S', b's', b'W', b'w', b'K', b'k', b'M', b'm', b'B', b'b', b'D', b'd', b'H', b'h', b'V', b'v', b'N', b'n', b'-', b'.'];
pub const IUPAC_NUCLEOTIDE_STR: [&str; 34] = ["A", "a", "C", "c", "G", "g", "T", "t", "U", "u", "R", "r", "Y", "y", "S", "s", "W", "w", "K", "k", "M", "m", "B", "b", "D", "d", "H", "h", "V", "v", "N", "n", r#"-"#, r#"."#];

// Full IUPAC amino acid charset
pub const IUPAC_AMINO_ACID_U8: [u8; 42] = [b'A', b'a', b'C', b'c', b'D', b'd', b'E', b'e', b'F', b'f', b'G', b'g', b'H', b'h', b'I', b'i', b'K', b'k', b'L', b'l', b'M', b'm', b'N', b'n', b'P', b'p', b'Q', b'q', b'R', b'r', b'S', b's', b'T', b't', b'U', b'u', b'V', b'v', b'W', b'w', b'Y', b'y'];
pub const IUPAC_AMINO_ACID_STR: [&str; 42] = ["A", "a", "C", "c", "D", "d", "E", "e", "F", "f", "G", "g", "H", "h", "I", "i", "K", "k", "L", "l", "M", "m", "N", "n", "P", "p", "Q", "q", "R", "r", "S", "s", "T", "t", "U", "u", "V", "v", "W", "w", "Y", "y"];

// Basic DNA charset
pub const BASIC_DNA_U8: [u8; 4] = [b'A', b'C', b'G', b'T'];
pub const BASIC_DNA_STR: [&str; 4] = ["A", "C", "G", "T"];

// Basic RNA charset
pub const BASIC_RNA_U8: [u8; 4] = [b'A', b'C', b'G', b'U'];
pub const BASIC_RNA_STR: [&str; 4] = ["A", "C", "G", "U"];

// Basic AA charset
pub const BASIC_AMINO_ACID_U8: [u8; 21] = [b'A', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'K', b'L', b'M', b'N', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'Y'];
pub const BASIC_AMINO_ACID_STR: [&str; 21] = ["A", "C", "D", "E", "F", "G", "H", "I", "K", "L", "M", "N", "P", "Q", "R", "S", "T", "U", "V", "W", "Y"];

#[cfg(test)]
mod tests {
    use super::{IUPAC_U8, IUPAC_NUCLEOTIDE_U8, IUPAC_AMINO_ACID_U8, BASIC_DNA_U8, BASIC_RNA_U8, BASIC_AMINO_ACID_U8};
    #[test]
    fn test_iupac() {
        let dec: [u8; 46] = [65, 97, 67, 99, 71, 103, 84, 116, 85, 117, 82, 114, 89, 121, 83, 115, 87, 119, 75, 107, 77, 109, 66, 98, 68, 100, 72, 104, 86, 118, 78, 110, 45, 46, 70, 102, 71, 103, 73, 105, 76, 108, 80, 112, 81, 113];
        assert_eq!(dec, IUPAC_U8);
    }
    #[test]
    fn test_iupac_nucleotide() {
        let dec: [u8; 34] = [65, 97, 67, 99, 71, 103, 84, 116, 85, 117, 82, 114, 89, 121, 83, 115, 87, 119, 75, 107, 77, 109, 66, 98, 68, 100, 72, 104, 86, 118, 78, 110, 45, 46];
        assert_eq!(dec, IUPAC_NUCLEOTIDE_U8);
    }
    #[test]
    fn test_iupac_amino_acid() {
        let dec: [u8; 42] = [65, 97, 67, 99, 68, 100, 69, 101, 70, 102, 71, 103, 72, 104, 73, 105, 75, 107, 76, 108, 77, 109, 78, 110, 80, 112, 81, 113, 82, 114, 83, 115, 84, 116, 85, 117, 86, 118, 87, 119, 89, 121];
        assert_eq!(dec, IUPAC_AMINO_ACID_U8);
    }
    #[test]
    fn test_basic_dna() {
        let dec: [u8; 4] = [65, 67, 71, 84];
        assert_eq!(dec, BASIC_DNA_U8);
    }
    #[test]
    fn test_basic_rna() {
        let dec: [u8; 4] = [65, 67, 71, 85];
        assert_eq!(dec, BASIC_RNA_U8);
    }
    #[test]
    fn test_basic_amino_acid() {
        let dec: [u8; 21] = [65, 67, 68, 69, 70, 71, 72, 73, 75, 76, 77, 78, 80, 81, 82, 83, 84, 85, 86, 87, 89];
        assert_eq!(dec, BASIC_AMINO_ACID_U8);
    }
}