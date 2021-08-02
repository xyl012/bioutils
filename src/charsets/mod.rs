use super::*;

pub mod ascii;
pub mod iupac;
pub mod quality;
pub mod percent;
pub mod flags;
pub mod bioutils;

pub fn new_str_hashset<'a>(array: &'a [&str]) -> HashSet<&'a str> {
    HashSet::from_iter(array.iter().cloned())
}

pub fn new_hashset(array: &[u8]) -> HashSet<u8> {
    HashSet::from_iter(array.iter().cloned())
}
