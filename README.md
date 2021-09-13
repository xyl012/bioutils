# Bioutils

New and updated! (but still a work in progress)

For a related bioinformatics resource:
[Bioinformatics in Rust](https://kana4.github.io/bioinformatics_rust_book/)

```
// Downloading reference files
// Download grch38 fasta gz and gtf gz to current directory
let path = Path::new("./");
download_grch38_primary_assembly_genome_fa_gz(&path);
download_gencode_vxx_primary_assembly_annotation_gtf_gz(&path);

// Download files through http
let fastq_ftp = "ftp://ftp.sra.ebi.ac.uk/vol1/fastq/SRR170/009/SRR1700869/";
let fastq_gz = "SRR1700869.fastq.gz";
let out_directory = Path::new("./")
bioutils::files::http::curl(fastq_ftp, fastq_gz, &out_directory);

// Checks if all elements in the slice are contained in a character set. Boolean version shown, but also available in result and option.

let dna_reference = &[67,67,67,67];
assert!(dna_reference.is_all_charset(BioUtilsCharSet::Dna), true);

// Also possible to use a custom &[u8] to check if all elements are contained in the character set.
let dna_reference_2 = &[67,67,67,67];
assert!(dna_reference_2.is_all_charset_with(&[2u8,3u8,4u8]), true);

// Create a new random vector of specific length with a Bioutils charset or choosing from u8s in a given slice

let test = Vec::<u8>::random_vec(&12, BioUtilsCharSet::Dna);
let test2 = Vec::<u8>::random_vec_with(&12, &[1,2,3,4]);
println!("{:?}", test);
println!("{:?}", test2);

// Takes a BioUtilsCharSet and a ThreadRng and replaces any character not in the charset with a random character from the characterset.
// create a random number generator
let mut rng1 = rand::thread_rng(); 
let mut dna_correction = b"ACTGQQQ";
dna_correction.mut_clean(BioUtilsCharSet::Dna, rng1);

// Recode can be used if we want to convert between PHRED, etc. scores and encodings or find the complement. Recode checks if contains non-encoding values and will return none if there are any.

// Get the reverse complement of the sequence. Other options for finding the complement are available.
let mut reverse_complement = b"ACTG".to_owned();
reverse_complement.mut_rev_recode(BioUtilsRecodeSet::DnaComplement);
println!("{:?}", reverse_complement);

let mut phred33_score = 12u8;
phred33_score.recode_u8(BioUtilsRecodeSet::Phred33Encode);
println!("{:?}", phred33_score); 

let mut phred33_encoding = 34u8;
&phred33_encoding.mut_recode_u8(BioUtilsRecodeSet::Phred33Decode);
println!("{:?}", phred33_encoding);

let mut phred33_score_2 = b"00000".to_owned();
phred33_score_2.mut_recode(BioUtilsRecodeSet::Phred33Encode);
println!("{:?}", phred33_score_2);

```