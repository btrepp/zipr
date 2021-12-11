use super::find_local_file_entries::local_entry;
use crate::find_central_directory_entries;
use alloc::vec::Vec;
use nom::IResult;
use zipr_data::borrowed::ZipEntry;
use zipr_domain::make_zip_entry;

/// Given the full file. Finds all the zip file entries
/// Note this uses the central directory header to find the locations.
/// So both must be valid/non-corrupt    
pub fn parse_zip_entries(input: &[u8]) -> IResult<&[u8], Vec<ZipEntry<'_>>> {
    let (_, directories) = find_central_directory_entries(input)?;

    // There should be a way to nicely do this with iterators, but trouble finding
    // sequence (Vec<IResult> -> IResult<Vec<_>>) for nom.
    let mut local = Vec::with_capacity(directories.len());
    for directory in directories.iter() {
        let (_, file) = local_entry(input, directory)?;

        let zip = make_zip_entry(directory, &file);
        local.push(zip);
    }
    Ok((&[], local))
}
