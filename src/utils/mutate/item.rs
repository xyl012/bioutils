// Copyright (c) 2021 Kana LLC

use crate::charsets::quality::PHRED64_HASHMAP_ENCODE_U8;
use crate::charsets::quality::PHRED33_HASHMAP_ENCODE_U8;
use crate::charsets::quality::PHRED64_HASHMAP_U8;
use crate::charsets::quality::PHRED33_HASHMAP_U8;

pub trait AsMutItemU8<T> {
    /// Returns the PHRED33 quality score from a PHRED33 quality encoding. The score is the u8 minus 33.
    fn mut_decode_qual(&mut self) -> &mut Self;
    /// Returns the PHRED64 quality score from a PHRED64 quality encoding. The score is the u8 minus 64.
    fn mut_decode_qual_phred64(&mut self) -> &mut Self;
    /// Returns the PHRED33 quality encoding from a PHRED33 quality score. The score is the u8 minus 33.
    fn mut_encode_qual(&mut self) -> &mut Self;
    /// Returns the PHRED64 quality encoding from a PHRED64 quality score. The score is the u8 minus 64.
    fn mut_encode_qual_phred64(&mut self) -> &mut Self;
}

impl<T> AsMutItemU8<T> for T
where
    T: AsMut<[u8]>,
{
    /// Returns the PHRED33 quality score from a raw PHRED33 quality encoding. The score is simply the u8 minus 33.
    fn mut_decode_qual(&mut self) -> &mut Self {
        for q in &mut self.as_mut().iter_mut() {
            PHRED33_HASHMAP_U8.get(&q).unwrap();
        }
        self
    }

    /// Returns the PHRED64 quality score from a raw PHRED64 quality encoding. The score is simply the u8 minus 64.
    fn mut_decode_qual_phred64(&mut self) -> &mut Self {
        for q in &mut self.as_mut().iter_mut() {
            PHRED64_HASHMAP_U8.get(&q).unwrap();
        }
        self
    }
    
    /// Returns the PHRED33 quality encoding from a PHRED33 quality score. The score is the u8 minus 33.
    fn mut_encode_qual(&mut self) -> &mut Self {
        for q in &mut self.as_mut().iter_mut() {
            PHRED33_HASHMAP_ENCODE_U8.get(&q).unwrap();
        }
        self
    }

    /// Returns the PHRED64 quality encoding from a PHRED64 quality score. The score is the u8 minus 64.
    fn mut_encode_qual_phred64(&mut self) -> &mut Self {
        for q in &mut self.as_mut().iter_mut() {
            PHRED64_HASHMAP_ENCODE_U8.get(&q).unwrap();
        }
        self
    }
}