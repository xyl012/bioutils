// Copyright 2020 Christopher Sugai

//! # Bioutils: Simple Biological Utilities with Rust's [u8]
//! Bioutils provides simple biological utilities ranging from complete iupac and quality character sets to checking sequence validity.
//! Utilities provided here are simple and built to be simple to call. For more complex algorithms and structures, we recommend rust-bio
//! Implementations are centered around [u8], although character sets are also provided as [&str], hashset u8 and hashset &str.
//! Check back as more functionality is added!


pub mod utils;
pub mod charsets;

#[macro_use] extern crate lazy_static;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
