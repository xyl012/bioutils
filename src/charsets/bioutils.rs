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
    Phred33,
    Phred64,
    Sanger,
    Solexa,
    Phred33Scores,
    Phred64Scores,
    Percent,
}

impl BioUtilsCharSet {
    pub const fn value(&self) -> &[u8] {
        match *self {
            BioUtilsCharSet::Letters => ASCII_LETTERS_SLICE,
            BioUtilsCharSet::LettersUppercase => ASCII_LETTERS_UPPERCASE_SLICE,
            BioUtilsCharSet::LettersLowercase => ASCII_LETTERS_LOWERCASE_SLICE,
            BioUtilsCharSet::Nucleotide => IUPAC_NUCLEOTIDE_SLICE,
            BioUtilsCharSet::Dna => DNA_SLICE,
            BioUtilsCharSet::DnaMixCase => DNA_MIX_CASE_SLICE,
            BioUtilsCharSet::Dnan => DNAN_SLICE,
            BioUtilsCharSet::DnanMixCase => DNAN_MIX_CASE_SLICE,
            BioUtilsCharSet::DnaLowercase => DNA_LOWERCASE_SLICE,
            BioUtilsCharSet::Rna => RNA_SLICE,
            BioUtilsCharSet::RnaMixCase => RNA_MIX_CASE_SLICE,
            BioUtilsCharSet::Rnan => RNAN_SLICE,
            BioUtilsCharSet::RnanMixCase => RNAN_MIX_CASE_SLICE,
            BioUtilsCharSet::RnaLowercase => RNA_LOWERCASE_SLICE,
            BioUtilsCharSet::Gap => GAP_SLICE,
            BioUtilsCharSet::N => N_SLICE,
            BioUtilsCharSet::NMixCase => N_MIX_CASE_SLICE,
            BioUtilsCharSet::Gc => GC_SLICE,
            BioUtilsCharSet::GcMixCase => GC_MIX_CASE_SLICE,
            BioUtilsCharSet::AminoAcid => AMINO_ACID_SLICE,
            BioUtilsCharSet::Phred33 => PHRED33_SLICE,
            BioUtilsCharSet::Phred64 => PHRED64_SLICE,
            BioUtilsCharSet::Solexa => SOLEXA_SLICE,
            BioUtilsCharSet::Sanger => SANGER_SLICE,
            BioUtilsCharSet::Phred33Scores => PHRED33_SCORE_SLICE,
            BioUtilsCharSet::Phred64Scores => PHRED64_SCORE_SLICE,
            BioUtilsCharSet::Percent => PERCENT_SLICE,
        }
    }
}

pub enum BioUtilsUsizeSet {
    Percent,
}

impl BioUtilsUsizeSet {
    pub const fn value(&self) -> &[usize] {
        match *self {
            BioUtilsUsizeSet::Percent => PERCENT_USIZE_SLICE,
        }
    }
}

pub enum BioUtilsU64Set {
    Percent,
}

impl BioUtilsU64Set {
    pub const fn value(&self) -> &[u64] {
        match *self {
            BioUtilsU64Set::Percent => PERCENT_U64_SLICE,
        }
    }
}




// BioUtilsCharSet::PercentUsize => PERCENT_USIZE_SLICE,
// BioUtilsCharSet::PercentU64 => PERCENT_U64_SLICE,