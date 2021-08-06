use super::*;

use super::tryfrom::*;

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
