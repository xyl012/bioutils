//! ```
//! use crate::bioutils::utils::recode::BioUtilsRecodeU8;
//! use crate::bioutils::utils::recode::BioUtilsRecodeAsMutSlice;
//! use bioutils::charsets::bioutils::*;
//! 
//! let mut phred33_score = 12u8;
//! phred33_score.recode_u8(BioUtilsRecodeSet::Phred33Encode);
//! println!("{:?}", phred33_score); 
//! 
//! let mut phred33_encoding = 34u8;
//! &phred33_encoding.mut_recode_u8(BioUtilsRecodeSet::Phred33Decode);
//! println!("{:?}", phred33_encoding);
//! 
//! let mut phred33_score_2 = b"00000".to_owned();
//! phred33_score_2.mut_recode(BioUtilsRecodeSet::Phred33Encode);
//! println!("{:?}", phred33_score_2);
//! ```

use super::*;
use crate::utils::check::AllAsRefSlice;

pub trait BioUtilsRecodeU8 {
    /// Checks if self can be encoded (encoding contains all u8 in self) and encodes self.
    fn recode_u8(&self, code: BioUtilsRecodeSet) -> Option<u8>;
    /// Checks if self can be recoded (charset contains all u8 in self) and recodes self.
    fn mut_recode_u8(&mut self, code: BioUtilsRecodeSet) -> Option<&mut Self>;
}

impl BioUtilsRecodeU8 for u8
{
    /// Checks if self can be encoded (encoding contains all u8 in self) and encodes self.
    fn recode_u8(&self, code: BioUtilsRecodeSet) -> Option<u8> {
        if code.value().charset.contains(self) {
            Some(*code.value().recode.get(*self as usize)?)
        } else {
            None
        }
    }
    
    /// Checks if self can be encoded (encoding contains all u8 in self) and encodes self.
    fn mut_recode_u8(&mut self, code: BioUtilsRecodeSet) -> Option<&mut Self> {
        if code.value().charset.contains(self) {
            *self = *code.value().recode.get(*self as usize)?;
            Some(self)
        } else {
            None
        }
    }
}

pub trait BioUtilsRecodeAsMutSlice<T> {
    /// Checks if self can be encoded (encoding contains all u8 in self) and encodes self.
    fn mut_recode(&mut self, code: BioUtilsRecodeSet) -> Option<&mut Self>;
}

impl<T> BioUtilsRecodeAsMutSlice<T> for T where
T: AsMut<[u8]>,
{
    /// Checks if self can be encoded (encoding contains all u8 in self) and encodes self.
    fn mut_recode(&mut self, code: BioUtilsRecodeSet) -> Option<&mut Self> {
        let mut target = self.as_mut().iter_mut();
        if target.all(|u| code.value().charset.contains(u)) {
            target.for_each(|u| *u = code.value().recode[*u as usize]);
            Some(self)
        } else {
            None
        }
    }
}

pub trait BioUtilsRecodeAsRefSlice<T> {
    /// Checks if self can be encoded (encoding contains all u8 in self) and encodes self.
    fn recode(&self, code: BioUtilsRecodeSet) -> Option<Vec<u8>>;
}

impl<T> BioUtilsRecodeAsRefSlice<T> for T where
T: AsRef<[u8]>,
{
    /// Checks if self can be encoded (encoding contains all u8 in self) and encodes self.
    fn recode(&self, code: BioUtilsRecodeSet) -> Option<Vec<u8>> {
        if self.is_all_charset_with(code.value().charset) {
            Some(self.as_ref().iter().map(|u| code.value().recode[*u as usize]).collect::<Vec<u8>>())
        } else {
            None
        }
    }
}

pub trait BioUtilsAsMutDoubleEndedIterator<T> {
    /// Recode the reverse of self, commonly for generating the reverse complement.
    fn mut_rev_recode(&mut self, code: BioUtilsRecodeSet) -> Option<&mut Self>;
}

impl<T> BioUtilsAsMutDoubleEndedIterator<T> for T where
T: AsMut<[u8]>,
T: DoubleEndedIterator,
{
    /// Recode the reverse of self, commonly for generating the reverse complement.
    fn mut_rev_recode(&mut self, code: BioUtilsRecodeSet) -> Option<&mut Self> {
        let mut target = self.as_mut().iter_mut().rev();
        if target.all(|u| code.value().charset.contains(u)) {
            target.for_each(|u| *u = code.value().recode[*u as usize]);
            Some(self)
        } else {
            None
        }
    }
}

pub trait BioUtilsAsRefDoubleEndedIterator<T> {
    /// Recode the reverse of self, commonly for generating the reverse complement.
    fn rev_recode(&self, code: BioUtilsRecodeSet) -> Option<Vec<u8>>;
}

impl<T> BioUtilsAsRefDoubleEndedIterator<T> for T where
T: AsRef<[u8]>,
T: DoubleEndedIterator,
{
    /// Recode the reverse of self, commonly for generating the reverse complement.
    fn rev_recode(&self, code: BioUtilsRecodeSet) -> Option<Vec<u8>> {
        if self.is_all_charset_with(code.value().charset) {
            Some(self.as_ref().iter().rev().map(|u| code.value().recode[*u as usize]).collect::<Vec<u8>>())
        } else {
            None
        }
    }
}

    // /// Checks if self can be decoded (decoding contains all u8 in self) and decodes self.
    // fn mut_decode(&'a mut self, encoding: BioUtilsRecodeSet) -> Result<&'a mut Self>;

    // /// Checks if self can be decoded (decoding contains all u8 in self) and decodes self.
    // fn mut_decode(&mut self, encoding: BioUtilsRecodeSet) -> Result<&mut Self> {

    // }

    // use crate::utils::recode::BioUtilsRecodeU8;
    // use std::ops::Try;
    // /// Checks if self can be decoded (decoding contains all u8 in self) and decodes self.
    // fn decode_u8(&self, code: BioUtilsRecodeSet) -> Option<u8>;
    // /// Checks if self can be encoded (encoding contains all u8 in self) and encodes self.
    // /// Checks if self can be decoded (decoding contains all u8 in self) and decodes self.
    // fn mut_decode_u8(&mut self, code: BioUtilsRecodeSet) -> Option<&mut Self>;
    // /// Checks if self can be encoded (encoding contains all u8 in self) and encodes self.
    // fn mut_encode_u8(&mut self, code: BioUtilsRecodeSet) -> Option<&mut Self> {
    //     if code.value().score.contains(&self) {
    //         *self = *code.value().encode.get(*self as usize)?;
    //         Some(self)
    //     } else {
    //         None
    //     }
    // }
        // /// Checks if self can be decoded (decoding contains all u8 in self) and decodes self.
    // fn decode_u8(&self, code: BioUtilsRecodeSet) -> Option<u8> {
    //     if code.value().encode.contains(&self) {
    //         Some(*code.value().decode.get(*self as usize)?)
    //     } else {
    //         None
    //     }
    // }
    // /// Checks if self can be decoded (decoding contains all u8 in self) and decodes self.
    // fn mut_decode_u8(&mut self, code: BioUtilsRecodeSet) -> Option<&mut Self> {
    //     if code.value().encode.contains(&self) {
    //         *self = *code.value().decode.get(*self as usize)?;
    //         Some(self)
    //     } else {
    //         None
    //     }
    // }

//     /// Checks if self can be encoded (encoding contains all u8 in self) and encodes self.
//     fn mut_encode(&mut self, code: BioUtilsRecodeSet) -> Result<&mut Self> {
//         if code.value().score.contains(self) {
//             code.value().encode.get(*code as usize).copied()
//         } else {
//             None
//         }
//     }

//     /// Checks if self can be decoded (decoding contains all u8 in self) and decodes self.
//     fn mut_decode(&mut self, code: BioUtilsRecodeSet) -> Result<&mut Self> {
//         code.value().score;
//         code.value().encode;
//         code.value().code;
//     }

// }
// pub trait CodeItemU8<T> {
//     /// Returns the PHRED33 quality score from a PHRED33 quality encoding. The score is the u8 minus 33.
//     fn decode_qual(&self) ->  Result<Vec<u8>>;
//     /// Returns the PHRED64 quality score from a PHRED64 quality encoding. The score is the u8 minus 64.
//     fn decode_qual_phred64(&self) ->  Result<Vec<u8>>;
//     /// Returns the SANGER quality score from a SANGER quality encoding. The score is the u8 minus 33.
//     fn decode_qual_sanger(&self) ->  Result<Vec<u8>>;
//     /// Returns the PHRED33 quality encoding from a PHRED33 quality score. The score is the u8 minus 33.
//     fn encode_qual(&self) ->  Result<Vec<u8>>;
//     /// Returns the PHRED64 quality encoding from a PHRED64 quality score. The score is the u8 minus 64.
//     fn encode_qual_phred64(&self) ->  Result<Vec<u8>>;
//     /// Returns the SANGER quality encoding from a SANGER quality score. The score is the u8 minus 33.
//     fn encode_qual_sanger(&self) -> Result<Vec<u8>>;
// }

// impl<T> CodeItemU8<T> for T where
//     T: AsRef<[u8]>
// {
//     /// Returns the PHRED33 quality score from a raw PHRED33 quality encoding. The score is simply the u8 minus 33.
//     fn decode_qual(&self) -> Result<Vec<u8>> {
//         self.check_phred33();
//         // }).collect::<Result<Vec<u8>>>()
//         // .for_each(|u| PHRED33_HASHMAP.get(u).unwrap().to_owned()).collect::<Result<Vec<u8>>>()
//     }
//     // .into_iter().map(|(_, v)| v)
//     /// Returns the PHRED64 quality score from a raw PHRED64 quality encoding. The score is simply the u8 minus 64.
//     fn decode_qual_phred64(&self) -> Result<Vec<u8>> {
//         self.as_ref().iter().map(|q| PHRED64_HASHMAP.get(&q)?.to_owned()).collect::<Result<Vec<u8>>>()
//     }
//     /// Returns the SANGER quality score from a SANGER quality encoding. The score is the u8 minus 33.
//     fn decode_qual_sanger(&self) -> Result<Vec<u8>> {
//         self.as_ref().iter().map(|q| SANGER_HASHMAP_DECODE.get(&q)?.to_owned()).collect::<Result<Vec<u8>>>()
//     }
//     /// Returns the PHRED33 quality encoding from a PHRED33 quality score. The score is the u8 minus 33.
//     fn encode_qual(&self) -> Result<Vec<u8>> {
//         self.as_ref().iter().map(|q| PHRED33_HASHMAP_ENCODE.get(&q)?.to_owned()).collect::<Result<Vec<u8>>>()
//     }
//     /// Returns the PHRED64 quality encoding from a PHRED64 quality score. The score is the u8 minus 64.
//     fn encode_qual_phred64(&self) -> Result<Vec<u8>> {
//         self.as_ref().iter().map(|q| PHRED64_HASHMAP_ENCODE.get(&q)?.to_owned()).collect::<Result<Vec<u8>>>()
//     }
//     /// Returns the SANGER quality encoding from a SANGER quality score. The score is the u8 minus 33.
//     fn encode_qual_sanger(&self) -> Result<Vec<u8>> {
//         self.as_ref().iter().map(|q| SANGER_HASHMAP_ENCODE.get(&q)?.to_owned()).collect::<Result<Vec<u8>>>()
//     }
// }

// pub trait OwnedRecodeAsRefSlice<T> {
//     fn decode_owned(&self, decoding: Biouti) -> Result<Vec<u8>>;
//     fn encode_owned(&self, encoding: ) -> Result<Vec<u8>>;
// }

// impl<T> OwnedRecodeAsRefSlice<T> for T
// where
//     T: AsRef<[u8]>,
// {
//     /// Returns the PHRED33 quality score from a raw PHRED33 quality encoding (-33).
//     fn decode_qual(&self) -> Result<Vec<u8>> {
//         self.as_ref().iter().map(|u| PHRED33_HASHMAP_DECODE.get(u).copied()).collect::<Option<Vec<u8>>>().ok_or(bail!("Contains non-PHRED33 u8s"))
//     }

//     /// Returns the PHRED33 quality encoding from a PHRED33 quality score. (+33)
//     fn encode_qual(&self) -> Result<Vec<u8>> {
//         self.as_ref().iter().map(|u| PHRED33_HASHMAP_ENCODE.get(u).copied()).collect::<Option<Vec<u8>>>().ok_or(bail!("Contains non-PHRED33 score u8s"))
//     }

// }

// pub trait RecodeQualityAsMutSlice<T> {
//     /// Returns the PHRED33 quality score from a PHRED33 quality encoding. The score is the u8 minus 33. Sanger is also shifted 33.
//     fn mut_decode_qual(&mut self) -> Result<&mut Self>;
//     /// Returns the PHRED33 quality encoding from a PHRED33 quality score. The score is the u8 minus 33. Sanger is also shifted 33.
//     fn mut_encode_qual(&mut self) -> Result<&mut Self>;
// }

// impl<T> RecodeQualityAsMutSlice<T> for T
// where
//     T: AsMut<[u8]>,
// {
//     /// Returns the PHRED33 quality score from a raw PHRED33 quality encoding (-33).
//     fn mut_decode_qual(&mut self, charset: BioUtilsCharSet) -> Result<&mut Self> {
//         match self.mut_is_slice_all_charset(charset) {
//             true => {self.as_mut().iter_mut().for_each(|u| *u = *u-33); Ok(self)},
//             false => bail!("Contains non-PHRED33 u8s")
//         }
//     }

//     /// Returns the PHRED33 quality encoding from a PHRED33 quality score (+33).
//     fn mut_encode_qual(&mut self) -> Result<&mut Self> {
//         match self.mut_is_phred33_scores() {
//             true => {self.as_mut().iter_mut().for_each(|u| *u = *u+33); Ok(self)},
//             false => bail!("Contains non-PHRED33 score u8s")
//         }
//     }

// }

// // /// Returns the PHRED64 quality score from a raw PHRED64 quality encoding (-64).
//     // fn decode_qual_phred64(&self) -> Result<Vec<u8>> {
//     //     match self.is_phred64() {
//     //         true => {Ok(self.as_ref().iter().map(|u| u-64).collect())},
//     //         false => bail!("Contains non-PHRED64 u8s")
//     //     } 
//     // }
//     // /// Returns the PHRED33 quality encoding from a PHRED33 quality score (+33).
//     // fn encode_qual(&self) -> Result<Vec<u8>> {
//     //     match self.is_phred33_scores() {
//     //         true => {Ok(self.as_ref().iter().map(|u| u+33).collect())},
//     //         false => bail!("Contains non-PHRED33 score u8s")
//     //     }
//     // }
//     // /// Returns the PHRED64 quality encoding from the quality score (+64).
//     // fn mut_encode_qual_phred64(&mut self) -> Result<&mut Self> {
//     //     match self.mut_is_phred64_scores() {
//     //         true => {self.as_mut().iter_mut().for_each(|u| *u = *u+64); Ok(self)},
//     //         false => bail!("Contains non-PHRED64 score u8s")
//     //     }
//     // }



// // pub enum QualityHashMap {
// //     Phred33Encode,
// //     Phred33Decode,
// //     Phred64Encode,
// //     Phred64Decode,
// //     SangerEncode,
// //     SangerDecode,
// // }
// // impl QualityHashMap {
// //     pub const fn value(&self) -> HashMap<u8,u8> {
// //         match *self {
// //             QualityHashMap::Phred33Encode => PHRED33_HASHMAP_ENCODE,
// //             QualityHashMap::Phred33Decode => PHRED33_HASHMAP_DECODE,
// //             QualityHashMap::Phred64Encode => PHRED64_HASHMAP_ENCODE,
// //             QualityHashMap::Phred64Decode => PHRED64_HASHMAP_DECODE,
// //             QualityHashMap::SangerEncode => SANGER_HASHMAP_ENCODE,
// //             QualityHashMap::SangerDecode => SANGER_HASHMAP_DECODE,
// //         }
// //     }
// // }


// pub fn encode_phred33(temp: &u8) -> Option<u8> { 
//     if PHRED33_SCORE.contains(temp) {
//         PHRED33_ENCODE.get(*temp as usize).copied()
//     } else {
//         None
//     }
// }

// pub fn decode_phred33(temp: &u8) -> Option<u8> { 
//     if PHRED33_ENCODE.contains(temp) {
//         PHRED33_DECODE.get(*temp as usize).copied()
//     } else {
//         None
//     }
// }
