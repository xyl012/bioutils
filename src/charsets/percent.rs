use super::*;

pub const PERCENT_RANGE_START: usize = 0;
pub const PERCENT_RANGE_END: usize = 100;
pub const PERCENT_RANGE: RangeInclusive<usize> = PERCENT_RANGE_START..=PERCENT_RANGE_END;

pub const PERCENT: [u8; 101] = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,64,65,66,67,68,69,70,71,72,73,74,75,76,77,78,79,80,81,82,83,84,85,86,87,88,89,90,91,92,93,94,95,96,97,98,99,100];
pub const PERCENT_USIZE: [usize; 101] = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,64,65,66,67,68,69,70,71,72,73,74,75,76,77,78,79,80,81,82,83,84,85,86,87,88,89,90,91,92,93,94,95,96,97,98,99,100];
pub const PERCENT_U64: [u64; 101] = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,64,65,66,67,68,69,70,71,72,73,74,75,76,77,78,79,80,81,82,83,84,85,86,87,88,89,90,91,92,93,94,95,96,97,98,99,100];

pub enum PercentCharSet {
    Percent
}

pub enum PercentUsizeCharSet {
    Percent
}

pub enum PercentU64CharSet {
    Percent
}

impl PercentCharSet {
    pub const fn value(&self) -> &[u8] {
        match *self {
            PercentCharSet::Percent => &PERCENT,
        }
    }
}

impl PercentUsizeCharSet {
    pub const fn value(&self) -> &[usize] {
        match *self {
            PercentUsizeCharSet::Percent => &PERCENT_USIZE,
        }
    }
}

impl PercentU64CharSet {
    pub const fn value(&self) -> &[u64] {
        match *self {
            PercentU64CharSet::Percent => &PERCENT_U64,
        }
    }
}
