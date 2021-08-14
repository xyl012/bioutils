
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
    /// Returns an Ordering if the percent of the slice is greater, equal, or less than the given value.
    fn percent_cmp(&self, value: &u8, ord: &Ordering) -> Result<Ordering>;
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
    /// Returns an Ordering if the mean of the slice is greater, equal, or less than the given value.
    fn percent_cmp(&self, value: &u8) -> Result<Ordering> {
        match self.percent_u8()?.cmp(value) {
            Ordering::Greater => Ok(Ordering::Greater),
            Ordering::Equal => Ok(Ordering::Equal),
            Ordering::Less => Ok(Ordering::Less),
            _ => bail!("Comparison error"),
        }
    }
}


pub trait PercentAsRefOrdSlice<T> {
    /// Returns a boolean if the total percent of elements above the cutoff u8 is above the supplied percent
    fn is_percent_ge(&self, cutoff_value: &u8, cutoff_percent: &usize) -> Result<bool>;
    /// Returns the total percent of elements above the cutoff
    fn percent_ge(&self, cutoff_value: &u8) -> Result<usize>;
}

impl<T> PercentAsRefOrdSlice<T> for T where
T: AsRef<[u8]>,
T: Ord,
{

    /// Get the total percent of elements above the cutoff u8 and return a boolean if total above supplied percent
    fn is_percent_ge(&self, cutoff_value: &u8, cutoff_percent: &usize) -> Result<bool> {
        if self.percent_ge(cutoff_value)? >= (*cutoff_percent) { //.into()
            Ok(true)
        } else { Ok(false) }
    }
    /// Returns the total percent of elements greater than or equal to the cutoff
    fn percent_ge(&self, cutoff_value: &u8) -> Result<usize> {
        percent_usize(usize::try_from(self.count_ge(cutoff_value))?, usize::try_from(self.as_ref().len())?)
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


// pub trait CountAsMutSlice<T> {
//     /// Returns the percent (0-100) of the quality u8 in bases (rounded) above the quality score supplied. Should be used when mapq scores are required.
//     fn mut_quality_percent_passing(&mut self, quality_score: &u8) -> usize;

//     /// Returns the number of iterators greater than criteria. Used for calculating percents/numerators
//     fn mut_count_greater_equal(&mut self, criteria:&u8)-> usize;

//     /// Returns the number of occurrences of the mode
//     fn mut_count_mode(&mut self) -> usize;

//     /// Returns the mean of u8s
//     fn mut_mean(&mut self) -> u64;

//     /// Returns the mode
//     fn mut_mode(&mut self) -> Option<&u8>;

//     /// Returns the count of a specific u8
//     fn mut_count_xu8(&mut self, x: &u8) -> usize;

// }

// impl<T> CountAsMutSlice<T> for T
// where
// T: AsMut<[u8]>,
// {
//     /// Checks each quality u8 and returns the percent above (passing) the given u8
//     fn mut_quality_percent_passing(&mut self, quality_score: &u8)-> usize {
//         percent_usize(self.mut_count_greater_equal(quality_score)?, self.as_mut().len())
//     }

//     /// Returns the number of iterations greater than the criteria
//     fn mut_count_greater_equal(&mut self, criteria: &u8)-> Result<usize> {
//         self.as_mut().iter().filter(|&s| s>=criteria).count()
//     }

//     /// Returns the number of occurrences of the mode
//     fn mut_count_mode(&mut self) -> usize {
//         let mode = *self.mut_mode().unwrap();
//         self.as_mut().iter().filter(|&q| q==&mode).count()
//     }

//     /// Returns the mean of u8s as u64 rounded
//     fn mut_mean(&mut self) -> u64 {
//         self.as_mut().iter().map(|x| *x as u64).sum::<u64>() / self.as_mut().len() as u64
//     }

//     /// Returns the mode of u8s
//     fn mut_mode(&mut self)-> Option<&u8> {
//         let mut counts = HashMap::new();
//         self.as_mut().iter().max_by_key(|&s| {
//             let count = counts.entry(s).or_insert(0);
//             *count += 1; *count})
//     }
    
//     /// Returns the count of a specific u8
//     fn mut_count_xu8(&mut self, x: &u8) -> usize {
//         self.as_mut().iter().filter(|&q| q==x).count()
//     }

// }

// /// Phred score (q) to probability (p): q = -10log10(p), p = 10 ^(-q/10)
// pub fn phred_to_prob(phred: &u8) -> f64 {-10f64 * (*phred as f64).log10()}
// /// Probability (p) to phred score (q): q = -10log10(p), p = 10 ^(-q/10)
// pub fn prob_to_phred(prob: &f64)-> u8 {(10f64.powf(-prob / 10f64)) as u8}

// // fn mean(numbers: &[u8]) -> Result<u64> {
// //     numbers.iter().map(|u| {u64::try_from(*u) }).sum::<Result<u64>>()// / numbers.len() as u64)
// // }

// // /// Calculate an alignment flag of user's option. This is just the sum of the flags, so we can add together the options "read paired" and "read mapped in proper pair"
// // pub fn alignment_flag(input_flags: Vec<&str>) -> u16 {
// //     input_flags.iter().map(|e| FLAGS_HASHMAP_U16.get(e).to_owned()).collect()?
// // }
