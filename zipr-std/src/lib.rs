use std::{path::Path, str::from_utf8};
use zipr_core::data::ZipPath;
pub trait ToPath {
    fn to_path(&self) -> &Path;
}

impl<'a> ToPath for ZipPath<'a> {
    fn to_path(&self) -> &Path {
        let bytes = self.to_bytes();
        let string = from_utf8(bytes).unwrap();
        Path::new(string)
    }
}
