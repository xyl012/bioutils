// Copyright 2020 Christopher Sugai
//! IUPAC character (sub-)sets including basic and full sets
//!
//!
//!
b''
// Full IUPAC charset including nucleotides and amino acids
pub const IUPAC: [u8; 78] = [b'A', b'a', b'C', b'c', b'G', b'g', b'T', b't', b'U', b'u', b'R', b'r', b'Y', b'y', b'S', b's', b'W', b'w', b'K', b'k', b'M', b'm', b'B', b'b', b'D', b'd', b'H', b'h', b'V', b'v', b'N', b'n', b'-', b'.', b'F', b'f', b'G', b'g', b'I', b'i', b'L', b'l', b'P', b'p', b'Q', b'q'];

// Full IUPAC nucleotide charset including N,n,-,.
pub const IUPAC_NUCLEOTIDE: [u8; 36] = [b'A', b'a', b'C', b'c', b'G', b'g', b'T', b't', b'U', b'u', b'R', b'r', b'Y', b'y', b'S', b's', b'W', b'w', b'K', b'k', b'M', b'm', b'B', b'b', b'D', b'd', b'H', b'h', b'V', b'v', b'N', b'n', b'-', b'.'];

// Full IUPAC amino acid charset
pub const IUPAC_AMINO_ACID: [u8; 42] = [b'A', b'a', b'C', b'c', b'D', b'd', b'E', b'e', b'F', b'f', b'G', b'g', b'H', b'h', b'I', b'i', b'K', b'k', b'L', b'l', b'M', b'm', b'N', b'n', b'P', b'p', b'Q', b'q', b'R', b'r', b'S', b's', b'T', b't', b'U', b'u', b'V', b'v', b'W', b'w', b'Y', b'y'];

// Basic DNA charset
pub const BASIC_DNA: [u8; 4] = [b'A', b'C', b'G', b'T'];

// Basic RNA charset
pub const BASIC_RNA: [u8; 4] = [b'A', b'C', b'G', b'U'];

// Basic AA charset
pub const BASIC_AMINO_ACID: [u8; 21] = [b'A', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'K', b'L', b'M', b'N', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'Y'];
