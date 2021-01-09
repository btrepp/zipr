use super::sequence::Sequence;
use anyhow::Result;
use comfy_table::Table;
use nom::{error::Error, Finish};
use std::{fmt::Display, path::Path, str::from_utf8};
use zipr::{
    compression::{DecompressError, DecompressToVec},
    core::data::{file::LocalFileEntry, CompressionMethod, ZipEntry, ZipPath},
    nom::{find_end_of_central_directory, find_local_file_entries, iter::ZipEntryIteratorError},
    std::{ToNaiveDate, ToNaiveTime, ToPath},
};

trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for ZipPath<'_> {
    fn to_string(&self) -> String {
        from_utf8(self.to_bytes()).unwrap().to_string()
    }
}

fn own_error<T>(e: Error<T>) -> Error<String> {
    Error::new(String::from("Unable to parse"), e.code)
}
#[derive(Debug)]
enum AppError {
    Decompression(DecompressError),
    Iterator,
}

impl From<ZipEntryIteratorError<'_>> for AppError {
    fn from(_: ZipEntryIteratorError<'_>) -> Self {
        AppError::Iterator
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

impl std::error::Error for AppError {}

pub fn list_files<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let mut table = Table::new();
    let mut total = 0;
    table.set_header(vec!["Length", "Date", "Time", "Name"]);

    let bytes = std::fs::read(path)?;

    let entries: Result<Vec<ZipEntry<'_>>, AppError> = zipr::nom::iter::zip_entry_iter(&bytes)
        .sequence()
        .map_err(|e| e.into());

    let entries = entries?;

    for e in entries.iter() {
        let row = vec![
            format!("{}", e.compressed_data.uncompressed_size()),
            format!("{}", e.file_modification_date.to_date()),
            format!("{}", e.file_modification_time.to_time()),
            e.file_name.to_string(),
        ];
        total += e.compressed_data.uncompressed_size();
        table.add_row(row);
    }
    table.add_row(vec![
        format!("{}", total),
        String::new(),
        String::new(),
        format!("{}", entries.len()),
    ]);

    println!("{}", table);
    Ok(())
}

pub fn show_comment<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let bytes = std::fs::read(path)?;
    let (_, file) = find_end_of_central_directory(&bytes)
        .finish()
        .map_err(own_error)?;
    println!("{}", file.comment.to_string());
    Ok(())
}

pub fn extract_files<P: AsRef<Path> + PartialEq>(file: P, files: Vec<P>, output: P) -> Result<()> {
    fn extract_bytes(file: &LocalFileEntry<'_>) -> Result<Vec<u8>> {
        let bytes = file
            .compressed_data
            .decompress_to_vec()
            .map_err(AppError::Decompression)?;
        Ok(bytes)
    }

    let bytes = std::fs::read(file)?;
    let (_, entries) = find_local_file_entries(&bytes)
        .finish()
        .map_err(own_error)?;

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
            .map_err(own_error)?;
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
