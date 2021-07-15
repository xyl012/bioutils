use super::*;

pub mod ascii;
pub mod iupac;
pub mod quality;

// pub const PERCENTAGE_RANGE: std::ops::Range<u8> = 0..100;
use std::ops::RangeInclusive;
pub const PERCENTAGE_RANGE_START: usize = 0;
pub const PERCENTAGE_RANGE_END: usize = 100;
pub const PERCENTAGE_RANGE: RangeInclusive<usize> = PERCENTAGE_RANGE_START..=PERCENTAGE_RANGE_END;

use std::collections::{HashSet,HashMap};
use std::iter::FromIterator;

pub fn new_str_hashset<'a>(array: &'a [&str]) -> HashSet<&'a str> {
    HashSet::from_iter(array.iter().cloned())
}

pub fn new_u8_hashset(array: &[u8]) -> HashSet<u8> {
    HashSet::from_iter(array.iter().cloned())
}
