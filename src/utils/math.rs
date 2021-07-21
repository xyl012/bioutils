use super::*;


/// Calculates percentage with usizes
pub fn percentage(numerator: usize, denominator: usize) -> Result<usize> {
    Ok((100 * numerator + denominator / 2) / denominator)
}

pub trait MathU8<T> {

    /// Returns the mean of u8s as u64
    fn mean(&self) -> u64;

    /// Returns the mode of u8s
    fn mode(&self) -> Option<&u8>;
}

impl<T> MathU8<T> for T where
T: AsRef<[u8]>
{
    
    /// Returns the mean of u8s as u8 rounded
    fn mean(&self) -> Result<u8> {
        u8::try_from(self.as_ref().iter().map(|x| *x as u64).sum::<u64>() / self.as_ref().len() as u64)
    }

    /// Returns the mode of u8s
    fn mode(&self)-> Option<&u8> {
        let mut counts = BTreeMap::new();
        self.as_ref().iter().max_by_key(|&s| {
            let count = counts.entry(s).or_insert(0);
            *count += 1; *count})
    }
    
}