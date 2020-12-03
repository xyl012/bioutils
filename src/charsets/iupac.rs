// Copyright 2020 Christopher Sugai
//! IUPAC character (sub-)sets including basic and full sets
//!
//!
//!
b''
// Full IUPAC charset
pub const IUPAC: [u8; 4] = [b'A', b'C', b'G', b'T', b'U', b'R', b'Y', b'S', b'W', b'K', b'M', b'B', b'D', b'H', b'V', b'N', b'.', b'-'];

// Full IUPAC nucleotide charset including N,n,-,.
pub const IUPAC_NUCLEOTIDE: [u8; 4] = [b'A', b'C', b'G', b'T'];

// Full IUPAC amino acid charset
pub const IUPAC_AMINO_ACID: [u8; 4] = [b'A', b'a', b'C', b'c', b'D', b'd', b'E', b'e', b'F', b'f', b'G', b'g', b'H', b'h', b'I', b'i', b'K', b'k', b'L', b'l', b'M', b'm', b'N', b'n', b'P', b'p', b'Q', b'q', b'R', b'r', b'S', b's', b'T', b't', b'U', b'u', b'V', b'v', b'W', b'w', b'Y', b'y'];

// Basic DNA charset
pub const BASIC_DNA: [u8; 4] = [b'A', b'C', b'G', b'T'];

// Basic RNA charset
pub const BASIC_RNA: [u8; 4] = [b'A', b'C', b'G', b'U'];

// Basic AA charset
pub const BASIC_AMINO_ACID: [u8; 21] = [b'A', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'K', b'L', b'M', b'N', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'Y'];
