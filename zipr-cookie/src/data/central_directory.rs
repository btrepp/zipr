use super::{ascii_char::ascii_chars, compression_method, extra_field, extra_field_len, zip_path};
use cookie_factory::{
    bytes::{le_u16, le_u32},
    combinator::slice,
    lib::std::io::Write,
    sequence::tuple,
    SerializeFn,
};
use zipr_data::{
    borrowed::file::CentralDirectoryEntry, constants::CENTRAL_DIRECTORY_HEADER_SIGNATURE,
};

pub fn central_directory_entry<'a, W: Write + 'a>(
    input: &'a CentralDirectoryEntry,
) -> impl SerializeFn<W> + 'a {
    tuple((
        slice(CENTRAL_DIRECTORY_HEADER_SIGNATURE),
        le_u16(input.version_made_by),
        le_u16(input.version_needed),
        le_u16(input.general_purpose),
        compression_method(&input.compression_method),
        le_u16(input.file_modification_time.as_bytes()),
        le_u16(input.file_modification_date.as_bytes()),
        le_u32(input.crc32),
        le_u32(input.compressed_size),
        le_u32(input.uncompressed_size),
        le_u16(input.file_name.len() as u16),
        extra_field_len(&input.extra_field),
        le_u16(input.comment.len() as u16),
        le_u16(0),
        le_u16(input.internal_file_attributes),
        le_u32(input.external_file_attributes),
        le_u32(input.relative_offset),
        zip_path(&input.file_name),
        extra_field(input.extra_field),
        ascii_chars(input.comment),
    ))
}

#[cfg(test)]
mod tests {
    use core::{convert::TryInto, panic};

    use ascii::{AsAsciiStr, AsciiStr};
    use cookie_factory::gen;
    use zipr_data::{
        borrowed::{
            extra_field::{ntfs::NTFS, ExtraField},
            ZipPath,
        },
        CompressionMethod, DosDate, DosTime,
    };

    use super::*;

    #[test]
    fn hello_world_store() {
        let hello = include_bytes!("../../../assets/hello_world_store.zip");
        let expected = &hello[0x2c..0x87];
        let input = CentralDirectoryEntry {
            version_made_by: 63,
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

        let mut buf = [0u8; 0x5B];
        let serializer = central_directory_entry(&input);
        let (_, _) = gen(serializer, &mut buf[..]).unwrap();
        assert_eq!(expected, buf);
    }

    #[test]
    fn hello_world_deflate() {
        let hello = include_bytes!("../../../assets/hello_world_deflate.zip");
        let expected = &hello[0x3d..0x3d + 91];
        let input = CentralDirectoryEntry {
            version_made_by: 63,
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

        let mut buf = [0u8; 0x5B];
        let serializer = central_directory_entry(&input);
        let (_, _) = gen(serializer, &mut buf[..]).unwrap();
        assert_eq!(expected, buf);
    }
}
