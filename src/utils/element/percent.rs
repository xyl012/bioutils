use super::*;

#[derive(Debug, PartialEq, Hash)]
pub struct PercentU8(u8);

#[derive(Debug, PartialEq, Hash)]
pub struct PercentUsize(usize);

impl TryFrom<u8> for PercentU8 {
    type Error = anyhow::Error;
    fn try_from(value: u8) -> Result<PercentU8> {
        if PERCENTAGE_U8.contains(&value) {
            Ok(PercentU8(value))
        } else {bail!("Not a valid percentage")}
    }
}

impl<'a> TryFrom<&'a u8> for PercentU8 {
    type Error = anyhow::Error;
    fn try_from(value: &'a u8) -> Result<PercentU8> {
        if PERCENTAGE_U8.contains(value) {
            Ok(PercentU8(*value))
        } else {bail!("Not a valid percentage")}
    }
}

impl<'a> TryFrom<&'a mut u8> for PercentU8 {
    type Error = anyhow::Error;
    fn try_from(value: &'a mut u8) -> Result<PercentU8> {
        if PERCENTAGE_U8.contains(value) {
            Ok(PercentU8(*value))
        } else {bail!("Not a valid percentage")}
    }
}

impl TryFrom<usize> for PercentUsize {
    type Error = anyhow::Error;
    fn try_from(value: usize) -> Result<PercentUsize> {
        if PERCENTAGE_RANGE.contains(&value) {
            Ok(PercentUsize(value))
        } else {bail!("Not a valid percentage")}
    }
}
impl<'a> TryFrom<&'a usize> for PercentUsize {
    type Error = anyhow::Error;
    fn try_from(value: &'a usize) -> Result<PercentUsize> {
        if PERCENTAGE_RANGE.contains(value) {
            Ok(PercentUsize(*value))
        } else {bail!("Not a valid percentage")}
    }
}
impl<'a> TryFrom<&'a mut usize> for PercentUsize {
    type Error = anyhow::Error;
    fn try_from(value: &'a mut usize) -> Result<PercentUsize> {
        if PERCENTAGE_RANGE.contains(value) {
            Ok(PercentUsize(*value))
        } else {bail!("Not a valid percentage")}
    }
}

/// Calculates percentage with usizes
pub fn percent_usize(numerator: usize, denominator: usize) -> Result<usize> {
    Ok((100 * numerator + denominator / 2) / denominator)
}

/// Calculates percentage with u8
pub fn percent_u8(numerator: u8, denominator: u8) -> Result<u8> {
    Ok((100 * numerator + denominator / 2) / denominator)
}

/// Calculates percentage with usizes
pub fn mut_percent_usize(mut numerator: usize, mut denominator: usize) -> Result<usize> {
    Ok((100 * numerator + denominator / 2) / denominator)
}

/// Calculates percentage with u8
pub fn mut_percent_u8(mut numerator: u8, mut denominator: u8) -> Result<u8> {
    Ok((100 * numerator + denominator / 2) / denominator)
}