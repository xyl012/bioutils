use super::*;

pub trait ArithmeticAsRefSlice<T> {
    /// Returns the mean of u8s as u64
    fn mean_u8(&self) -> Result<u8, TryFromIntError>;
    /// Returns the mean of u8s as u64
    fn mean_usize(&self) -> usize;

    /// Returns the mode of u8s
    fn mode_u8(&self) -> Option<u8>;
    /// Returns the mode of u8s
    fn mode_usize(&self) -> Option<usize>;
}

impl<T> ArithmeticAsRefSlice<T> for T where
T: AsRef<[u8]>
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

