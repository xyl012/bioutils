// Copyright 2021 Christopher Sugai

//! Functions to download genome references. These are used to download specific references and do not need to be used directly.

use std::io::{Write};
use std::fs::File;
use curl::easy::Easy;
use url::{Url, ParseError};

/// Function to intake a base url and a filename and paste them together to generate a url where the file may be found.
pub fn build_url(url: &str, filename: &str) -> Result<Url, ParseError> {
    let base = Url::parse(url).expect("Base url cannot be parsed");
    let joined = base.join(filename)?;
    Ok(joined)
}

/// Function to download a reference from a base url and a filename.
/// This function only needs to be completed once for the reference of choice, as the file will be stored on the hard drive.
pub fn download_reference(url: &str, filename: &str) {
    // Generate url to get file
    let mut easy = Easy::new();
    let file_url = build_url(url, filename).expect("Cannot build file url").to_string();
    easy.url(&file_url).unwrap();
    // Create file on system to write on
    let mut file = match File::create(filename) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };
    // Write to file on system
    easy.write_function(move |data| {
        file.write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();
    // println!("{}", easy.response_code().unwrap());
}

// /// Function to read a downloaded reference as a [u8]. Please run download_reference() prior to using this method. download_reference() only needs to be completed once for the reference of choice.


// /// Function to read a downloaded reference as a &str. Please run download_reference() prior to using this method. download_reference() only needs to be completed once for the reference of choice.







