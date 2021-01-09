use crate::{
    display::{display_entries, ToString},
    error::AppError,
};
use super::sequence::Sequence;
use anyhow::Result;
use ascii::AsAsciiStr;
use nom::Finish;
use std::path::Path;
use zipr::{compression::DecompressToVec, core::data::{AsciiStr, CompressionMethod, ZipEntry, file::LocalFileEntry}, nom::{find_end_of_central_directory, find_local_file_entries}, std::ToPath};

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

    fn to_zip<'a>(path:&'a Path,bytes:&'a [u8]) -> Result<ZipEntry<'a>, anyhow::Error>{
        let comment = AsciiStr::from_ascii("".as_bytes()).unwrap();
        let extra_field = zipr::core::data::extra_field::ExtraField::Unknown(&[]);
        let file_modification_time = zipr::core::data::DosTime::from_u16_unchecked(0);
        let file_modification_date = zipr::core::data::DosDate::from_u16_unchecked(0);
        let file_name = zipr::core::data::ZipPath::create_from_string(path.to_str().unwrap().as_ascii_str().unwrap()).unwrap();
        let entry = ZipEntry{
            version_made_by: 0,
            version_needed: 0,
            general_purpose: 0,
            file_modification_time,
            file_modification_date,
            internal_file_attributes: 0,
            external_file_attributes: 0,
            file_name,
            extra_field,
            comment,
            compressed_data: zipr::compression::deflate(&bytes),
        };
        Ok(entry)
        
    }

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
    let entries = if !bytes.is_empty() {
        let entries = zipr::nom::iter::zip_entry_iter(&bytes)
            .sequence()
            .map_err(Into::<AppError>::into)?;
        entries
    } else {
        Vec::new()
    };

    // Filter out the entries that we already have
    let mut existing : Vec<_>= entries
        .into_iter()
        .filter(|x| !files.contains(&x.file_name.to_path()))
        .collect();

    let file_data = files.iter().map(|i| std::fs::read(i)).sequence()?;
    // Calculate new additions
    let mut new_entries:Vec<_> = 
        files.into_iter().enumerate().map(|(i,p)| {
            let bytes = &file_data[i];
            to_zip(p, bytes)
        }).sequence()?;

    existing.append(&mut new_entries);    

    println!("{:?}", existing);

    Ok(())
}
