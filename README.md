# Bioutils

## Simple Biological Utilities with Rust's [u8]

<p>Bioutils provides simple biological utilities including: 
    <ul>complete iupac and quality character sets</ul>
    <ul> Functions to check sequence validity and content (palindromes too!)</ul>
    <ul> Functions to download human and mouse Gencode reference files</ul>
    <ul> Functions to download fastq files</ul>
    <ul> Functions to replace N or gaps with pseudorandom nucleotides</ul>
    <ul> Functions to create new random IUPAC sequences</ul>
</p>

<p> Please take a look at the align example to get a full practical walkthrough!</p>

<li>Character sets include punctuation, are subdivided, and implemented in Rust's [u8] rather than bitset</li>
<li>Implementations are centered around [u8], although character sets are also provided as [&amp;str], hashset u8 and hashset &amp;str.
Check back as more functionality gets added!</li>
</ul>
<h2 id="quick-start" class="section-header"><a href="#quick-start">Quick Start</a></h2>
<div class="example-wrap"><pre class="rust rust-example-rendered"><p>

//! Check out the align example for a full practical walkthrough from downloading files to finding read positions!


//! This example creates a simple (D/R/...)NA aligner that can be run from the command line.
//! Steps in this example:
//! 
//! Download and read the latest GrCH38 reference
//! Download and read a sample fastq
//! Create a lookup structure (suffix array) for each chromosome and it's reverse complement
//! Find positions of input fastq sequences in the reference by searching the lookup structure
//! You can make this example by creating it with `cargo build --example align`. This will generate a command line tool that can simply be run with `./align`. The command line tool will be in bioutils/target/debug/examples
//! 
//! This example will keep aliging reads for quite some time, as it is aligning a complete fastq to the complete human genome with one core. Please cancel it with cntrl+c at any time during the alignment. 
//! 
//! Even with the 'long' alignment, we use very little resources and computation time to generate a practical example.
//! 

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



//! Examples for using checks:

//! use bioutils::charsets::*;
//! use bioutils::utils::*;
//! use bioutils::utils::check::CheckU8;
//!
//! let dna = b"ACTG";
//! let rna = b"ACUG";
//! let homopolymerN = b"NNNN";
//! let homopolymerA = b"AAAA";
//! let gapna = b"AC-G";
//! let nna = b"ACnG";
//! let quality = b"@ABC";
//!
//! assert!(homopolymerN.is_homopolymer());
//! assert!(homopolymerA.is_homopolymer_not_n());
//! assert!(homopolymerN.is_homopolymer_n());
//!
//! assert!(gapna.has_gap());
//! assert!(nna.has_n());
//! assert!(dna.is_iupac());
//! assert!(rna.is_basic_rna());
//!
//! assert!(quality.is_phred33());
//! assert!(quality.is_phred64());
//! assert!(quality.is_solexa());</p></pre></div>
</div><h2 id='modules' class='section-header'><a href="#modules">Modules</a></h2>
<table><tr class='module-item'><td><a class="mod" href="charsets/index.html" title='bioutils::charsets mod'>charsets</a></td><td class='docblock-short'><p>Numerous IUPAC character sets to either use directly or create your own mix and match</p></td></tr><tr class='module-item'><td><a class="mod" href="files/index.html" title='bioutils::files mod'>files</a></td><td class='docblock-short'><p>Download human and mouse Gencode references, download fastq sample files</p></td></tr><tr class='module-item'><td><a class="mod" href="references/index.html" title='bioutils::references mod'>references</a></td><td class='docblock-short'><p>Currently includes human NCBI gencode GRCh38. Automatically downloads the latest version of user's choice.</p>
</td></tr><tr class='module-item'><td><a class="mod" href="utils/index.html" title='bioutils::utils mod'>utils</a></td><td class='docblock-short'><p>Functions for sequence checks, pseudorandom replacement of N or gaps, and functions to create new pseudoranndom sequences</p></td></tr></table></section><section id="search" class="content hidden"></section><section class="footer"></section>