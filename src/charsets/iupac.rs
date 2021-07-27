//! IUPAC character (sub-)sets including basic and full sets. Provided as u8 and str arrays.

use super::*;

pub enum IupacCharSet {
    Nucleotide,
    Dna,
    DnaMixCase,
    Dnan,
    DnanMixCase,
    DnaLowercase,
    Rna,
    RnaMixCase,
    Rnan,
    RnanMixCase,
    RnaLowercase,
    Gap,
    N,
    NMixCase,
    Gc,
    GcMixCase,
    AminoAcid,
}

impl IupacCharSet {
    pub const fn value(&self) -> &[u8] {
        match *self {
            IupacCharSet::Nucleotide => IUPAC_NUCLEOTIDE_SLICE,
            IupacCharSet::Dna => DNA_SLICE,
            IupacCharSet::DnaMixCase => DNA_MIX_CASE_SLICE,
            IupacCharSet::Dnan => DNAN_SLICE,
            IupacCharSet::DnanMixCase => DNAN_MIX_CASE_SLICE,
            IupacCharSet::DnaLowercase => DNA_LOWERCASE_SLICE,
            IupacCharSet::Rna => RNA_SLICE,
            IupacCharSet::RnaMixCase => RNA_MIX_CASE_SLICE,
            IupacCharSet::Rnan => RNAN_SLICE,
            IupacCharSet::RnanMixCase => RNAN_MIX_CASE_SLICE,
            IupacCharSet::RnaLowercase => RNA_LOWERCASE_SLICE,
            IupacCharSet::Gap => GAP_SLICE,
            IupacCharSet::N => N_SLICE,
            IupacCharSet::NMixCase => N_MIX_CASE_SLICE,
            IupacCharSet::Gc => GC_SLICE,
            IupacCharSet::GcMixCase => GC_MIX_CASE_SLICE,
            IupacCharSet::AminoAcid => AMINO_ACID_SLICE,
        }
    }
}

pub const IUPAC_NUCLEOTIDE_SLICE: &'static [u8] = &IUPAC_NUCLEOTIDE;
pub const IUPAC_AMINO_ACID_SLICE: &'static [u8] = &IUPAC_AMINO_ACID;
pub const DNA_SLICE: &'static [u8] = &DNA;
pub const DNA_MIX_CASE_SLICE : &'static [u8] = &DNA_MIX_CASE;
pub const DNAN_SLICE: &'static [u8] = &DNAN;
pub const DNAN_MIX_CASE_SLICE: &'static [u8] = &DNAN_MIX_CASE;
pub const DNA_LOWERCASE_SLICE: &'static [u8] = &DNA_LOWERCASE;
pub const RNA_SLICE: &'static [u8] = &RNA;
pub const RNA_MIX_CASE_SLICE : &'static [u8] = &RNA_MIX_CASE;
pub const RNAN_SLICE: &'static [u8] = &RNAN;
pub const RNAN_MIX_CASE_SLICE: &'static [u8] = &RNAN_MIX_CASE;
pub const RNA_LOWERCASE_SLICE: &'static [u8] = &RNA_LOWERCASE;
pub const GAP_SLICE: &'static [u8] = &GAP;
pub const N_SLICE: &'static [u8] = &N;
pub const N_MIX_CASE_SLICE: &'static [u8] = &N_MIX_CASE;
pub const GC_SLICE: &'static [u8] = &GC;
pub const GC_MIX_CASE_SLICE: &'static [u8] = &GC_MIX_CASE;
pub const AMINO_ACID_SLICE: &'static [u8] = &AMINO_ACID;

// Full IUPAC charset including nucleotides and amino acids
pub const IUPAC: [u8; 46] = [
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
    pub static ref IUPAC_HASHSET: HashSet<u8> = new_hashset(&IUPAC);
}
lazy_static! {
    pub static ref IUPAC_HASHSET_STR: HashSet<&'static str> = new_str_hashset(&IUPAC_STR);
}

// Full IUPAC nucleotide charset including N,n,-,.
pub const IUPAC_NUCLEOTIDE: [u8; 34] = [
    b'A', b'a', b'C', b'c', b'G', b'g', b'T', b't', b'U', b'u', b'R', b'r', b'Y', b'y', b'S', b's',
    b'W', b'w', b'K', b'k', b'M', b'm', b'B', b'b', b'D', b'd', b'H', b'h', b'V', b'v', b'N', b'n',
    b'-', b'.',
];
pub const IUPAC_NUCLEOTIDE_STR: [&str; 34] = [
    "A", "a", "C", "c", "G", "g", "T", "t", "U", "u", "R", "r", "Y", "y", "S", "s", "W", "w", "K",
    "k", "M", "m", "B", "b", "D", "d", "H", "h", "V", "v", "N", "n", r#"-"#, r#"."#,
];
lazy_static! {
    pub static ref IUPAC_NUCLEOTIDE_HASHSET: HashSet<u8> = new_hashset(&IUPAC_NUCLEOTIDE);
}
lazy_static! {
    pub static ref IUPAC_NUCLEOTIDE_HASHSET_STR: HashSet<&'static str> =
        new_str_hashset(&IUPAC_NUCLEOTIDE_STR);
}

lazy_static! {
    pub static ref NUCLEOTIDE_COMPLEMENT_HASHMAP: HashMap<u8, u8> = vec![
        (b'A', b'T'), (b'a', b't'), (b'C', b'G'), (b'c', b'g'), (b'G', b'C'), (b'g', b'c'), (b'T', b'A'), (b't', b'a'), (b'U', b'A'), (b'u', b'a'), (b'N', b'N'), (b'n', b'n'), (b'-', b'-'), (b'.', b'.')
        ].into_iter().collect();
}

// Full IUPAC amino acid charset
pub const IUPAC_AMINO_ACID: [u8; 42] = [
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
    pub static ref IUPAC_AMINO_ACID_HASHSET: HashSet<u8> = new_hashset(&IUPAC_AMINO_ACID);
}
lazy_static! {
    pub static ref IUPAC_AMINO_ACID_HASHSET_STR: HashSet<&'static str> =
        new_str_hashset(&IUPAC_AMINO_ACID_STR);
}

// Basic DNA charset
pub const DNA: [u8; 4] = [b'A', b'C', b'G', b'T'];
pub const DNA_STR: [&str; 4] = ["A", "C", "G", "T"];
pub const DNA_MIX_CASE: [u8; 8] = [b'A', b'a', b'C', b'c', b'G', b'g', b'T', b't'];
pub const DNA_MIX_CASE_STR: [&str; 8] = ["A", "a", "C", "c", "G", "g","T", "t"];

lazy_static! {
    pub static ref DNA_HASHSET: HashSet<u8> = new_hashset(&DNA);
}
lazy_static! {
    pub static ref DNA_HASHSET_STR: HashSet<&'static str> = new_str_hashset(&DNA_STR);
}
lazy_static! {
    pub static ref DNA_MIX_CASE_HASHSET: HashSet<u8> = new_hashset(&DNA_MIX_CASE);
}
lazy_static! {
    pub static ref DNA_MIX_CASE_HASHSET_STR: HashSet<&'static str> = new_str_hashset(&DNA_MIX_CASE_STR);
}
// Basic DNAN charset
pub const DNAN: [u8; 5] = [b'A', b'C', b'G', b'T', b'N'];
pub const DNAN_STR: [&str; 5] = ["A", "C", "G", "T", "N"];
pub const DNAN_MIX_CASE: [u8; 10] = [b'A', b'a', b'C', b'c', b'G', b'g', b'T', b't', b'N', b'n'];
pub const DNAN_MIX_CASE_STR: [&str; 10] = ["A", "a", "C", "c", "G", "g","T", "t", "N", "n"];

lazy_static! {
    pub static ref DNAN_HASHSET: HashSet<u8> = new_hashset(&DNAN);
}
lazy_static! {
    pub static ref DNAN_HASHSET_STR: HashSet<&'static str> = new_str_hashset(&DNAN_STR);
}
lazy_static! {
    pub static ref DNAN_MIX_CASE_HASHSET: HashSet<u8> = new_hashset(&DNAN_MIX_CASE);
}
lazy_static! {
    pub static ref DNAN_MIX_CASE_HASHSET_STR: HashSet<&'static str> = new_str_hashset(&DNAN_MIX_CASE_STR);
}

// Basic RNA charset
pub const RNA: [u8; 4] = [b'A', b'C', b'G', b'U'];
pub const RNA_STR: [&str; 4] = ["A", "C", "G", "U"];
pub const RNA_MIX_CASE: [u8; 8] = [b'A', b'a', b'C', b'c', b'G', b'g', b'U', b'u'];
pub const RNA_MIX_CASE_STR: [&str; 8] = ["A", "a", "C", "c", "G", "g", "U", "u"];

lazy_static! {
    pub static ref RNA_HASHSET: HashSet<u8> = new_hashset(&RNA);
}
lazy_static! {
    pub static ref RNA_HASHSET_STR: HashSet<&'static str> = new_str_hashset(&RNA_STR);
}
lazy_static! {
    pub static ref RNA_MIX_CASE_HASHSET: HashSet<u8> = new_hashset(&RNA_MIX_CASE);
}
lazy_static! {
    pub static ref RNA_MIX_CASE_HASHSET_STR: HashSet<&'static str> = new_str_hashset(&RNA_MIX_CASE_STR);
}

// Basic RNAN charset
pub const RNAN: [u8; 5] = [b'A', b'C', b'G', b'U', b'N'];
pub const RNAN_STR: [&str; 5] = ["A", "C", "G", "U", "N"];
pub const RNAN_MIX_CASE: [u8; 10] = [b'A', b'a', b'C', b'c', b'G', b'g', b'U', b'u', b'N', b'n'];
pub const RNAN_MIX_CASE_STR: [&str; 10] = ["A", "a", "C", "c", "G", "g", "U", "u", "N", "n"];

lazy_static! {
    pub static ref RNAN_HASHSET: HashSet<u8> = new_hashset(&RNAN);
}
lazy_static! {
    pub static ref RNAN_HASHSET_STR: HashSet<&'static str> = new_str_hashset(&RNAN_STR);
}
lazy_static! {
    pub static ref RNAN_MIX_CASE_HASHSET: HashSet<u8> = new_hashset(&RNAN_MIX_CASE);
}
lazy_static! {
    pub static ref RNAN_MIX_CASE_HASHSET_STR: HashSet<&'static str> = new_str_hashset(&RNAN_MIX_CASE_STR);
}

pub const DNA_LOWERCASE: [u8; 4] = [b'a', b'c', b'g', b't'];
pub const DNA_LOWERCASE_STR: [&str; 4] = ["a", "c", "g", "t"];
lazy_static! {
    pub static ref DNA_LOWERCASE_HASHSET: HashSet<u8> =
        new_hashset(&DNA_LOWERCASE);
}
lazy_static! {
    pub static ref DNA_LOWERCASE_HASHSET_STR: HashSet<&'static str> =
        new_str_hashset(&DNA_LOWERCASE_STR);
}

pub const RNA_LOWERCASE: [u8; 4] = [b'a', b'c', b'g', b'u'];
pub const RNA_LOWERCASE_STR: [&str; 4] = ["a", "c", "g", "u"];
lazy_static! {
    pub static ref RNA_LOWERCASE_HASHSET: HashSet<u8> =
        new_hashset(&RNA_LOWERCASE);
}
lazy_static! {
    pub static ref RNA_LOWERCASE_HASHSET_STR: HashSet<&'static str> =
        new_str_hashset(&RNA_LOWERCASE_STR);
}

// Basic AA charset
pub const AMINO_ACID: [u8; 21] = [
    b'A', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'K', b'L', b'M', b'N', b'P', b'Q', b'R', b'S',
    b'T', b'U', b'V', b'W', b'Y',
];
pub const AMINO_ACID_STR: [&str; 21] = [
    "A", "C", "D", "E", "F", "G", "H", "I", "K", "L", "M", "N", "P", "Q", "R", "S", "T", "U", "V",
    "W", "Y",
];
lazy_static! {
    pub static ref AMINO_ACID_HASHSET: HashSet<u8> = new_hashset(&AMINO_ACID);
}
lazy_static! {
    pub static ref AMINO_ACID_HASHSET_STR: HashSet<&'static str> =
        new_str_hashset(&AMINO_ACID_STR);
}

// Only gap charset
pub const GAP: [u8; 2] = [b'.', b'-'];
pub const GAP_STR: [&str; 2] = [".", "-"];

// Only N charset
pub const N: [u8; 1] = [b'N'];
pub const N_STR: [&str; 1] = ["N"];
pub const N_MIX_CASE: [u8; 2] = [b'N', b'n'];
pub const N_MIX_CASE_STR: [&str; 2] = ["N", "n"];

// Only CG charset
pub const GC: [u8; 2] = [b'C', b'G'];
pub const GC_STR: [&str; 2] = ["C", "G"];
pub const GC_MIX_CASE: [u8; 4] = [b'C', b'G', b'c', b'g'];
pub const GC_MIX_CASE_STR: [&str; 4] = ["C", "G", "c", "g"];

// IUPAC equivalent charsets
pub const Y_BASES_RNA: [u8; 2] = [b'C', b'U'];
pub const Y_BASES_RNA_LOWERCASE: [u8; 2] = [b'c', b'u'];
pub const W_BASES_RNA: [u8; 2] = [b'A', b'U'];
pub const W_BASES_RNA_LOWERCASE: [u8; 2] = [b'a', b'u'];
pub const K_BASES_RNA: [u8; 2] = [b'U', b'G'];
pub const K_BASES_RNA_LOWERCASE: [u8; 2] = [b'u', b'g'];
pub const B_BASES_RNA: [u8; 3] = [b'C', b'U', b'G'];
pub const B_BASES_RNA_LOWERCASE: [u8; 3] = [b'c', b'u', b'g'];
pub const D_BASES_RNA: [u8; 3] = [b'A', b'U', b'G'];
pub const D_BASES_RNA_LOWERCASE: [u8; 3] = [b'a', b'u', b'g'];
pub const H_BASES_RNA: [u8; 3] = [b'A', b'C', b'U'];
pub const H_BASES_RNA_LOWERCASE: [u8; 3] = [b'a', b'c', b'u'];

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

lazy_static! {
    pub static ref CODON_HASHMAP: HashMap<Vec<u8>, u8> =
    vec![
        (b"TAA", b'*'), (b"TAG", b'*'), (b"TGA", b'*'), (b"TTT", b'F'), (b"TTC", b'F'), (b"TTA", b'L'), (b"TTG", b'L'), (b"TCT", b'S'), (b"TCC", b'S'), (b"TCA", b'S'), (b"TCG", b'S'), (b"TAT", b'Y'), (b"TAC", b'Y'), (b"TGT", b'C'), (b"TGC", b'C'), (b"TGG", b'W'), (b"CTT", b'L'), (b"CTC", b'L'), (b"CTA", b'L'), (b"CTG", b'L'), (b"CCT", b'P'), (b"CCC", b'P'), (b"CCA", b'P'), (b"CCG", b'P'), (b"CAT", b'H'), (b"CAC", b'H'), (b"CAA", b'Q'), (b"CAG", b'Q'), (b"CGT", b'R'), (b"CGC", b'R'), (b"CGA", b'R'), (b"CGG", b'R'), (b"ATT", b'I'), (b"ATC", b'I'), (b"ATA", b'I'), (b"ATG", b'M'), (b"ACT", b'T'), (b"ACC", b'T'), (b"ACA", b'T'), (b"ACG", b'T'), (b"AAT", b'N'), (b"AAC", b'N'), (b"AAA", b'K'), (b"AAG", b'K'), (b"AGT", b'S'), (b"AGC", b'S'), (b"AGA", b'R'), (b"AGG", b'R'), (b"GTT", b'V'), (b"GTC", b'V'), (b"GTA", b'V'), (b"GTG", b'V'), (b"GTN", b'V'), (b"GCT", b'A'), (b"GCC", b'A'), (b"GCA", b'A'), (b"GCG", b'A'), (b"GAT", b'D'), (b"GAC", b'D'), (b"GAA", b'E'), (b"GAG", b'E'), (b"GGT", b'G'), (b"GGC", b'G'), (b"GGA", b'G'), (b"GGG", b'G'),
    ].into_iter().map(|(c,b)| (c.to_vec(), b) ).collect();
}

// <[T; N] as std::borrow::Borrow<[T]>>
// CODON_HASHMAP.get(nt).to_owned()

// #[cfg(test)]
// mod tests {
//     use super::{
//         AMINO_ACID, DNA, RNA, IUPAC_AMINO_ACID, IUPAC_NUCLEOTIDE,
//         IUPAC,
//     };
//     #[test]
//     fn test_iupac() {
//         let dec: [u8; 46] = [
//             65, 97, 67, 99, 71, 103, 84, 116, 85, 117, 82, 114, 89, 121, 83, 115, 87, 119, 75, 107,
//             77, 109, 66, 98, 68, 100, 72, 104, 86, 118, 78, 110, 45, 46, 70, 102, 71, 103, 73, 105,
//             76, 108, 80, 112, 81, 113,
//         ];
//         assert_eq!(dec, IUPAC);
//     }
//     #[test]
//     fn test_iupac_nucleotide() {
//         let dec: [u8; 34] = [
//             65, 97, 67, 99, 71, 103, 84, 116, 85, 117, 82, 114, 89, 121, 83, 115, 87, 119, 75, 107,
//             77, 109, 66, 98, 68, 100, 72, 104, 86, 118, 78, 110, 45, 46,
//         ];
//         assert_eq!(dec, IUPAC_NUCLEOTIDE);
//     }
//     #[test]
//     fn test_iupac_amino_acid() {
//         let dec: [u8; 42] = [
//             65, 97, 67, 99, 68, 100, 69, 101, 70, 102, 71, 103, 72, 104, 73, 105, 75, 107, 76, 108,
//             77, 109, 78, 110, 80, 112, 81, 113, 82, 114, 83, 115, 84, 116, 85, 117, 86, 118, 87,
//             119, 89, 121,
//         ];
//         assert_eq!(dec, IUPAC_AMINO_ACID);
//     }
//     #[test]
//     fn test_dna() {
//         let dec: [u8; 4] = [65, 67, 71, 84];
//         assert_eq!(dec, DNA);
//     }
//     #[test]
//     fn test_rna() {
//         let dec: [u8; 4] = [65, 67, 71, 85];
//         assert_eq!(dec, RNA);
//     }
//     #[test]
//     fn test_AMINO_ACID() {
//         let dec: [u8; 21] = [
//             65, 67, 68, 69, 70, 71, 72, 73, 75, 76, 77, 78, 80, 81, 82, 83, 84, 85, 86, 87, 89,
//         ];
//         assert_eq!(dec, AMINO_ACID);
//     }
// }
