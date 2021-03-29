// Copyright 2021 Christopher Sugai

//! Functions to curl files. Currently, these are downloaded into the bioutils directory.

use std::io::{Write};
use std::fs::File;
use curl::easy::Easy;
use url::{Url, ParseError};

// pub const VERSION: &str = "v37";
// pub const BASE_URL: &str = "ftp://ftp.ebi.ac.uk/pub/databases/gencode/Gencode_human/latest_release/";

/// Function to intake a base url and a filename and paste them together to generate a url where the file may be found.
pub fn build_url(url: &str, filename: &str) -> Result<Url, ParseError> {
    let base = Url::parse(url).expect("Base url cannot be parsed");
    let joined = base.join(filename)?;
    Ok(joined)
}

/// Function to curl (download) a file from a base url and a filename.
pub fn curl(url: &str, filename: &str) {
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

// Paste file names together with changeable version.
pub fn paste_prefix_suffix(prefix: &str, suffix: &str) -> String {
    let mut fname: String = prefix.to_owned();
    fname.push_str(suffix); fname
}

// pub fn paste_prefix_version_suffix(prefix: &str, suffix: &str) -> String {
//     let mut fname: String = prefix.to_owned();
//     fname.push_str(VERSION); fname.push_str(suffix); fname
// }

// Get links from the latest gencode release page
// ftp://ftp.ebi.ac.uk/pub/databases/gencode/Gencode_human/latest_release

// /// Function to read a downloaded reference as a [u8]. Please run download_reference() prior to using this method. download_reference() only needs to be completed once for the reference of choice.


// /// Function to read a downloaded reference as a &str. Please run download_reference() prior to using this method. download_reference() only needs to be completed once for the reference of choice.



