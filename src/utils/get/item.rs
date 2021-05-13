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





// pub trait ItemU8<T> {
//     /// Returns the PHRED33 quality score from a raw PHRED33 quality encoding. The score is simply the u8 minus 33.
//     pub fn shift_PHRED33_qual_encoding(&self) -> ;
//     // fn check_u8(&self, is_what: &str) -> Result<bool, &str>;

// }

// impl<T> ItemU8<T> for T
// where
//     for<'a> &'a T: IntoIterator<Item = &'a u8>, 
// {
//     /// Returns the PHRED33 quality score from a raw PHRED33 quality encoding. The score is simply the u8 minus 33.
//     fn shift_PHRED33_qual_encoding(&self) -> bool {
//         self.into_iter().all(|x| IUPAC_U8.contains(&x))
//     }
// }

