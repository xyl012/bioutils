
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

use crate::utils::item::arithmetic::*;
use crate::utils::element::percent::*;

pub trait CountAsRefSlice<T> {

    /// Returns the count of a specific u8
    fn count_u8(&self, x: &u8) -> Result<usize>;

    /// Returns the number of occurrences of the mode
    fn count_mode(&self) -> Option<u64>;

}

impl<T> CountAsRefSlice<T> for T
where
    T: AsRef<[u8]>,
{
    /// Returns the number of occurrences of the mode
    fn count_mode(&self) -> Option<u64> {
        let mut counts: BTreeMap<u8, u64> = BTreeMap::new();
        let mode = self.as_ref().iter().max_by_key(|&s| {
            let count = counts.entry(*s).or_insert(0);
            *count += 1; *count})?;
        counts.get(mode).copied()
    }

    /// Returns the count of a specific u8
    fn count_u8(&self, x: &u8) -> Result<usize> {
        Ok(self.as_ref().iter().filter(|&q| q==x).count())
    }
}

pub trait CountAsMutSlice<T> {

    /// Returns the count of a specific u8
    fn mut_count_u8(&mut self, x: &u8) -> Result<usize>;

    /// Returns the number of occurrences of the mode
    fn mut_count_mode(&mut self) -> Option<u64>;

}

impl<T> CountAsMutSlice<T> for T
where
    T: AsMut<[u8]>,
{
    /// Returns the number of occurrences of the mode
    fn mut_count_mode(&mut self) -> Option<u64> {
        let mut counts: BTreeMap<u8, u64> = BTreeMap::new();
        let mode = self.as_mut().iter().max_by_key(|&s| {
            let count = counts.entry(*s).or_insert(0);
            *count += 1; *count})?;
        counts.get(mode).copied()
    }

    /// Returns the count of a specific u8
    fn mut_count_u8(&mut self, x: &u8) -> Result<usize> {
        Ok(self.as_mut().iter().filter(|&q| q==x).count())
    }
}

pub trait CmpAsRefOrdSlice<T> {
    /// Returns the number of elements greater, equal, and less than the given value as an Ordering enum (std::cmp::Ordering).
    fn count_cmp(&self, value:&u8) -> BTreeMap<Ordering, u64>;
    /// Returns the number of elements greater or equal to the given value.
    fn count_ge(&self, value:&u8) -> u64;
    /// Returns the number of elements less than or equal to the given value.
    fn count_le(&self, value:&u8) -> u64;
    /// Returns an Ordering if the mean of the slice is greater, equal, or less than the given value.
    fn mean_cmp(&self, value: &u8) -> Result<Ordering>;
}

impl<T> CmpAsRefOrdSlice<T> for T where
T: AsRef<[u8]>,
T: Ord,
{
    /// Returns the number of elements greater, equal, and less than the given value. Used for calculating percents/numerators
    fn count_cmp(&self, value: &u8) -> BTreeMap<Ordering, u64> {
        let mut btree: BTreeMap<Ordering, u64> = BTreeMap::new();
        self.as_ref().iter().for_each(|u| match u.cmp(value) {
            Ordering::Less => {let count = btree.entry(Ordering::Less).or_insert(0); *count +=1},
            Ordering::Equal => {let count = btree.entry(Ordering::Equal).or_insert(0); *count +=1},
            Ordering::Greater => {let count = btree.entry(Ordering::Greater).or_insert(0); *count +=1},
        });
        btree
    }
    /// Returns the number of elements greater or equal to the given value.
    fn count_ge(&self, value:&u8) -> u64 {
        let mut count = 0u64;
        self.as_ref().iter().for_each(|u| if u.cmp(value).is_ge() {count += 1});
        count
    }
    /// Returns the number of elements less than or equal to the given value.
    fn count_le(&self, value:&u8) -> u64 {
        let mut count = 0u64;
        self.as_ref().iter().for_each(|u| if u.cmp(value).is_le() {count += 1});
        count
    }
    /// Returns an Ordering if the mean of the slice is greater, equal, or less than the given value.
    fn mean_cmp(&self, value: &u8) -> Result<Ordering> {
        match self.mean_u8()?.cmp(value) {
            Ordering::Greater => Ok(Ordering::Greater),
            Ordering::Equal => Ok(Ordering::Equal),
            Ordering::Less => Ok(Ordering::Less),
            _ => bail!("Comparison error"),
        }
    }
}

pub trait PercentAsRefOrdSlice<T> {
    /// Counts the elements greater, equal, and less than the value and returns a btree where the values are a percentage of the length
    fn percent_cmp(&self, value: &u8) -> Result<BTreeMap<Ordering, u64>>;
    /// Returns a boolean if the total percent of elements above the cutoff u8 is above the supplied percent
    fn is_percent_ge(&self, cutoff_value: &u8, cutoff_percent: &usize) -> Result<bool>;
    /// Returns the total percent of elements above the cutoff
    fn percent_ge(&self, cutoff_value: &u8) -> Result<usize>;
    /// Returns a boolean if the total percent of elements above the cutoff u8 is above the supplied percent
    fn is_percent_le(&self, cutoff_value: &u8, cutoff_percent: &usize) -> Result<bool>;
    /// Returns the total percent of elements above the cutoff
    fn percent_le(&self, cutoff_value: &u8) -> Result<usize>;
}

impl<T> PercentAsRefOrdSlice<T> for T where
T: AsRef<[u8]>,
T: Ord,
{
    fn percent_cmp(&self, value: &u8) -> Result<BTreeMap<Ordering, u64>> {
        let mut percent_btree: BTreeMap<Ordering, u64> = BTreeMap::new();
        let length = u64::try_from(self.as_ref().len())?;
        for (k, v) in &self.count_cmp(value) {
            percent_btree.insert(*k, v.percent_u64(&length)?);
        };
        Ok(percent_btree)
    }

    /// Get the total percent of elements above the cutoff u8 and return a boolean if total above supplied percent
    fn is_percent_ge(&self, cutoff_value: &u8, cutoff_percent: &usize) -> Result<bool> {
        if self.percent_ge(cutoff_value)? >= (*cutoff_percent) { //.into()
            Ok(true)
        } else { Ok(false) }
    }

    /// Returns the total percent of elements greater than or equal to the cutoff
    fn percent_ge(&self, cutoff_value: &u8) -> Result<usize> {
        percent_usize(&usize::try_from(self.count_ge(cutoff_value))?, &usize::try_from(self.as_ref().len())?)
    }
    /// Get the total percent of elements above the cutoff u8 and return a boolean if total above supplied percent
    fn is_percent_le(&self, cutoff_value: &u8, cutoff_percent: &usize) -> Result<bool> {
        if self.percent_le(cutoff_value)? <= (*cutoff_percent) { //.into()
            Ok(true)
        } else { Ok(false) }
    }

    /// Returns the total percent of elements greater than or equal to the cutoff
    fn percent_le(&self, cutoff_value: &u8) -> Result<usize> {
        percent_usize(&usize::try_from(self.count_le(cutoff_value))?, &usize::try_from(self.as_ref().len())?)
    }
}

pub trait CmpAsMutOrdSlice<T> {
    /// Returns the number of elements greater, equal, and less than the given value. Used for calculating percents/numerators
    fn mut_count_cmp(&mut self, value:&u8) -> BTreeMap<Ordering, u64>;
    /// Returns the number of elements greater or equal to the given value.
    fn mut_count_ge(&mut self, value:&u8) -> u64;
    /// Returns the number of elements less than or equal to the given value.
    fn mut_count_le(&mut self, value:&u8) -> u64;
}

impl<T> CmpAsMutOrdSlice<T> for T where
T: AsMut<[u8]>,
T: Ord,
{
    /// Returns the number of elements greater, equal, and less than the given value. Used for calculating percents/numerators
    fn mut_count_cmp(&mut self, value: &u8) -> BTreeMap<Ordering, u64> {
        let mut btree: BTreeMap<Ordering, u64> = BTreeMap::new();
        self.as_mut().iter().for_each(|u| match u.cmp(value) {
            Ordering::Less => {let count = btree.entry(Ordering::Less).or_insert(0); *count +=1},
            Ordering::Equal => {let count = btree.entry(Ordering::Equal).or_insert(0); *count +=1},
            Ordering::Greater => {let count = btree.entry(Ordering::Greater).or_insert(0); *count +=1},
        });
        btree
    }
    /// Returns the number of elements greater or equal to the given value.
    fn mut_count_ge(&mut self, value:&u8) -> u64 {
        let mut count = 0u64;
        self.as_mut().iter().for_each(|u| if u.cmp(value).is_ge() {count += 1});
        count
    }
    /// Returns the number of elements less than or equal to the given value.
    fn mut_count_le(&mut self, value:&u8) -> u64 {
        let mut count = 0u64;
        self.as_mut().iter().for_each(|u| if u.cmp(value).is_le() {count += 1});
        count
    }
}

/// Phred score (q) to probability (p): q = -10log10(p), p = 10 ^(-q/10)
pub fn phred_to_prob(phred: &u8) -> f64 {-10f64 * (*phred as f64).log10()}
/// Probability (p) to phred score (q): q = -10log10(p), p = 10 ^(-q/10)
pub fn prob_to_phred(prob: &f64)-> u8 {(10f64.powf(-prob / 10f64)) as u8}

// /// Calculate an alignment flag of user's option. This is just the sum of the flags, so we can add together the options "read paired" and "read mapped in proper pair"
// pub fn alignment_flag(input_flags: Vec<&str>) -> u16 {
//     input_flags.iter().map(|e| FLAG_HASHMAP.get(e).to_owned()).collect()?
// }
