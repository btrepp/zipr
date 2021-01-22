use alloc::vec::Vec;
use nom::{
    combinator::{eof, iterator},
    IResult,
};
use zipr_data::borrowed::file::CentralDirectoryEntry;

use crate::data::parse_directory_header;

use super::find_end_of_central_directory::find_end_of_central_directory;

fn parse_directory_entries<'a>(
    input: &'a [u8],
) -> IResult<&'a [u8], Vec<CentralDirectoryEntry<'a>>> {
    let mut it = iterator(input, parse_directory_header);
    let result = it.collect::<Vec<CentralDirectoryEntry<'a>>>();
    let (input, _) = it.finish()?;
    let (input, _eof) = eof(input)?;
    Ok((input, result))
}

/// Given the full file, find the central directory entries
pub fn find_central_directory_entries(
    input: &[u8],
) -> IResult<&[u8], Vec<CentralDirectoryEntry<'_>>> {
    let (_, end) = find_end_of_central_directory(input)?;
    let start = end.offset_start_directory as usize;
    let end = start + end.size_of_directory as usize;
    let input = &input[start..end];
    let (input, entries) = parse_directory_entries(input)?;
    Ok((input, entries))
}

#[cfg(test)]
mod tests {
    use core::panic;
    use nom::Finish;
    use zipr_data::borrowed::{OEM437Str, ZipPath};

    use super::*;
    #[test]
    fn hello_world_store_as_entries() {
        let hello = include_bytes!("../../../assets/hello_world_store.zip");
        let data = &hello[0x2c..0x87];
        let result = parse_directory_entries(data).finish();

        let (rem, result) = result.unwrap();

        assert_eq!(0, rem.len());
        assert_eq!(1, result.len());
        assert_eq!(
            ZipPath::from_cp437(OEM437Str::from_slice(b"hello.txt")).unwrap(),
            result[0].file_name
        );
    }

    #[test]
    fn two_files_store_as_entries() {
        let hello = include_bytes!("../../../assets/two_files_store.zip");
        let data = &hello[0x59..(0x59 + 185)];
        let result = parse_directory_entries(data).finish();

        let (rem, result) = result.unwrap();

        assert_eq!(0, rem.len());
        assert_eq!(2, result.len());
        assert_eq!(
            ZipPath::from_cp437(OEM437Str::from_slice(b"hello.txt")).unwrap(),
            result[0].file_name
        );
        assert_eq!(
            ZipPath::from_cp437(OEM437Str::from_slice(b"moredata.txt")).unwrap(),
            result[1].file_name
        );
    }
}
