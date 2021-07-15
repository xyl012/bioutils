use anyhow::{Result, Error, bail};

use crate::charsets::iupac::*;
use crate::charsets::quality::*;
use crate::charsets::ascii::*;

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

pub mod charsets;
pub mod files;
pub mod utils;
pub mod references;
pub mod img;

#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
