use super::*;

use crate::utils::item::count::*;
use crate::utils::element::percent::*;

pub trait CheckPercentAsRefSlice<T> {
    /// Checks if T is comprised of valid percentages
    fn check_slice_percentages(&self) -> Result<&Self>;
    /// Returns a boolean if T is comprised of valid percentages
    fn is_slice_percentages(&self) -> bool;
    /// Get the total percent of elements above the cutoff u8 and return a boolean if total above supplied percent
    fn is_slice_passing_percent(&self, cutoff_value: &u8, cutoff_percent: &u8) -> Result<bool>;
    /// Returns the total percent of elements above the cutoff
    fn slice_passing_percent(&self, cutoff_value: &u8) -> Result<usize>;
}

impl<T> CheckPercentAsRefSlice<T> for T where
T: AsRef<[u8]>
{
    /// Checks if T is comprised of valid percentages
    fn check_slice_percentages(&self) -> Result<&Self> {
        match self.is_slice_percentages() {
            true => Ok(self),
            false => bail!("Contains non-percentage elements"),
        }
    }
    /// Returns a boolean if T is comprised of valid percentages
    fn is_slice_percentages(&self) -> bool {
        self.as_ref().iter().all(|x| PERCENTAGE.contains(&x))
    }
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
    /// Checks if T is comprised of valid percentages
    fn mut_check_slice_percentages(&mut self) -> Result<&mut Self>;
    /// Returns a boolean if T is comprised of valid percentages
    fn mut_is_slice_percentages(&mut self) -> bool;
    /// Returns a boolean if the total percent of elements above the cutoff u8 is above the supplied percent
    fn mut_is_slice_passing_percent(&mut self, cutoff_value: &u8, cutoff_percent: &u8) -> Result<bool>;
    /// Returns the total percent of elements above the cutoff
    fn mut_slice_passing_percent(&mut self, cutoff_value: &u8) -> Result<usize>;

}

impl<T> CheckPercentAsMutSlice<T> for T where
T: AsMut<[u8]>
{
    /// Checks if T is comprised of valid percentages
    fn mut_check_slice_percentages(&mut self) -> Result<&mut Self> {
        match self.mut_is_slice_percentages() {
            true => Ok(self),
            false => bail!("Contains non-percentage elements"),
        }
    }
    /// Returns a boolean if T is comprised of valid percentages
    fn mut_is_slice_percentages(&mut self) -> bool {
        self.as_mut().iter().all(|x| PERCENTAGE.contains(&x))
    }
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