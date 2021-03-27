// Copyright 2021 Christopher Sugai

//! Functions to download genome references.

extern crate ftp;
extern crate regex;

use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::io::BufWriter;
use std::io::BufReader;
use std::path::Path;
use std::fs::File;
use std::str;
use std::io::Cursor;
use ftp::FtpStream;
use regex::Regex;
use regex::RegexSet;

pub const RELEASE_URL: &str = "/pub/databases/gencode/Gencode_human/latest_release/";
pub const FTP_SITE: &str = "ftp.ebi.ac.uk:21";

// let regex = r"gencode.v\d{2}.tRNAs.gtf.gz";

// Functions for each file type. Checks for latest version and creates a regex that finds the file for each file/function pair

// Downloads the latest version of GRCh38.primary_assembly.genome.fa.gz
pub fn download_grch38_primary_assembly_genome_fa_gz() {
    let regex = r"GRCh38.primary_assembly.genome.fa.gz";
    download_reference_file(regex);
}

// Downloads the latest version of gencode.vxx.primary_assembly.annotation.gtf.gz
pub fn download_gencode_vxx_primary_assembly_annotation_gtf_gz() {
    let regex = r"gencode.v37.primary_assembly.annotation.gtf.gz";
    download_reference_file(regex)
}

// Downloads the latest version of GRCh38.p13.genome.fa.gz
pub fn download_grch38_p13_genome_fa_gz() {
    let regex = r"GRCh38.p13.genome.fa.gz";
    download_reference_file(regex)
}

// General reference download function
pub fn download_reference_file(regex: &str) {
    // Connect and make ftp stream
    let mut ftp_stream = FtpStream::connect(&FTP_SITE).expect("Cannot connect to {}");
    let _ = ftp_stream.login("anonymous", "").unwrap();
    let _ = ftp_stream.cwd(&RELEASE_URL).unwrap();
    let file_paths = ftp_stream.nlst(Some(&RELEASE_URL)).unwrap();
    // Get base names of files as osstr vector from file paths string vector
    let file_names: Vec<&std::ffi::OsStr> = file_paths.iter().map(|x| Path::new(x).file_name().unwrap()).collect();
    // Search files for a file matching the regex, get an index in the vector for the match
    let target_index=search_files(&file_names, regex);
    // Input the index into the vectors of paths and file names to get the matching file
    let target_name = &file_names[target_index];
    let target_path = &file_paths[target_index];
    // Create a stream of the file and download to a file of the same name on disk
    let file = File::create(&target_name).unwrap();
    let mut buf = BufWriter::new(file);
    let mut cursor = ftp_stream.simple_retr(target_path).unwrap();
    // cursor.seek(SeekFrom::Start(0)).unwrap();
    let mut out = Vec::new();
    cursor.read_to_end(&mut out).unwrap();
    buf.write_all(&out).expect("Error writing file");
}

// For use within download_reference_file to search all files within the directory for a file name matching the regex and return the index in the vector that matches.
pub fn search_files(file_names: &Vec<&std::ffi::OsStr>, regex: &str)-> usize {
    let re = Regex::new(regex).unwrap();
    let target_index: usize = file_names.iter().position(|&x| re.is_match(x.to_str().expect("Cannot convert file name to UTF8"))).expect("Regex cannot match a target file");
    target_index
}





