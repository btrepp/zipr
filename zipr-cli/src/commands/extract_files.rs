use std::path::Path;
use zipr::{compression::DecompressToVec, data::borrowed::ZipEntry, std::ToPath};

use crate::{
    display::ToString,
    error::{AppError, AppResult},
    sequence::Sequence,
};

fn extract_bytes(file: &ZipEntry<'_>) -> AppResult<Vec<u8>> {
    let bytes = file
        .compressed_data
        .decompress_to_vec()
        .map_err(Into::<AppError>::into)?;
    Ok(bytes)
}

/// Extract files to the supplied path
pub fn extract_files<P: AsRef<Path> + PartialEq>(
    file: P,
    files: Vec<P>,
    output: P,
) -> AppResult<()> {
    let bytes = std::fs::read(file)?;
    let entries = zipr::nom::iter::zip_entry_iter(&bytes)
        .sequence()
        .map_err(Into::<AppError>::into)?;

    let files: Vec<&Path> = files.iter().map(|x| x.as_ref()).collect();
    for entry in entries.iter() {
        if !files.is_empty() && !files.contains(&entry.file_name.to_path().as_path()) {
            println!("Skipping: {}", entry.file_name.to_string());
        } else {
            let bytes = extract_bytes(entry)?;
            let path = output.as_ref().join(entry.file_name.to_path());
            std::fs::write(path.clone(), bytes)?;
            println!("Extracted: {} ", path.to_string_lossy());
        }
    }
    Ok(())
}
