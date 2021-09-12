use super::*;




pub trait ArithmeticAsRefSlice<T> {
    /// Returns the mean of the slice as u8
    fn u8_mean(&self) -> Result<u8, TryFromIntError>;
    /// Returns the mean of the slice as usize
    fn usize_mean(&self) -> usize;
    /// Returns the mode of u8s as u8
    fn u8_mode(&self) -> Option<u8>;
    /// Returns the mode of u8s as usize
    fn usize_mode(&self) -> Option<usize>;
}

impl<T> ArithmeticAsRefSlice<T> for T where
T: AsRef<[u8]>,
{

    /// Returns the mean of u8s as u8
    fn u8_mean(&self) -> Result<u8, TryFromIntError> {
        u8::try_from(self.as_ref().iter().map(|x| *x as u64).sum::<u64>() / self.as_ref().len() as u64)
    }
    
    /// Returns the mean of u8s as usize
    fn usize_mean(&self) -> usize {
        self.as_ref().iter().map(|x| *x as usize).sum::<usize>() / (self.as_ref().len() as usize)
    }

    /// Returns the mode of u8s
    fn u8_mode(&self)-> Option<u8> {
        let mut counts = BTreeMap::new();
        self.as_ref().iter().max_by_key(|&s| {
            let count = counts.entry(s).or_insert(0);
            *count += 1; *count}).copied()
    }
    /// Returns the mode of u8s as usize
    fn usize_mode(&self) -> Option<usize> {
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
    fn mut_u8_mean(&mut self) -> Result<u8, TryFromIntError>;
    /// Returns the mean of u8s as u64
    fn mut_usize_mean(&mut self) -> usize;
    /// Returns the mode of u8s
    fn mut_u8_mode(&mut self) -> Option<u8>;
    /// Returns the mode of u8s
    fn mut_usize_mode(&mut self) -> Option<usize>;
}

impl<T> ArithmeticAsMutSlice<T> for T where
T: AsMut<[u8]>
{
    /// Returns the mean of u8s as u8
    fn mut_u8_mean(&mut self) -> Result<u8, TryFromIntError> {
        u8::try_from(self.as_mut().iter().map(|x| *x as u64).sum::<u64>() / self.as_mut().len() as u64)
    }
    
    /// Returns the mean of u8s as usize
    fn mut_usize_mean(&mut self) -> usize {
        self.as_mut().iter().map(|x| *x as usize).sum::<usize>() / (self.as_mut().len() as usize)
    }

    /// Returns the mode of u8s
    fn mut_u8_mode(&mut self)-> Option<u8> {
        let mut counts = BTreeMap::new();
        self.as_mut().iter().max_by_key(|&s| {
            let count = counts.entry(s).or_insert(0);
            *count += 1; *count}).copied()
    }
    /// Returns the mode of u8s as usize
    fn mut_usize_mode(&mut self) -> Option<usize> {
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
