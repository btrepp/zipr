use super::{compression_method, extra_field, extra_field_len, zip_path};
use cookie_factory::{
    bytes::{le_u16, le_u32},
    combinator::slice,
    lib::std::io::Write,
    sequence::tuple,
    SerializeFn,
};
use zipr_data::{borrowed::file::LocalFileEntry, constants::LOCAL_FILE_HEADER_SIGNATURE};

pub fn local_file_entry<'a, W: Write + 'a>(
    input: &'a LocalFileEntry<'a>,
) -> impl SerializeFn<W> + 'a {
    tuple((
        slice(LOCAL_FILE_HEADER_SIGNATURE),
        le_u16(input.version_needed),
        le_u16(input.general_purpose),
        compression_method(&input.compressed_data.compression_method()),
        le_u16(input.file_modification_time.as_bytes()),
        le_u16(input.file_modification_date.as_bytes()),
        le_u32(input.compressed_data.crc32()),
        le_u32(input.compressed_data.bytes().len() as u32),
        le_u32(input.compressed_data.uncompressed_size()),
        le_u16(input.file_name.len() as u16),
        extra_field_len(&input.extra_field),
        zip_path(&input.file_name),
        extra_field(input.extra_field),
        slice(input.compressed_data.bytes()),
    ))
}

#[cfg(test)]
mod tests {
    use ascii::AsAsciiStr;
    use cookie_factory::gen;
    use core::panic;
    use zipr_data::{
        borrowed::{extra_field::ExtraField, file::CompressedData, ZipPath},
        CompressionMethod, DosDate, DosTime,
    };

    use super::*;
    #[test]
    fn hello_world_store() {
        let hello = include_bytes!("../../../assets/hello_world_store.zip");
        let expected = &hello[0..0x2c];
        let compression_method = CompressionMethod::Stored;
        let uncompressed_size = 5;
        let crc32 = 980881731;
        let bytes = "world".as_bytes();
        let compressed_data =
            CompressedData::create_unchecked(uncompressed_size, compression_method, crc32, bytes);

        let input = LocalFileEntry {
            version_needed: 10,
            general_purpose: 0,
            file_modification_time: DosTime::from_u16_unchecked(41164),
            file_modification_date: DosDate::from_u16_unchecked(20867),
            file_name: ZipPath::create_from_string("hello.txt".as_ascii_str().unwrap()).unwrap(),
            extra_field: ExtraField::Unknown(&[]),
            compressed_data,
        };

        let mut buf = [0u8; 0x2c];
        let serializer = local_file_entry(&input);
        let (_, pos) = gen(serializer, &mut buf[..]).unwrap();
        assert_eq!(0x2c, pos);
        assert_eq!(expected, buf);
    }
}
