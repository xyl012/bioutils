
//! use bioutils::utils::get::item::all_positions;
//! let dna = b"ACTGCGACG";
//! let target: u8 = 65;
//! let matching = all_positions(dna, |x| x == &&target);
//! println!("{:?}", dna);
//! println!("{:?}", target);
//! println!("{:?}", matching) // Returns the 0 based index;

use crate::charsets::quality::PHRED33_HASHMAP_U8;
use crate::charsets::quality::PHRED64_HASHMAP_U8;
use crate::charsets::quality::PHRED33_HASHMAP_ENCODE_U8;
use crate::charsets::quality::PHRED64_HASHMAP_ENCODE_U8;
use std::iter::FromIterator;
use std::convert::TryInto;

pub trait GetItemU8<T> {
    /// Cuts the read to a specific length
    fn cut_to_length(&self, length: &usize) -> Vec<u8>;
}

impl<T> GetItemU8<T> for T
where
    T: AsRef<[u8]>,
{
    /// Cuts u8 to a specified length. Takes the first x number of u8s. Should be used with check functions for length.
    fn cut_to_length(&self, length: &usize) -> Vec<u8> {
        self.as_ref().iter().take(*length).cloned().collect::<Vec<u8>>()
    }
}

pub trait CodeItemU8<T> {
    /// Returns the PHRED33 quality score from a PHRED33 quality encoding. The score is the u8 minus 33.
    fn decode_qual(&self) -> Vec<u8>;
    /// Returns the PHRED64 quality score from a PHRED64 quality encoding. The score is the u8 minus 64.
    fn decode_qual_phred64(&self) -> Vec<u8>;
    /// Returns the PHRED33 quality encoding from a PHRED33 quality score. The score is the u8 minus 33.
    fn encode_qual(&self) -> Vec<u8>;
    /// Returns the PHRED64 quality encoding from a PHRED64 quality score. The score is the u8 minus 64.
    fn encode_qual_phred64(&self) -> Vec<u8>;
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
    
    /// Returns the PHRED33 quality encoding from a PHRED33 quality score. The score is the u8 minus 33.
    fn encode_qual(&self) -> Vec<u8> {
        self.as_ref().iter().map(|q| PHRED33_HASHMAP_ENCODE_U8.get(&q).unwrap().to_owned()).collect::<Vec<u8>>()
    }

    /// Returns the PHRED64 quality encoding from a PHRED64 quality score. The score is the u8 minus 64.
    fn encode_qual_phred64(&self) -> Vec<u8> {
        self.as_ref().iter().map(|q| PHRED64_HASHMAP_ENCODE_U8.get(&q).unwrap().to_owned()).collect::<Vec<u8>>()
    }
}

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