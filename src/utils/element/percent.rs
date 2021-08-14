use super::*;
use super::tryfrom::*;

pub trait PercentU8<T> {
    /// Returns the percent of u8s as u8
    fn percent_u8(&self) -> Result<u8>;
}

// /// Returns the percent of u8s as usize
// fn percent_usize(&self) -> Result<usize>;

impl<T> PercentU8<T> for T where
T: AsRef<[u8]>,
{
    /// Returns the percent of u8s as u8
    fn percent_u8(&self) -> Result<u8> {
        
    }
}

/// Calculates a percent with u8 numerator and denominator
pub fn percent_u8(numerator: u8, denominator: u8) -> Result<u8> {
    let percent = (100 * numerator + denominator / 2) / denominator;
    match PercentU8::try_from(percent) {
        Ok(_) => Ok(percent),
        Err(_) => bail!("Not valid percent"),
    }
}

/// Calculates a percent with usize numerator and denominator
pub fn percent_usize(numerator: usize, denominator: usize) -> Result<usize> {
    let percent = (100 * numerator + denominator / 2) / denominator;
    match PercentUsize::try_from(percent) {
        Ok(_) => Ok(percent),
        Err(_) => bail!("Not valid percent"),
    }
}

/// Calculates a percent with u64 numerator and denominator
pub fn percent_u64(numerator: u64, denominator: u64) -> Result<u64> {
    let percent = (100 * numerator + denominator / 2) / denominator;
    match PercentU64::try_from(percent) {
        Ok(_) => Ok(percent),
        Err(_) => bail!("Not valid percent"),
    }
}

