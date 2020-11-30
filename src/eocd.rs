use crate::constants::END_OF_CENTRAL_DIRECTORY_HEADER;

use nom::{IResult, number::complete::be_u32, bytes::complete::{tag}, number::complete::be_u16};

#[derive(Debug,PartialEq)]
pub struct EndOfCentralDirectory {
    number_of_this_disk: u16,
    directory_start_disk: u16,
    records_on_this_disk: u16,
    total_number_records: u16,
    size_of_directory: u32,
    offset_start_directory: u32,
    comment_length: u16
}

pub fn parse_eocd(input:&[u8]) -> IResult<&[u8], EndOfCentralDirectory> {
    let (input, _) = tag(END_OF_CENTRAL_DIRECTORY_HEADER)(input)?;
    let (input,number_of_this_disk) = be_u16(input)?;
    let (input, directory_start_disk) = be_u16(input)?;
    let (input, records_on_this_disk) = be_u16(input)?;
    let (input, total_number_records) = be_u16(input)?;
    let (input, size_of_directory) = be_u32(input)?;
    let (input, offset_start_directory) = be_u32(input)?;
    let (input, comment_length ) = be_u16(input)?;
    let result = 
        EndOfCentralDirectory {
            number_of_this_disk,
            directory_start_disk,
            records_on_this_disk,
            total_number_records,
            size_of_directory,
            offset_start_directory,
            comment_length
        };
    Ok((input,result))
}

#[cfg(test)]
mod tests {
    use super::{EndOfCentralDirectory, parse_eocd};


    const MINIMAL : [u8;22] = [0x50,0x4B,0x05,0x06,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00];
    #[test]
    fn minimal() {

        let result = parse_eocd(&MINIMAL);
        let expected = {
            let remaining: &[u8] = &[];
            let directory = EndOfCentralDirectory {
                number_of_this_disk : 0,
                directory_start_disk : 0, 
                records_on_this_disk: 0,
                total_number_records : 0,
                size_of_directory: 0,
                offset_start_directory: 0,
                comment_length: 0
            };
            Ok((remaining, directory))
        };
        assert_eq!(expected,result);
    }
}



