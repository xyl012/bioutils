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
        if (PERCENT_MIN_U8..=PERCENT_MAX_U8).contains(&value) {
            Ok(PercentU8(value))
        } else {bail!("Not a valid PERCENT")}
    }
}

impl<'a> TryFrom<&'a u8> for PercentU8 {
    type Error = anyhow::Error;
    fn try_from(value: &'a u8) -> Result<PercentU8> {
        if (&PERCENT_MIN_U8..=&PERCENT_MAX_U8).contains(&value) {
            Ok(PercentU8(*value))
        } else {bail!("Not a valid PERCENT")}
    }
}

impl<'a> TryFrom<&'a mut u8> for PercentU8 {
    type Error = anyhow::Error;
    fn try_from(value: &'a mut u8) -> Result<PercentU8> {
        if (&mut PERCENT_MIN_U8..=&mut PERCENT_MAX_U8).contains(&value) {
            Ok(PercentU8(*value))
        } else {bail!("Not a valid PERCENT")}
    }
}

impl TryFrom<u64> for PercentU64 {
    type Error = anyhow::Error;
    fn try_from(value: u64) -> Result<PercentU64> {
        if (PERCENT_MIN_U64..=PERCENT_MAX_U64).contains(&value) {
            Ok(PercentU64(value))
        } else {bail!("Not a valid PERCENT")}
    }
}

impl<'a> TryFrom<&'a u64> for PercentU64 {
    type Error = anyhow::Error;
    fn try_from(value: &'a u64) -> Result<PercentU64> {
        if (&PERCENT_MIN_U64..=&PERCENT_MAX_U64).contains(&value) {
            Ok(PercentU64(*value))
        } else {bail!("Not a valid PERCENT")}
    }
}

impl<'a> TryFrom<&'a mut u64> for PercentU64 {
    type Error = anyhow::Error;
    fn try_from(value: &'a mut u64) -> Result<PercentU64> {
        if (&mut PERCENT_MIN_U64..=&mut PERCENT_MAX_U64).contains(&value) {
            Ok(PercentU64(*value))
        } else {bail!("Not a valid PERCENT")}
    }
}

impl TryFrom<usize> for PercentUsize {
    type Error = anyhow::Error;
    fn try_from(value: usize) -> Result<PercentUsize> {
        if (PERCENT_MIN_USIZE..=PERCENT_MAX_USIZE).contains(&value) {
            Ok(PercentUsize(value))
        } else {bail!("Not a valid PERCENT")}
    }
}

impl<'a> TryFrom<&'a usize> for PercentUsize {
    type Error = anyhow::Error;
    fn try_from(value: &'a usize) -> Result<PercentUsize> {
        if (&PERCENT_MIN_USIZE..=&PERCENT_MAX_USIZE).contains(&value) {
            Ok(PercentUsize(*value))
        } else {bail!("Not a valid PERCENT")}
    }
}

impl<'a> TryFrom<&'a mut usize> for PercentUsize {
    type Error = anyhow::Error;
    fn try_from(value: &'a mut usize) -> Result<PercentUsize> {
        if (&mut PERCENT_MIN_USIZE..=&mut PERCENT_MAX_USIZE).contains(&value) {
            Ok(PercentUsize(*value))
        } else {bail!("Not a valid PERCENT")}
    }
}

impl TryFrom<u8> for Phred33U8 {
    type Error = anyhow::Error;
    fn try_from(value: u8) -> Result<Phred33U8> {
        if (PHRED33_MIN_U8..=PHRED33_MAX_U8).contains(&value) {
            Ok(Phred33U8(value))
        } else {bail!("Not a valid PHRED33")}
    }
}

impl<'a> TryFrom<&'a u8> for Phred33U8 {
    type Error = anyhow::Error;
    fn try_from(value: &'a u8) -> Result<Phred33U8> {
        if (&PHRED33_MIN_U8..=&PHRED33_MAX_U8).contains(&value) {
            Ok(Phred33U8(*value))
        } else {bail!("Not a valid PHRED33")}
    }
}

impl<'a> TryFrom<&'a mut u8> for Phred33U8 {
    type Error = anyhow::Error;
    fn try_from(value: &'a mut u8) -> Result<Phred33U8> {
        if (&mut PHRED33_MIN_U8..=&mut PHRED33_MAX_U8).contains(&value) {
            Ok(Phred33U8(*value))
        } else {bail!("Not a valid PHRED33")}
    }
}

impl TryFrom<u8> for Phred64U8 {
    type Error = anyhow::Error;
    fn try_from(value: u8) -> Result<Phred64U8> {
        if (PHRED64_MIN_U8..=PHRED64_MAX_U8).contains(&value) {
            Ok(Phred64U8(value))
        } else {bail!("Not a valid PHRED64")}
    }
}

impl<'a> TryFrom<&'a u8> for Phred64U8 {
    type Error = anyhow::Error;
    fn try_from(value: &'a u8) -> Result<Phred64U8> {
        if (&PHRED64_MIN_U8..=&PHRED64_MAX_U8).contains(&value) {
            Ok(Phred64U8(*value))
        } else {bail!("Not a valid PHRED64")}
    }
}

impl<'a> TryFrom<&'a mut u8> for Phred64U8 {
    type Error = anyhow::Error;
    fn try_from(value: &'a mut u8) -> Result<Phred64U8> {
        if (&mut PHRED64_MIN_U8..=&mut PHRED64_MAX_U8).contains(&value) {
            Ok(Phred64U8(*value))
        } else {bail!("Not a valid PHRED64")}
    }
}
