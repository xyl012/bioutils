# Bioutils

Open for feature requests and edits!

For a related bioinformatics resource:
[Bioinformatics in Rust](https://kana4.github.io/bioinformatics_rust_book/)

    // Download the latest grch38 reference fasta gz and gtf gz to current directory
    use std::path::Path;
    let path = Path::new("./");
    download_grch38_primary_assembly_genome_fa_gz(&path);
    download_gencode_vxx_primary_assembly_annotation_gtf_gz(&path);

    // Download a fastq from the ENA
    use std::path::Path;
    use bioutils::files::http::bioutils_curl;
    let fastq_ftp = "ftp://ftp.sra.ebi.ac.uk/vol1/fastq/SRR170/009/SRR1700869/";
    let fastq_gz = "SRR1700869.fastq.gz";
    let out_directory = Path::new("./");
    bioutils_curl(fastq_ftp, fastq_gz, &out_directory);    

    // Checks if all elements in the slice are contained in a character set. Boolean version shown, but also available in result and option.
    use bioutils::utils::check::AllAsRefSlice;
    use bioutils::charsets::bioutils::*;
    let dna_reference = &[67u8,67u8,67u8,67u8];
    println!("{:?}", dna_reference.result_is_all_charset(BioUtilsCharSet::Dna));

    // Also possible to use a custom &[u8] to check if all elements are contained in the character set.
    let dna_reference_2 = &[67u8,68u8,67u8,68u8];
    println!("{:?}", dna_reference_2.result_is_all_charset_with(&[67u8, 68u8]));
    let dna_reference_3 = &[67u8,68u8,67u8,67u8];
    println!("{:?}", dna_reference_3.result_is_all_charset_with(&[67u8,68u8]).unwrap().is_all_charset_with(&[5u8,6u8,7u8]));

    // Create a new random vector of specific length with a Bioutils charset or choosing from u8s in a given slice

    use bioutils::utils::new::RandomBioVec;
    use bioutils::charsets::bioutils::*;
    let new_vec = Vec::<u8>::random_vec(&12, BioUtilsCharSet::Dna);
    let new_vec_2 = Vec::<u8>::random_vec_with(&12, &[1u8,2u8,3u8]);

    // Takes a BioUtilsCharSet and a ThreadRng and replaces any character not in the charset with a random character from the characterset.
    // Also possible with custom &[u8]
    // create a random number generator
    let mut rng1 = rand::thread_rng(); 
    let mut dna_correction = b"ACTGQQQ";
    dna_correction.mut_clean(BioUtilsCharSet::Dna, rng1);
    // dna_correction.mut_clean_with(&[1u8,2u8,3u8], rng1);

    // Recode can be used if we want to convert between PHRED, etc. scores and encodings or find the complement. Recode checks if contains non-encoding values and will return none if there are any.

    // Get the reverse complement of the sequence. Other options for finding the complement are available (mix case, Nn included).
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

    // Wrapper around memchr that returns an iterator over cg sites
    use memchr::memmem;
    use memchr::memmem::FindIter;
    use bioutils::utils::find::*;
    let seqcg = b"ACGA";
    // Returns an iterator over the cg sites
    let mut iter = seqcg.iter_cg();
    // Returns all cpg sites in a vector
    let cgpos = b"ACGA".all_positions_cg();