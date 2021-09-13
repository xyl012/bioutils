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
    DnaComplement,
    DnaComplementMixCase,
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
            BioUtilsCharSet::DnaComplement => &DNA_COMPLEMENT,
            BioUtilsCharSet::DnaComplementMixCase => &DNA_COMPLEMENT_MIX_CASE,
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
    Phred33Encode,
    Phred33Decode,
    Phred64Encode,
    Phred64Decode,
    DnaComplement,
    DnanComplement,
    DnaComplementMixCase,
    DnanComplementMixCase,
}

pub struct BioUtilsRecodeStruct<'a> {
    pub charset: &'a [u8],
    pub recode: &'a [u8],
}

impl BioUtilsRecodeSet {
    pub const fn value(&self) -> BioUtilsRecodeStruct {
        match *self {
            BioUtilsRecodeSet::Phred33Encode => BioUtilsRecodeStruct{ charset: BioUtilsCharSet::Phred33Score.value(), recode: BioUtilsCharSet::Phred33Encode.value(), },
            BioUtilsRecodeSet::Phred33Decode => BioUtilsRecodeStruct{ charset: BioUtilsCharSet::Phred33Encode.value(), recode: BioUtilsCharSet::Phred33Decode.value(), },
            BioUtilsRecodeSet::Phred64Encode => BioUtilsRecodeStruct{ charset: BioUtilsCharSet::Phred64Score.value(), recode: BioUtilsCharSet::Phred64Encode.value(), },
            BioUtilsRecodeSet::Phred64Decode => BioUtilsRecodeStruct{ charset: BioUtilsCharSet::Phred64Encode.value(), recode: BioUtilsCharSet::Phred64Decode.value(), },
            BioUtilsRecodeSet::DnaComplement => BioUtilsRecodeStruct{ charset: BioUtilsCharSet::Dna.value(), recode: BioUtilsCharSet::DnaComplement.value(), },
            BioUtilsRecodeSet::DnanComplement => BioUtilsRecodeStruct{ charset: BioUtilsCharSet::Dnan.value(), recode: BioUtilsCharSet::DnaComplement.value(), },
            BioUtilsRecodeSet::DnaComplementMixCase => BioUtilsRecodeStruct{ charset: BioUtilsCharSet::DnaMixCase.value(), recode: BioUtilsCharSet::DnaComplementMixCase.value(), },
            BioUtilsRecodeSet::DnanComplementMixCase => BioUtilsRecodeStruct{ charset: BioUtilsCharSet::DnanMixCase.value(), recode: BioUtilsCharSet::DnaComplementMixCase.value(), },
        }
    }
}