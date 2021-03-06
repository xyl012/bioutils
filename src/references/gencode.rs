// Copyright 2021 Christopher Sugai

//! Downloads the latest Gencode reference files. Currently includes all GRCh38 files. Each function downloads a specific file, the most popular being FASTA and GFF/GTF files related to GRCh38 primary assembly or GRCh38 reference chromosomes

use crate::references::functions::*;

use std::io::{Write};
use std::fs::File;
use curl::easy::Easy;
use url::{Url, ParseError};

// Base url
pub const BASE_URL: &str = "ftp://ftp.ebi.ac.uk/pub/databases/gencode/Gencode_human/latest_release/";
// Vector of file names within the base url
pub const FILENAMES: Vec<&str> = vec!["GRCh38.primary_assembly.genome.fa.gz", "gencode.v37.annotation.gtf.gz"];

// Comprehensive gene annotation on the reference chromosomes only. Main annotation file for most users.

pub fn download_grch38_primary_assembly_genome_fa_gz() {
    download_reference(url: BASE_URL, filename: FILENAMES[1]);
}

// pub fn download_gencode_vxx_annotation_gtf_gz() {
//     download_reference(url: BASE_URL, filename: FILENAMES[2]);
// }



// fn download_reference() {
//     // Generate url to get file
//     let mut easy = Easy::new();
//     let url = "ftp://ftp.ebi.ac.uk/pub/databases/gencode/Gencode_human/latest_release/";
//     let filename = "GRCh38.primary_assembly.genome.fa.gz";
//     let file_url = build_url(url, filename).expect("Cannot build file url").to_string();
//     easy.url(&file_url).unwrap();
//     // Create file on system to write on
//     let mut file = match File::create(filename) {
//         Err(why) => panic!("couldn't create {}", why),
//         Ok(file) => file,
//     };
//     // Write to file on system
//     easy.write_function(move |data| {
//         file.write_all(data).unwrap();
//         Ok(data.len())
//     }).unwrap();
//     easy.perform().unwrap();
//     println!("{}", easy.response_code().unwrap());
// }

// /// Reference options for downloading. Currently includes human NCBI gencode GRCh38 and UCSC hg38. Automatically downloads the latest version of user's choice.

// /// Function to intake a base url and a filename and paste them together to generate a url where the file may be found.
// fn build_url(url: &str, filename: &str) -> Result<Url, ParseError> {
//     let base = Url::parse(url).expect("Base url cannot be parsed");
//     let joined = base.join(filename)?;
//     Ok(joined)
// }


// Gencode files downloaded and their descriptions.

// Annotation Type | Genomic Regions Included | File Content Description | File Type
// GTF/GFF

// Comprehensive gene annotation | CHR | It contains the comprehensive gene annotation on the reference chromosomes only. This is the main annotation file for most users | GTF GFF3
// Comprehensive gene annotation | ALL | It contains the comprehensive gene annotation on the reference chromosomes, scaffolds, assembly patches and alternate loci (haplotypes). This is a superset of the main annotation file | GTF GFF3
// Comprehensive gene annotation | PRI | It contains the comprehensive gene annotation on the primary assembly (chromosomes and scaffolds) sequence regions. This is a superset of the main annotation file. | GTF GFF3
// Basic gene annotation | CHR | It contains the basic gene annotation on the reference chromosomes only. This is a subset of the corresponding comprehensive annotation, including only those transcripts tagged as 'basic' in every gene | GTF GFF3
// Basic gene annotation | ALL | It contains the basic gene annotation on the reference chromosomes, scaffolds, assembly patches and alternate loci (haplotypes). This is a subset of the corresponding comprehensive annotation, including only those transcripts tagged as 'basic' in every gene | GTF GFF3
// Long non-coding RNA gene annotation | CHR | It contains the comprehensive gene annotation of lncRNA genes on the reference chromosomes | This is a subset of the main annotation file. | GTF GFF3
// PolyA feature annotation | CHR | It contains the polyA features (polyA_signal, polyA_site, pseudo_polyA) manually annotated by HAVANA on the reference chromosomes. This dataset does not form part of the main annotation file | GTF GFF3
// Consensus pseudogenes predicted by the Yale and UCSC pipelines | CHR | 2-way consensus (retrotransposed) pseudogenes predicted by the Yale and UCSC pipelines, but not by HAVANA, on the reference chromosomes. This dataset does not form part of the main annotation file | GTF GFF3
// Predicted tRNA genes	CHR	tRNA genes predicted by ENSEMBL on the reference chromosomes using tRNAscan-SE | This dataset does not form part of the main annotation file | GTF GFF3

// Annotation Type | Genomic Regions Included | File Content Description | File Type
// FASTA

// Transcript sequences | CHR | Nucleotide sequences of all transcripts on the reference chromosomes | Fasta
// Protein-coding transcript sequences | CHR | Nucleotide sequences of coding transcripts on the reference chromosomes Transcript biotypes: protein_coding, nonsense_mediated_decay, non_stop_decay, IG_*_gene, TR_*_gene, polymorphic_pseudogene | Fasta
// Protein-coding transcript translation sequences | CHR | Amino acid sequences of coding transcript translations on the reference chromosomes. Transcript biotypes: protein_coding, nonsense_mediated_decay, non_stop_decay, IG_*_gene, TR_*_gene, polymorphic_pseudogene | Fasta
// Long non-coding RNA transcript sequences | CHR | Nucleotide sequences of long non-coding RNA transcripts on the reference chromosomes | Fasta
// Genome sequence (GRCh38.p13) | ALL | Nucleotide sequence of the GRCh38.p13 genome assembly version on all regions, including reference chromosomes, scaffolds, assembly patches and haplotypes. The sequence region names are the same as in the GTF/GFF3 files | Fasta
// Genome sequence, primary assembly (GRCh38) | PRI | Nucleotide sequence of the GRCh38 primary genome assembly (chromosomes and scaffolds). The sequence region names are the same as in the GTF/GFF3 files | Fasta

// Metadata files

// Annotation Type | Genomic Regions Included | File Content Description | File Type
// Annotation remarks | ALL | Remarks made during the manual annotation of the transcript | Metadata
// Entrez gene ids | ALL | Entrez gene ids associated to GENCODE transcripts (from Ensembl xref pipeline) | Metadata
// Exon annotation evidence | ALL | Piece of evidence used in the annotation of an exon (usually peptides, mRNAs, ESTs) | Metadata
// Gene source | ALL | Source of the gene annotation (Ensembl, Havana, Ensembl-Havana merged model or imported in the case of small RNA and mitochondrial genes) | Metadata
// Gene symbol | ALL | HGNC approved gene symbol (from Ensembl xref pipeline) | Metadata
// PDB id | ALL | PDB entries associated to the transcript (from Ensembl xref pipeline) | Metadata
// PolyA features | ALL | Manually annotated polyA features overlapping the transcript 3'-end | Metadata
// PubMed id | ALL | Pubmed ids of publications associated to the transcript (from HGNC website) | Metadata
// RefSeq | ALL | RefSeq RNA and/or protein associated to the transcript (from Ensembl xref pipeline) | Metadata
// Selenocysteine | ALL | Amino acid position of a selenocysteine residue in the transcript | Metadata
// SwissProt | ALL | UniProtKB/SwissProt entry associated to the transcript (from Ensembl xref pipeline) | Metadata
// Transcript source | ALL | Source of the transcript annotation | Metadata
// Transcript annotation evidence | ALL | Piece of evidence used in the annotation of the transcript | Metadata
// TrEMBL | ALL | UniProtKB/TrEMBL entry associated to the transcript (from Ensembl xref pipeline) | Metadata


