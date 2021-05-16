
//! use bioutils::utils::get::item::all_positions;
//! let dna = b"ACTGCGACG";
//! let target: u8 = 65;
//! let matching = all_positions(dna, |x| x == &&target);
//! println!("{:?}", dna);
//! println!("{:?}", target);
//! println!("{:?}", matching) // Returns the 0 based index;

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
    /// Cuts u8 to a length. Should be used with check functions for length.
    fn cut_to_length(&self, length: &usize) -> Vec<u8> {
        self.as_ref().iter().take(*length).cloned().collect::<Vec<u8>>()
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

/// Get the counts in a u8 slice of each u8 with the bytecount crate. Possible to use with charsets.
pub fn multi_count_bytecount(needles: &[u8], haystack: &[u8])-> Vec<u64> {
    let mut count = Vec::new();
    for i in needles.iter() {
        let c: u64 = bytecount::count(haystack, *i).try_into().unwrap();
        count.push(c);
    }
    count
}

// pub trait FindKey<T>{
//     fn find(&self, key: &T) -> Option<usize>;
// }

// impl<T, K> FindKeyPosition<K> for T
// where
//     T: AsRef<[K]>,
//     K: PartialEq,
// {
//     fn find(&self, key: &K) -> Option<usize> {
//         self.as_ref().iter().filter(|x| x == key)
//     }
// }