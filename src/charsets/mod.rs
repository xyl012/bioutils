use super::*;

pub mod ascii;
pub mod iupac;
pub mod quality;

// enum CharSetArray {
//     Phred33U8,
//     Phred64U8,
// }

// impl CharSetArray {
//     fn value(&self) -> [u8] {
//         match *self {
//             CharSetArray::Phred33U8 => PHRED33,
//             CharSetArray::Phred64U8 => PHRED64,
//         }
//     }
// }

pub const PERCENTAGE_RANGE_START: usize = 0;
pub const PERCENTAGE_RANGE_END: usize = 100;
pub const PERCENTAGE_RANGE: RangeInclusive<usize> = PERCENTAGE_RANGE_START..=PERCENTAGE_RANGE_END;

pub const PERCENTAGE: [u8; 101] = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,64,65,66,67,68,69,70,71,72,73,74,75,76,77,78,79,80,81,82,83,84,85,86,87,88,89,90,91,92,93,94,95,96,97,98,99,100];

pub fn new_str_hashset<'a>(array: &'a [&str]) -> HashSet<&'a str> {
    HashSet::from_iter(array.iter().cloned())
}

pub fn new_hashset(array: &[u8]) -> HashSet<u8> {
    HashSet::from_iter(array.iter().cloned())
}
