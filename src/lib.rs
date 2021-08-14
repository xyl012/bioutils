use anyhow::{Result, Error, bail};

use std::collections::{BTreeMap, HashMap, HashSet};

use std::convert::{TryFrom, TryInto};
use core::num::TryFromIntError;

use std::ops::RangeInclusive;
use std::iter::FromIterator;
use std::cmp::Ordering;
use std::path::Path;

use crate::charsets::bioutils::*;
use crate::charsets::iupac::*;
use crate::charsets::quality::*;
use crate::charsets::ascii::*;
use crate::charsets::percent::*;

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

