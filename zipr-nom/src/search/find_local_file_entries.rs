use alloc::vec::Vec;
use nom::IResult;
use zipr_data::borrowed::file::{CentralDirectoryEntry, LocalFileEntry};

use crate::data::parse_local_file;

use super::find_central_directory_entries;

pub fn local_entry<'a>(
    full_file: &'a [u8],
    directory: &CentralDirectoryEntry,
) -> IResult<&'a [u8], LocalFileEntry<'a>> {
    let start = directory.relative_offset as usize;
    let end = full_file.len();
    let local_bytes = &full_file[start..end];
    let (rem, entry) = parse_local_file(local_bytes)?;
    Ok((rem, entry))
}

/// Given the full file. Finds all the local file entries
/// Note this uses the central directory header to find the locations.
/// So both must be valid/non-corrupt    
pub fn find_local_file_entries(input: &[u8]) -> IResult<&[u8], Vec<LocalFileEntry<'_>>> {
    let (_, directories) = find_central_directory_entries(input)?;

    // There should be a way to nicely do this with iterators, but trouble finding
    // sequence (Vec<IResult> -> IResult<Vec<_>>) for nom.
    let mut local = Vec::with_capacity(directories.len());
    for directory in directories.iter() {
        let (_, file) = local_entry(input, directory)?;
        local.push(file);
    }
    Ok((&[], local))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::Finish;
    use zipr_data::{borrowed::ZipPath, CP437Str};

    #[test]
    fn hello_world_store_as_entries() {
        let hello = include_bytes!("../../../assets/hello_world_store.zip");
        let data = hello;
        let result = find_local_file_entries(data).finish();

        let (rem, result) = result.unwrap();

        assert_eq!(0, rem.len());
        assert_eq!(1, result.len());
        assert_eq!(
            ZipPath::from_cp437(CP437Str::from_slice(b"hello.txt")).unwrap(),
            result[0].file_name
        );
    }

    #[test]
    fn two_files_store_as_entries() {
        let hello = include_bytes!("../../../assets/two_files_store.zip");
        let data = hello;
        let result = find_local_file_entries(data).finish();

        let (rem, result) = result.unwrap();

        assert_eq!(0, rem.len());
        assert_eq!(2, result.len());
        assert_eq!(
            ZipPath::from_cp437(CP437Str::from_slice(b"hello.txt")).unwrap(),
            result[0].file_name
        );
        assert_eq!(
            ZipPath::from_cp437(CP437Str::from_slice(b"moredata.txt")).unwrap(),
            result[1].file_name
        );
    }
}
