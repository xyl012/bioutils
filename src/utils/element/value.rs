
//! Functions that do not fall into the general categories of checking, mutating, or generating a new sequence. Generally includes statistics and transformations.
//! # Examples
//! ``
//! use crate::bioutils::get::value;
//! use crate::bioutils::utils::new::random_dna;
//! use rand::rngs::ThreadRng;
//! use std::string::String;
//! use rand::seq::SliceRandom;
//! 
//! let mut rng = rand::thread_rng(); //create a random number generator
//! let dna = random_dna(4,rng);
//! let distance = dna.hamming_distance(b"AAAA");
//! println!("{:?}", distance);
// ! ``

use super::*;

pub trait ValueU8<T> {
    /// Returns the percent (0-100) of the quality u8 in bases (rounded) above the quality score supplied. Should be used when mapq scores are required.
    fn quality_percent_passing(&self, quality_score: &u8) -> usize;

    /// Returns the number of iterators greater than criteria. Used for calculating percents/numerators
    fn count_greater_than(&self, criteria:&u8)-> usize;

    /// Returns the number of occurrences of the mode
    fn count_mode(&self) -> usize;

    /// Returns the count of a specific u8
    fn count_xu8(&self, x: &u8) -> usize;

}

impl<T> ValueU8<T> for T
where
    T: AsRef<[u8]>,
{
    /// Checks each quality u8 and returns the percent above (passing) the given u8
    fn quality_percent_passing(&self, quality_score: &u8)-> usize {
        percentage(self.count_greater_than(quality_score), self.as_ref().len())
    }

    /// Returns the number of iterations greater than the criteria
    fn count_greater_than(&self, criteria: &u8)-> usize {
        self.as_ref().iter().filter(|&s| s>=criteria).count()
    }

    /// Returns the number of occurrences of the mode
    fn count_mode(&self) -> usize {
        let mode = self.mode_u8().unwrap();
        self.as_ref().iter().filter(|&q| q==mode).count()
    }

    /// Returns the count of a specific u8
    fn count_xu8(&self, x: &u8) -> usize {
        self.as_ref().iter().filter(|&q| q==x).count()
    }

}

pub trait AsMutValueU8<T> {
    /// Returns the percent (0-100) of the quality u8 in bases (rounded) above the quality score supplied. Should be used when mapq scores are required.
    fn mut_quality_percent_passing(&mut self, quality_score: &u8) -> usize;

    /// Returns the number of iterators greater than criteria. Used for calculating percents/numerators
    fn mut_count_greater_than(&mut self, criteria:&u8)-> usize;

    /// Returns the number of occurrences of the mode
    fn mut_count_mode(&mut self) -> usize;

    /// Returns the mean of u8s
    fn mut_mean(&mut self) -> u64;

    /// Returns the mode
    fn mut_mode(&mut self) -> Option<&u8>;

    /// Returns the count of a specific u8
    fn mut_count_xu8(&mut self, x: &u8) -> usize;

}

impl<T> AsMutValueU8<T> for T
where
T: AsMut<[u8]>,
{
    /// Checks each quality u8 and returns the percent above (passing) the given u8
    fn mut_quality_percent_passing(&mut self, quality_score: &u8)-> usize {
        percentage(self.mut_count_greater_than(quality_score), self.as_mut().len())
    }

    /// Returns the number of iterations greater than the criteria
    fn mut_count_greater_than(&mut self, criteria: &u8)-> usize {
        self.as_mut().iter().filter(|&s| s>=criteria).count()
    }

    /// Returns the number of occurrences of the mode
    fn mut_count_mode(&mut self) -> usize {
        let mode = *self.mut_mode().unwrap();
        self.as_mut().iter().filter(|&q| q==&mode).count()
    }

    /// Returns the mean of u8s as u64 rounded
    fn mut_mean(&mut self) -> u64 {
        self.as_mut().iter().map(|x| *x as u64).sum::<u64>() / self.as_mut().len() as u64
    }

    /// Returns the mode of u8s
    fn mut_mode(&mut self)-> Option<&u8> {
        let mut counts = HashMap::new();
        self.as_mut().iter().max_by_key(|&s| {
            let count = counts.entry(s).or_insert(0);
            *count += 1; *count})
    }
    
    /// Returns the count of a specific u8
    fn mut_count_xu8(&mut self, x: &u8) -> usize {
        self.as_mut().iter().filter(|&q| q==x).count()
    }

}

/// Phred score (q) to probability (p): q = -10log10(p), p = 10 ^(-q/10)
pub fn phred_to_prob(phred: &u8) -> f64 {-10f64 * (*phred as f64).log10()}
/// Probability (p) to phred score (q): q = -10log10(p), p = 10 ^(-q/10)
pub fn prob_to_phred(prob: &f64)-> u8 {(10f64.powf(-prob / 10f64)) as u8}

// fn mean(numbers: &[u8]) -> Result<u64> {
//     numbers.iter().map(|u| {u64::try_from(*u) }).sum::<Result<u64>>()// / numbers.len() as u64)
// }

// /// Calculate an alignment flag of user's option. This is just the sum of the flags, so we can add together the options "read paired" and "read mapped in proper pair"
// pub fn alignment_flag(input_flags: Vec<&str>) -> u16 {
//     input_flags.iter().map(|e| FLAGS_HASHMAP_U16.get(e).to_owned()).collect()?
// }
