// use super::*;
// use crate::utils::element::percent::*;
// use crate::utils::item::count::*;
// use crate::utils::item::arithmetic::*;

// pub trait QualityAsRefSlice<T> {
//     /// Get the total percent of elements above the cutoff u8 and return a boolean if total above supplied percent
//     fn is_quality_slice_passing_percent(&self, cutoff_value: &u8, cutoff_percent: &u8) -> Result<bool>;
//     // /// Checks the slice mean is greater than or equal to the given value.
//     // fn is_quality_slice_passing_score_mean(&self, cutoff_value: &u8) -> Result<bool>;
//     // /// Check if valid phred and calculate the mean of a quality slice.
//     // fn quality_slice_mean(&self) -> Result<u8>;
//     // /// Checks the encoded sequence has a quality score above greater than or equal to the supplied mean. Decodes from raw read from fastq file with phred64 encoding. Commonly done per base in fastqc.
//     // fn is_qual_passing_mean_phred64(&self, mean_quality_score: &u8) -> Result<bool>;
// }

// impl<T> QualityAsRefSlice<T> for T
// where
// T: AsRef<[u8]>,
// {
//     /// Get the total percent of elements above the cutoff u8 and return a boolean if total above supplied percent
//     fn is_quality_slice_passing_percent(&self, cutoff_value: &u8, cutoff_percent: &u8) -> Result<bool> {
//         //TODO if encoding ==, try from, then use is percent passing
//         Phred33U8::try_from(cutoff_value)?;
//         self.is_slice_passing_percent(cutoff_value, cutoff_percent)
//     }
//     // /// Get the total percent of elements above the cutoff u8 and return a boolean if total above supplied percent
//     // fn is_quality_slice_passing_score_percent(&self, cutoff_value: &u8, cutoff_percent: &u8) -> Result<bool> {
//     //     ::try_from(cutoff_value)?;
//     //     self.is_passing_percent(cutoff_value, cutoff_percent)
//     // }

//     // /// Checks the slice mean is greater than or equal to the given value.
//     // fn is_quality_slice_passing_mean(&self, cutoff_value: &u8) -> Result<bool> {
//     //     self.mean_u8();
//     //     Ok(true)
//     // }

//     // /// Checks the encoded sequence has a quality score above greater than or equal to the supplied mean. Decodes from raw read from fastq file with phred33 encoding. Commonly done per base in fastqc.
//     // fn is_quality_slice_passing_mean(&self, mean_quality_score: &u8) -> Result<bool> {
//     //     Phred33U8::try_from(mean_quality_score)?;
//     //         if self.decode_qual()?.mean_u8()? >= (*mean_quality_score).into() {
//     //             Ok(true)
//     //         } else { Ok(false) }
//     // }

//     // /// Checks the encoded sequence has a quality score above greater than or equal to the supplied mean. Decodes from raw read from fastq file with phred64 encoding. Commonly done per base in fastqc.
//     // fn is_qual_passing_mean_phred64(&self, mean_quality_score: &u8) -> Result<bool> {
//     //     Phred33U8::try_from(mean_quality_score)?;
//     //     if self.as_ref().decode_qual_phred64().mean()? >= (*mean_quality_score).into() {
//     //         Ok(true)
//     //     } else { Ok(false) }
//     // }

//     // /// Checks if the sequence is a homopolymer with percentage cutoff
//     // fn is_percent_homopolymer(&self, percent: &u8) -> Result<bool> {
//     //     PercentU8::try_from(percent)?;
//     //     if percentage_u8(self.count_mode(), self.as_ref().len())? >= (*percent).into() {
//     //         Ok(true)
//     //     } else {Ok(false)}
//     // }

//     // /// Checks if the sequence is comprised of 'x' greater than 'percent' cutoff. Primary use is for filtering for reads with >90% percent N's or A's
//     // fn is_percent_homopolymer_x(&self, x: &u8, percent: &u8) -> Result<bool> {
//     //     PercentU8::try_from(percent)?;
//     //         if percentage_u8(self.count_xu8(x), self.as_ref().len())? >= (*percent).into() {
//     //             Ok(true)
//     //         } else {Ok(false)}
//     // }
// }

// pub trait QualityAsMutSlice<T> {
//     /// Get the total percent of elements above the cutoff u8 and return a boolean if total above supplied percent
//     fn mut_is_quality_slice_passing_percent(&mut self, cutoff_value: &u8, cutoff_percent: &u8) -> Result<bool>;

//     // /// Checks the sequence has the percent bases (rounded) greater than or equal to the quality score
//     // fn mut_is_quality_slice_passing_percent(&self, quality_score: &u8, percent: &u8) -> Result<bool>;
//     // /// Returns the percent (0-100) of the quality u8 in bases (rounded) above the quality score supplied. Should be used when mapq scores are required.
//     // fn mut_quality_slice_passing_percent(&self, quality_score: &u8) -> Result<usize>;
// }

// impl<T> QualityAsMutSlice<T> for T
// where
// T: AsMut<[u8]>,
// {
//     /// Checks the sequence has a number of bases (percent rounded) greater than or equal to the supplied quality score
//     fn mut_is_quality_slice_passing_percent(&mut self, cutoff_value: &u8, cutoff_percent: &u8) -> Result<bool> {
//         Phred33U8::try_from(cutoff_value)?;
//         self.mut_is_passing_percent(cutoff_value, cutoff_percent)
//     }
//     // /// Checks each quality u8 and returns the percent greater than or equal to (passing) the given u8
//     // fn mut_quality_slice_passing_percent(&self, quality_score: &u8)-> Result<usize> {
//     //     Phred33U8::try_from(quality_score)?;
//     //     mut_percent_usize(self.mut_count_greater_than(quality_score)?, self.as_mut().len())
//     // }
// }