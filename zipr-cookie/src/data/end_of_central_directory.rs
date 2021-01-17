use super::ascii_char::ascii_chars;
use cookie_factory::{
    bytes::{le_u16, le_u32},
    combinator::slice,
    lib::std::io::Write,
    sequence::tuple,
    SerializeFn,
};
use zipr_data::{
    borrowed::file::EndOfCentralDirectory, constants::END_OF_CENTRAL_DIRECTORY_HEADER,
};

pub fn end_of_central_directory<'a, W: Write + 'a>(
    input: &'a EndOfCentralDirectory,
) -> impl SerializeFn<W> + 'a {
    tuple((
        slice(END_OF_CENTRAL_DIRECTORY_HEADER),
        le_u16(0),
        le_u16(0),
        le_u16(input.total_number_records),
        le_u16(input.total_number_records),
        le_u32(input.size_of_directory),
        le_u32(input.offset_start_directory),
        le_u16(input.comment.len() as u16),
        ascii_chars(input.comment),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ascii::AsciiStr;
    use cookie_factory::gen;
    use zipr_data::{
        borrowed::file::EndOfCentralDirectory, constants::END_OF_CENTRAL_DIRECTORY_MIN_SIZE,
    };

    const MINIMAL: [u8; 22] = [
        0x50, 0x4B, 0x05, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    #[test]
    fn minimal() {
        let input = EndOfCentralDirectory {
            total_number_records: 0,
            size_of_directory: 0,
            offset_start_directory: 0,
            comment: AsciiStr::from_ascii(b"").unwrap(),
        };
        let mut buffer = [0u8; END_OF_CENTRAL_DIRECTORY_MIN_SIZE];
        let serializer = end_of_central_directory(&input);
        {
            let (rem, pos) = gen(serializer, &mut buffer[..]).unwrap();
            assert_eq!(pos, END_OF_CENTRAL_DIRECTORY_MIN_SIZE as u64);
            assert_eq!(rem, &[0u8; 0]);
        };
        assert_eq!(MINIMAL, buffer);
    }

    #[test]
    fn hello_world_store() {
        let hello = include_bytes!("../../../assets/hello_world_store.zip");
        let header = {
            let len = hello.len();
            &hello[len - END_OF_CENTRAL_DIRECTORY_MIN_SIZE..len]
        };
        let input = EndOfCentralDirectory {
            total_number_records: 1,
            size_of_directory: 91,
            offset_start_directory: 0x2C,
            ..Default::default()
        };

        let mut buffer = [0u8; END_OF_CENTRAL_DIRECTORY_MIN_SIZE];
        let serializer = end_of_central_directory(&input);
        {
            let (rem, pos) = gen(serializer, &mut buffer[..]).unwrap();
            assert_eq!(pos, END_OF_CENTRAL_DIRECTORY_MIN_SIZE as u64);
            assert_eq!(rem, &[0u8; 0]);
        };
        assert_eq!(header, buffer);
    }

    #[test]
    fn two_files_store() {
        let hello = include_bytes!("../../../assets/two_files_store.zip");
        let header = {
            let len = hello.len();
            &hello[len - END_OF_CENTRAL_DIRECTORY_MIN_SIZE..len]
        };
        let input = EndOfCentralDirectory {
            total_number_records: 2,
            size_of_directory: 185,
            offset_start_directory: 0x59,
            ..Default::default()
        };

        let mut buffer = [0u8; END_OF_CENTRAL_DIRECTORY_MIN_SIZE];
        let serializer = end_of_central_directory(&input);
        {
            let (rem, pos) = gen(serializer, &mut buffer[..]).unwrap();
            assert_eq!(pos, END_OF_CENTRAL_DIRECTORY_MIN_SIZE as u64);
            assert_eq!(rem, &[0u8; 0]);
        };
        assert_eq!(header, buffer);
    }
}
