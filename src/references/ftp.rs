// Copyright 2021 Christopher Sugai

// //! Downloads the latest Gencode reference files. In addition to functions for downloading reference files, this file includes the gencode file names and base url as an array. 
// //! Each function downloads a specific file, the most popular being FASTA and GFF/GTF files related to GRCh38 primary assembly or GRCh38 reference chromosomes.

extern crate ftp;
extern crate regex;

use std::io::Read;


use std::io::Write;
use std::io::BufWriter;

use std::path::Path;
use std::fs::File;
use std::str;

use ftp::FtpStream;
use regex::Regex;

pub const FTP_SITE: &str = "ftp.ebi.ac.uk:21";
pub const HUMAN_RELEASE_URL: &str = "/pub/databases/gencode/Gencode_human/latest_release/";
pub const MOUSE_RELEASE_URL: &str = "/pub/databases/gencode/Gencode_mouse/latest_release/";

// General reference download function
pub fn download_reference_file(regex: &str, url: &str) {
    // Connect and make ftp stream
    let mut ftp_stream = FtpStream::connect(&FTP_SITE).expect("Cannot connect to {}");
    let _ = ftp_stream.login("anonymous", "").unwrap();
    let _ = ftp_stream.cwd(url).unwrap();
    let file_paths = ftp_stream.nlst(Some(url)).unwrap();
    // Get base names of files as osstr vector from file paths string vector
    let file_names: Vec<&std::ffi::OsStr> = file_paths.iter().map(|x| Path::new(x).file_name().unwrap()).collect();
    // Search files for a file matching the regex, get an index in the vector for the match
    let target_index=search_files(&file_names, regex);
    // Input the index into the vectors of paths and file names to get the matching file
    let target_name = &file_names[target_index];
    let target_path = &file_paths[target_index];
    // Create a stream of the file and download to a file of the same name on disk
    let file = File::create(&target_name).unwrap();
    let mut buf = BufWriter::new(file);
    let mut cursor = ftp_stream.simple_retr(target_path).unwrap();
    // cursor.seek(SeekFrom::Start(0)).unwrap();
    let mut out = Vec::new();
    cursor.read_to_end(&mut out).unwrap();
    buf.write_all(&out).expect("Error writing file");
}

// For use within download_reference_file to search all files within the directory for a file name matching the regex and return the index in the vector that matches.
pub fn search_files(file_names: &Vec<&std::ffi::OsStr>, regex: &str)-> usize {
    let re = Regex::new(regex).unwrap();
    let target_index: usize = file_names.iter().position(|&x| re.is_match(x.to_str().expect("Cannot convert file name to UTF8"))).expect("Regex cannot match a target file");
    target_index
}

/// Convenience functions for each file type.

/// Downloads the latest version of GRCh38.primary_assembly.genome.fa.gz
/// Annotation Type | Genomic Regions Included | File Content Description | File Type
/// Genome sequence, primary assembly (GRCh38) | PRI | Nucleotide sequence of the GRCh38 primary genome assembly (chromosomes and scaffolds). The sequence region names are the same as in the GTF/GFF3 files | Fasta
pub fn download_grch38_primary_assembly_genome_fa_gz() {
    let regex: &str = r"GRCh38.primary_assembly.genome.fa.gz";
    download_reference_file(regex, &HUMAN_RELEASE_URL);
}

/// Downloads the latest version of gencode.vxx.primary_assembly.annotation.gtf.gz
/// Annotation Type | Genomic Regions Included | File Content Description | File Type
/// Comprehensive gene annotation | PRI | It contains the comprehensive gene annotation on the primary assembly (chromosomes and scaffolds) sequence regions. This is a superset of the main annotation file. | GTF
pub fn download_gencode_vxx_primary_assembly_annotation_gtf_gz() {
    let regex: &str = r"gencode.v\d{2}.primary_assembly.annotation.gtf.gz";
    download_reference_file(regex, &HUMAN_RELEASE_URL);
}

/// Downloads the latest version of gencode.v37.primary_assembly.annotation.gff3.gz
/// Annotation Type | Genomic Regions Included | File Content Description | File Type
/// Comprehensive gene annotation | PRI | It contains the comprehensive gene annotation on the primary assembly (chromosomes and scaffolds) sequence regions. This is a superset of the main annotation file. | GFF3
pub fn download_gencode_vxx_primary_assembly_annotation_gff3_gz() {
    let regex: &str = r"gencode.v\d{2}.primary_assembly.annotation.gff3.gz";
    download_reference_file(regex, &HUMAN_RELEASE_URL);
}

/// Downloads the latest version of GRCh38.p13.genome.fa.gz
pub fn download_grch38_p13_genome_fa_gz() {
    let regex: &str = r"GRCh38.p13.genome.fa.gz";
    download_reference_file(regex, &HUMAN_RELEASE_URL);
}

/// Downloads the latest version of gencode.vxx.2wayconspseudos.gff3.gz
pub fn download_gencode_vxx_2wayconspseudos_gff3_gz() {
    let regex: &str = r"gencode.v\d{2}.2wayconspseudos.gff3.gz";
    download_reference_file(regex, &HUMAN_RELEASE_URL);
}

/// Downloads the latest version of gencode.vxx.2wayconspseudos.gtf.gz
pub fn download_gencode_vxx_2wayconspseudos_gtf_gz() {
    let regex: &str = r"gencode.v\d{2}.2wayconspseudos.gtf.gz";
    download_reference_file(regex, &HUMAN_RELEASE_URL);
}

/// Downloads the latest version of gencode.vxx.annotation.gff3.gz
pub fn download_gencode_vxx_annotation_gff3_gz() {
    let regex: &str = r"gencode.v\d{2}.annotation.gff3.gz";
    download_reference_file(regex, &HUMAN_RELEASE_URL);
}

/// Downloads the latest version of gencode.vxx.annotation.gtf.gz
pub fn download_gencode_vxx_annotation_gtf_gz() {
    let regex: &str = r"gencode.v\d{2}.annotation.gtf.gz";
    download_reference_file(regex, &HUMAN_RELEASE_URL);
}

/// Downloads the latest version of gencode.vxx.basic.annotation.gff3.gz
pub fn download_gencode_vxx_basic_annotation_gff3_gz() {
    let regex: &str = r"gencode.v\d{2}.basic.annotation.gff3.gz";
    download_reference_file(regex, &HUMAN_RELEASE_URL);
}

/// Downloads the latest version of gencode.vxx.basic.annotation.gtf.gz
pub fn download_gencode_vxx_basic_annotation_gtf_gz() {
    let regex: &str = r"gencode.v\d{2}.basic.annotation.gtf.gz";
    download_reference_file(regex, &HUMAN_RELEASE_URL);
}

/// Downloads the latest version of gencode.vxx.chr_patch_hapl_scaff.annotation.gff3.gz
pub fn download_gencode_vxx_chr_patch_hapl_scaff_annotation_gff3_gz() {
    let regex: &str = r"gencode.v\d{2}.chr_patch_hapl_scaff.annotation.gff3.gz";
    download_reference_file(regex, &HUMAN_RELEASE_URL);
}

/// Downloads the latest version of gencode.vxx.chr_patch_hapl_scaff.annotation.gtf.gz
pub fn download_gencode_vxx_chr_patch_hapl_scaff_annotation_gtf_gz() {
    let regex: &str = r"gencode.v\d{2}.chr_patch_hapl_scaff.annotation.gtf.gz";
    download_reference_file(regex, &HUMAN_RELEASE_URL);
}

/// Downloads the latest version of gencode.vxx.chr_patch_hapl_scaff.basic.annotation.gff3.gz
pub fn download_gencode_vxx_chr_patch_hapl_scaff_basic_annotation_gff3_gz() {
    let regex: &str = r"gencode.v\d{2}.chr_patch_hapl_scaff.basic.annotation.gff3.gz";
    download_reference_file(regex, &HUMAN_RELEASE_URL);
}

/// Downloads the latest version of gencode.vxx.chr_patch_hapl_scaff.basic.annotation.gtf.gz
pub fn download_gencode_vxx_chr_patch_hapl_scaff_basic_annotation_gtf_gz() {
    let regex: &str = r"gencode.v\d{2}.chr_patch_hapl_scaff.basic.annotation.gtf.gz";
    download_reference_file(regex, &HUMAN_RELEASE_URL);
}

/// Downloads the latest version of gencode.vxx.lncRNA_transcripts.fa.gz
pub fn download_gencode_vxx_lncrna_transcripts_fa_gz() {
    let regex: &str = r"gencode.v\d{2}.lncRNA_transcripts.fa.gz";
    download_reference_file(regex, &HUMAN_RELEASE_URL);
}

/// Downloads the latest version of gencode.vxx.long_noncoding_RNAs.gff3.gz
pub fn download_gencode_vxx_long_noncoding_rnas_gff3_gz() {
    let regex: &str = r"gencode.v\d{2}.long_noncoding_RNAs.gff3.gz";
    download_reference_file(regex, &HUMAN_RELEASE_URL);
}

/// Downloads the latest version of gencode.vxx.long_noncoding_RNAs.gtf.gz
pub fn download_gencode_vxx_long_noncoding_rnas_gtf_gz() {
    let regex: &str = r"gencode.v\d{2}.long_noncoding_RNAs.gtf.gz";
    download_reference_file(regex, &HUMAN_RELEASE_URL);
}

/// Downloads the latest version of gencode.vxx.metadata.Annotation_remark.gz
pub fn download_gencode_vxx_metadata_annotation_remark_gz() {
    let regex: &str = r"gencode.v\d{2}.metadata.Annotation_remark.gz";
    download_reference_file(regex, &HUMAN_RELEASE_URL);
}

/// Downloads the latest version of gencode.vxx.metadata.EntrezGene.gz
pub fn download_gencode_vxx_metadata_entrezgene_gz() {
    let regex: &str = r"gencode.v\d{2}.metadata.EntrezGene.gz";
    download_reference_file(regex, &HUMAN_RELEASE_URL);
}

/// Downloads the latest version of gencode.vxx.metadata.Exon_supporting_feature.gz
pub fn download_gencode_vxx_metadata_exon_supporting_feature_gz() {
    let regex: &str = r"gencode.v\d{2}.metadata.Exon_supporting_feature.gz";
    download_reference_file(regex, &HUMAN_RELEASE_URL);
}

/// Downloads the latest version of gencode.vxx.metadata.Gene_source.gz
pub fn download_gencode_vxx_metadata_gene_source_gz() {
    let regex: &str = r"gencode.v\d{2}.metadata.Gene_source.gz";
    download_reference_file(regex, &HUMAN_RELEASE_URL);
}

/// Downloads the latest version of gencode.vxx.metadata.HGNC.gz
pub fn download_gencode_vxx_metadata_hgnc_gz() {
    let regex: &str = r"gencode.v\d{2}.metadata.HGNC.gz";
    download_reference_file(regex, &HUMAN_RELEASE_URL);
}

/// Downloads the latest version of gencode.vxx.metadata.PDB.gz
pub fn download_gencode_vxx_metadata_pdb_gz() {
    let regex: &str = r"gencode.v\d{2}.metadata.PDB.gz";
    download_reference_file(regex, &HUMAN_RELEASE_URL);
}

/// Downloads the latest version of gencode.vxx.metadata.PolyA_feature.gz
pub fn download_gencode_vxx_metadata_polya_feature_gz() {
    let regex: &str = r"gencode.v\d{2}.metadata.PolyA_feature.gz";
    download_reference_file(regex, &HUMAN_RELEASE_URL);
}

/// Downloads the latest version of gencode.vxx.metadata.Pubmed_id.gz
pub fn download_gencode_vxx_metadata_pubmed_id_gz() {
    let regex: &str = r"gencode.v\d{2}.metadata.Pubmed_id.gz";
    download_reference_file(regex, &HUMAN_RELEASE_URL);
}

/// Downloads the latest version of gencode.vxx.metadata.RefSeq.gz
pub fn download_gencode_vxx_metadata_refseq_gz() {
    let regex: &str = r"gencode.v\d{2}.metadata.RefSeq.gz";
    download_reference_file(regex, &HUMAN_RELEASE_URL);
}

/// Downloads the latest version of gencode.vxx.metadata.Selenocysteine.gz
pub fn download_gencode_vxx_metadata_selenocysteine_gz() {
    let regex: &str = r"gencode.v\d{2}.metadata.Selenocysteine.gz";
    download_reference_file(regex, &HUMAN_RELEASE_URL);
}

/// Downloads the latest version of gencode.vxx.metadata.SwissProt.gz
pub fn download_gencode_vxx_metadata_swissprot_gz() {
    let regex: &str = r"gencode.v\d{2}.metadata.SwissProt.gz";
    download_reference_file(regex, &HUMAN_RELEASE_URL);
}

/// Downloads the latest version of gencode.vxx.metadata.TrEMBL.gz
pub fn download_gencode_vxx_metadata_trembl_gz() {
    let regex: &str = r"gencode.v\d{2}.metadata.TrEMBL.gz";
    download_reference_file(regex, &HUMAN_RELEASE_URL);
}

/// Downloads the latest version of gencode.vxx.metadata.Transcript_source.gz
pub fn download_gencode_vxx_metadata_transcript_source_gz() {
    let regex: &str = r"gencode.v\d{2}.metadata.Transcript_source.gz";
    download_reference_file(regex, &HUMAN_RELEASE_URL);
}

/// Downloads the latest version of gencode.vxx.metadata.Transcript_supporting_feature.gz
pub fn download_gencode_vxx_metadata_transcript_supporting_feature_gz() {
    let regex: &str = r"gencode.v\d{2}.metadata.Transcript_supporting_feature.gz";
    download_reference_file(regex, &HUMAN_RELEASE_URL);
}

/// Downloads the latest version of gencode.vxx.pc_transcripts.fa.gz
pub fn download_gencode_vxx_pc_transcripts_fa_gz() {
    let regex: &str = r"gencode.v\d{2}.pc_transcripts.fa.gz";
    download_reference_file(regex, &HUMAN_RELEASE_URL);
}

/// Downloads the latest version of gencode.vxx.pc_translations.fa.gz
pub fn download_gencode_vxx_pc_translations_fa_gz() {
    let regex: &str = r"gencode.v\d{2}.pc_translations.fa.gz";
    download_reference_file(regex, &HUMAN_RELEASE_URL);
}

/// Downloads the latest version of gencode.vxx.polyAs.gff3.gz
pub fn download_gencode_vxx_polyas_gff3_gz() {
    let regex: &str = r"gencode.v\d{2}.polyAs.gff3.gz";
    download_reference_file(regex, &HUMAN_RELEASE_URL);
}

/// Downloads the latest version of gencode.vxx.polyAs.gtf.gz
pub fn download_gencode_vxx_polyas_gtf_gz() {
    let regex: &str = r"gencode.v\d{2}.polyAs.gtf.gz";
    download_reference_file(regex, &HUMAN_RELEASE_URL);
}

/// Downloads the latest version of gencode.vxx.tRNAs.gff3.gz
pub fn download_gencode_vxx_trnas_gff3_gz() {
    let regex: &str = r"gencode.v\d{2}.tRNAs.gff3.gz";
    download_reference_file(regex, &HUMAN_RELEASE_URL);
}

/// Downloads the latest version of gencode.vxx.tRNAs.gtf.gz
pub fn download_gencode_vxx_trnas_gtf_gz() {
    let regex: &str = r"gencode.v\d{2}.tRNAs.gtf.gz";
    download_reference_file(regex, &HUMAN_RELEASE_URL);
}

/// Downloads the latest version of gencode.vxx.transcripts.fa.gz
pub fn download_gencode_vxx_transcripts_fa_gz() {
    let regex: &str = r"gencode.v\d{2}.transcripts.fa.gz";
    download_reference_file(regex, &HUMAN_RELEASE_URL);
}

// ################################ Mouse Reference Functions ################################ 



