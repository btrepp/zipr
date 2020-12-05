use extra_field::ExtraField;
use winstructs::timestamp::{DosDate, DosTime};

pub mod extra_field;

/// End of central directory header
/// This appears at the end of the file
/// Mainly used to tell  where the central directory
/// starts
#[derive(Debug, PartialEq, Default)]
pub struct EndOfCentralDirectory<'a> {
    pub total_number_records: u16,
    pub size_of_directory: u32,
    pub offset_start_directory: u32,
    pub comment: &'a [u8],
}

/// An entry for a file in the central directory
/// This is the 'true' source of where a file is
/// and what properties it has
#[derive(Debug, PartialEq)]
pub struct CentralDirectoryEntry<'a> {
    pub version_made_by: u16,
    pub version_needed: u16,
    pub general_purpose: u16,
    pub compression_method: CompressionMethod,
    pub file_modification_time: DosTime,
    pub file_modification_date: DosDate,
    pub crc32: u32,
    pub compressed_size: u32,
    pub uncompressed_size: u32,
    pub internal_file_attributes: u16,
    pub external_file_attributes: u32,
    pub relative_offset: u32,
    pub file_name: &'a str,
    pub extra_field: ExtraField<'a>,
    pub comment: &'a str,
}

/// Enum describing the compression method
/// note there are many of these. We don't implement them all
#[derive(Debug, PartialEq)]
pub enum CompressionMethod {
    Stored,
    Deflate,
}
