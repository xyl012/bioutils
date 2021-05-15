use std::path::Path;
use infer::Type;
use infer::get_from_path;

pub fn infer_kind(path: &Path) -> infer::Type {
    let kind = infer::get_from_path(path)
    .expect("file read successfully")
    .expect("file type is known");
    kind
}

