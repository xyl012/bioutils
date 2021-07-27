use super::*;
use crate::utils::element::percent::*;
use crate::utils::element::quality::*;
use crate::utils::item::quality::*;

pub trait CheckAsRefSlice<T> {
    check_with_charset(&self, charset: [u8], )
}

impl CheckAsRefSlice<T> for T where
T: AsRef<[u8]> 
{

}