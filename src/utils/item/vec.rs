
//! use bioutils::utils::get::item::all_positions;
//! let dna = b"ACTGCGACG";
//! let target: u8 = 65;
//! let matching = all_positions(dna, |x| x == &&target);
//! println!("{:?}", dna);
//! println!("{:?}", target);
//! println!("{:?}", matching) // Returns the 0 based index;
use super::*;

pub trait GetItemU8<T> {
    /// Returns the reverse nucleotide complement
    fn reverse_nucleotide_complement(&self) -> Result<Vec<u8>>;
    /// Returns the nucleotide complement
    fn nucleotide_complement(&self) -> Result<Vec<u8>>;
}

impl<T> GetItemU8<T> for T
where
    T: AsRef<[u8]>,
{
    /// Returns the reverse nucleotide complement
    fn reverse_nucleotide_complement(&self) -> Option<Vec<u8>> {
        self.as_ref().iter()
            .rev()
            .map(|nt| NUCLEOTIDE_COMPLEMENT_HASHMAP_U8.get(nt).to_owned())
            .collect()
    }
    /// Returns the nucleotide complement
    fn nucleotide_complement(&self) -> Result<Vec<u8>> {
        self.as_ref().iter()
            .map(|nt| NUCLEOTIDE_COMPLEMENT_HASHMAP_U8.get(nt).to_owned())
            .collect()
    }
}


pub trait Get<T> {
    /// Returns the index of the subsequence if present in self
    fn find_subsequence(&self, subsequence: &[T]) -> Option<usize>;
}

impl<T> Get<T> for T
where
    T: AsRef<[T]>,
    T: PartialEq, 
{
    /// Returns the index of the subsequence if present in self
    fn find_subsequence(&self, subsequence: &[T]) -> Option<usize> {
        self.as_ref().windows(subsequence.len()).position(|window| window == subsequence)
    }
}

/// Returns CG positions in the given &[u8]
pub fn cg_positions(seq:&[u8])-> Result<Vec<usize>> {
    Ok(seq.windows(2).enumerate()
        .filter(move |(_, x)| x == b"CG")
        .map(|(idx, _)| idx).collect::<Vec<usize>>())
}

/// Returns positions in an iterator that match a predicate
pub fn all_positions<I, P, T>(iter: I, mut pred: P) -> Result<Vec<usize>> 
where
    I: IntoIterator<Item = T>,
    P: FnMut(&T) -> bool, 
{
    Ok(iter.into_iter().enumerate()
        .filter(move |(_, x)| pred(x))
        .map(|(idx, _)| idx).collect::<Vec<usize>>())
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