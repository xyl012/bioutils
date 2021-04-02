// Copyright 2021 Christopher Sugai

//! This example creates a simple (D/R/...)NA aligner that can be run from the command line.
//! Steps in this example:
//! Download and read the latest GrCH38 reference
//! Download and read a sample fastq
//! Uses rust bio to create a lookup structure for the reference
//! Find positions of input fastq sequences in the reference by searching the lookup structure

use std::path::Path;
use std::fs::File;
use std::fs::create_dir;
use seq_io::fastq::Record;
use bioutils::references::ftp::download_grch38_primary_assembly_genome_fa_gz;
use bioutils::references::ftp;
use bioutils::files::http;
use bioutils::charsets::*;
use flate2::*;
use seq_io::fasta::*;
use seq_io::fastq::*;

fn main()-> std::io::Result<()>{
    // Create references and samples directories
    let data_directory = Path::new("./data/");
    let reference_directory = "references/";
    let references_directory = data_directory.join(&reference_directory);
    let samples_directory = "samples/";
    let samples_directory = data_directory.join(&samples_directory);
    let fastq_path="SRR1700869.fastq.gz";
    let reference_path="GRCh38.primary_assembly.genome.fa.gz";
    create_dir(data_directory)?;
    create_dir(references_directory)?;
    create_dir(samples_directory)?;
    
    println!("Downloading reference with: ");
    println!("download_grch38_primary_assembly_genome_fa_gz()");
    download_grch38_primary_assembly_genome_fa_gz(&references_directory);
    println!("Downloading fastq with: ");
    println!("bioutils::files::http::curl()");
    // example
    // bioutils::files::http::curl("ftp://ftp.sra.ebi.ac.uk/vol1/fastq/SRR170/009/SRR1700869/", fastq_path);
    // 10x sra format
    // bioutils::files::http::curl("ftp://ftp.sra.ebi.ac.uk/vol1/fastq/SRR641/000/SRR6413640/","SRR6413640.fastq.gz");
    // 10x raw format
    // bioutils::files::http::curl("https://sra-pub-sars-cov2.s3.amazonaws.com/sra-src/SRR13734384/","IVAR2_5D_CKDL200155936-1a-SI_GA_B2_H5NG5DSXY_S3_L004_R2_001.fastq.gz.1");
    println!("Read reference (GrCH38) fasta");
    // let reference = File::open(reference_path).expect("Could not open reference fasta (fa) gz");
    // let reference = flate2::read::GzDecoder::new(reference);
    // let mut reference_reader = seq_io::fasta::Reader::new(reference);
    // while let Some(record) = reference_reader.next() {
    //     let record = record.expect("Error reading record");
    //     let chromosome = record.full_seq();
    //     // record.head();
    //     // record.seq();
    // }
    println!("Create reference lookup structure");
    println!("Read input fastq and find read positions");
    // let fq = File::open(fastq_path).expect("Could not open input Fastq");
    // let fq = flate2::read::GzDecoder::new(fq);
    // let mut fq_reader = seq_io::fastq::Reader::new(fq);
    // while let Some(record) = fq_reader.next() {
    //     let record = record.expect("Error reading record");
    //     // let id = record.id().unwrap();
    //     let seq = record.seq();
    // }
    Ok(())
}




