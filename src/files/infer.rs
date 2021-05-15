use std::path::Path;
use infer::Type;
use infer::get_from_path;

/// Use the Infer crate to get the mime type and extension
pub fn infer_kind(path: &Path) -> infer::Type {
    let kind = infer::get_from_path(path)
    .expect("file read successfully")
    .expect("file type is known");
    kind
}

// /// Use a flate2 reader if gz, otherwise use a regular reader
// pub fn noodles_reader(path: Path){
//     File::open(smpl_path.unwrap().path())
//         .map(flate2::read::GzDecoder::new)
//         .map(BufReader::new)
//         .map(fastq::Reader::new);
// }