use crate::constants::{END_OF_CENTRAL_DIRECTORY_HEADER, END_OF_CENTRAL_DIRECTORY_MIN_SIZE};
use crate::data::EndOfCentralDirectory;

use nom::{
    bytes::complete::tag, bytes::complete::take, combinator::eof, number::complete::le_u16,
    number::complete::le_u32, IResult,
};

/// Parses the end of central directory record exactly
/// Fails if its not present
pub fn parse_eocd(input: &[u8]) -> IResult<&[u8], EndOfCentralDirectory> {
    let (input, _) = tag(END_OF_CENTRAL_DIRECTORY_HEADER)(input)?;
    //For now only support a single zip file
    let (input, _number_of_this_disk) = tag([0, 0])(input)?;
    let (input, _directory_start_disk) = tag([0, 0])(input)?;
    // don't use this, but should equal total number of records
    let (input, _records_on_this_disk) = le_u16(input)?;
    let (input, total_number_records) = le_u16(input)?;
    let (input, size_of_directory) = le_u32(input)?;
    let (input, offset_start_directory) = le_u32(input)?;
    let (input, comment_length) = le_u16(input)?;
    let (input, comment) = take(comment_length)(input)?;
    let (input, _eof) = eof(input)?;
    let result = EndOfCentralDirectory {
        total_number_records,
        size_of_directory,
        offset_start_directory,
        comment,
    };

    Ok((input, result))
}

/// Like parse eocd, but walks backwards in the slice trying to find
/// where the end of central directory record is
pub fn try_find_parse_eocd(input: &[u8]) -> IResult<&[u8], EndOfCentralDirectory> {
    let length = input.len();
    let minimal = length - END_OF_CENTRAL_DIRECTORY_MIN_SIZE + 1;

    let start = &input[0..minimal];

    for (index, _) in start.iter().rev().enumerate() {
        let input = &input[index..length];
        match parse_eocd(input) {
            Ok(result) => return Ok(result),
            Err(_) => (),
        }
    }

    return parse_eocd(input);
}

#[cfg(test)]
mod tests {
    use crate::{constants::END_OF_CENTRAL_DIRECTORY_MIN_SIZE, data::EndOfCentralDirectory};

    use super::{parse_eocd, try_find_parse_eocd};

    const MINIMAL: [u8; 22] = [
        0x50, 0x4B, 0x05, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    #[test]
    fn minimal() {
        let result = parse_eocd(&MINIMAL);
        let expected = {
            let remaining: &[u8] = &[];
            let directory = EndOfCentralDirectory {
                ..Default::default()
            };
            Ok((remaining, directory))
        };
        assert_eq!(expected, result);
    }

    #[test]
    fn hello_world_store() {
        let hello = include_bytes!("../../assets/hello_world_store.zip");
        let header = {
            let len = hello.len();
            &hello[len - END_OF_CENTRAL_DIRECTORY_MIN_SIZE..len]
        };
        let result = parse_eocd(header);
        let expected = EndOfCentralDirectory {
            total_number_records: 1,
            size_of_directory: 91,
            offset_start_directory: 0x2C,
            ..Default::default()
        };

        assert_eq!(Ok((&[] as &[u8], expected)), result);
    }

    #[test]
    fn hello_world_store_without_position() {
        let input = include_bytes!("../../assets/hello_world_store.zip");
        let result = try_find_parse_eocd(input);
        let expected = EndOfCentralDirectory {
            total_number_records: 1,
            size_of_directory: 91,
            offset_start_directory: 44,
            ..Default::default()
        };

        assert_eq!(Ok((&[] as &[u8], expected)), result);
    }

    #[test]
    fn hello_world_store_with_comment() {
        let input = include_bytes!("../../assets/hello_world_store_with_comment.zip");
        let comment = "tricky".as_bytes();
        let result = try_find_parse_eocd(input);
        let expected = EndOfCentralDirectory {
            total_number_records: 1,
            size_of_directory: 91,
            offset_start_directory: 44,
            comment,
        };

        assert_eq!(Ok((&[] as &[u8], expected)), result);
    }

    #[test]
    fn two_files_store() {
        let hello = include_bytes!("../../assets/two_files_store.zip");
        let header = {
            let len = hello.len();
            &hello[len - END_OF_CENTRAL_DIRECTORY_MIN_SIZE..len]
        };
        let result = parse_eocd(header);
        let expected = EndOfCentralDirectory {
            total_number_records: 2,
            size_of_directory: 185,
            offset_start_directory: 0x59,
            ..Default::default()
        };

        assert_eq!(Ok((&[] as &[u8], expected)), result);
    }
}
