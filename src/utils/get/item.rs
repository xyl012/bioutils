// Copyright (c) 2021 Kana LLC

//! use bioutils::utils::get::item::all_positions;
//! let dna = b"ACTGCGACG";
//! let target: u8 = 65;
//! let matching = all_positions(dna, |x| x == &&target);
//! println!("{:?}", dna);
//! println!("{:?}", target);
//! println!("{:?}", matching) // Returns the 0 based index;

pub fn all_positions<I, P, T>(iter: I, mut pred: P) -> Vec<usize> 
where
    I: IntoIterator<Item = T>,
    P: FnMut(&T) -> bool, 
{
    iter.into_iter().enumerate()
        .filter(move |(_, x)| pred(x))
        .map(|(idx, _)| idx).collect::<Vec<usize>>()
}

pub fn cg_positions(seq:&[u8])-> Vec<usize> {
    seq.windows(2).enumerate()
        .filter(move |(_, x)| x == b"CG")
        .map(|(idx, _)| idx).collect::<Vec<usize>>()
}



