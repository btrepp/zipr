use nom::{IResult, bytes::complete::tag};

use crate::constants::CENTRAL_DIRECTORY_HEADER;

#[derive(Debug, PartialEq)]
pub struct CentralDirectoryHeader<'a> {
    version_made_by: u16,
    version_needed: u16,
    general_purpose: u16,
    compression_method: u16,
    file_modification_time: u16,
    file_modification_date: u16,
    crc32: u32,
    compressed_size: u32,
    uncompressed_size: u32,
    file_name_length: u16,
    extra_field_length: u16,
    file_comment_length: u16,
    disk_file_start: u16,
    internal_file_attributes: u16,
    external_file_attributes: u32,
    relative_offset: u32,
    file_name: &'a str,
    extra_field: &'a [u8],
    comment: &'a str
}

pub fn parse_directory_header(input: &[u8]) -> IResult<&[u8], CentralDirectoryHeader> {
    let (input, _) = tag(CENTRAL_DIRECTORY_HEADER)(input)?;
    let (input, number_of_this_disk) = le_u16(input)?;
    let (input, directory_start_disk) = le_u16(input)?;
    let (input, records_on_this_disk) = le_u16(input)?;
    let (input, total_number_records) = le_u16(input)?;
    let (input, size_of_directory) = le_u32(input)?;
    let (input, offset_start_directory) = le_u32(input)?;
    let (input, comment_length) = le_u16(input)?;
    let result = EndOfCentralDirectory {
        number_of_this_disk,
        directory_start_disk,
        records_on_this_disk,
        total_number_records,
        size_of_directory,
        offset_start_directory,
        comment_length,
    };
    Ok((input, result))
}