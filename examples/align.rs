// Copyright 2021 Christopher Sugai

//! This example creates a simple (D/R/...)NA aligner that can be run from the command line.
//! Steps in this example:
//! Download and read the latest GrCH38 reference
//! Download and read a sample fastq
//! Uses rust bio to create a lookup structure for the reference
//! Find positions of input fastq sequences in the reference by searching the lookup structure

use seq_io::fasta::Record as FastaRecord;
use seq_io::fastq::Record as FastqRecord;

use std::fs::File;

use bioutils::references::ftp::download_grch38_primary_assembly_genome_fa_gz;
use suffix_array::SuffixArray;

fn main()-> std::io::Result<()>{
    // // Create references/samples directories by creating a new path and creating all directories if the directory doesn't exist
    let references_directory = std::path::Path::new("./data/references/");
    let samples_directory = std::path::Path::new("./data/samples/");
    std::fs::create_dir_all(&references_directory)?;
    std::fs::create_dir_all(&samples_directory)?;
    let fastq_ftp = "ftp://ftp.sra.ebi.ac.uk/vol1/fastq/SRR170/009/SRR1700869/";
    let fastq_gz = "SRR1700869.fastq.gz";
    let reference_name = "GRCh38.primary_assembly.genome.fa.gz";
    let fastq_file = &samples_directory.join("SRR1700869.fastq.gz");
    println!("Downloading reference with: ");
    println!("download_grch38_primary_assembly_genome_fa_gz()");
    println!("...Please Wait...");
    println!("...If over 10 minutes, there may be a connection issue...");
    download_grch38_primary_assembly_genome_fa_gz(&references_directory);
    println!("Reference functions take a path, we make this with std::path::path::new()");
    println!("Downloading fastq with: ");
    println!("bioutils::files::http::curl()");
    println!("...Please Wait...");
    println!("...If over 10 minutes, there may be a connection issue...");
    // example Illumina format
    bioutils::files::http::curl(fastq_ftp, fastq_gz, &samples_directory);
    // 10x Genomics sra format
    // bioutils::files::http::curl("ftp://ftp.sra.ebi.ac.uk/vol1/fastq/SRR641/000/SRR6413640/",fastq_gz);
    // 10x Genomics raw format
    // bioutils::files::http::curl("https://sra-pub-sars-cov2.s3.amazonaws.com/sra-src/SRR13734384/","IVAR2_5D_CKDL200155936-1a-SI_GA_B2_H5NG5DSXY_S3_L004_R2_001.fastq.gz.1");
    println!("We now have the reference in our references directory");
    println!("Let's check which are available");
    let ref_paths = std::fs::read_dir(&references_directory).unwrap();
    for ref_path in ref_paths {
        println!("Reference: {}", ref_path.unwrap().path().display())
    }
    println!("We now have the sample data in our samples directory");
    println!("Let's check which are available");
    let smpl_paths = std::fs::read_dir(&samples_directory).unwrap();
    for smpl_path in smpl_paths {
        println!("Sample: {}", smpl_path.unwrap().path().display())
    }
    println!("Read reference (GrCH38) fasta (fa) gz");
    let reference = File::open(&references_directory.join(&reference_name)).expect("Could not open reference fasta (fa) gz");
    let reference = flate2::read::GzDecoder::new(reference);
    let mut reference_reader = seq_io::fasta::Reader::new(reference);

    println!("Create fastq reader");
    let fq = File::open(&fastq_file).expect("Could not open input Fastq");
    let fq = flate2::read::GzDecoder::new(fq);
    let mut fq_reader = seq_io::fastq::Reader::new(fq);

    while let Some(record) = reference_reader.next() {
        let record = record.expect("Error reading record");
        println!("...Get chromosome sequence, this should print multiple times...");
        let mut chromosome = record.full_seq();
        let record_head = record.head();
        println!("...Creating positive strand reference lookup structure, this should print multiple times...");
        let sa = SuffixArray::new(&chromosome);
        println!("...Get positions of reads on positive strand, this should print multiple times...");
        let positions_forward = fastq_aligner(&mut fq_reader, sa);
        println!("...Creating negative strand reference lookup structure, this should print multiple times...");
        let reverse_chromosome = chromosome.to_mut();
        reverse_chromosome.reverse();
        let sar = SuffixArray::new(&reverse_chromosome);
        println!("...Get positions of reads on negative strand, this should print multiple times...");
        let positions_reverse = fastq_aligner(&mut fq_reader, sar);
    }
    Ok(())
}


pub fn fastq_aligner<R>(fq_reader: &mut seq_io::fastq::Reader<R>, sa: SuffixArray) 
where 
R: std::io::Read,
{
    println!("Read input fastq and find read positions");
    let mut read_counter = 0;
    while let Some(record) = fq_reader.next() {
        let record = record.expect("Error reading record");
        // let id = record.id().unwrap();
        let seq = record.seq();
        let sa = SuffixArray::new(b"GATCGATCGATCGATC");
        let positions = sa.search_all(seq);
        // let lcp = sa.search_lcp(b"splash");
        read_counter+=1;
        println!("Reads aligned: {}",read_counter);
    }
}

