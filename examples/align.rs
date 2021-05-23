
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
//! Even with the 'long' alignment, we use very little resources.
//! 
use seq_io::fasta::Record as FastaRecord;
use seq_io::fastq::Record as FastqRecord;

use std::fs::File;

use bioutils::references::ftp::download_grch38_primary_assembly_genome_fa_gz;
use suffix_array::SuffixArray;

fn main()-> std::io::Result<()>{

    // Running everything from Step 1

    // Create references/samples directories by creating a new path and creating all directories if the directory doesn't exist
    let references_directory = std::path::Path::new("./data/references/");
    let samples_directory = std::path::Path::new("./data/samples/");
    std::fs::create_dir_all(&references_directory)?;
    std::fs::create_dir_all(&samples_directory)?;
    let fastq_ftp = "ftp://ftp.sra.ebi.ac.uk/vol1/fastq/SRR170/009/SRR1700869/";
    let fastq_gz = "SRR1700869.fastq.gz";
    let reference_name = "GRCh38.primary_assembly.genome.fa.gz";
    let fastq_file = &samples_directory.join("SRR1700869.fastq.gz");
    
    // Step 1 Complete!
    // Running everything from Step 2

    // Now we start downloading our files with a lot of printing as we go.
    // This may fail or give an error if the connection is interrupted. Please try again and be patient if so.
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

    // Example Illumina format
    bioutils::files::http::curl(fastq_ftp, fastq_gz, &samples_directory);

    println!("We now have the reference in our references directory");
    println!("Let's check which are available");

    // Get our reference paths
    let ref_paths = std::fs::read_dir(&references_directory).unwrap();
    // Print our reference paths
    for ref_path in ref_paths {
        println!("Reference: {}", ref_path.unwrap().path().display())
    }
    println!("We now have the sample data in our samples directory");
    println!("Let's check which are available");
    let smpl_paths = std::fs::read_dir(&samples_directory).unwrap();
    for smpl_path in smpl_paths {
        println!("Sample: {}", smpl_path.unwrap().path().display())
    }

    // Step 2 Complete!
    // Running everything from Step 3

    println!("Read reference (GrCH38) fasta (fa) gz");
    let reference = File::open(&references_directory.join(&reference_name)).expect("Could not open reference fasta (fa) gz");
    let reference = flate2::read::GzDecoder::new(reference);
    let mut reference_reader = seq_io::fasta::Reader::new(reference);

    println!("Create fastq reader");
    let fq = File::open(&fastq_file).expect("Could not open input Fastq");
    let fq = flate2::read::GzDecoder::new(fq);
    let mut fq_reader = seq_io::fastq::Reader::new(fq);

    // Step 3 Complete!
    // Running everything from Step 4

    let search_seq = b"ACTGACTGACTG";
    
    while let Some(record) = reference_reader.next() {
        let record = record.expect("Error reading record");
        println!("...Get chromosome sequence, this should print multiple times...");
        let mut chromosome = record.full_seq();
        let record_head = record.head();
        println!("...Creating positive strand reference lookup structure, this should print multiple times...");
        let sa = SuffixArray::new(&chromosome);
        println!("...Get positions of reads on positive strand, this should print multiple times...");
        let positions_forward = fastq_aligner(&mut fq_reader, sa, search_seq);
        println!("...Creating negative strand reference lookup structure, this should print multiple times...");
        let reverse_chromosome = chromosome.to_mut();
        reverse_chromosome.reverse();
        let sar = SuffixArray::new(&reverse_chromosome);
        println!("...Get positions of reads on negative strand, this should print multiple times...");
        let positions_reverse = fastq_aligner(&mut fq_reader, sar, search_seq);
    }
    Ok(())
}


// This is our code to find positions with the suffix array we created.
pub fn fastq_aligner<R>(fq_reader: &mut seq_io::fastq::Reader<R>, sa: SuffixArray, search_seq: &[u8]) 
where 
R: std::io::Read,
{
    println!("Read input fastq and find read positions");
    let mut read_counter = 0;
    while let Some(record) = fq_reader.next() {
        let record = record.expect("Error reading record");
        let seq = record.seq();
        let sa = SuffixArray::new(search_seq);
        let positions = sa.search_all(seq);
        // let lcp = sa.search_lcp(b"ACTG");
        read_counter+=1;
        println!("Reads aligned: {}",read_counter);
    }
}



