use crate::{
    display::{display_entries, ToString},
    error::AppError,
};
use super::sequence::Sequence;
use anyhow::Result;
use nom::Finish;
use std::path::Path;
use zipr::{
    compression::DecompressToVec,
    core::data::{file::LocalFileEntry, CompressionMethod},
    nom::{find_end_of_central_directory, find_local_file_entries},
    std::ToPath,
};


/// List all the files to console
pub fn list_files<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let bytes = std::fs::read(path)?;
    let entries = zipr::nom::iter::zip_entry_iter(&bytes)
        .sequence()
        .map_err(Into::<AppError>::into)?;

    let table = display_entries(entries);
    println!("{}", table);
    Ok(())
}

/// Shows the comment of the zip archive
pub fn show_comment<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let bytes = std::fs::read(path)?;
    let (_, file) = find_end_of_central_directory(&bytes)
        .finish()
        .map_err(Into::<AppError>::into)?;
    println!("{}", file.comment.to_string());
    Ok(())
}

/// Extract files to the supplied path
pub fn extract_files<P: AsRef<Path> + PartialEq>(file: P, files: Vec<P>, output: P) -> Result<()> {
    fn extract_bytes(file: &LocalFileEntry<'_>) -> Result<Vec<u8>> {
        let bytes = file
            .compressed_data
            .decompress_to_vec()
            .map_err(Into::<AppError>::into)?;
        Ok(bytes)
    }

    let bytes = std::fs::read(file)?;
    let (_, entries) = find_local_file_entries(&bytes)
        .finish()
        .map_err(Into::<AppError>::into)?;

    for entry in entries.iter() {
        let files: Vec<&Path> = files.iter().map(|x| x.as_ref()).collect();
        if !files.is_empty() && !files.contains(&entry.file_name.to_path()) {
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

/// Adds files to an existing archive
pub fn add_files<P: AsRef<Path>>(
    file: P,
    files: Vec<P>,
    _compression: CompressionMethod,
) -> Result<()> {
    let path = file.as_ref();
    let files: Vec<&Path> = files.iter().map(|x| x.as_ref()).collect();
    println!("{}", path.to_string_lossy());

    // Get the input bytes
    let bytes = if path.exists() {
        std::fs::read(path)?
    } else {
        Vec::new()
    };

    // At its core we need existing entries to determine what entries to store
    let entries = if path.exists() {
        let (_, entries) = find_local_file_entries(&bytes)
            .finish()
            .map_err(Into::<AppError>::into)?;
        entries
    } else {
        Vec::new()
    };

    // Filter out the entries that we already have
    let existing: Vec<LocalFileEntry> = entries
        .into_iter()
        .filter(|x| !files.contains(&x.file_name.to_path()))
        .collect();

    // convert files into LocalFileEntry using deflate
    println!("{:?}", existing);

    Ok(())
}
