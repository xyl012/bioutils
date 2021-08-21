use super::*;

pub trait FindAsRef<T> {
    /// Returns the index of the subsequence if present in self
    fn find_subsequence(&self, subsequence: &[T]) -> Option<usize>;
}

impl<T> FindAsRef<T> for T
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