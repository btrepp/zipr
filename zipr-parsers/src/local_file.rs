use nom::{
    bytes::complete::tag, bytes::complete::take, combinator::map, combinator::map_parser,
    number::complete::le_u16, number::complete::le_u32, IResult,
};
use winstructs::timestamp::{DosDate, DosTime};

use zipr_core::{constants::LOCAL_FILE_HEADER_SIGNATURE, data::LocalFileEntry};

use super::{
    compression_method::parse_compression_method, extra_field::parse_extra_field, path::parse_path,
};

pub fn parse_local_file(input: &[u8]) -> IResult<&[u8], LocalFileEntry> {
    let (input, _) = tag(LOCAL_FILE_HEADER_SIGNATURE)(input)?;
    let (input, version_needed) = le_u16(input)?;
    let (input, general_purpose) = le_u16(input)?;
    let (input, compression_method) = parse_compression_method(input)?;
    let (input, file_modification_time) = map(le_u16, DosTime::new)(input)?;
    let (input, file_modification_date) = map(le_u16, DosDate::new)(input)?;
    let (input, crc32) = le_u32(input)?;
    let (input, compressed_size) = le_u32(input)?;
    let (input, uncompressed_size) = le_u32(input)?;
    let (input, file_name_length) = le_u16(input)?;
    let (input, extra_field_length) = le_u16(input)?;

    let (input, file_name) = map_parser(take(file_name_length), parse_path)(input)?;

    let (input, extra_field) = map_parser(take(extra_field_length), parse_extra_field)(input)?;

    let (input, bytes) = take(compressed_size)(input)?;

    let result = LocalFileEntry {
        version_needed,
        general_purpose,
        compression_method,
        file_modification_time,
        file_modification_date,
        crc32,
        uncompressed_size,
        file_name,
        extra_field,
        bytes,
    };
    Ok((input, result))
}

#[cfg(test)]
mod tests {
    use core::panic;
    use std::path::Path;
    use zipr_core::data::extra_field::ExtraField;

    use super::*;
    #[test]
    fn hello_world_store() {
        let hello = include_bytes!("../../assets/hello_world_store.zip");
        let data = &hello[0..0x2c];
        let result = parse_local_file(data);
        let expected = LocalFileEntry {
            version_needed: 10,
            general_purpose: 0,
            compression_method: zipr_core::data::CompressionMethod::Stored,
            file_modification_time: DosTime::new(41164),
            file_modification_date: DosDate::new(20867),
            crc32: 980881731,
            uncompressed_size: 5,
            file_name: Path::new("hello.txt"),
            extra_field: ExtraField::Unknown(&[]),
            bytes: "world".as_bytes(),
        };

        assert_eq!(Ok((&[] as &[u8], expected)), result);
    }
}
