// Copyright 2021 Christopher Sugai

//! This example creates a simple (D/R/...)NA aligner that can be run from the command line.
//! Steps in this example:
//! Download and read the latest GrCH38 reference
//! Download and read a sample fastq
//! Uses rust bio to create a lookup structure for the reference
//! Find positions of input fastq sequences in the reference by searching the lookup structure

use bioutils::references::ftp;
use bioutils::io::http;

/// gunzip gencode.vM26.pc_translations.fa.gz
/// wc -l gencode.vM26.pc_translations.fa.gz
fn main() {
    println!("Downloading reference with: ");
    println!("bioutils::references::ftp::download_gencode_vmxx_pc_translations_fa_gz()");
    bioutils::references::ftp::download_gencode_vmxx_pc_translations_fa_gz();
    println!("Downloading fastq with: ");
    println!("bioutils::io::http::curl("https://github.com/otsukaresamadeshita/test_files/raw/main/","40_lines_R1_001.fastq.gz")");
    bioutils::io::http::curl("https://github.com/otsukaresamadeshita/test_files/raw/main/","40_lines_R1_001.fastq.gz")
    println!("Read reference fasta");
    // let fq = File::open(fastq_path).expect("Could not open Fastq");
    // let fq = flate2::read::GzDecoder::new(fq).into_inner();
    // seq_io::fastq::Reader::new(fq);
    println!("Create reference lookup structure");
    println!("Read input fastq");

    // let fq = File::open(fastq_path).expect("Could not open Fastq");
    // let fq = flate2::read::GzDecoder::new(fq).into_inner();
    // seq_io::fastq::Reader::new(fq);
    println!("Generate lookup structure");

}
