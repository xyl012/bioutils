
// Import libraries we're using
use seq_io::fasta::Record as FastaRecord;
use seq_io::fastq::Record as FastqRecord;
use std::fs::File;
use bioutils::references::ftp::download_grch38_primary_assembly_genome_fa_gz;
use suffix_array::SuffixArray;

// Everything is run from the main function in Rust programs, while libraries don't have a main function.
fn main()-> std::io::Result<()>{
    // Create references/samples directories by creating a new path and creating all directories if the directory doesn't exist
    let references_directory = std::path::Path::new("./data/references/");
    let samples_directory = std::path::Path::new("./data/samples/");
    std::fs::create_dir_all(&references_directory)?;
    std::fs::create_dir_all(&samples_directory)?;
    let fastq_ftp = "ftp://ftp.sra.ebi.ac.uk/vol1/fastq/SRR170/009/SRR1700869/";
    let fastq_gz = "SRR1700869.fastq.gz";
    let reference_name = "GRCh38.primary_assembly.genome.fa.gz";
    let fastq_file = &samples_directory.join("SRR1700869.fastq.gz");
    Ok(())
}