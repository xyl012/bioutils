//! Flags and standard Samtools tags

use super::*;

/// Flags In Order: read paired: 1, read mapped in proper pair: 2, read unmapped: 4, mate unmapped: 8, read reverse strand: 16, mate reverse strand: 32, first in pair: 64, second in pair: 128, not primary alignment: 256, read fails quality checks: 512, read is PCR or optical duplicate: 1024, supplementary alignment: 2048
pub const FLAG_U16: [u16; 12] = [1,2,4,8,16,32,64,128,256,512,1024,2048];
lazy_static! {
    /// Common alignment explanations with their associated bit
    pub static ref FLAG_HASHMAP: HashMap<&'static str, u16> = vec![("is_paired",1), ("is_proper_pair",2), ("reads unmapped", 4), ("mate unmapped", 8), ("read reverse strand", 16), ("mate reverse strand", 32), ("first in pair", 64), ("second in pair", 128), ("not primary alignment",256), ("read fails quality checks", 512), ("read is PCR or optical duplicate", 1024), ("supplementary alignment",2048)].into_iter().collect();
}

// Predefined standard tags are listed in the following table and described in greater detail in later subsections. Optional fields are usually displayed as TAG:TYPE:VALUE; the type may be one of A (character), B (general array), f (real number), H (hexadecimal array), i (integer), or Z (string).
pub const TAG: [&[u8]; 63] = [b"AM", b"AS", b"BC", b"BQ", b"BZ", b"CB", b"CC", b"CG", b"CM", b"CO", b"CP", b"CQ", b"CR", b"CS", b"CT", b"CY", b"E2", b"FI", b"FS", b"FZ", b"GZ", b"GQ", b"GS", b"H0", b"H1", b"H2", b"HI", b"IH", b"LB", b"MC", b"MD", b"MF", b"MI", b"MQ", b"NH", b"NM", b"OA", b"OC", b"OP", b"OQ", b"OX", b"PG", b"PQ", b"PT", b"PU", b"Q2", b"QT", b"QX", b"R2", b"RG", b"RT", b"RX", b"S2", b"SA", b"SM", b"SQ", b"TC", b"TS", b"U2", b"UQ", b"X?", b"Y?", b"Z?"];
pub const TAG_EXPLANATION: [&str; 63] = [r#"The smallest template-independent mapping quality in the template"#, r#"Alignment score generated by aligner"#, r#"Barcode sequence identifying the sample"#, r#"Offset to base alignment quality (BAQ)"#, r#"Phred quality of the unique molecular barcode bases in the OX tag"#, r#"Cell identifier"#, r#"Reference name of the next hit"#, r#"CIGAR in BAM’s binary encoding if (and only if) it consists of >65535 operators"#, r#"Edit distance between the color sequence and the color reference (see also NM)"#, r#"Free-text comments"#, r#"Leftmost coordinate of the next hit"#, r#"Color read base qualities"#, r#"Cellular barcode sequence bases (uncorrected)"#, r#"Color read sequence"#, r#"Complete read annotation tag, used for consensus annotation dummy features"#, r#"Phred quality of the cellular barcode sequence in the CR tag"#, r#"The 2nd most likely base calls"#, r#"The index of segment in the template"#, r#"Segment suffix"#, r#"Flow signal intensities"#, r#"Reserved for backwards compatibility reasons"#, r#"Reserved for backwards compatibility reasons"#, r#"Reserved for backwards compatibility reasons"#, r#"Number of perfect hits"#, r#"Number of 1-difference hits (see also NM)"#, r#"Number of 2-difference hits"#, r#"Query hit index"#, r#"Query hit total count"#, r#"Library"#, r#"CIGAR string for mate/next segment"#, r#"String encoding mismatched and deleted reference bases"#, r#"Reserved for backwards compatibility reasons"#, r#"Molecular identifier; a string that uniquely identifies the molecule from which the record was derived"#, r#"Mapping quality of the mate/next segment"#, r#"Number of reported alignments that contain the query in the current record"#, r#"Edit distance to the reference"#, r#"Original alignment"#, r#"Original CIGAR (deprecated; use OA instead)"#, r#"Original mapping position (deprecated; use OA instead)"#, r#"Original base quality"#, r#"Original unique molecular barcode bases"#, r#"Program"#, r#"Phred likelihood of the template"#, r#"Read annotations for parts of the padded read sequence"#, r#"Platform unit"#, r#"Phred quality of the mate/next segment sequence in the R2 tag"#, r#"Phred quality of the sample barcode sequence in the BC tag"#, r#"Quality score of the unique molecular identifier in the RX tag"#, r#"Sequence of the mate/next segment in the template"#, r#"Read group"#, r#"Reserved for backwards compatibility reasons"#, r#"Sequence bases of the (possibly corrected) unique molecular identifier"#, r#"Reserved for backwards compatibility reasons"#, r#"Other canonical alignments in a chimeric alignment"#, r#"Template-independent mapping quality"#, r#"Reserved for backwards compatibility reasons"#, r#"The number of segments in the template"#, r#"Transcript strand"#, r#"Phred probability of the 2nd call being wrong conditional on the best being wrong"#, r#"Phred likelihood of the segment, conditional on the mapping being correct"#, r#"Reserved for end users"#, r#"Reserved for end users"#, r#"Reserved for end users"# ];

// AM i AS i BC Z BQ Z BZ Z CB Z CC Z CG B,I 
// CM i CO Z CP i CQ Z CR Z CS Z CT Z CY Z E2 Z 
// FI i FS Z FZ B,S GC ? GQ ? GS ? H0 i H1 i 
// H2 i HI i IH i LB Z MC Z MD Z MF ? MI Z 
// MQ i NH i NM i OA Z OC Z OP i OQ Z OX Z PG Z 
// PQ i PT Z PU Z Q2 Z QT Z QX Z R2 Z RG Z RT ? 
// RX Z S2 ? SA Z SM i SQ ? 
// TC i TS A U2 Z UQ i X? ? Y? ? Z? ? 
