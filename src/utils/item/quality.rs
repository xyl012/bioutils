use super::*;
use crate::utils::element::percent::*;
use crate::utils::item::count::*;

pub trait QualityAsRefSlice<T> {
    /// Returns the percent (0-100) of the quality u8 in bases (rounded) above the quality score supplied. Should be used when mapq scores are required.
    fn quality_percent_passing(&self, quality_score: &u8) -> Result<usize>;
}

impl<T> QualityAsRefSlice<T> for T
where
T: AsRef<[u8]>,
{
    /// Checks each quality u8 and returns the percent above (passing) the given u8
    fn quality_percent_passing(&self, quality_score: &u8)-> Result<usize> {
        percent_usize(self.count_greater_than(quality_score)?, self.as_ref().len())
    }
}


pub trait QualityAsMutSlice<T> {
    /// Returns the percent (0-100) of the quality u8 in bases (rounded) above the quality score supplied. Should be used when mapq scores are required.
    fn mut_quality_percent_passing(&self, quality_score: &u8) -> Result<usize>;
}

impl<T> QualityAsMutSlice<T> for T
where
T: AsMut<[u8]>,
{
    /// Checks each quality u8 and returns the percent above (passing) the given u8
    fn mut_quality_percent_passing(&self, quality_score: &u8)-> Result<usize> {
        mut_percent_usize(self.mut_count_greater_than(quality_score)?, self.as_mut().len())
    }
}