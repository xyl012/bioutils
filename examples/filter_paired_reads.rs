// Copyright (c) 2021 Kana LLC

//! This example filters for paired reads in fastq files and returns the number of reads after filtering.

use bioutils::files::fastq::hash;
use seq_io::fastq::Record as FastqRecord;
use std::fs::File;

fn main()-> std::io::Result<()>{
    // Create references/samples directories by creating a new path and creating all directories if the directory doesn't exist
    let references_directory = std::path::Path::new("./data/references/");
    let samples_directory = std::path::Path::new("./data/samples/");
    std::fs::create_dir_all(&references_directory)?;
    std::fs::create_dir_all(&samples_directory)?;
    let fastq_ftp = "ftp://ftp.sra.ebi.ac.uk/vol1/fastq/SRR065/SRR065495/";
    let fq1_gz = "SRR065495_1.fastq.gz";
    let fq2_gz = "SRR065495_2.fastq.gz";
    let fq1_path = &samples_directory.join(fq1_gz);
    let fq2_path = &samples_directory.join(fq2_gz);
    println!("Reference functions take a path, we make this with std::path::path::new()");
    println!("Downloading fastq1 with: ");
    println!("bioutils::files::http::curl()");
    println!("...Please Wait...");
    println!("...If over 10 minutes, there may be a connection issue...");
    bioutils::files::http::curl(fastq_ftp, fq1_gz, &samples_directory);
    println!("Downloading fastq2 with: ");
    println!("bioutils::files::http::curl()");
    println!("...Please Wait...");
    println!("...If over 10 minutes, there may be a connection issue...");
    bioutils::files::http::curl(fastq_ftp, fq2_gz, &samples_directory);
    println!("Create fastq reader");
    let fq1 = File::open(&fq1_path).expect("Could not open input Fastq");
    let fq1 = flate2::read::GzDecoder::new(fq1);
    let mut fq1_reader = seq_io::fastq::Reader::new(fq1);
    println!("Create fastq reader");
    let fq2 = File::open(&fq2_path).expect("Could not open input Fastq");
    let fq2 = flate2::read::GzDecoder::new(fq2);
    let mut fq2_reader = seq_io::fastq::Reader::new(fq2);
    println!("Filter fastqs");
    let format = "sra";
    let field = "head";
    let filtered_read_names = bioutils::files::fastq::hash::find_paired_fastq_reads(fq1_reader, fq2_reader, &field, &format);
    println!("Number of reads after filter: {}", filtered_read_names.len());
    Ok(())
}