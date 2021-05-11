# Bioutils

## Simple Biological Utilities in Rust

<p>Bioutils provides simple biological utilities including: 
    <ul> Functions to check sequence validity and content (palindromes too!)</ul>
    <ul> Functions to create new random IUPAC sequences</ul>
    <ul> Functions to download human and mouse Gencode reference files</ul>
    <ul> Functions to download fastq files</ul>
    <ul> Functions to replace N or gaps with pseudorandom nucleotides</ul>
    <ul>complete iupac and quality character sets</ul>
</p>

<p> Check back as more functionality gets added, pull requests welcome!</p>
<h2 id="quick-start" class="section-header"><a href="#quick-start">Quick Start</a></h2>
<div class="example-wrap"><pre class="rust rust-example-rendered"><p>//! Check out the download_read_align_seq_io.rs example for a full practical walkthrough from downloading files to finding read positions!

//! # Bioutils: Simple Biological Utilities in Rust
//! Bioutils provides simple biological utilities including:
//! Functions to check sequence validity and content (palindromes too!) 
//! Functions to create new random IUPAC sequences
//! Functions to download human and mouse Gencode reference files
//! Functions to download fastq files
//! Functions to replace N or gaps with pseudorandom nucleotides
//! Complete iupac and quality character sets (and quality charset with matching shifted value).
//! 
//! ## Quick Start
//!
//! // Examples for using checks:
//! 
//! use bioutils::charsets;
//! use bioutils::utils;
//! use crate::bioutils::utils::check::value::CheckU8;
//! use bioutils::utils::new::random::random_dna;
//! use bioutils::utils::new::random::random_quality;
//! use bioutils::utils::mutate::random::AsMutRandomU8;
//! use rand::rngs::ThreadRng;
//! use rand::seq::SliceRandom;
//! use std::string::String;
//! use std::str;
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
//! assert!(quality.is_solexa());
//! 
//! // We can also do checks this way:
//! assert!(quality.check_u8("is_phred33").unwrap());
//! assert!(dna.check_u8("is_basic_dna").unwrap());
//!
//! // Examples for creating a new random sequence and quality 
//!
//! let mut rng1 = rand::thread_rng(); // Create a random number generator
//! let dna = random_dna(4,rng1); // Create a random dna sequence
//! let mut rng2 = rand::thread_rng();
//! let quality = random_quality(4,rng2); // Create a random quality string
//! println!("{:?}", dna.to_owned());
//! println!("{:?}", quality.to_owned());
//!
//! // Examples for replacing nucleotides
//!
//! let mut rng3 = rand::thread_rng(); //create a random number generator
//! let mut rng4 = rand::thread_rng(); //create a random number generator
//! let mut seq = b"acugnnnqqq".to_owned(); // or by *: let mut seq = *b"acugnnnqqq";
//! let mut seq = seq.mut_random_replace_non_basic("RNA", rng4).mut_random_replace_n("RNA", rng3).mut_to_upper_basic();
//! let printseq = str::from_utf8(seq).unwrap();
//! println!("{:?}", printseq);

</p></pre></div>
</div><h2 id='modules' class='section-header'><a href="#modules">Modules</a></h2>
<table><tr class='module-item'><td><a class="mod" href="charsets/index.html" title='bioutils::charsets mod'>charsets</a></td><td class='docblock-short'><p>Numerous IUPAC character sets to either use directly or create your own mix and match</p></td></tr><tr class='module-item'><td><a class="mod" href="files/index.html" title='bioutils::files mod'>files</a></td><td class='docblock-short'><p>Download human and mouse Gencode references, download fastq sample files</p></td></tr><tr class='module-item'><td><a class="mod" href="references/index.html" title='bioutils::references mod'>references</a></td><td class='docblock-short'><p>Currently includes human NCBI gencode GRCh38. Automatically downloads the latest version of user's choice.</p>
</td></tr><tr class='module-item'><td><a class="mod" href="utils/index.html" title='bioutils::utils mod'>utils</a></td><td class='docblock-short'><p>Functions for sequence checks, pseudorandom replacement of N or gaps, and functions to create new pseudoranndom sequences</p></td></tr></table></section><section id="search" class="content hidden"></section><section class="footer"></section>