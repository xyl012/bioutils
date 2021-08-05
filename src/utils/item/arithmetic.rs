use super::*;

pub trait ArithmeticSlice<T> {
    /// Returns the mean of u8s as u64
    fn mean_u8(&self) -> Result<u8, TryFromIntError>;

    /// Returns the mode of u8s
    fn mode_u8(&self) -> Option<u8>;
}

impl<T> ArithmeticSlice<T> for T where
T: AsRef<[u8]>
{

    /// Returns the mean of u8s as u8
    fn mean_u8(&self) -> Result<u8, TryFromIntError> {
        u8::try_from(self.as_ref().iter().map(|x| *x as u64).sum::<u64>() / self.as_ref().len() as u64)
    }

    /// Returns the mode of u8s
    fn mode_u8(&self)-> Option<u8> {
        let mut counts = BTreeMap::new();
        self.as_ref().iter().max_by_key(|&s| {
            let count = counts.entry(s).or_insert(0);
            *count += 1; *count}).copied()
    }
}

