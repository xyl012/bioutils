
//! Functions to curl/download ENA or other files. Currently, these are downloaded into the bioutils directory.


use std::path::Path;
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


/// Curl (download) a file from a base url.
pub fn curl_url(url: &str, output_directory: &std::path::Path) {
    // Generate url to get file
    let mut easy = Easy::new();
    let file_url = url.to_string();
    easy.url(&file_url).unwrap();
    let file_name = Path::new(&file_url).file_name().unwrap();
    // Create file on system to write on
    let mut file = match File::create(&output_directory.join(file_name)) {
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

/// Curl (download) a file from a base url and a filename.
pub fn curl(url: &str, filename: &str, output_directory: &std::path::Path) {
    // Generate url to get file
    let mut easy = Easy::new();
    let file_url = build_url(url, filename).expect("Cannot build file url").to_string();
    easy.url(&file_url).unwrap();
    // Create file on system to write on
    let mut file = match File::create(&output_directory.join(filename)) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };
    // Write to file on system
    easy.write_function(move |data| {
        file.write_all(data).expect("Cannot write to file");
        Ok(data.len())
    }).unwrap();
    easy.perform().expect("Cannot write to file");
    // println!("{}", easy.response_code().unwrap());
}