// Copyright (c) 2021 Kana LLC

//! Downloads the latest Gencode reference files. In addition to functions for downloading reference files, this file includes the gencode file names and base url as an array. 
//! Each function downloads a specific file, the most popular being FASTA and GFF/GTF files related to GRCh38 primary assembly or GRCh38 reference chromosomes.

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
pub const HUMAN_RELEASE_DIRECTORY: &str = "/pub/databases/gencode/Gencode_human/latest_release/";
pub const MOUSE_RELEASE_DIRECTORY: &str = "/pub/databases/gencode/Gencode_mouse/latest_release/";

/// Convenience functions for each file type.

/// Downloads the latest version of GRCh38.primary_assembly.genome.fa.gz
/// Annotation Type | Genomic Regions Included | File Content Description | File Type
/// Genome sequence, primary assembly (GRCh38) | PRI | Nucleotide sequence of the GRCh38 primary genome assembly (chromosomes and scaffolds). The sequence region names are the same as in the GTF/GFF3 files | Fasta
pub fn download_grch38_primary_assembly_genome_fa_gz(output_directory: &std::path::Path) {
    let regex: &str = r"GRCh38.primary_assembly.genome.fa.gz";
    download_reference_file(regex, &HUMAN_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vxx.primary_assembly.annotation.gtf.gz
/// Annotation Type | Genomic Regions Included | File Content Description | File Type
/// Comprehensive gene annotation | PRI | It contains the comprehensive gene annotation on the primary assembly (chromosomes and scaffolds) sequence regions. This is a superset of the main annotation file. | GTF
pub fn download_gencode_vxx_primary_assembly_annotation_gtf_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.v\d{2}.primary_assembly.annotation.gtf.gz";
    download_reference_file(regex, &HUMAN_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.v37.primary_assembly.annotation.gff3.gz
/// Annotation Type | Genomic Regions Included | File Content Description | File Type
/// Comprehensive gene annotation | PRI | It contains the comprehensive gene annotation on the primary assembly (chromosomes and scaffolds) sequence regions. This is a superset of the main annotation file. | GFF3
pub fn download_gencode_vxx_primary_assembly_annotation_gff3_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.v\d{2}.primary_assembly.annotation.gff3.gz";
    download_reference_file(regex, &HUMAN_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of GRCh38.p13.genome.fa.gz
pub fn download_grch38_p13_genome_fa_gz(output_directory: &std::path::Path) {
    let regex: &str = r"GRCh38.p13.genome.fa.gz";
    download_reference_file(regex, &HUMAN_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vxx.2wayconspseudos.gff3.gz
pub fn download_gencode_vxx_2wayconspseudos_gff3_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.v\d{2}.2wayconspseudos.gff3.gz";
    download_reference_file(regex, &HUMAN_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vxx.2wayconspseudos.gtf.gz
pub fn download_gencode_vxx_2wayconspseudos_gtf_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.v\d{2}.2wayconspseudos.gtf.gz";
    download_reference_file(regex, &HUMAN_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vxx.annotation.gff3.gz
pub fn download_gencode_vxx_annotation_gff3_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.v\d{2}.annotation.gff3.gz";
    download_reference_file(regex, &HUMAN_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vxx.annotation.gtf.gz
pub fn download_gencode_vxx_annotation_gtf_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.v\d{2}.annotation.gtf.gz";
    download_reference_file(regex, &HUMAN_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vxx.basic.annotation.gff3.gz
pub fn download_gencode_vxx_basic_annotation_gff3_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.v\d{2}.basic.annotation.gff3.gz";
    download_reference_file(regex, &HUMAN_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vxx.basic.annotation.gtf.gz
pub fn download_gencode_vxx_basic_annotation_gtf_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.v\d{2}.basic.annotation.gtf.gz";
    download_reference_file(regex, &HUMAN_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vxx.chr_patch_hapl_scaff.annotation.gff3.gz
pub fn download_gencode_vxx_chr_patch_hapl_scaff_annotation_gff3_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.v\d{2}.chr_patch_hapl_scaff.annotation.gff3.gz";
    download_reference_file(regex, &HUMAN_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vxx.chr_patch_hapl_scaff.annotation.gtf.gz
pub fn download_gencode_vxx_chr_patch_hapl_scaff_annotation_gtf_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.v\d{2}.chr_patch_hapl_scaff.annotation.gtf.gz";
    download_reference_file(regex, &HUMAN_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vxx.chr_patch_hapl_scaff.basic.annotation.gff3.gz
pub fn download_gencode_vxx_chr_patch_hapl_scaff_basic_annotation_gff3_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.v\d{2}.chr_patch_hapl_scaff.basic.annotation.gff3.gz";
    download_reference_file(regex, &HUMAN_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vxx.chr_patch_hapl_scaff.basic.annotation.gtf.gz
pub fn download_gencode_vxx_chr_patch_hapl_scaff_basic_annotation_gtf_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.v\d{2}.chr_patch_hapl_scaff.basic.annotation.gtf.gz";
    download_reference_file(regex, &HUMAN_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vxx.lncRNA_transcripts.fa.gz
pub fn download_gencode_vxx_lncrna_transcripts_fa_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.v\d{2}.lncRNA_transcripts.fa.gz";
    download_reference_file(regex, &HUMAN_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vxx.long_noncoding_RNAs.gff3.gz
pub fn download_gencode_vxx_long_noncoding_rnas_gff3_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.v\d{2}.long_noncoding_RNAs.gff3.gz";
    download_reference_file(regex, &HUMAN_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vxx.long_noncoding_RNAs.gtf.gz
pub fn download_gencode_vxx_long_noncoding_rnas_gtf_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.v\d{2}.long_noncoding_RNAs.gtf.gz";
    download_reference_file(regex, &HUMAN_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vxx.metadata.Annotation_remark.gz
pub fn download_gencode_vxx_metadata_annotation_remark_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.v\d{2}.metadata.Annotation_remark.gz";
    download_reference_file(regex, &HUMAN_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vxx.metadata.EntrezGene.gz
pub fn download_gencode_vxx_metadata_entrezgene_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.v\d{2}.metadata.EntrezGene.gz";
    download_reference_file(regex, &HUMAN_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vxx.metadata.Exon_supporting_feature.gz
pub fn download_gencode_vxx_metadata_exon_supporting_feature_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.v\d{2}.metadata.Exon_supporting_feature.gz";
    download_reference_file(regex, &HUMAN_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vxx.metadata.Gene_source.gz
pub fn download_gencode_vxx_metadata_gene_source_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.v\d{2}.metadata.Gene_source.gz";
    download_reference_file(regex, &HUMAN_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vxx.metadata.HGNC.gz
pub fn download_gencode_vxx_metadata_hgnc_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.v\d{2}.metadata.HGNC.gz";
    download_reference_file(regex, &HUMAN_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vxx.metadata.PDB.gz
pub fn download_gencode_vxx_metadata_pdb_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.v\d{2}.metadata.PDB.gz";
    download_reference_file(regex, &HUMAN_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vxx.metadata.PolyA_feature.gz
pub fn download_gencode_vxx_metadata_polya_feature_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.v\d{2}.metadata.PolyA_feature.gz";
    download_reference_file(regex, &HUMAN_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vxx.metadata.Pubmed_id.gz
pub fn download_gencode_vxx_metadata_pubmed_id_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.v\d{2}.metadata.Pubmed_id.gz";
    download_reference_file(regex, &HUMAN_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vxx.metadata.RefSeq.gz
pub fn download_gencode_vxx_metadata_refseq_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.v\d{2}.metadata.RefSeq.gz";
    download_reference_file(regex, &HUMAN_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vxx.metadata.Selenocysteine.gz
pub fn download_gencode_vxx_metadata_selenocysteine_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.v\d{2}.metadata.Selenocysteine.gz";
    download_reference_file(regex, &HUMAN_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vxx.metadata.SwissProt.gz
pub fn download_gencode_vxx_metadata_swissprot_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.v\d{2}.metadata.SwissProt.gz";
    download_reference_file(regex, &HUMAN_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vxx.metadata.TrEMBL.gz
pub fn download_gencode_vxx_metadata_trembl_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.v\d{2}.metadata.TrEMBL.gz";
    download_reference_file(regex, &HUMAN_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vxx.metadata.Transcript_source.gz
pub fn download_gencode_vxx_metadata_transcript_source_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.v\d{2}.metadata.Transcript_source.gz";
    download_reference_file(regex, &HUMAN_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vxx.metadata.Transcript_supporting_feature.gz
pub fn download_gencode_vxx_metadata_transcript_supporting_feature_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.v\d{2}.metadata.Transcript_supporting_feature.gz";
    download_reference_file(regex, &HUMAN_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vxx.pc_transcripts.fa.gz
pub fn download_gencode_vxx_pc_transcripts_fa_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.v\d{2}.pc_transcripts.fa.gz";
    download_reference_file(regex, &HUMAN_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vxx.pc_translations.fa.gz
pub fn download_gencode_vxx_pc_translations_fa_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.v\d{2}.pc_translations.fa.gz";
    download_reference_file(regex, &HUMAN_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vxx.polyAs.gff3.gz
pub fn download_gencode_vxx_polyas_gff3_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.v\d{2}.polyAs.gff3.gz";
    download_reference_file(regex, &HUMAN_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vxx.polyAs.gtf.gz
pub fn download_gencode_vxx_polyas_gtf_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.v\d{2}.polyAs.gtf.gz";
    download_reference_file(regex, &HUMAN_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vxx.tRNAs.gff3.gz
pub fn download_gencode_vxx_trnas_gff3_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.v\d{2}.tRNAs.gff3.gz";
    download_reference_file(regex, &HUMAN_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vxx.tRNAs.gtf.gz
pub fn download_gencode_vxx_trnas_gtf_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.v\d{2}.tRNAs.gtf.gz";
    download_reference_file(regex, &HUMAN_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vxx.transcripts.fa.gz
pub fn download_gencode_vxx_transcripts_fa_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.v\d{2}.transcripts.fa.gz";
    download_reference_file(regex, &HUMAN_RELEASE_DIRECTORY, output_directory);
}

// ################################ Mouse Reference Functions ################################ 
// ################################ Mouse Reference Functions ################################ 
// ################################ Mouse Reference Functions ################################ 


/// Downloads the latest version of GRCm39.primary_assembly.genome.fa.gz
pub fn download_grcm39_primary_assembly_genome_fa_gz(output_directory: &std::path::Path) {
    let regex: &str = r"GRCm39.primary_assembly.genome.fa.gz";
    download_reference_file(regex, &MOUSE_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vMxx.primary_assembly.annotation.gff3.gz
pub fn download_gencode_vmxx_primary_assembly_annotation_gff3_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.vM\d{2}.primary_assembly.annotation.gff3.gz";
    download_reference_file(regex, &MOUSE_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vMxx.primary_assembly.annotation.gtf.gz
pub fn download_gencode_vmxx_primary_assembly_annotation_gtf_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.vM\d{2}.primary_assembly.annotation.gtf.gz";
    download_reference_file(regex, &MOUSE_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of GRCm39.genome.fa.gz
pub fn download_grcm39_genome_fa_gz(output_directory: &std::path::Path) {
    let regex: &str = r"GRCm39.genome.fa.gz";
    download_reference_file(regex, &MOUSE_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vMxx.2wayconspseudos.gff3.gz
pub fn download_gencode_vmxx_2wayconspseudos_gff3_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.vM\d{2}.2wayconspseudos.gff3.gz";
    download_reference_file(regex, &MOUSE_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vMxx.2wayconspseudos.gtf.gz
pub fn download_gencode_vmxx_2wayconspseudos_gtf_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.vM\d{2}.2wayconspseudos.gtf.gz";
    download_reference_file(regex, &MOUSE_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vMxx.annotation.gff3.gz
pub fn download_gencode_vmxx_annotation_gff3_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.vM\d{2}.annotation.gff3.gz";
    download_reference_file(regex, &MOUSE_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vMxx.annotation.gtf.gz
pub fn download_gencode_vmxx_annotation_gtf_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.vM\d{2}.annotation.gtf.gz";
    download_reference_file(regex, &MOUSE_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vMxx.basic.annotation.gff3.gz
pub fn download_gencode_vmxx_basic_annotation_gff3_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.vM\d{2}.basic.annotation.gff3.gz";
    download_reference_file(regex, &MOUSE_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vMxx.basic.annotation.gtf.gz
pub fn download_gencode_vmxx_basic_annotation_gtf_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.vM\d{2}.basic.annotation.gtf.gz";
    download_reference_file(regex, &MOUSE_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vMxx.chr_patch_hapl_scaff.annotation.gff3.gz
pub fn download_gencode_vmxx_chr_patch_hapl_scaff_annotation_gff3_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.vM\d{2}.chr_patch_hapl_scaff.annotation.gff3.gz";
    download_reference_file(regex, &MOUSE_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vMxx.chr_patch_hapl_scaff.annotation.gtf.gz
pub fn download_gencode_vmxx_chr_patch_hapl_scaff_annotation_gtf_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.vM\d{2}.chr_patch_hapl_scaff.annotation.gtf.gz";
    download_reference_file(regex, &MOUSE_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vMxx.chr_patch_hapl_scaff.basic.annotation.gff3.gz
pub fn download_gencode_vmxx_chr_patch_hapl_scaff_basic_annotation_gff3_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.vM\d{2}.chr_patch_hapl_scaff.basic.annotation.gff3.gz";
    download_reference_file(regex, &MOUSE_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vMxx.chr_patch_hapl_scaff.basic.annotation.gtf.gz
pub fn download_gencode_vmxx_chr_patch_hapl_scaff_basic_annotation_gtf_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.vM\d{2}.chr_patch_hapl_scaff.basic.annotation.gtf.gz";
    download_reference_file(regex, &MOUSE_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vMxx.lncRNA_transcripts.fa.gz
pub fn download_gencode_vmxx_lncrna_transcripts_fa_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.vM\d{2}.lncRNA_transcripts.fa.gz";
    download_reference_file(regex, &MOUSE_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vMxx.long_noncoding_RNAs.gff3.gz
pub fn download_gencode_vmxx_long_noncoding_rnas_gff3_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.vM\d{2}.long_noncoding_RNAs.gff3.gz";
    download_reference_file(regex, &MOUSE_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vMxx.long_noncoding_RNAs.gtf.gz
pub fn download_gencode_vmxx_long_noncoding_rnas_gtf_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.vM\d{2}.long_noncoding_RNAs.gtf.gz";
    download_reference_file(regex, &MOUSE_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vMxx.metadata.Annotation_remark.gz
pub fn download_gencode_vmxx_metadata_annotation_remark_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.vM\d{2}.metadata.Annotation_remark.gz";
    download_reference_file(regex, &MOUSE_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vMxx.metadata.EntrezGene.gz
pub fn download_gencode_vmxx_metadata_entrezgene_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.vM\d{2}.metadata.EntrezGene.gz";
    download_reference_file(regex, &MOUSE_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vMxx.metadata.Exon_supporting_feature.gz
pub fn download_gencode_vmxx_metadata_exon_supporting_feature_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.vM\d{2}.metadata.Exon_supporting_feature.gz";
    download_reference_file(regex, &MOUSE_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vMxx.metadata.Gene_source.gz
pub fn download_gencode_vmxx_metadata_gene_source_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.vM\d{2}.metadata.Gene_source.gz";
    download_reference_file(regex, &MOUSE_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vMxx.metadata.MGI.gz
pub fn download_gencode_vmxx_metadata_mgi_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.vM\d{2}.metadata.MGI.gz";
    download_reference_file(regex, &MOUSE_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vMxx.metadata.PDB.gz
pub fn download_gencode_vmxx_metadata_pdb_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.vM\d{2}.metadata.PDB.gz";
    download_reference_file(regex, &MOUSE_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vMxx.metadata.PolyA_feature.gz
pub fn download_gencode_vmxx_metadata_polya_feature_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.vM\d{2}.metadata.PolyA_feature.gz";
    download_reference_file(regex, &MOUSE_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vMxx.metadata.Pubmed_id.gz
pub fn download_gencode_vmxx_metadata_pubmed_id_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.vM\d{2}.metadata.Pubmed_id.gz";
    download_reference_file(regex, &MOUSE_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vMxx.metadata.RefSeq.gz
pub fn download_gencode_vmxx_metadata_refseq_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.vM\d{2}.metadata.RefSeq.gz";
    download_reference_file(regex, &MOUSE_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vM\d{2}.metadata.Selenocysteine.gz
pub fn download_gencode_vmxx_metadata_selenocysteine_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.vM\d{2}.metadata.Selenocysteine.gz";
    download_reference_file(regex, &MOUSE_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vMxx.metadata.SwissProt.gz
pub fn download_gencode_vmxx_metadata_swissprot_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.vM\d{2}.metadata.SwissProt.gz";
    download_reference_file(regex, &MOUSE_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vMxx.metadata.TrEMBL.gz
pub fn download_gencode_vmxx_metadata_trembl_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.vM\d{2}.metadata.TrEMBL.gz";
    download_reference_file(regex, &MOUSE_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vMxx.metadata.Transcript_source.gz
pub fn download_gencode_vmxx_metadata_transcript_source_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.vM\d{2}.metadata.Transcript_source.gz";
    download_reference_file(regex, &MOUSE_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vMxx.metadata.Transcript_supporting_feature.gz
pub fn download_gencode_vmxx_metadata_transcript_supporting_feature_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.vM\d{2}.metadata.Transcript_supporting_feature.gz";
    download_reference_file(regex, &MOUSE_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vMxx.pc_transcripts.fa.gz
pub fn download_gencode_vmxx_pc_transcripts_fa_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.vM\d{2}.pc_transcripts.fa.gz";
    download_reference_file(regex, &MOUSE_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vMxx.pc_translations.fa.gz
pub fn download_gencode_vmxx_pc_translations_fa_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.vM\d{2}.pc_translations.fa.gz";
    download_reference_file(regex, &MOUSE_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vMxx.polyAs.gff3.gz
pub fn download_gencode_vmxx_polyas_gff3_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.vM\d{2}.polyAs.gff3.gz";
    download_reference_file(regex, &MOUSE_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vMxx.polyAs.gtf.gz
pub fn download_gencode_vmxx_polyas_gtf_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.vM\d{2}.polyAs.gtf.gz";
    download_reference_file(regex, &MOUSE_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vMxx.tRNAs.gff3.gz
pub fn download_gencode_vmxx_trnas_gff3_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.vM\d{2}.tRNAs.gff3.gz";
    download_reference_file(regex, &MOUSE_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vMxx.tRNAs.gtf.gz
pub fn download_gencode_vmxx_trnas_gtf_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.vM\d{2}.tRNAs.gtf.gz";
    download_reference_file(regex, &MOUSE_RELEASE_DIRECTORY, output_directory);
}

/// Downloads the latest version of gencode.vM26.transcripts.fa.gz
pub fn download_gencode_vmxx_transcripts_fa_gz(output_directory: &std::path::Path) {
    let regex: &str = r"gencode.vM\d{2}.transcripts.fa.gz";
    download_reference_file(regex, &MOUSE_RELEASE_DIRECTORY, output_directory);
}

// General reference download function
pub fn download_reference_file(regex: &str, ftp_directory: &str, output_directory: &std::path::Path) {
    // Connect and make ftp stream
    let mut ftp_stream = FtpStream::connect(&FTP_SITE).expect("Cannot connect to {}");
    let _ = ftp_stream.login("anonymous", "").unwrap();
    let _ = ftp_stream.cwd(ftp_directory).unwrap();
    let file_paths = ftp_stream.nlst(Some(ftp_directory)).unwrap();
    // Get base names of files as osstr vector from file paths string vector
    let file_names: Vec<&std::ffi::OsStr> = file_paths.iter().map(|x| Path::new(x).file_name().unwrap()).collect();
    // Search files for a file matching the regex, get an index in the vector for the match
    let target_index=search_files(&file_names, regex);
    // Input the index into the vectors of paths and file names to get the matching file
    let target_name = &file_names[target_index];
    let target_path = &file_paths[target_index];
    // Create a stream of the file and download to a file of the same name on disk
    // Convert directory str to a new path and join the target file name
    // let output_directory = Path::new(&output_directory);
    let target_output_file = &output_directory.join(&target_name);
    // Create a file on disk in the specified directory and make a writer to it
    let file = File::create(&target_output_file).unwrap();
    let mut buf = BufWriter::new(file);
    // Stream the data and write to the file with the bufwriter
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

// gencode.v37.transcripts.fa.gz
// gencode.v37.pc_transcripts.fa.gz
// gencode.v37.pc_translations.fa.gz
// gencode.v37.lncRNA_transcripts.fa.gz
// GRCh38.p13.genome.fa.gz
// GRCh38.primary_assembly.genome.fa.gz


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


