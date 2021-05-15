use std::io::Error;
use std::fs::File;
use std::path::Path;
use std::io::BufReader;

use infer::Type;
use infer::get_from_path;

/// Check whether is a gz file type with the Infer crate and return a boolean
pub fn is_gz(path: &Path) -> bool {
    let kind = infer_kind(path);
    if kind.extension() == "gz" {true} else {false}
}

/// Use the Infer crate to get the mime type and extension
pub fn infer_kind(path: &Path) -> infer::Type {
    let kind = infer::get_from_path(path)
    .expect("file read successfully")
    .expect("file type is known");
    kind
}

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

// pub fn noodles_fasta_reader(path: &Path) -> noodles::fasta::Reader {
//     File::open(smpl_path.unwrap().path())
//     .map(flate2::read::GzDecoder::new)
//     .map(BufReader::new)
//     .map(fastq::Reader::new)?;
// }




// pub fn noodles_reader(path: Path){
//     File::open(smpl_path.unwrap().path())
//         .map(flate2::read::GzDecoder::new)
//         .map(BufReader::new)
//         .map(fastq::Reader::new);
// }