//! ```
//! use crate::bioutils::utils::recode::BioUtilsRecodeU8;
//! use crate::bioutils::utils::recode::BioUtilsRecodeAsMutSlice;
//! use crate::bioutils::utils::recode::BioUtilsAsMutDoubleEndedIterator;
//! use bioutils::charsets::bioutils::*;
//! 
//! // Get the reverse complement of the sequence. Other options for finding the complement are available.
//! let mut reverse_complement = b"ACTG".to_owned();
//! reverse_complement.mut_rev_recode(BioUtilsRecodeSet::DnaComplement);
//! println!("{:?}", reverse_complement);
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
    /// Checks if self can be recoded (recoding contains all u8 in self) and recodes self.
    fn recode_u8(&self, code: BioUtilsRecodeSet) -> Option<u8>;
    /// Checks if self can be recoded (recoding contains all u8 in self) and recodes self.
    fn mut_recode_u8(&mut self, code: BioUtilsRecodeSet) -> Option<&mut Self>;
}

impl BioUtilsRecodeU8 for u8
{
    /// Checks if self can be recoded (recoding contains all u8 in self) and recodes self.
    fn recode_u8(&self, code: BioUtilsRecodeSet) -> Option<u8> {
        if code.value().charset.contains(self) {
            Some(*code.value().recode.get(*self as usize)?)
        } else {
            None
        }
    }
    
    /// Checks if self can be recoded (recoding contains all u8 in self) and recodes self.
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
    /// Checks if self can be recoded (recoding contains all u8 in self) and recodes self.
    fn mut_recode(&mut self, code: BioUtilsRecodeSet) -> Option<&mut Self>;
}

impl<T> BioUtilsRecodeAsMutSlice<T> for T where
T: AsMut<[u8]>,
{
    /// Checks if self can be recoded (recoding contains all u8 in self) and recodes self.
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
    /// Checks if self can be recoded (recoding contains all u8 in self) and recodes self.
    fn recode(&self, code: BioUtilsRecodeSet) -> Option<Vec<u8>>;
}

impl<T> BioUtilsRecodeAsRefSlice<T> for T where
T: AsRef<[u8]>,
{
    /// Checks if self can be recoded (recoding contains all u8 in self) and recodes self.
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
