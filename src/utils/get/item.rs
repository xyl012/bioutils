
//! use bioutils::utils::get::item::all_positions;
//! let dna = b"ACTGCGACG";
//! let target: u8 = 65;
//! let matching = all_positions(dna, |x| x == &&target);
//! println!("{:?}", dna);
//! println!("{:?}", target);
//! println!("{:?}", matching) // Returns the 0 based index;

use crate::charsets::iupac::CODON_HASHMAP;
use crate::charsets::iupac::NUCLEOTIDE_COMPLEMENT_HASHMAP_U8;
use std::collections::HashMap;
use crate::charsets::quality::SANGER_HASHMAP_ENCODE_U8;
use crate::charsets::quality::SANGER_HASHMAP_DECODE_U8;
use crate::charsets::quality::PHRED33_HASHMAP_U8;
use crate::charsets::quality::PHRED64_HASHMAP_U8;
use crate::charsets::quality::PHRED33_HASHMAP_ENCODE_U8;
use crate::charsets::quality::PHRED64_HASHMAP_ENCODE_U8;
use std::iter::FromIterator;
use std::convert::TryInto;


use crate::utils::check::value::Check;

pub trait GetItemU8<T> {
    /// Cuts u8 to a specified length from index position. index starts at 0, so to get the first two u8s index would be 0 and length would be 2
    fn subseq(&self, index: &usize, length: &usize) -> Result<&[u8], &'static str>;
    /// Returns the reverse nucleotide complement
    fn reverse_nucleotide_complement(&self) -> Vec<u8>;
    /// Returns the nucleotide complement
    fn nucleotide_complement(&self) -> Vec<u8>;
}

impl<T> GetItemU8<T> for T
where
    T: AsRef<[u8]>,
{
    /// Cuts u8 to a specified length from index position. index starts at 0, so to get the first two u8s index would be 0 and length would be 2
    fn subseq(&self, index: &usize, length: &usize) -> Result<&[u8], &'static str> {
        let self_length = self.as_ref().len();
        if self_length < *index && self_length < *length {
                Ok(&self.as_ref()[*index..*length])
        } else { Err("Index or length out of bounds") }
    }
    /// Returns the reverse nucleotide complement
    fn reverse_nucleotide_complement(&self) -> Vec<u8> {
        self.as_ref().iter()
            .rev()
            .map(|nt| NUCLEOTIDE_COMPLEMENT_HASHMAP_U8.get(nt).unwrap().to_owned())
            .collect()
    }
    /// Returns the nucleotide complement
    fn nucleotide_complement(&self) -> Vec<u8> {
        self.as_ref().iter()
            .map(|nt| NUCLEOTIDE_COMPLEMENT_HASHMAP_U8.get(nt).unwrap().to_owned())
            .collect()
    }
}

pub trait CodeItemU8<T> {
    /// Returns the PHRED33 quality score from a PHRED33 quality encoding. The score is the u8 minus 33.
    fn decode_qual(&self) -> Vec<u8>;
    /// Returns the PHRED64 quality score from a PHRED64 quality encoding. The score is the u8 minus 64.
    fn decode_qual_phred64(&self) -> Vec<u8>;
    /// Returns the SANGER quality score from a SANGER quality encoding. The score is the u8 minus 33.
    fn decode_qual_sanger(&self) -> Vec<u8>;
    /// Returns the PHRED33 quality encoding from a PHRED33 quality score. The score is the u8 minus 33.
    fn encode_qual(&self) -> Vec<u8>;
    /// Returns the PHRED64 quality encoding from a PHRED64 quality score. The score is the u8 minus 64.
    fn encode_qual_phred64(&self) -> Vec<u8>;
    /// Returns the SANGER quality encoding from a SANGER quality score. The score is the u8 minus 33.
    fn encode_qual_sanger(&self) -> Vec<u8>;
}

impl<T> CodeItemU8<T> for T
where
    T: AsRef<[u8]>,
{
    /// Returns the PHRED33 quality score from a raw PHRED33 quality encoding. The score is simply the u8 minus 33.
    fn decode_qual(&self) -> Vec<u8> {
        self.as_ref().iter().map(|q| PHRED33_HASHMAP_U8.get(&q).unwrap().to_owned()).collect::<Vec<u8>>()
    }
    /// Returns the PHRED64 quality score from a raw PHRED64 quality encoding. The score is simply the u8 minus 64.
    fn decode_qual_phred64(&self) -> Vec<u8> {
        self.as_ref().iter().map(|q| PHRED64_HASHMAP_U8.get(&q).unwrap().to_owned()).collect::<Vec<u8>>()
    }
    /// Returns the SANGER quality score from a SANGER quality encoding. The score is the u8 minus 33.
    fn decode_qual_sanger(&self) -> Vec<u8> {
        self.as_ref().iter().map(|q| SANGER_HASHMAP_DECODE_U8.get(&q).unwrap().to_owned()).collect::<Vec<u8>>()
    }
    /// Returns the PHRED33 quality encoding from a PHRED33 quality score. The score is the u8 minus 33.
    fn encode_qual(&self) -> Vec<u8> {
        self.as_ref().iter().map(|q| PHRED33_HASHMAP_ENCODE_U8.get(&q).unwrap().to_owned()).collect::<Vec<u8>>()
    }
    /// Returns the PHRED64 quality encoding from a PHRED64 quality score. The score is the u8 minus 64.
    fn encode_qual_phred64(&self) -> Vec<u8> {
        self.as_ref().iter().map(|q| PHRED64_HASHMAP_ENCODE_U8.get(&q).unwrap().to_owned()).collect::<Vec<u8>>()
    }
    /// Returns the SANGER quality encoding from a SANGER quality score. The score is the u8 minus 33.
    fn encode_qual_sanger(&self) -> Vec<u8> {
        self.as_ref().iter().map(|q| SANGER_HASHMAP_ENCODE_U8.get(&q).unwrap().to_owned()).collect::<Vec<u8>>()
    }
}


    // /// Returns the nucleotide complement
    // fn nucleotide_complement(&self) -> Vec<u8> {
    //     self.as_ref().iter()
    //         .map(|nt| NUCLEOTIDE_COMPLEMENT_HASHMAP_U8.get(&nt).unwrap().to_owned())
    //         .collect()
    // }
    // /// Translates the nucleotide sequence
    // fn translate_nucleotide(seq: &[u8]) -> Vec<u8> {
    //     seq.iter().take(3)
    //         .map(|nt| CODON_HASHMAP.get(nt).unwrap().to_owned())
    //         .collect()
    //     }

    // new trait for removing adapter
    // /// Removes a sequence (most commonly an adapter) if present from the index given. index starts at 0, if adapter starts at beginning of sequence index = 0
    // fn remove_sequence(&self, sequence: &[u8], index: &usize) -> Result<T, E> {
    //     let theoretical_length = sequence.len() + index;
    //     match self.as_ref().len() < theoretical_length {
    //         true => Ok(self),
    //         false => Err(self),
    //     }
    //     // if self.as_ref().len() < theoretical_length {
    //     //     self.as_ref().remove_matches(sequence)
    //     // }
    // }
    // /// Removes a sequence (most commonly an adapter) if present from the position given. position starts at 0.
    // fn remove_sequence(&self, sequence: &[u8], index: &usize) -> Result<Vec<u8>, Self>;

// pub fn test(seq: &[u8], index: usize)->&[u8]{
//     &seq[0..index]
// }

/// Returns CG positions in the given &[u8]
pub fn cg_positions(seq:&[u8])-> Vec<usize> {
    seq.windows(2).enumerate()
        .filter(move |(_, x)| x == b"CG")
        .map(|(idx, _)| idx).collect::<Vec<usize>>()
}

/// Returns positions in an iterator that match a predicate
pub fn all_positions<I, P, T>(iter: I, mut pred: P) -> Vec<usize> 
where
    I: IntoIterator<Item = T>,
    P: FnMut(&T) -> bool, 
{
    iter.into_iter().enumerate()
        .filter(move |(_, x)| pred(x))
        .map(|(idx, _)| idx).collect::<Vec<usize>>()
}


// /// Creates a hashmap of the count of each position returned from a suffix array alignment using the suffix array crate. This only includes the start of the aligned section with no other information.
// pub fn position_pileup(positions: &[u32]) -> HashMap<u32, u64> {
//     let mut hashmap = HashMap::new();
//     for i in positions.iter(){
//     let position_count = hashmap.entry(i.to_owned()).or_insert(0); *position_count += 1u64;
//     }
//     hashmap
// }

// pub trait FindKey<T>{
//     fn find(&self, key: &T) -> Option<usize>;
// }

// impl<T, K> FindKey<K> for T
// where
//     T: AsRef<[K]>,
//     K: PartialEq,
// {
//     fn find(&self, key: &K) -> Option<usize> {
//         self.as_ref().iter().filter(|x| x == key)
//     }
// }