use super::*;

#[derive(Debug, PartialEq, Hash)]
pub struct Phred33U8(u8);
#[derive(Debug, PartialEq, Hash)]
pub struct Phred64U8(u8);

impl TryFrom<u8> for Phred33U8 {
    type Error = anyhow::Error;
    fn try_from(value: u8) -> Result<Phred33U8> {
        if PHRED33_U8.contains(&value) {
            Ok(Phred33U8(value))
        } else {bail!("Not a valid PHRED33")}
    }
}

impl<'a> TryFrom<&'a u8> for Phred33U8 {
    type Error = anyhow::Error;
    fn try_from(value: &'a u8) -> Result<Phred33U8> {
        if PHRED33_U8.contains(value) {
            Ok(Phred33U8(*value))
        } else {bail!("Not a valid PHRED33")}
    }
}

impl<'a> TryFrom<&'a mut u8> for Phred33U8 {
    type Error = anyhow::Error;
    fn try_from(value: &'a mut u8) -> Result<Phred33U8> {
        if PHRED33_U8.contains(value) {
            Ok(Phred33U8(*value))
        } else {bail!("Not a valid PHRED33")}
    }
}

impl TryFrom<u8> for Phred64U8 {
    type Error = anyhow::Error;
    fn try_from(value: u8) -> Result<Phred64U8> {
        if PHRED64_U8.contains(&value) {
            Ok(Phred64U8(value))
        } else {bail!("Not a valid PHRED64")}
    }
}

impl<'a> TryFrom<&'a u8> for Phred64U8 {
    type Error = anyhow::Error;
    fn try_from(value: &'a u8) -> Result<Phred64U8> {
        if PHRED64_U8.contains(value) {
            Ok(Phred64U8(*value))
        } else {bail!("Not a valid PHRED64")}
    }
}

impl<'a> TryFrom<&'a mut u8> for Phred64U8 {
    type Error = anyhow::Error;
    fn try_from(value: &'a mut u8) -> Result<Phred64U8> {
        if PHRED64_U8.contains(value) {
            Ok(Phred64U8(*value))
        } else {bail!("Not a valid PHRED64")}
    }
}