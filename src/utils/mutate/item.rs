// Copyright (c) 2021 Kana LLC

use crate::charsets::quality::PHRED64_HASHMAP_U8;
use crate::charsets::quality::PHRED33_HASHMAP_U8;

/// TODO cut read to length, trimmomatic, fastp

pub trait AsMutItemU8<T> {
    /// Returns the PHRED33 quality score from a raw PHRED33 quality encoding. The score is simply the u8 minus 33.
    fn shift_qual_encoding(&mut self) -> &mut Self;
    /// Returns the PHRED64 quality score from a raw PHRED64 quality encoding. The score is simply the u8 minus 64.
    fn shift_phred64_qual_encoding(&mut self) -> &mut Self;

    // /// Cuts the read to a specific length. 
    
    
}

impl<T> AsMutItemU8<T> for T
where
    T: AsMut<[u8]>,
{
    /// Returns the PHRED33 quality score from a raw PHRED33 quality encoding. The score is simply the u8 minus 33.
    fn shift_qual_encoding(&mut self) -> &mut Self {
        for q in &mut self.as_mut().iter_mut() {
            PHRED33_HASHMAP_U8.get(&q).unwrap();
        }
        self
    }
    /// Returns the PHRED64 quality score from a raw PHRED64 quality encoding. The score is simply the u8 minus 64.
    fn shift_phred64_qual_encoding(&mut self) -> &mut Self {
        for q in &mut self.as_mut().iter_mut() {
            PHRED64_HASHMAP_U8.get(&q).unwrap();
        }
        self
    }
}