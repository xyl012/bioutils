// Copyright 2021 Christopher Sugai

//! This example creates a simple (D/R/...)NA aligner that can be run from the command line.
//! Steps in this example:
//! Download and read the latest GrCH38 reference
//! Download and read a sample fastq
//! Uses rust bio to create a lookup structure for the reference
//! Find positions of input fastq sequences in the reference by searching the lookup structure



use std::fs::File;

use seq_io::fastq::Record;
use bioutils::references::ftp::download_grch38_primary_assembly_genome_fa_gz;







fn main()-> std::io::Result<()>{
    // Create references/samples directories by creating a new path and creating all directories if the directory doesn't exist
    let references_directory = std::path::Path::new("./data/references");
    let samples_directory = std::path::Path::new("./data/samples");
    std::fs::create_dir_all(&references_directory)?;
    std::fs::create_dir_all(&samples_directory)?;
    let fastq_ftp= "ftp://ftp.sra.ebi.ac.uk/vol1/fastq/SRR170/009/SRR1700869/";
    let fastq_gz="SRR1700869.fastq.gz";
    let _reference_path="GRCh38.primary_assembly.genome.fa.gz";
    println!("Downloading reference with: ");
    println!("download_grch38_primary_assembly_genome_fa_gz()");
    println!("Reference functions take a path, we make this with std::path::path::new()");
    download_grch38_primary_assembly_genome_fa_gz(&references_directory);
    println!("Downloading fastq with: ");
    println!("bioutils::files::http::curl()");
    // example
    bioutils::files::http::curl(fastq_ftp, fastq_gz, &samples_directory);
    // 10x sra format
    // bioutils::files::http::curl("ftp://ftp.sra.ebi.ac.uk/vol1/fastq/SRR641/000/SRR6413640/",fastq_gz);
    // 10x raw format
    // bioutils::files::http::curl("https://sra-pub-sars-cov2.s3.amazonaws.com/sra-src/SRR13734384/","IVAR2_5D_CKDL200155936-1a-SI_GA_B2_H5NG5DSXY_S3_L004_R2_001.fastq.gz.1");
    
    // println!("Read reference (GrCH38) fasta");
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
    // 

    println!("Read input fastq and find read positions");
    let fq = File::open(fastq_gz).expect("Could not open input Fastq");
    let fq = flate2::read::GzDecoder::new(fq);
    let mut fq_reader = seq_io::fastq::Reader::new(fq);
    while let Some(record) = fq_reader.next() {
        let record = record.expect("Error reading record");
        // let id = record.id().unwrap();
        let _seq = record.seq();
    }
    Ok(())
}

// #[derive(StructOpt)]
// #[structopt(name = "dd_test", about = "Get disk read bandwidth.")]
// struct Opt {
//     #[structopt(short, default_value = "4096")]
//     block_size: u64,
//     #[structopt(short, default_value = "1")]
//     thread_num: u64,
//     #[structopt(short)]
//     file: String,
// }

// use cmd_lib::*;
// use std::io::Result;

// fn main() {


// let msg = "I love rust";
// run_cmd!(du -ah . | sort -hr | head -n 10)?;
// }
