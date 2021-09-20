//! Check whether input is related to a character set and return a boolean, result, or option.

use super::*;

pub trait AllAsRefSlice<T> {
    /// Checks if all elements in the slice are contained in a character set (bool).
    fn is_all_charset(&self, charset: BioUtilsCharSet) -> bool;
    /// Checks if all elements in the slice are contained in a character set (Ok if true, Err if false).
    fn result_is_all_charset(&self, charset: BioUtilsCharSet) -> Result<&Self>;
    /// Checks if all elements in the slice are contained in a character set (Some if true, None if false).
    fn option_is_all_charset(&self, charset: BioUtilsCharSet) -> Option<&Self>;

    /// Checks if all elements in the slice are contained in a character set (bool).
    fn is_all_charset_with(&self, charset: &[u8]) -> bool;
    /// Checks if all elements in the slice are contained in a character set (Ok if true, Err if false).
    fn result_is_all_charset_with(&self, charset: &[u8]) -> Result<&Self>;
    /// Checks if all elements in the slice are contained in a character set (Some if true, None if false).
    fn option_is_all_charset_with(&self, charset: &[u8]) -> Option<&Self>;
}

impl<T> AllAsRefSlice<T> for T where
T: AsRef<[u8]> 
{
    /// Checks if all elements in the slice are contained in a character set (bool).
    fn is_all_charset(&self, charset: BioUtilsCharSet) -> bool {
        self.as_ref().iter().all(|u| charset.value().contains(u))
    }
    /// Checks if all elements in the slice are contained in a character set (Ok if true, Err if false).
    fn result_is_all_charset(&self, charset: BioUtilsCharSet) -> Result<&Self> {
        match self.is_all_charset(charset) {
            true => Ok(self),
            false => bail!("Slice is not all charset"),
        }
    }
    /// Checks if all elements in the slice are contained in a character set (Some if true, None if false).
    fn option_is_all_charset(&self, charset: BioUtilsCharSet) -> Option<&Self> {
        match self.is_all_charset(charset) {
            true => Some(self),
            false => None,
        }
    }

    /// Checks if all elements in the slice are contained in a character set (bool).
    fn is_all_charset_with(&self, charset: &[u8]) -> bool {
        self.as_ref().iter().all(|u| charset.contains(u))
    }
    /// Checks if all elements in the slice are contained in a character set (Ok if true, Err if false).
    fn result_is_all_charset_with(&self, charset: &[u8]) -> Result<&Self> {
        match self.is_all_charset_with(charset) {
            true => Ok(self),
            false => bail!("Slice is not all charset"),
        }
    }
    /// Checks if all elements in the slice are contained in a character set (Some if true, None if false).
    fn option_is_all_charset_with(&self, charset: &[u8]) -> Option<&Self> {
        match self.is_all_charset_with(charset) {
            true => Some(self),
            false => None,
        }
    }
}

pub trait AnyAsRefSlice<T> {
    /// Checks if the slice contains any from the character set (bool).
    fn has_charset(&self, charset: BioUtilsCharSet) -> bool;
    /// Checks if the slice contains any from the character set (Ok if true, Err if false).
    fn result_has_charset(&self, charset: BioUtilsCharSet) -> Result<&Self>;
    /// Checks if the slice contains any from the character set (Some if true, None if false).
    fn option_has_charset(&self, charset: BioUtilsCharSet) -> Option<&Self>;
    
    /// Checks if the slice contains any from the character set (bool).
    fn has_charset_with(&self, charset: &[u8]) -> bool;
    /// Checks if the slice contains any from the character set (Ok if true, Err if false).
    fn result_has_charset_with(&self, charset: &[u8]) -> Result<&Self>;
    /// Checks if the slice contains any from the character set (Some if true, None if false).
    fn option_has_charset_with(&self, charset: &[u8]) -> Option<&Self>;
}

impl<T> AnyAsRefSlice<T> for T where
T: AsRef<[u8]> 
{
    /// Checks if the slice contains any from the character set (bool).
    fn has_charset(&self, charset: BioUtilsCharSet) -> bool {
        self.as_ref().iter().any(|u| charset.value().contains(u))
    }
    /// Checks if the slice contains any from the character set (Ok if true, Err if false).
    fn result_has_charset(&self, charset: BioUtilsCharSet) -> Result<&Self> {
        match self.has_charset(charset) {
            true => Ok(self),
            false => bail!("Slice does not have the charset"),
        }
    }
    /// Checks if the slice contains any from the character set (Some if true, None if false).
    fn option_has_charset(&self, charset: BioUtilsCharSet) -> Option<&Self> {
        match self.has_charset(charset) {
            true => Some(self),
            false => None,
        }
    }

    /// Checks if the slice contains any from the character set (bool).
    fn has_charset_with(&self, charset: &[u8]) -> bool {
        self.as_ref().iter().any(|u| charset.contains(u))
    }
    /// Checks if the slice contains any from the character set (Ok if true, Err if false).
    fn result_has_charset_with(&self, charset: &[u8]) -> Result<&Self> {
        match self.has_charset_with(charset) {
            true => Ok(self),
            false => bail!("Slice does not have the charset"),
        }
    }
    /// Checks if the slice contains any from the character set (Some if true, None if false).
    fn option_has_charset_with(&self, charset: &[u8]) -> Option<&Self> {
        match self.has_charset_with(charset) {
            true => Some(self),
            false => None,
        }
    }

}

pub trait AllAsMutSlice<T> {
    /// Checks if all elements in the slice are contained in a character set (bool).
    fn mut_is_all_charset(&mut self, charset: BioUtilsCharSet) -> bool;
    /// Checks if all elements in the slice are contained in a character set (Ok if true, Err if false).
    fn mut_result_is_all_charset(&mut self, charset: BioUtilsCharSet) -> Result<&mut Self>;
    /// Checks if all elements in the slice are contained in a character set (Some if true, None if false).
    fn mut_option_is_all_charset(&mut self, charset: BioUtilsCharSet) -> Option<&mut Self>;

    /// Checks if all elements in the slice are contained in a character set (bool).
    fn mut_is_all_charset_with(&mut self, charset: &[u8]) -> bool;
    /// Checks if all elements in the slice are contained in a character set (Ok if true, Err if false).
    fn mut_result_is_all_charset_with(&mut self, charset: &[u8]) -> Result<&mut Self>;
    /// Checks if all elements in the slice are contained in a character set (Some if true, None if false).
    fn mut_option_is_all_charset_with(&mut self, charset: &[u8]) -> Option<&mut Self>;
}

impl<T> AllAsMutSlice<T> for T where
T: AsMut<[u8]> 
{
    /// Checks if all elements in the slice are contained in a character set (bool).
    fn mut_is_all_charset(&mut self, charset: BioUtilsCharSet) -> bool {
        self.as_mut().iter().all(|u| charset.value().contains(u))
    }
    /// Checks if all elements in the slice are contained in a character set (Ok if true, Err if false).
    fn mut_result_is_all_charset(&mut self, charset: BioUtilsCharSet) -> Result<&mut Self> {
        match self.mut_is_all_charset(charset) {
            true => Ok(self),
            false => bail!("Slice is not all charset"),
        }
    }
    /// Checks if all elements in the slice are contained in a character set (Some if true, None if false).
    fn mut_option_is_all_charset(&mut self, charset: BioUtilsCharSet) -> Option<&mut Self> {
        match self.mut_is_all_charset(charset) {
            true => Some(self),
            false => None,
        }
    }

    /// Checks if all elements in the slice are contained in a character set (bool).
    fn mut_is_all_charset_with(&mut self, charset: &[u8]) -> bool {
        self.as_mut().iter().all(|u| charset.contains(u))
    }
    /// Checks if all elements in the slice are contained in a character set (Ok if true, Err if false).
    fn mut_result_is_all_charset_with(&mut self, charset: &[u8]) -> Result<&mut Self> {
        match self.mut_is_all_charset_with(charset) {
            true => Ok(self),
            false => bail!("Slice is not all charset"),
        }
    }
    /// Checks if all elements in the slice are contained in a character set (Some if true, None if false).
    fn mut_option_is_all_charset_with(&mut self, charset: &[u8]) -> Option<&mut Self> {
        match self.mut_is_all_charset_with(charset) {
            true => Some(self),
            false => None,
        }
    }
}

trait AnyAsMutSlice<T> {
    /// Checks if the slice contains any from the character set (bool).
    fn mut_has_charset(&mut self, charset: BioUtilsCharSet) -> bool;
    /// Checks if the slice contains any from the character set (Ok if true, Err if false).
    fn mut_result_has_charset(&mut self, charset: BioUtilsCharSet) -> Result<&mut Self>;
    /// Checks if the slice contains any from the character set (Some if true, None if false).
    fn mut_option_has_charset(&mut self, charset: BioUtilsCharSet) -> Option<&mut Self>;
    
    /// Checks if the slice contains any from the character set (bool).
    fn mut_has_charset_with(&mut self, charset: &[u8]) -> bool;
    /// Checks if the slice contains any from the character set (Ok if true, Err if false).
    fn mut_result_has_charset_with(&mut self, charset: &[u8]) -> Result<&mut Self>;
    /// Checks if the slice contains any from the character set (Some if true, None if false).
    fn mut_option_has_charset_with(&mut self, charset: &[u8]) -> Option<&mut Self>;
}

impl<T> AnyAsMutSlice<T> for T where
T: AsMut<[u8]>,
{
    /// Checks if the slice contains any from the character set (bool).
    fn mut_has_charset(&mut self, charset: BioUtilsCharSet) -> bool {
        self.as_mut().iter().any(|u| charset.value().contains(u))
    }
    /// Checks if the slice contains any from the character set (Ok if true, Err if false).
    fn mut_result_has_charset(&mut self, charset: BioUtilsCharSet) -> Result<&mut Self> {
        match self.mut_has_charset(charset) {
            true => Ok(self),
            false => bail!("Slice does not have the charset"),
        }
    }
    /// Checks if the slice contains any from the character set (Some if true, None if false).
    fn mut_option_has_charset(&mut self, charset: BioUtilsCharSet) -> Option<&mut Self> {
        match self.mut_has_charset(charset) {
            true => Some(self),
            false => None,
        }
    }
    /// Checks if the slice contains any from the character set (bool).
    fn mut_has_charset_with(&mut self, charset: &[u8]) -> bool {
        self.as_mut().iter().any(|u| charset.contains(u))
    }
    /// Checks if the slice contains any from the character set (Ok if true, Err if false).
    fn mut_result_has_charset_with(&mut self, charset: &[u8]) -> Result<&mut Self> {
        match self.mut_has_charset_with(charset) {
            true => Ok(self),
            false => bail!("Slice does not have the charset"),
        }
    }
    /// Checks if the slice contains any from the character set (Some if true, None if false).
    fn mut_option_has_charset_with(&mut self, charset: &[u8]) -> Option<&mut Self> {
        match self.mut_has_charset_with(charset) {
            true => Some(self),
            false => None,
        }
    }
}

pub trait CheckAsRefPartialEq<K>{
    /// Checks if the sequence and quality u8 vectors are the same length. Generally checks two u8 items for length against each other
    fn is_length_eq(&self, comparison: &K) -> bool;
}

impl<T, K> CheckAsRefPartialEq<K> for T
where
    T: AsRef<[K]>,
    T: PartialEq,
    K: AsRef<[T]>,
    K: PartialEq,
{
    /// Checks if two items are the same length.
    fn is_length_eq(&self, comparison: &K)-> bool {
        self.as_ref().len() == comparison.as_ref().len()
    }
}


pub trait CheckAsMutPartialEq<K>{
    /// Checks if the sequence and quality u8 vectors are the same length. Generally checks two u8 items for length against each other
    fn mut_is_length_eq(&mut self, comparison: &mut K) -> bool;
}

impl<T, K> CheckAsMutPartialEq<K> for T
where
    T: AsMut<[K]>,
    T: PartialEq,
    K: AsMut<[T]>,
    K: PartialEq,
{
    /// Checks if two items are the same length.
    fn mut_is_length_eq(&mut self, comparison: &mut K)-> bool {
        self.as_mut().len() == comparison.as_mut().len()
    }
}

pub trait CheckPalindrome<T> {
    /// Generic trait to check if T is a palindrome
    fn is_palindrome(&self) -> Result<bool>;
}

impl<T> CheckPalindrome<T> for T
where
    T: IntoIterator,
    T::Item: PartialEq,
    T::IntoIter: DoubleEndedIterator,
    T: Copy,
{
    /// Generic impl to check if T is a palindrome
    fn is_palindrome(&self) -> Result<bool> {
    let mut iter = self.into_iter();
    while let (Some(front), Some(back)) = (iter.next(), iter.next_back()) {
        if front != back {
            return Ok(false);
        }
    }
    Ok(true)
    }
}

/// Generic function to check if T is a palindrome.
pub fn is_palindrome<T>(iterable: T) -> Result<bool>
where
    T: IntoIterator,
    T::Item: PartialEq,
    T::IntoIter: DoubleEndedIterator,
{
    let mut iter = iterable.into_iter();
    while let (Some(forward), Some(backward)) = (iter.next(), iter.next_back()) {
        if forward != backward {
            return Ok(false);
        }
    };
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    use crate::utils::check::AllAsRefSlice;
    
    #[test]
    fn checking_slice() {
        let test = &[67,67,67,67];
        assert!(test.is_all_charset(BioUtilsCharSet::Dna), true);
    }
}
