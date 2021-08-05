use super::*;

#[derive(Debug, PartialEq, Hash)]
pub struct PercentU8(u8);

#[derive(Debug, PartialEq, Hash)]
pub struct PercentUsize(usize);

impl TryFrom<u8> for PercentU8 {
    type Error = anyhow::Error;
    fn try_from(value: u8) -> Result<PercentU8> {
        if PERCENT.contains(&value) {
            Ok(PercentU8(value))
        } else {bail!("Not a valid PERCENT")}
    }
}

impl<'a> TryFrom<&'a u8> for PercentU8 {
    type Error = anyhow::Error;
    fn try_from(value: &'a u8) -> Result<PercentU8> {
        if PERCENT.contains(value) {
            Ok(PercentU8(*value))
        } else {bail!("Not a valid PERCENT")}
    }
}

impl<'a> TryFrom<&'a mut u8> for PercentU8 {
    type Error = anyhow::Error;
    fn try_from(value: &'a mut u8) -> Result<PercentU8> {
        if PERCENT.contains(value) {
            Ok(PercentU8(*value))
        } else {bail!("Not a valid PERCENT")}
    }
}

impl TryFrom<usize> for PercentUsize {
    type Error = anyhow::Error;
    fn try_from(value: usize) -> Result<PercentUsize> {
        if PERCENT_RANGE.contains(&value) {
            Ok(PercentUsize(value))
        } else {bail!("Not a valid PERCENT")}
    }
}
impl<'a> TryFrom<&'a usize> for PercentUsize {
    type Error = anyhow::Error;
    fn try_from(value: &'a usize) -> Result<PercentUsize> {
        if PERCENT_RANGE.contains(value) {
            Ok(PercentUsize(*value))
        } else {bail!("Not a valid PERCENT")}
    }
}
impl<'a> TryFrom<&'a mut usize> for PercentUsize {
    type Error = anyhow::Error;
    fn try_from(value: &'a mut usize) -> Result<PercentUsize> {
        if PERCENT_RANGE.contains(value) {
            Ok(PercentUsize(*value))
        } else {bail!("Not a valid PERCENT")}
    }
}

/// Checks PERCENT with usizes
pub fn percent_usize(numerator: usize, denominator: usize) -> Result<usize> {
    let percent = (100 * numerator + denominator / 2) / denominator;
    match PercentUsize::try_from(percent) {
        Ok(_) => Ok(percent),
        Err(_) => bail!("Not valid percent"),
    }
}

/// Checks PERCENT with u8
pub fn percent_u8(numerator: u8, denominator: u8) -> Result<u8> {
    let percent = (100 * numerator + denominator / 2) / denominator;
    match PercentU8::try_from(percent) {
        Ok(_) => Ok(percent),
        Err(_) => bail!("Not valid percent"),
    }
}
