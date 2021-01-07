use std::{path::Path, str::from_utf8};

use anyhow::Result;
use comfy_table::Table;
use nom::{error::Error, Finish};
use zipr::{
    core::data::{LocalFileEntry, ZipPath},
    nom::{find_central_directory_entries, find_end_of_central_directory, find_local_file_entries},
    std::ToPath,
};

trait ToString {
    fn to_string(&self) -> &str;
}

impl ToString for ZipPath<'_> {
    fn to_string(&self) -> &str {
        from_utf8(self.to_bytes()).unwrap()
    }
}

fn own_error<T>(e: Error<T>) -> Error<String> {
    Error::new(String::from("Unable to parse"), e.code)
}

pub fn list_files<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let mut table = Table::new();
    let mut total = 0;
    table.set_header(vec!["Length", "Date", "Time", "Name"]);

    let bytes = std::fs::read(path)?;
    let entries = find_central_directory_entries(&bytes)
        .finish()
        .map(|(_, entries)| entries)
        .map_err(own_error)?;

    for e in entries.iter() {
        let row = vec![
            format!("{}", e.uncompressed_size),
            format!("{}", e.file_modification_date),
            format!("{}", e.file_modification_time),
            format!("{}", e.file_name.to_string()),
        ];
        total += e.uncompressed_size;
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
    println!("{}", file.comment);
    Ok(())
}

pub fn extract_files<P: AsRef<Path> + PartialEq>(file: P, files: Vec<P>, output: P) -> Result<()> {
    fn extract_bytes<'a>(file: &LocalFileEntry<'a>) -> Result<Vec<u8>> {
        let (_, vec) = zipr::nom::parse_compressed_data(&file.compressed_data)
            .finish()
            .map_err(own_error)?;
        Ok(vec)
    }

    let bytes = std::fs::read(file)?;
    let (_, entries) = find_local_file_entries(&bytes)
        .finish()
        .map_err(own_error)?;

    for entry in entries.iter() {
        let files: Vec<&Path> = files.iter().map(|x| x.as_ref()).collect();
        if files.len() != 0 && !files.contains(&entry.file_name.to_path()) {
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
