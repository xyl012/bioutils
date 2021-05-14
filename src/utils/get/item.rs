
//! use bioutils::utils::get::item::all_positions;
//! let dna = b"ACTGCGACG";
//! let target: u8 = 65;
//! let matching = all_positions(dna, |x| x == &&target);
//! println!("{:?}", dna);
//! println!("{:?}", target);
//! println!("{:?}", matching) // Returns the 0 based index;

use std::iter::FromIterator;

pub fn all_positions<I, P, T>(iter: I, mut pred: P) -> Vec<usize> 
where
    I: IntoIterator<Item = T>,
    P: FnMut(&T) -> bool, 
{
    iter.into_iter().enumerate()
        .filter(move |(_, x)| pred(x))
        .map(|(idx, _)| idx).collect::<Vec<usize>>()
}

/// Returns CG positions in the given &[u8]
pub fn cg_positions(seq:&[u8])-> Vec<usize> {
    seq.windows(2).enumerate()
        .filter(move |(_, x)| x == b"CG")
        .map(|(idx, _)| idx).collect::<Vec<usize>>()
}



pub trait GetItemU8<T> {
    /// Cuts the read to a specific length
    fn cut_to_length(&self, length: &usize) -> &Self;

}

impl<T> GetItemU8<T> for T
where
    for<'a> &'a T: IntoIterator<Item = &'a u8>,
    for<'a> &'a T: FromIterator<&'a u8>,
{
    /// Cuts u8 to a length. Should be used with check functions for length.
    fn cut_to_length(&self, length: &usize) -> &Self {
        self.into_iter().take(*length).collect::<&Self>()
    }
}
