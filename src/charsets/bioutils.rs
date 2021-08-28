use super::*;

pub enum BioUtilsCharSet {
    Letters,
    LettersUppercase,
    LettersLowercase,
    Nucleotide,
    Dna,
    DnaMixCase,
    Dnan,
    DnanMixCase,
    DnaLowercase,
    Rna,
    RnaMixCase,
    Rnan,
    RnanMixCase,
    RnaLowercase,
    Gap,
    N,
    NMixCase,
    Gc,
    GcMixCase,
    AminoAcid,
    Phred33Encode,
    Phred33Decode,
    Phred64Encode,
    Phred64Decode,
    Phred33Score,
    Phred64Score,
    SangerEncode,
    SolexaEncode,
    Percent,
}

impl BioUtilsCharSet {
    pub const fn value(&self) -> &[u8] {
        match *self {
            BioUtilsCharSet::Letters => &ASCII_LETTERS,
            BioUtilsCharSet::LettersUppercase => &ASCII_LETTERS_UPPERCASE,
            BioUtilsCharSet::LettersLowercase => &ASCII_LETTERS_LOWERCASE,
            BioUtilsCharSet::Nucleotide => &IUPAC_NUCLEOTIDE,
            BioUtilsCharSet::Dna => &DNA,
            BioUtilsCharSet::DnaMixCase => &DNA_MIX_CASE,
            BioUtilsCharSet::Dnan => &DNAN,
            BioUtilsCharSet::DnanMixCase => &DNAN_MIX_CASE,
            BioUtilsCharSet::DnaLowercase => &DNA_LOWERCASE,
            BioUtilsCharSet::Rna => &RNA,
            BioUtilsCharSet::RnaMixCase => &RNA_MIX_CASE,
            BioUtilsCharSet::Rnan => &RNAN,
            BioUtilsCharSet::RnanMixCase => &RNAN_MIX_CASE,
            BioUtilsCharSet::RnaLowercase => &RNA_LOWERCASE,
            BioUtilsCharSet::Gap => &GAP,
            BioUtilsCharSet::N => &N,
            BioUtilsCharSet::NMixCase => &N_MIX_CASE,
            BioUtilsCharSet::Gc => &GC,
            BioUtilsCharSet::GcMixCase => &GC_MIX_CASE,
            BioUtilsCharSet::AminoAcid => &AMINO_ACID,
            BioUtilsCharSet::Phred33Encode => &PHRED33_ENCODE,
            BioUtilsCharSet::Phred33Decode => &PHRED33_DECODE,
            BioUtilsCharSet::Phred33Score => &PHRED33_SCORE,
            BioUtilsCharSet::Phred64Encode => &PHRED64_ENCODE,
            BioUtilsCharSet::Phred64Decode => &PHRED64_DECODE,
            BioUtilsCharSet::Phred64Score => &PHRED64_SCORE,
            BioUtilsCharSet::SolexaEncode => &SOLEXA_ENCODE,
            BioUtilsCharSet::SangerEncode => &SANGER_ENCODE,
            BioUtilsCharSet::Percent => &PERCENT,
        }
    }
}

pub enum BioUtilsUsizeSet {
    Percent,
}

impl BioUtilsUsizeSet {
    pub const fn value(&self) -> &[usize] {
        match *self {
            BioUtilsUsizeSet::Percent => &PERCENT_USIZE,
        }
    }
}

pub enum BioUtilsU64Set {
    Percent,
}

impl BioUtilsU64Set {
    pub const fn value(&self) -> &[u64] {
        match *self {
            BioUtilsU64Set::Percent => &PERCENT_U64,
        }
    }
}

pub enum BioUtilsRecodeSet {
    Phred33,
    Phred64,
}

pub struct BioUtilsRecodeStruct<'a> {
    pub score: &'a [u8],
    pub encode: &'a [u8],
    pub decode: &'a [u8],
}

// pub enum BioUtilsRecodeSet {
    // Phred33Score,
    // Phred33Encode,
    // Phred33Decode,
    // Phred64Score,
    // Phred64Encode,
    // Phred64Decode,
    // Sanger,
    // Solexa,
// }

impl BioUtilsRecodeSet {
    pub const fn value(&self) -> BioUtilsRecodeStruct {
        match *self {
            BioUtilsRecodeSet::Phred33 => BioUtilsRecodeStruct{ score: BioUtilsCharSet::Phred33Score.value(), encode: BioUtilsCharSet::Phred33Encode.value(), decode: BioUtilsCharSet::Phred33Decode.value(), },
            BioUtilsRecodeSet::Phred64 => BioUtilsRecodeStruct{ score: BioUtilsCharSet::Phred64Score.value(), encode: BioUtilsCharSet::Phred64Encode.value(), decode: BioUtilsCharSet::Phred64Decode.value(), },
            // BioUtilsRecodeSet::Phred33Score => &PHRED33_SCORE,
            // BioUtilsRecodeSet::Phred33Encode => &PHRED33_ENCODE,
            // BioUtilsRecodeSet::Phred33Decode => &PHRED64_DECODE,
            // BioUtilsRecodeSet::Phred64Score => &PHRED64_SCORE,
            // BioUtilsRecodeSet::Phred64Encode => &PHRED64_ENCODE,
            // BioUtilsRecodeSet::Phred64Decode => &PHRED64_DECODE,
        }
    }
}
