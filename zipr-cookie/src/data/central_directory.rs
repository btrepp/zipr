use super::{
    compression_method, cp437str::cp437_chars, extra_field, extra_field_len, version, zip_path,
};
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
        version(input.version_made_by),
        version(input.version_needed),
        le_u16(input.general_purpose),
        compression_method(&input.compression_method),
        le_u16(input.file_modification_time.as_bytes()),
        le_u16(input.file_modification_date.as_bytes()),
        le_u32(input.crc32),
        le_u32(input.compressed_size),
        le_u32(input.uncompressed_size),
        le_u16(input.file_name.as_ref().len() as u16),
        extra_field_len(&input.extra_field),
        le_u16(input.comment.len() as u16),
        le_u16(0),
        le_u16(input.internal_file_attributes),
        le_u32(input.external_file_attributes),
        le_u32(input.relative_offset),
        zip_path(&input.file_name),
        extra_field(input.extra_field),
        cp437_chars(&input.comment),
    ))
}

#[cfg(test)]
mod tests {
    use cookie_factory::gen;
    use core::{
        convert::{TryFrom, TryInto},
        panic,
    };
    use zipr_data::{
        borrowed::{
            extra_field::{ntfs::NTFS, ExtraField},
            OEM437Str, ZipPath,
        },
        CompressionMethod, DosDate, DosTime, HostCompatibility, Version, ZipSpecification,
    };

    use super::*;

    #[test]
    fn hello_world_store() {
        let hello = include_bytes!("../../../assets/hello_world_store.zip");
        let expected = &hello[0x2c..0x87];
        let input = CentralDirectoryEntry {
            version_made_by: Version {
                host: HostCompatibility::MSDOS,
                spec: ZipSpecification {
                    major: 6u8.try_into().unwrap(),
                    minor: 3u8.try_into().unwrap(),
                },
            },
            version_needed: Version {
                host: HostCompatibility::MSDOS,
                spec: ZipSpecification {
                    major: 1u8.try_into().unwrap(),
                    minor: 0u8.try_into().unwrap(),
                },
            },
            file_modification_time: DosTime::from_u16_unchecked(41164),
            file_modification_date: DosDate::from_u16_unchecked(20867),
            crc32: 980881731,
            compressed_size: 5,
            uncompressed_size: 5,
            internal_file_attributes: 0,
            external_file_attributes: 32,
            file_name: ZipPath::try_from(OEM437Str::from(b"hello.txt")).unwrap(),
            comment: Default::default(),
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
            version_made_by: Version {
                host: HostCompatibility::MSDOS,
                spec: ZipSpecification {
                    major: 6u8.try_into().unwrap(),
                    minor: 3u8.try_into().unwrap(),
                },
            },
            version_needed: Version {
                host: HostCompatibility::MSDOS,
                spec: ZipSpecification {
                    major: 2u8.try_into().unwrap(),
                    minor: 0u8.try_into().unwrap(),
                },
            },
            file_modification_time: DosTime::from_u16_unchecked(43312),
            file_modification_date: DosDate::from_u16_unchecked(20870),
            crc32: 810231625,
            compressed_size: 22,
            uncompressed_size: 215,
            internal_file_attributes: 0,
            external_file_attributes: 32,
            file_name: ZipPath::try_from(OEM437Str::from(b"hello.txt")).unwrap(),
            comment: Default::default(),
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
