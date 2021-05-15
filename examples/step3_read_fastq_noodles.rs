
use std::borrow::Borrow;
use std::fs::File;

use bioutils::references::ftp::download_grch38_primary_assembly_genome_fa_gz;
use bioutils::utils::check::value::CheckU8;
use suffix_array::SuffixArray;
use noodles::fastq;
use std::{
    // env,
    // fs::File,
    io::{BufReader},
};

fn main()-> std::io::Result<()>{

    // Running everything from Step 1 and 2

    // Create references/samples directories by creating a new path and creating all directories if the directory doesn't exist
    let references_directory = std::path::Path::new("./data/references/");
    let samples_directory = std::path::Path::new("./data/samples/");
    std::fs::create_dir_all(&references_directory)?;
    std::fs::create_dir_all(&samples_directory)?;
    let fastq_ftp = "ftp://ftp.sra.ebi.ac.uk/vol1/fastq/SRR170/009/SRR1700869/";
    let fastq_gz = "SRR1700869.fastq.gz";
    let reference_name = "GRCh38.primary_assembly.genome.fa.gz";
    let fastq_file = &samples_directory.join("SRR1700869.fastq.gz");
    
    // Get our reference paths
    let ref_paths = std::fs::read_dir(&references_directory).unwrap();
    // Print our reference paths
    for ref_path in ref_paths {
        println!("Reference: {}", ref_path.unwrap().path().display())
    }
    println!("We now have the sample data in our samples directory");
    println!("Let's check which are available");
    let smpl_paths = std::fs::read_dir(&samples_directory).unwrap();

    // Step 1 and 2 Complete!

    for smpl_path in smpl_paths {
        let mut reader =  File::open(smpl_path.unwrap().path())
        .map(flate2::read::GzDecoder::new)
        .map(BufReader::new)
        .map(fastq::Reader::new)?;
    
        let mut record = fastq::Record::default();
        let mut n = 0;
        let mut n_homopolymers = 0;

        loop {
            match reader.read_record(&mut record) {
                Ok(0) => break,
                Ok(_) => {n += 1; if record.sequence().is_homopolymer() {n_homopolymers +=1 } else {continue}}, // Count homopolymers
                Err(e) => return Err(e),
            }
            println!("Reads read: {}", n);
        }
        println!("Reads read: {}", n);
        println!("Number of homopolymers: {}", n_homopolymers);
    }
    Ok(())
}