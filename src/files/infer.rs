use super::*;

/// Check whether is a gz file type with the Infer crate and return a boolean
pub fn is_gz(path: &Path) -> bool {
    let kind = infer_kind(path);
    kind.extension() == "gz"
}

/// Use the Infer crate to get the mime type and extension
pub fn infer_kind(path: &Path) -> infer::Type {
    let kind = infer::get_from_path(path)
    .expect("file read successfully")
    .expect("file type is known");
    kind
}

// pub fn fastq_aligner<R>(fq_reader: &mut seq_io::fastq::Reader<R>, sa: SuffixArray) 
// where 
// R: std::io::Read,

// pub fn noodles_fastq_reader<R>(path: &Path) -> Result<R> {
//     if is_gz(&path) {
//         File::open(&path)
//         .map(flate2::read::GzDecoder::new)
//         .map(BufReader::new)
//         .map(fastq::Reader::new)
//     }
//     else {
//         File::open(&path)
//         .map(BufReader::new)
//         .map(fastq::Reader::new)
//     }
// }
