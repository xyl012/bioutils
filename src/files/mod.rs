use super::*;

pub mod http;

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


