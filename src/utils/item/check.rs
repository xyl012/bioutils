use super::*;
use crate::utils::element::percent::*;
use crate::utils::element::tryfrom::*;
use crate::utils::item::count::*;
use crate::utils::item::arithmetic::*;

pub trait CheckAsRefSlice<T> {
    /// Checks if all elements in the slice are contained in a character set (bool).
    fn is_all_charset(&self, charset: BioUtilsCharSet) -> bool;
    /// Checks if all elements in the slice are contained in a character set (Ok if true, Err if false).
    fn result_is_all_charset(&self, charset: BioUtilsCharSet) -> Result<&Self>;
    /// Checks if all elements in the slice are contained in a character set (Some if true, None if false).
    fn option_is_all_charset(&self, charset: BioUtilsCharSet) -> Option<&Self>;
    /// Checks if the slice contains any from the character set (bool).
    fn has_charset(&self, charset: BioUtilsCharSet) -> bool;
    /// Checks if the slice contains any from the character set (Ok if true, Err if false).
    fn result_has_charset(&self, charset: BioUtilsCharSet) -> Result<&Self>;
    /// Checks if the slice contains any from the character set (Some if true, None if false).
    fn option_has_charset(&self, charset: BioUtilsCharSet) -> Option<&Self>;
}

impl<T> CheckAsRefSlice<T> for T where
T: AsRef<[u8]> 
{
    /// Checks if all elements in the slice are contained in a character set (bool).
    fn is_all_charset(&self, charset: BioUtilsCharSet) -> bool {
        self.as_ref().iter().all(|u| charset.value().contains(u))
    }
    /// Checks if all elements in the slice are contained in a character set (Ok if true, Err if false).
    fn result_is_all_charset(&self, charset: BioUtilsCharSet) -> Result<&Self> {
        match self.is_all_charset(charset) {
            true => Ok(self),
            false => bail!("Slice is not all charset"),
        }
    }
    /// Checks if all elements in the slice are contained in a character set (Some if true, None if false).
    fn option_is_all_charset(&self, charset: BioUtilsCharSet) -> Option<&Self> {
        match self.is_all_charset(charset) {
            true => Some(self),
            false => None,
        }
    }
    /// Checks if the slice contains any from the character set (bool).
    fn has_charset(&self, charset: BioUtilsCharSet) -> bool {
        self.as_ref().iter().any(|u| charset.value().contains(u))
    }
    /// Checks if the slice contains any from the character set (Ok if true, Err if false).
    fn result_has_charset(&self, charset: BioUtilsCharSet) -> Result<&Self> {
        match self.has_charset(charset) {
            true => Ok(self),
            false => bail!("Slice does not have the charset"),
        }
    }
    /// Checks if the slice contains any from the character set (Some if true, None if false).
    fn option_has_charset(&self, charset: BioUtilsCharSet) -> Option<&Self> {
        match self.has_charset(charset) {
            true => Some(self),
            false => None,
        }
    }
}

pub trait CheckAsMutSlice<T> {
    /// Checks if all elements in the slice are contained in a character set (bool).
    fn mut_is_all_charset(&mut self, charset: BioUtilsCharSet) -> bool;
    /// Checks if all elements in the slice are contained in a character set (Ok if true, Err if false).
    fn mut_result_is_all_charset(&mut self, charset: BioUtilsCharSet) -> Result<&mut Self>;
    /// Checks if all elements in the slice are contained in a character set (Some if true, None if false).
    fn mut_option_is_all_charset(&mut self, charset: BioUtilsCharSet) -> Option<&mut Self>;
    /// Checks if the slice contains any from the character set (bool).
    fn mut_has_charset(&mut self, charset: BioUtilsCharSet) -> bool;
    /// Checks if the slice contains any from the character set (Ok if true, Err if false).
    fn mut_result_has_charset(&mut self, charset: BioUtilsCharSet) -> Result<&mut Self>;
    /// Checks if the slice contains any from the character set (Some if true, None if false).
    fn mut_option_has_charset(&mut self, charset: BioUtilsCharSet) -> Option<&mut Self>;
}

impl<T> CheckAsMutSlice<T> for T where
T: AsMut<[u8]> 
{
    /// Checks if all elements in the slice are contained in a character set (bool).
    fn mut_is_all_charset(&mut self, charset: BioUtilsCharSet) -> bool {
        self.as_mut().iter().all(|u| charset.value().contains(u))
    }
    /// Checks if all elements in the slice are contained in a character set (Ok if true, Err if false).
    fn mut_result_is_all_charset(&mut self, charset: BioUtilsCharSet) -> Result<&mut Self> {
        match self.mut_is_all_charset(charset) {
            true => Ok(self),
            false => bail!("Slice is not all charset"),
        }
    }
    /// Checks if all elements in the slice are contained in a character set (Some if true, None if false).
    fn mut_option_is_all_charset(&mut self, charset: BioUtilsCharSet) -> Option<&mut Self> {
        match self.mut_is_all_charset(charset) {
            true => Some(self),
            false => None,
        }
    }
    /// Checks if the slice contains any from the character set (bool).
    fn mut_has_charset(&mut self, charset: BioUtilsCharSet) -> bool {
        self.as_mut().iter().any(|u| charset.value().contains(u))
    }
    /// Checks if the slice contains any from the character set (Ok if true, Err if false).
    fn mut_result_has_charset(&mut self, charset: BioUtilsCharSet) -> Result<&mut Self> {
        match self.mut_has_charset(charset) {
            true => Ok(self),
            false => bail!("Slice does not have the charset"),
        }
    }
    /// Checks if the slice contains any from the character set (Some if true, None if false).
    fn mut_option_has_charset(&mut self, charset: BioUtilsCharSet) -> Option<&mut Self> {
        match self.mut_has_charset(charset) {
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
        assert!(test.is_all_charset(charset: BioUtilsCharSet::Dna), true);
    }
}

pub trait CheckMeanAsRefSlice<T> {
    /// Returns a boolean if the mean of the slice is greater than the cutoff value. Common use case is filter for mean quaity score.
    fn is_mean_greater_equal(&self, cutoff_value: &u8) -> Result<bool>;
}

impl<T> CheckMeanAsRefSlice<T> for T where 
T:AsRef<[u8]>
{
    /// Returns a boolean if the mean of the slice is greater than the cutoff value. Common use case is filter for mean quaity score.
    fn is_mean_greater_equal(&self, cutoff_value: &u8) -> Result<bool> {
        PercentU8::try_from(cutoff_value)?;
        let mean = self.mean_u8()?;
        if mean >= *cutoff_value {
            Ok(true)
        } else { Ok(false) }
    }
}

pub trait CheckMeanAsMutSlice<T> {
    /// Returns a boolean if the mean of the slice is greater than the cutoff value. Common use case is filter for mean quaity score.
    fn mut_is_mean_greater_equal(&mut self, cutoff_value: &u8) -> Result<bool>;
}

impl<T> CheckMeanAsMutSlice<T> for T where 
T:AsMut<[u8]>
{
    /// Returns a boolean if the mean of the slice is greater than the cutoff value. Common use case is filter for mean quaity score.
    fn mut_is_mean_greater_equal(&mut self, cutoff_value: &u8) -> Result<bool> {
        PercentU8::try_from(cutoff_value)?;
        let mean = self.mut_mean_u8()?;
        if mean >= *cutoff_value {
            Ok(true)
        } else { Ok(false) }
    }
}

pub trait CheckPercentAsRefSlice<T> {
    /// Returns a boolean if the total percent of elements above the cutoff u8 is above the supplied percent
    fn is_percent_passing_greater_equal(&self, cutoff_value: &u8, cutoff_percent: &u8) -> Result<bool>;
    /// Returns the total percent of elements above the cutoff
    fn percent_passing(&self, cutoff_value: &u8) -> Result<usize>;
}

impl<T> CheckPercentAsRefSlice<T> for T where
T: AsRef<[u8]>
{
    /// Get the total percent of elements above the cutoff u8 and return a boolean if total above supplied percent
    fn is_percent_passing_greater_equal(&self, cutoff_value: &u8, cutoff_percent: &u8) -> Result<bool> {
        if self.percent_passing(cutoff_value)? >= (*cutoff_percent).into() {
            Ok(true)
        } else { Ok(false) }
    }
    /// Returns the total percent of elements above the cutoff
    fn percent_passing(&self, cutoff_value: &u8) -> Result<usize> {
        percent_usize(self.count_greater_equal(cutoff_value)?, self.as_ref().len())
    }
}

pub trait CheckPercentAsMutSlice<T> {
    /// Returns a boolean if the total percent of elements above the cutoff u8 is above the supplied percent
    fn mut_is_percent_passing_greater_equal(&mut self, cutoff_value: &u8, cutoff_percent: &u8) -> Result<bool>;
    /// Returns the total percent of elements above the cutoff
    fn mut_percent_passing(&mut self, cutoff_value: &u8) -> Result<usize>;
}

impl<T> CheckPercentAsMutSlice<T> for T where
T: AsMut<[u8]>
{

    /// Get the total percent of elements above the cutoff u8 and return a boolean if total above supplied percent
    fn mut_is_percent_passing_greater_equal(&mut self, cutoff_value: &u8, cutoff_percent: &u8) -> Result<bool> {
        if self.mut_percent_passing(cutoff_value)? >= (*cutoff_percent).into() {
            Ok(true)
        } else { Ok(false) }
    }
    /// Returns the total percent of elements above the cutoff
    fn mut_percent_passing(&mut self, cutoff_value: &u8) -> Result<usize> {
        percent_usize(self.mut_count_ge(cutoff_value), self.as_mut().len())
    }
}