use nom::{
    bytes::complete::tag, bytes::complete::take, combinator::map, combinator::map_parser,
    combinator::map_res, lib::std::str::from_utf8, number::complete::le_u16,
    number::complete::le_u32, IResult,
};
use winstructs::timestamp::{DosDate, DosTime};
use zipr_core::{constants::CENTRAL_DIRECTORY_HEADER_SIGNATURE, data::CentralDirectoryEntry};

use super::{
    compression_method::parse_compression_method, extra_field::parse_extra_field, path::parse_path,
};

/// Parses a single directory header
pub fn parse_directory_header(input: &[u8]) -> IResult<&[u8], CentralDirectoryEntry> {
    let (input, _) = tag(CENTRAL_DIRECTORY_HEADER_SIGNATURE)(input)?;
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

    let (input, file_name) = map_parser(take(file_name_length), parse_path)(input)?;

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

#[cfg(test)]
mod tests {
    use core::panic;
    use std::path::Path;

    use winstructs::timestamp::WinTimestamp;

    use zipr_core::data::extra_field::{ntfs::NTFS, ExtraField};

    use super::*;

    #[test]
    fn hello_world_store() {
        let hello = include_bytes!("../../../assets/hello_world_store.zip");
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
            file_name: Path::new("hello.txt"),
            comment: "",
            extra_field: ExtraField::NTFS(NTFS {
                atime: WinTimestamp::from_u64(132514708162669827),
                mtime: WinTimestamp::from_u64(132514707831351075),
                ctime: WinTimestamp::from_u64(132514707783459448),
            }),
            compression_method: zipr_core::data::CompressionMethod::Stored,
            general_purpose: 0,
            relative_offset: 0,
        };

        assert_eq!(Ok((&[] as &[u8], expected)), result);
    }

    #[test]
    fn hello_world_deflate() {
        let hello = include_bytes!("../../../assets/hello_world_deflate.zip");
        let data = &hello[0x3d..0x3d + 91];
        let result = parse_directory_header(data);
        let expected = CentralDirectoryEntry {
            version_made_by: 63,
            version_needed: 20,
            file_modification_time: DosTime::new(43312),
            file_modification_date: DosDate::new(20870),
            crc32: 810231625,
            compressed_size: 22,
            uncompressed_size: 215,
            internal_file_attributes: 0,
            external_file_attributes: 32,
            file_name: Path::new("hello.txt"),
            comment: "",
            extra_field: ExtraField::NTFS(NTFS {
                atime: WinTimestamp::new(&[0x1c, 0x52, 0x77, 0x08, 0xd1, 0xcb, 0xd6, 0x01])
                    .unwrap(),
                mtime: WinTimestamp::new(&[0x1c, 0x52, 0x77, 0x08, 0xd1, 0xcb, 0xd6, 0x01])
                    .unwrap(),
                ctime: WinTimestamp::new(&[0x78, 0xa2, 0xf2, 0xb4, 0x6c, 0xc9, 0xd6, 0x01])
                    .unwrap(),
            }),
            compression_method: zipr_core::data::CompressionMethod::Deflate,
            general_purpose: 0,
            relative_offset: 0,
        };
        assert_eq!(Ok((&[] as &[u8], expected)), result);
    }

    #[test]
    fn two_file_store() {
        let hello = include_bytes!("../../../assets/two_files_store.zip");
        let data = &hello[0xb4..(0x59 + 185)];
        let result = parse_directory_header(data);

        let (input, result) = result.unwrap();

        assert_eq!(0, input.len());

        assert_eq!(44, result.relative_offset);
        assert_eq!(Path::new("moredata.txt"), result.file_name);
    }
}
