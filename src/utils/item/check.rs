use super::*;
use crate::utils::element::percent::*;
use crate::utils::element::quality::*;
use crate::utils::item::quality::*;
use crate::utils::charsets::*;

use crate::utils::item::count::*;

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

pub trait CheckAsMutSlice<T> {
    /// 
    fn mut_is_slice_all_charset(&mut self, charset: BioUtilsCharSet) -> bool;
    fn mut_result_slice_all_charset(&mut self, charset: BioUtilsCharSet) -> Result<&mut Self>;
    fn mut_option_slice_all_charset(&mut self, charset: BioUtilsCharSet) -> Option<&mut Self>;
    fn mut_slice_has_charset(&mut self, charset: BioUtilsCharSet) -> bool;
    fn mut_result_slice_has_charset(&mut self, charset: BioUtilsCharSet) -> Result<&mut Self>;
    fn mut_option_slice_has_charset(&mut self, charset: BioUtilsCharSet) -> Option<&mut Self>;
}

impl<T> CheckAsMutSlice<T> for T where
T: AsMut<[u8]> 
{
    fn mut_is_slice_all_charset(&mut self, charset: BioUtilsCharSet) -> bool {
        self.as_mut().iter().all(|u| charset.value().contains(u))
    }
    fn mut_result_slice_all_charset(&mut self, charset: BioUtilsCharSet) -> Result<&mut Self> {
        match self.mut_is_slice_all_charset(charset) {
            true => Ok(self),
            false => bail!("Slice is not all charset"),
        }
    }
    fn mut_option_slice_all_charset(&mut self, charset: BioUtilsCharSet) -> Option<&mut Self> {
        match self.mut_is_slice_all_charset(charset) {
            true => Some(self),
            false => None,
        }
    }
    fn mut_slice_has_charset(&mut self, charset: BioUtilsCharSet) -> bool {
        self.as_mut().iter().any(|u| charset.value().contains(u))
    }
    fn mut_result_slice_has_charset(&mut self, charset: BioUtilsCharSet) -> Result<&mut Self> {
        match self.mut_slice_has_charset(charset) {
            true => Ok(self),
            false => bail!("Slice does not have the charset"),
        }
    }
    fn mut_option_slice_has_charset(&mut self, charset: BioUtilsCharSet) -> Option<&mut Self> {
        match self.mut_slice_has_charset(charset) {
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

pub trait CheckPercentAsRefSlice<T> {
    /// Get the total percent of elements above the cutoff u8 and return a boolean if total above supplied percent
    fn is_slice_passing_percent(&self, cutoff_value: &u8, cutoff_percent: &u8) -> Result<bool>;
    /// Returns the total percent of elements above the cutoff
    fn slice_passing_percent(&self, cutoff_value: &u8) -> Result<usize>;
}

impl<T> CheckPercentAsRefSlice<T> for T where
T: AsRef<[u8]>
{
    /// Get the total percent of elements above the cutoff u8 and return a boolean if total above supplied percent
    fn is_slice_passing_percent(&self, cutoff_value: &u8, cutoff_percent: &u8) -> Result<bool> {
        PercentU8::try_from(cutoff_percent)?;
        if self.slice_passing_percent(cutoff_value)? >= (*cutoff_percent).into() {
            Ok(true)
        } else { Ok(false) }
    }
    /// Returns the total percent of elements above the cutoff
    fn slice_passing_percent(&self, cutoff_value: &u8) -> Result<usize> {
        percent_usize(self.count_greater_than(cutoff_value)?, self.as_ref().len())
    }
}

pub trait CheckPercentAsMutSlice<T> {
    /// Returns a boolean if the total percent of elements above the cutoff u8 is above the supplied percent
    fn mut_is_slice_passing_percent(&mut self, cutoff_value: &u8, cutoff_percent: &u8) -> Result<bool>;
    /// Returns the total percent of elements above the cutoff
    fn mut_slice_passing_percent(&mut self, cutoff_value: &u8) -> Result<usize>;
}

impl<T> CheckPercentAsMutSlice<T> for T where
T: AsMut<[u8]>
{

    /// Get the total percent of elements above the cutoff u8 and return a boolean if total above supplied percent
    fn mut_is_slice_passing_percent(&mut self, cutoff_value: &u8, cutoff_percent: &u8) -> Result<bool> {
        PercentU8::try_from(cutoff_percent)?;
        if self.mut_slice_passing_percent(cutoff_value)? >= (*cutoff_percent).into() {
            Ok(true)
        } else { Ok(false) }
    }
    /// Returns the total percent of elements above the cutoff
    fn mut_slice_passing_percent(&mut self, cutoff_value: &u8) -> Result<usize> {
        percent_usize(self.mut_count_greater_than(cutoff_value)?, self.as_mut().len())
    }
}