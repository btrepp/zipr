use crate::data::parse_end_of_central_directory;
use nom::IResult;
use zipr_data::{
    borrowed::file::EndOfCentralDirectory, constants::END_OF_CENTRAL_DIRECTORY_MIN_SIZE,
};

/// Like parse eocd, but walks backwards in the slice trying to find
/// where the end of central directory record is
pub fn find_end_of_central_directory(input: &[u8]) -> IResult<&[u8], EndOfCentralDirectory> {
    let length = input.len();
    let minimal = length - END_OF_CENTRAL_DIRECTORY_MIN_SIZE + 1;

    let start = &input[0..minimal];

    for (index, _) in start.iter().rev().enumerate() {
        let input = &input[index..length];
        if let Ok(result) = parse_end_of_central_directory(input) {
            return Ok(result);
        }
    }

    parse_end_of_central_directory(input)
}

#[cfg(test)]
mod tests {
    use ascii::AsciiStr;

    use super::*;

    #[test]
    fn hello_world_store_without_position() {
        let input = include_bytes!("../../../assets/hello_world_store.zip");
        let result = find_end_of_central_directory(input);
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
        let input = include_bytes!("../../../assets/hello_world_store_with_comment.zip");
        let comment = AsciiStr::from_ascii("tricky").unwrap();
        let result = find_end_of_central_directory(input);
        let expected = EndOfCentralDirectory {
            total_number_records: 1,
            size_of_directory: 91,
            offset_start_directory: 44,
            comment,
        };

        assert_eq!(Ok((&[] as &[u8], expected)), result);
    }
}
