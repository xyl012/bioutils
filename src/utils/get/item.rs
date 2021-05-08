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

//    /// Returns the iterations greater than the criteria
//    fn iters_greater_than(&self, criteria:&u8)-> usize;

//     /// Returns the iterations greater than the criteria
//     fn iters_greater_than(&self, criteria:&u8)-> usize {
//         self.into_iter().filter_map(|s| Some(s>=criteria)).count()
//     }

// Position types: If output is multiple, for example positions in a sequence which are 'CG', return a vector (in our example vec<bool>)
// i32 if returning start and end

// pub trait PositionU8<T, P> {
//     fn all_positions(&self, pred: P) ->Vec<u8>;
// }

// impl<T, P> PositionU8<T, P> where
//     P: FnMut(&T) -> bool,
//     T: IntoIterator<Item = u8>,
// {
//     /// Find the positions of all occurrences of a predicate, for example xpred==b"CG"
//     fn all_positions(&self){
//         self.into_iter().positions(|v| v == b"C")
//     }

//     // fn all_positions(&self, predicate: P)->Vec<usize> {
//     //     self.into_iter().enumerate()
//     //         .filter(move |(_, xpred)| predicate(xpred))
//     //         .map(|(idx, _)| idx).collect::<Vec<usize>>()
//     // }
// }




// Get the positions of cgs in a u8 sequence
// pub fn cg_positions(seq:&[u8])-> &[u8] {
//     seq.windows(2).rposition(|&x| x == &seq)
// /// Get the positions of sequence in u8 sequence
// pub fn seq_positions(seq:&[u8])-> &[u8] {
    
// }

// }
// /// Get the percentage content in a u8 sequence

// /// Get the percentage content of a sequence in a u8 sequence

// /// Encodes a u8 vector of bytes with information from the sequence as well as quality. Each byte being phred33-33 or {0..40}+{128..255}, which is the phred33 score plus ascii
// pub fn qs_bytes(bytes_1: &mut Vec<u8>,bytes_2: &mut Vec<u8>)-> Vec<u16> {
//     let mut bytes_1_i = bytes_1.len() - 1;
//     let mut bytes_2_i = bytes_2.len() - 1;
//     let mut bytes_1: Vec<u16> = bytes_1.iter().map(|c| *c as u16).collect();
//     let mut bytes_2: Vec<u16> = bytes_2.iter().map(|c| *c as u16).collect();
//     while bytes_1_i >= 0 {
//         bytes_1[bytes_1_i] = (bytes_1[bytes_1_i] - 33) * (bytes_2[bytes_1_i] + 128);
//         bytes_1[bytes_1_i];
//         if bytes_1_i == 0 { break; }
//         bytes_1_i -= 1;
//     }
//     bytes_1
// }

// // pub fn qs_bytes<'a>(bytes_1: &'a mut Vec<u8>,bytes_2: &'a mut Vec<u8>)-> &'a mut Vec<u8> {

// //     let mut bytes_1_i = bytes_1.len() - 1;
// //     let mut bytes_2_i = bytes_2.len() - 1;
// //     while bytes_1_i > 0 {
// //         bytes_1[bytes_1_i] = (bytes_1[bytes_1_i] - 33) * (bytes_2[bytes_1_i] + 128);
// //         bytes_1[bytes_1_i];
// //         if bytes_1_i == 0 { break; }
// //         bytes_1_i -= 1;
// //     }
// //     bytes_1
// // }

// // let x = b"ABCD";
// // let y = b"1010";
// // println!(b"BBDD");
// // println!(b"BBDD");

// // /// Take in a sequence string and create a vector of sequence strings with hamming distance 1 using the bases ACTG. Requires the sequence to be ACTGs, use replace if N.- or other symbols present.
// // // Example: AAAA -> CAAA GAAA TAAA ACAA AGAA ATAA etc.
// // pub fn nucleotide_set_hamming(nucl: String) -> Vec<String>  {
// //     let mut rng = rand::thread_rng();
// //     for base in 0 .. nucl.len() + 1 {
// //     let results: Vec<String> = Vec::new();
// //         for symbol in BASIC_DNA_U8.iter() {
// //             let (first, last) = symbol.split_at(base);
// //             let mut buffer = [0; 1];
// //             let result = symbol.encode_utf8(&mut buffer);
// //             results.push([first, result, last].concat());
// //         }
// //     results
// //     }
// // }
// // //-> Vec<String> 
// // // /// A function that returns the correction for the specified word.
// // // pub fn correct(&mut self, word: &str) -> String {
// // //     // A word in our word frequency map is already correct.
// // //     if self.n_words.contains_key(word) {
// // //         return word.to_string();
// // //     }

