use super::*;

#[derive(Debug, PartialEq, Hash)]
pub struct PercentU8(u8);
#[derive(Debug, PartialEq, Hash)]
pub struct PercentU64(u64);
#[derive(Debug, PartialEq, Hash)]
pub struct PercentUsize(usize);
#[derive(Debug, PartialEq, Hash)]
pub struct Phred33U8(u8);
#[derive(Debug, PartialEq, Hash)]
pub struct Phred64U8(u8);

impl TryFrom<u8> for PercentU8 {
    type Error = anyhow::Error;
    fn try_from(value: u8) -> Result<PercentU8> {
        if value >= PERCENT_MIN_U8 && value <= PERCENT_MAX_U8 {
            Ok(PercentU8(value))
        } else {bail!("Not a valid PERCENT")}
    }
}

impl<'a> TryFrom<&'a u8> for PercentU8 {
    type Error = anyhow::Error;
    fn try_from(value: &'a u8) -> Result<PercentU8> {
        if value >= &PERCENT_MIN_U8 && value <= &PERCENT_MAX_U8 {
            Ok(PercentU8(*value))
        } else {bail!("Not a valid PERCENT")}
    }
}

impl<'a> TryFrom<&'a mut u8> for PercentU8 {
    type Error = anyhow::Error;
    fn try_from(value: &'a mut u8) -> Result<PercentU8> {
        if value >= &mut PERCENT_MIN_U8 && value <= &mut PERCENT_MAX_U8 {
            Ok(PercentU8(*value))
        } else {bail!("Not a valid PERCENT")}
    }
}

impl TryFrom<u64> for PercentU64 {
    type Error = anyhow::Error;
    fn try_from(value: u64) -> Result<PercentU64> {
        if value >= PERCENT_MIN_U64 && value <= PERCENT_MAX_U64 {
            Ok(PercentU64(value))
        } else {bail!("Not a valid PERCENT")}
    }
}

impl<'a> TryFrom<&'a u64> for PercentU64 {
    type Error = anyhow::Error;
    fn try_from(value: &'a u64) -> Result<PercentU64> {
        if value >= &PERCENT_MIN_U64 && value <= &PERCENT_MAX_U64 {
            Ok(PercentU64(*value))
        } else {bail!("Not a valid PERCENT")}
    }
}

impl<'a> TryFrom<&'a mut u64> for PercentU64 {
    type Error = anyhow::Error;
    fn try_from(value: &'a mut u64) -> Result<PercentU64> {
        if value >= &mut PERCENT_MIN_U64 && value <= &mut PERCENT_MAX_U64 {
            Ok(PercentU64(*value))
        } else {bail!("Not a valid PERCENT")}
    }
}

impl TryFrom<usize> for PercentUsize {
    type Error = anyhow::Error;
    fn try_from(value: usize) -> Result<PercentUsize> {
        if value >= PERCENT_MIN_USIZE && value <= PERCENT_MAX_USIZE {
            Ok(PercentUsize(value))
        } else {bail!("Not a valid PERCENT")}
    }
}

impl<'a> TryFrom<&'a usize> for PercentUsize {
    type Error = anyhow::Error;
    fn try_from(value: &'a usize) -> Result<PercentUsize> {
        if value >= &PERCENT_MIN_USIZE && value <= &PERCENT_MAX_USIZE {
            Ok(PercentUsize(*value))
        } else {bail!("Not a valid PERCENT")}
    }
}

impl<'a> TryFrom<&'a mut usize> for PercentUsize {
    type Error = anyhow::Error;
    fn try_from(value: &'a mut usize) -> Result<PercentUsize> {
        if value >= &mut PERCENT_MIN_USIZE && value <= &mut PERCENT_MAX_USIZE {
            Ok(PercentUsize(*value))
        } else {bail!("Not a valid PERCENT")}
    }
}

impl TryFrom<u8> for Phred33U8 {
    type Error = anyhow::Error;
    fn try_from(value: u8) -> Result<Phred33U8> {
        if value >= PHRED33_MIN_U8 && value <= PHRED33_MAX_U8 {
            Ok(Phred33U8(value))
        } else {bail!("Not a valid PHRED33")}
    }
}

impl<'a> TryFrom<&'a u8> for Phred33U8 {
    type Error = anyhow::Error;
    fn try_from(value: &'a u8) -> Result<Phred33U8> {
        if value >= &PHRED33_MIN_U8 && value <= &PHRED33_MAX_U8 {
            Ok(Phred33U8(*value))
        } else {bail!("Not a valid PHRED33")}
    }
}

impl<'a> TryFrom<&'a mut u8> for Phred33U8 {
    type Error = anyhow::Error;
    fn try_from(value: &'a mut u8) -> Result<Phred33U8> {
        if value >= &mut PHRED33_MIN_U8 && value <= &mut PHRED33_MAX_U8 {
            Ok(Phred33U8(*value))
        } else {bail!("Not a valid PHRED33")}
    }
}

impl TryFrom<u8> for Phred64U8 {
    type Error = anyhow::Error;
    fn try_from(value: u8) -> Result<Phred64U8> {
        if value >= PHRED64_MIN_U8 && value <= PHRED64_MAX_U8 {
            Ok(Phred64U8(value))
        } else {bail!("Not a valid PHRED64")}
    }
}

impl<'a> TryFrom<&'a u8> for Phred64U8 {
    type Error = anyhow::Error;
    fn try_from(value: &'a u8) -> Result<Phred64U8> {
        if value >= &PHRED64_MIN_U8 && value <= &PHRED64_MAX_U8 {
            Ok(Phred64U8(*value))
        } else {bail!("Not a valid PHRED64")}
    }
}

impl<'a> TryFrom<&'a mut u8> for Phred64U8 {
    type Error = anyhow::Error;
    fn try_from(value: &'a mut u8) -> Result<Phred64U8> {
        if value >= &mut PHRED64_MIN_U8 && value <= &mut PHRED64_MAX_U8 {
            Ok(Phred64U8(*value))
        } else {bail!("Not a valid PHRED64")}
    }
}
