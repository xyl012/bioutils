//! Convenience wrapper around the memmem crates find_iter to find b"CG"
//! ```
//! use memchr::memmem;
//! use memchr::memmem::FindIter;
//! use bioutils::utils::find::*;
//! let test = b"ACGA";
//! let mut iter = test.iter_cg();
//! 
//! ```

use super::*;
use memchr::memmem;
use memchr::memmem::FindIter;

pub trait MemChrAsRef<T> {
    /// Returns all positions of a byte slice by memchr search.
    fn all_positions(&self, slice: &T) -> Vec<usize>;
    /// Convenience wrapper around the memmem crates find_iter to return all positions of b"CG".
    fn all_positions_cg(&self) -> Vec<usize>;
    /// Convenience wrapper around the memmem crates find_iter to find b"CG"
    fn iter_cg(&self) -> FindIter<'_, '_>;

}

impl<T> MemChrAsRef<T> for T
where
    T: AsRef<[u8]>,
    T: Sized,
{
    /// Returns all positions of a byte slice by memchr search.
    fn all_positions(&self, slice: &T) -> Vec<usize> {
        memmem::find_iter(self.as_ref(), slice.as_ref()).collect::<Vec<usize>>()
    }

    /// Convenience wrapper around the memmem crates find_iter to find b"CG"
    fn iter_cg(&self) -> FindIter<'_, '_> {
        memmem::find_iter(self.as_ref(), b"CG")
    }

    /// Convenience wrapper around the memmem crates find_iter to return all positions of b"CG".
    fn all_positions_cg(&self) -> Vec<usize> {
        memmem::find_iter(self.as_ref(), b"CG").collect::<Vec<usize>>()
    }
    
}

pub trait FindAsRef<T> {
    /// Returns the index of the subsequence if present in self, short circuiting first occurrence only
    fn find_subsequence(&self, subsequence: &[T]) -> Option<usize>;
}

impl<T> FindAsRef<T> for T
where
    T: AsRef<[T]>,
    T: PartialEq, 
{
    /// Returns the index of the subsequence if present in self, short circuiting first occurrence only
    fn find_subsequence(&self, subsequence: &[T]) -> Option<usize> {
        self.as_ref().windows(subsequence.len()).position(|window| window == subsequence)
    }
}

/// Returns CG positions in the given &[u8]. Non short circuiting, will return all positions that have a 'CG'
pub fn cg_positions(seq:&[u8])-> Result<Vec<usize>> {
    Ok(seq.windows(2).enumerate()
        .filter(move |(_, x)| x == b"CG")
        .map(|(idx, _)| idx).collect::<Vec<usize>>())
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