# Bioutils

New and updated! (but still a work in progress)

For a related bioinformatics resource:
[Bioinformatics in Rust](https://kana4.github.io/bioinformatics_rust_book/)

```
//! // Downloading reference files
//! // Download grch38 fasta gz and gtf gz to current directory
//! let path = Path::new("./");
//! download_grch38_primary_assembly_genome_fa_gz(&path);
//! download_gencode_vxx_primary_assembly_annotation_gtf_gz(&path);
//!
//! // Download files through http
//! let fastq_ftp = "ftp://ftp.sra.ebi.ac.uk/vol1/fastq/SRR170/009/SRR1700869/";
//! let fastq_gz = "SRR1700869.fastq.gz";
//! let out_directory = Path::new("./")
//! bioutils::files::http::curl(fastq_ftp, fastq_gz, &out_directory);
//! 
//! // Checks if all elements in the slice are contained in a character set. Boolean version shown, but also available in result and option.
//! 
//! let dna_reference = &[67,67,67,67];
//! assert!(dna_reference.is_all_charset(BioUtilsCharSet::Dna), true);
//! 
//! // Create a new random vector of specific length with a Bioutils charset or choosing from u8s in a given slice
//! 
//! let test = Vec::<u8>::random_vec(&12, BioUtilsCharSet::Dna);
//! let test2 = Vec::<u8>::random_vec_with(&12, &[1,2,3,4]);
//! println!("{:?}", test);
//! println!("{:?}", test2);
//! 
//! /// Takes a BioUtilsCharSet and a ThreadRng and replaces any character not in the charset with a random character from the characterset.
//! let mut rng1 = rand::thread_rng(); //create a random number generator
//! let mut dna_correction = b"ACTGQQQ";
//! dna_correction.mut_clean(BioUtilsCharSet::Dna, rng1);
```