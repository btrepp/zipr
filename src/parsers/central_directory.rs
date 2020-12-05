use crate::{constants::CENTRAL_DIRECTORY_HEADER, data::CentralDirectoryEntry};
use nom::{
    bytes::complete::tag, bytes::complete::take, combinator::eof, combinator::iterator,
    combinator::map, combinator::map_parser, combinator::map_res, lib::std::str::from_utf8,
    number::complete::le_u16, number::complete::le_u32, IResult,
};
use winstructs::timestamp::{DosDate, DosTime};

use super::{compression_method::parse_compression_method, extra_field::parse_extra_field};

pub fn parse_directory_header(input: &[u8]) -> IResult<&[u8], CentralDirectoryEntry> {
    let (input, _) = tag(CENTRAL_DIRECTORY_HEADER)(input)?;
    let (input, version_made_by) = le_u16(input)?;
    let (input, version_needed) = le_u16(input)?;
    let (input, general_purpose) = le_u16(input)?;
    let (input, compression_method) = map_parser(take(2u16), parse_compression_method)(input)?;
    let (input, file_modification_time) = map(le_u16, DosTime::new)(input)?;
    let (input, file_modification_date) = map(le_u16, DosDate::new)(input)?;
    let (input, crc32) = le_u32(input)?;
    let (input, compressed_size) = le_u32(input)?;
    let (input, uncompressed_size) = le_u32(input)?;
    let (input, file_name_length) = le_u16(input)?;
    let (input, extra_field_length) = le_u16(input)?;
    let (input, comment_length) = le_u16(input)?;
    // Assume single zip for now
    let (input, _disk_file_starts) = tag([0, 0])(input)?;
    let (input, internal_file_attributes) = le_u16(input)?;
    let (input, external_file_attributes) = le_u32(input)?;
    let (input, relative_offset) = le_u32(input)?;
    let (input, file_name) = map_res(take(file_name_length), from_utf8)(input)?;

    let (input, extra_field) = map_parser(take(extra_field_length), parse_extra_field)(input)?;

    let (input, comment) = map_res(take(comment_length), from_utf8)(input)?;
    let result = CentralDirectoryEntry {
        version_made_by,
        version_needed,
        general_purpose,
        compression_method,
        file_modification_time,
        file_modification_date,
        crc32,
        compressed_size,
        uncompressed_size,
        internal_file_attributes,
        external_file_attributes,
        relative_offset,
        file_name,
        extra_field,
        comment,
    };
    Ok((input, result))
}

pub fn parse_directory_entries<'a>(
    input: &'a [u8],
) -> IResult<&'a [u8], Vec<CentralDirectoryEntry<'a>>> {
    let mut it = iterator(input, parse_directory_header);
    let result = it.collect::<Vec<CentralDirectoryEntry<'a>>>();
    let (input, _) = it.finish()?;
    let (input, _eof) = eof(input)?;
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    use core::panic;

    use nom::Finish;
    use winstructs::timestamp::WinTimestamp;

    use crate::data::extra_field::{ntfs::NTFS, ExtraField};

    use super::*;

    #[test]
    fn hello_world_store() {
        let hello = include_bytes!("../../assets/hello_world_store.zip");
        let data = &hello[0x2c..0x87];
        let result = parse_directory_header(data);
        let expected = CentralDirectoryEntry {
            version_made_by: 63,
            version_needed: 10,
            file_modification_time: DosTime::new(41164),
            file_modification_date: DosDate::new(20867),
            crc32: 980881731,
            compressed_size: 5,
            uncompressed_size: 5,
            internal_file_attributes: 0,
            external_file_attributes: 32,
            file_name: "hello.txt",
            comment: "",
            extra_field: ExtraField::NTFS(NTFS {
                atime: WinTimestamp::from_u64(132514708162669827),
                mtime: WinTimestamp::from_u64(132514707831351075),
                ctime: WinTimestamp::from_u64(132514707783459448),
            }),
            compression_method: crate::data::CompressionMethod::Stored,
            general_purpose: 0,
            relative_offset: 0,
        };

        assert_eq!(Ok((&[] as &[u8], expected)), result);
    }

    #[test]
    fn two_file_store() {
        let hello = include_bytes!("../../assets/two_files_store.zip");
        let data = &hello[0xb4..(0x59 + 185)];
        let result = parse_directory_header(data);

        let (input, result) = result.unwrap();

        assert_eq!(0, input.len());

        assert_eq!(44, result.relative_offset);
        assert_eq!("moredata.txt", result.file_name);
    }

    #[test]
    fn hello_world_store_as_entries() {
        let hello = include_bytes!("../../assets/hello_world_store.zip");
        let data = &hello[0x2c..0x87];
        let result = parse_directory_entries(data).finish();

        let (rem, result) = result.unwrap();

        assert_eq!(0, rem.len());
        assert_eq!(1, result.len());
        assert_eq!("hello.txt", result[0].file_name);
    }

    #[test]
    fn two_files_store_as_entries() {
        let hello = include_bytes!("../../assets/two_files_store.zip");
        let data = &hello[0x59..(0x59 + 185)];
        let result = parse_directory_entries(data).finish();

        let (rem, result) = result.unwrap();

        assert_eq!(0, rem.len());
        assert_eq!(2, result.len());
        assert_eq!("hello.txt", result[0].file_name);
        assert_eq!("moredata.txt", result[1].file_name);
    }
}
