pub mod iupac;
pub mod quality;

use std::collections::HashSet;
use std::iter::FromIterator;

pub fn new_str_hashset<'a>(array: &'a [&str]) -> HashSet<&'a str> {
    HashSet::from_iter(array.iter().cloned())
}

pub fn new_u8_hashset(array: &[u8]) -> HashSet<u8> {
    HashSet::from_iter(array.iter().cloned())
}
