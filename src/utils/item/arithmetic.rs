use super::*;
use crate::utils::element::percent::*;
use crate::utils::element::tryfrom::*;
use crate::utils::item::count::*;

pub trait ArithmeticAsRefSlice<T> {
    /// Returns the mean of the slice as u8
    fn mean_u8(&self) -> Result<u8, TryFromIntError>;
    /// Returns the mean of the slice as usize
    fn mean_usize(&self) -> usize;
    /// Returns the mode of u8s as u8
    fn mode_u8(&self) -> Option<u8>;
    /// Returns the mode of u8s as usize
    fn mode_usize(&self) -> Option<usize>;
}

impl<T> ArithmeticAsRefSlice<T> for T where
T: AsRef<[u8]>,
{

    /// Returns the mean of u8s as u8
    fn mean_u8(&self) -> Result<u8, TryFromIntError> {
        u8::try_from(self.as_ref().iter().map(|x| *x as u64).sum::<u64>() / self.as_ref().len() as u64)
    }
    
    /// Returns the mean of u8s as usize
    fn mean_usize(&self) -> usize {
        self.as_ref().iter().map(|x| *x as usize).sum::<usize>() / (self.as_ref().len() as usize)
    }

    /// Returns the mode of u8s
    fn mode_u8(&self)-> Option<u8> {
        let mut counts = BTreeMap::new();
        self.as_ref().iter().max_by_key(|&s| {
            let count = counts.entry(s).or_insert(0);
            *count += 1; *count}).copied()
    }
    /// Returns the mode of u8s as usize
    fn mode_usize(&self) -> Option<usize> {
        let mut counts = BTreeMap::new();
        let convert = self.as_ref().iter().max_by_key(|&s| {
            let count = counts.entry(s).or_insert(0);
            *count += 1; *count}).copied();
        match convert {
            Some(_) => Some(convert? as usize),
            None => None,
        }
    }
}

pub trait ArithmeticAsMutSlice<T> {
    /// Returns the mean of u8s as u64
    fn mut_mean_u8(&mut self) -> Result<u8, TryFromIntError>;
    /// Returns the mean of u8s as u64
    fn mut_mean_usize(&mut self) -> usize;
    /// Returns the mode of u8s
    fn mut_mode_u8(&mut self) -> Option<u8>;
    /// Returns the mode of u8s
    fn mut_mode_usize(&mut self) -> Option<usize>;
}

impl<T> ArithmeticAsMutSlice<T> for T where
T: AsMut<[u8]>
{
    /// Returns the mean of u8s as u8
    fn mut_mean_u8(&mut self) -> Result<u8, TryFromIntError> {
        u8::try_from(self.as_mut().iter().map(|x| *x as u64).sum::<u64>() / self.as_mut().len() as u64)
    }
    
    /// Returns the mean of u8s as usize
    fn mut_mean_usize(&mut self) -> usize {
        self.as_mut().iter().map(|x| *x as usize).sum::<usize>() / (self.as_mut().len() as usize)
    }

    /// Returns the mode of u8s
    fn mut_mode_u8(&mut self)-> Option<u8> {
        let mut counts = BTreeMap::new();
        self.as_mut().iter().max_by_key(|&s| {
            let count = counts.entry(s).or_insert(0);
            *count += 1; *count}).copied()
    }
    /// Returns the mode of u8s as usize
    fn mut_mode_usize(&mut self) -> Option<usize> {
        let mut counts = BTreeMap::new();
        let convert = self.as_mut().iter().max_by_key(|&s| {
            let count = counts.entry(s).or_insert(0);
            *count += 1; *count}).copied();
        match convert {
            Some(_) => Some(convert? as usize),
            None => None,
        }
    }
}
