use super::*;
use crate::utils::element::percent::*;
use crate::utils::element::quality::*;
use crate::utils::item::quality::*;

pub trait CheckAsRefSlice<T> {
    fn check_slice(&self, charset: AsciiCharSet) -> bool;
}

impl<T> CheckAsRefSlice<T> for T where
T: AsRef<[u8]> 
{
    fn check_slice(&self, charset: AsciiCharSet) -> bool {
        self.as_ref().iter().all(|u| charset.value().contains(u))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_checking_slice() {
        assert!(check_slice(&[67,67,67,67], charset: AsciiCharSet::Dna), true);
    }
}