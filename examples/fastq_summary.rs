// Copyright 2021 Christopher Sugai

// Returns summary statistics for a fastq file
// A work in progress, will add more statistics as functions are written, includes:

// Median length
// Number of reads

use rayon::prelude::*;
use seq_io::fastq::Record as FastqRecord;
use std::fs::File;

fn main(){
    let samples_directory = std::path::Path::new("./data/samples/");
    std::fs::create_dir_all(&samples_directory);
    let fastq_ftp = "ftp://ftp.sra.ebi.ac.uk/vol1/fastq/SRR065/SRR065495/";
    let fq1_gz = "SRR065495_1.fastq.gz";
    let fq1_path = &samples_directory.join(fq1_gz);
    println!("Reference functions take a path, we make this with std::path::path::new()");
    println!("Downloading fastq1 with: ");
    println!("bioutils::files::http::curl()");
    println!("...Please Wait...");
    println!("...If over 10 minutes, there may be a connection issue...");
    bioutils::files::http::curl(fastq_ftp, fq1_gz, &samples_directory);
    let fq1 = File::open(fq1_gz).expect("Could not open fastq gz");
    let fq1 = flate2::read::GzDecoder::new(fq1);
    let mut fq1_reader = seq_io::fasta::Reader::new(fq1);
    // let mut read_counter = 0;
    // while let Some(record) = fq1_reader.next() {
    //     read_counter+=1;
    // }
    // println!("Number of reads: {} Median length: {}", read_counter, );

}

// fn median(numbers: &mut [i32]) -> i32 {
//     numbers.sort();
//     let mid = numbers.len() / 2;
//     numbers[mid]
// }

