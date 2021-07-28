use super::*;
use crate::utils::element::percent::*;
use crate::utils::element::quality::*;
use crate::utils::item::quality::*;
use crate::utils::charsets::*;

pub trait CheckAsRefSlice<T> {
    /// 
    fn is_slice_all_charset(&self, charset: BioUtilsCharSet) -> bool;
    fn result_slice_all_charset(&self, charset: BioUtilsCharSet) -> Result<&Self>;
    fn option_slice_all_charset(&self, charset: BioUtilsCharSet) -> Option<&Self>;
    fn slice_has_charset(&self, charset: BioUtilsCharSet) -> bool;
    fn result_slice_has_charset(&self, charset: BioUtilsCharSet) -> Result<&Self>;
    fn option_slice_has_charset(&self, charset: BioUtilsCharSet) -> Option<&Self>;
}

impl<T> CheckAsRefSlice<T> for T where
T: AsRef<[u8]> 
{
    fn is_slice_all_charset(&self, charset: BioUtilsCharSet) -> bool {
        self.as_ref().iter().all(|u| charset.value().contains(u))
    }
    fn result_slice_all_charset(&self, charset: BioUtilsCharSet) -> Result<&Self> {
        match self.is_slice_all_charset(charset) {
            true => Ok(self),
            false => bail!("Slice is not all charset"),
        }
    }
    fn option_slice_all_charset(&self, charset: BioUtilsCharSet) -> Option<&Self> {
        match self.is_slice_all_charset(charset) {
            true => Some(self),
            false => None,
        }
    }
    fn slice_has_charset(&self, charset: BioUtilsCharSet) -> bool {
        self.as_ref().iter().any(|u| charset.value().contains(u))
    }
    fn result_slice_has_charset(&self, charset: BioUtilsCharSet) -> Result<&Self> {
        match self.slice_has_charset(charset) {
            true => Ok(self),
            false => bail!("Slice does not have the charset"),
        }
    }
    fn option_slice_has_charset(&self, charset: BioUtilsCharSet) -> Option<&Self> {
        match self.slice_has_charset(charset) {
            true => Some(self),
            false => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::item::check::*;
    use bioutils::utils::item::check::CheckAsRefSlice;
    #[test]
    fn checking_slice() {
        let test = &[67,67,67,67];
        assert!(test.is_slice_all_charset(charset: BioUtilsCharSet::Dna), true);
    }
}