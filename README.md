# Bioutils

<h2 id="quick-start" class="section-header"><a href="#quick-start">Simple Biological Utilities in Rust</a></h2>

<p>Bioutils provides simple biological utilities including: 
    <ul> Functions to create new random IUPAC sequences</ul>
    <ul> Functions to download human and mouse Gencode reference files</ul>
    <ul> Common genomic sequences (PhiX and ERCC) </ul>
    <ul> Functions to download fastq files</ul>
    <ul> Functions to replace N or gaps with pseudorandom nucleotides</ul>
    <ul> Functions to check sequence validity and content (palindromes too!)</ul>
    <ul>complete iupac and quality character sets</ul>
</p>

<p> Pull requests welcome, suggestions for improvement always welcome!</p>

</p></pre></div>
</div><h2 id='modules' class='section-header'><a href="#modules">Modules</a></h2>
<table><tr class='module-item'><td><a class="mod" href="charsets/index.html" title='bioutils::charsets mod'>charsets</a></td><td class='docblock-short'><p>Numerous IUPAC character sets to either use directly or create your own mix and match.</p></td></tr><tr class='module-item'><td><a class="mod" href="files/index.html" title='bioutils::files mod'>files</a></td><td class='docblock-short'><p>Download fastq sample files with curl using a url. See step 2 download files example in bioutils/examples.</p></td></tr><tr class='module-item'><td><a class="mod" href="references/index.html" title='bioutils::references mod'>references</a></td><td class='docblock-short'><p>Currently includes human NCBI gencode GRCh38. Automatically downloads the latest version of user's choice. See step 2 download files example in bioutils/examples.</p>
</td></tr><tr class='module-item'><td><a class="mod" href="utils/index.html" title='bioutils::utils mod'>utils</a></td><td class='docblock-short'><p>Functions for sequence checks, pseudorandom replacement of N or gaps, and functions to create new pseudoranndom sequences.</p></td></tr>
<tr class='module-item'><td><a class="mod" href="charsets/index.html" title='bioutils::charsets mod'>image</a></td><td class='docblock-short'><p>Create images from biological data. See Image example in bioutils/examples.</p></td></tr>
</table></section><section id="search" class="content hidden"></section><section class="footer"></section>

<h2 id="quick-start" class="section-header"><a href="#quick-start">TL;DR</a></h2>
<div class="example-wrap"><pre class="rust rust-example-rendered"><p>


//! // Examples for downloading refernce files
//! // Download grch38 fasta gz and gtf gz to current directory
//! let path = Path::new("./");
//! download_grch38_primary_assembly_genome_fa_gz(&path);
//! download_gencode_vxx_primary_assembly_annotation_gtf_gz(&path);
//! // Download files through http
//! let fastq_ftp = "ftp://ftp.sra.ebi.ac.uk/vol1/fastq/SRR170/009/SRR1700869/";
//! let fastq_gz = "SRR1700869.fastq.gz";
//! let out_directory = Path::new("./")
//! bioutils::files::http::curl(fastq_ftp, fastq_gz, &out_directory);
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
