# Bioutils

For a related bioinformatics resource:
[Bioinformatics in Rust](https://kana4.github.io/bioinformatics_rust_book/)
ls

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
//! // Creating a new random sequence and quality 
//!
//! let mut rng1 = rand::thread_rng(); // Create a random number generator
//! let dna = random_dna(4,rng1); // Create a random dna sequence
//! let mut rng2 = rand::thread_rng();
//! let quality = random_quality(4,rng2); // Create a random quality string
//! println!("{:?}", dna.to_owned());
//! println!("{:?}", quality.to_owned());
//!
//! // Replacing nucleotides
//!
//! let mut rng3 = rand::thread_rng(); //create a random number generator
//! let mut rng4 = rand::thread_rng(); //create a random number generator
//! let mut seq = b"acugnnnqqq".to_owned(); // or by *: let mut seq = *b"acugnnnqqq";
//! let mut seq = seq.mut_random_replace_non_basic("RNA", rng4).mut_random_replace_n("RNA", rng3).mut_to_upper_basic();
//! let printseq = str::from_utf8(seq).unwrap();
//! println!("{:?}", printseq);
```