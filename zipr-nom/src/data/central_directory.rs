use nom::{
    bytes::complete::tag, bytes::complete::take, combinator::map, combinator::map_parser,
    number::complete::le_u16, number::complete::le_u32, IResult,
};
use zipr_data::{
    borrowed::file::CentralDirectoryEntry, constants::CENTRAL_DIRECTORY_HEADER_SIGNATURE, DosDate,
    DosTime,
};

use super::{
    ascii_char::parse_ascii_chars, compression_method::parse_compression_method,
    extra_field::parse_extra_field, parse_version, zip_path::parse_zip_path,
};

/// Parses a single directory header
pub fn parse_directory_header(input: &[u8]) -> IResult<&[u8], CentralDirectoryEntry> {
    let (input, _) = tag(CENTRAL_DIRECTORY_HEADER_SIGNATURE)(input)?;
    let (input, version_made_by) = parse_version(input)?;
    let (input, version_needed) = le_u16(input)?;
    let (input, general_purpose) = le_u16(input)?;
    let (input, compression_method) = map_parser(take(2u16), parse_compression_method)(input)?;
    let (input, file_modification_time) = map(le_u16, DosTime::from_u16_unchecked)(input)?;
    let (input, file_modification_date) = map(le_u16, DosDate::from_u16_unchecked)(input)?;
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

    let (input, file_name) = map_parser(take(file_name_length), parse_zip_path)(input)?;

    let (input, extra_field) = map_parser(take(extra_field_length), parse_extra_field)(input)?;

    let (input, comment) = map_parser(take(comment_length), parse_ascii_chars)(input)?;
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
    use core::{convert::TryInto, panic};

    use ascii::{AsAsciiStr, AsciiStr};
    use zipr_data::{
        borrowed::{
            extra_field::{ntfs::NTFS, ExtraField},
            ZipPath,
        },
        CompressionMethod, HostCompatibility, Version, ZipSpecification,
    };

    use super::*;

    #[test]
    fn hello_world_store() {
        let hello = include_bytes!("../../../assets/hello_world_store.zip");
        let data = &hello[0x2c..0x87];
        let result = parse_directory_header(data);
        let expected = CentralDirectoryEntry {
            version_made_by: Version {
                host: HostCompatibility::MSDOS,
                spec: ZipSpecification {
                    major: 6u8.try_into().unwrap(),
                    minor: 3u8.try_into().unwrap(),
                },
            },
            version_needed: 10,
            file_modification_time: DosTime::from_u16_unchecked(41164),
            file_modification_date: DosDate::from_u16_unchecked(20867),
            crc32: 980881731,
            compressed_size: 5,
            uncompressed_size: 5,
            internal_file_attributes: 0,
            external_file_attributes: 32,
            file_name: ZipPath::create_from_string("hello.txt".as_ascii_str().unwrap()).unwrap(),
            comment: AsciiStr::from_ascii("").unwrap(),
            extra_field: ExtraField::NTFS(NTFS {
                atime: 132514708162669827.try_into().unwrap(),
                mtime: 132514707831351075.try_into().unwrap(),
                ctime: 132514707783459448.try_into().unwrap(),
            }),
            compression_method: CompressionMethod::Stored,
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
            version_made_by: Version {
                host: HostCompatibility::MSDOS,
                spec: ZipSpecification {
                    major: 6u8.try_into().unwrap(),
                    minor: 3u8.try_into().unwrap(),
                },
            },
            version_needed: 20,
            file_modification_time: DosTime::from_u16_unchecked(43312),
            file_modification_date: DosDate::from_u16_unchecked(20870),
            crc32: 810231625,
            compressed_size: 22,
            uncompressed_size: 215,
            internal_file_attributes: 0,
            external_file_attributes: 32,
            file_name: ZipPath::create_from_string("hello.txt".as_ascii_str().unwrap()).unwrap(),
            comment: AsciiStr::from_ascii("").unwrap(),
            extra_field: ExtraField::NTFS(NTFS {
                atime: 132517337704649244.try_into().unwrap(),
                mtime: 132517337704649244.try_into().unwrap(),
                ctime: 132514707783459448.try_into().unwrap(),
            }),
            compression_method: CompressionMethod::Deflate,
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
        assert_eq!(
            ZipPath::create_from_string("moredata.txt".as_ascii_str().unwrap()).unwrap(),
            result.file_name
        );
    }
}
