use nom::{
    combinator::{eof, iterator},
    IResult,
};
use zipr_core::data::CentralDirectoryEntry;

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

pub fn find_central_directory_entries<'a>(
    input: &'a [u8],
) -> IResult<&'a [u8], Vec<CentralDirectoryEntry<'a>>> {
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
    use std::path::Path;

    use nom::Finish;

    use super::*;
    #[test]
    fn hello_world_store_as_entries() {
        let hello = include_bytes!("../../../assets/hello_world_store.zip");
        let data = &hello[0x2c..0x87];
        let result = parse_directory_entries(data).finish();

        let (rem, result) = result.unwrap();

        assert_eq!(0, rem.len());
        assert_eq!(1, result.len());
        assert_eq!(Path::new("hello.txt"), result[0].file_name);
    }

    #[test]
    fn two_files_store_as_entries() {
        let hello = include_bytes!("../../../assets/two_files_store.zip");
        let data = &hello[0x59..(0x59 + 185)];
        let result = parse_directory_entries(data).finish();

        let (rem, result) = result.unwrap();

        assert_eq!(0, rem.len());
        assert_eq!(2, result.len());
        assert_eq!(Path::new("hello.txt"), result[0].file_name);
        assert_eq!(Path::new("moredata.txt"), result[1].file_name);
    }
}