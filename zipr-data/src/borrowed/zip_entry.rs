use crate::{DosDate, DosTime, Version};

use super::{extra_field::ExtraField, file::CompressedData, ZipPath};
use ascii::AsciiStr;

/// A logical zip entry
/// This contains all the information about the zip entry
/// It is produced from a combination of the local file (which has data)
/// and the directory entry, which has some permissions
/// Note: as is this doesn't represent items in a zip file
/// But it is easier to work with in an applicative style
#[derive(Debug, PartialEq)]
pub struct ZipEntry<'a> {
    pub version_made_by: Version,
    pub version_needed: u16,
    pub general_purpose: u16,
    pub file_modification_time: DosTime,
    pub file_modification_date: DosDate,
    pub internal_file_attributes: u16,
    pub external_file_attributes: u32,
    pub file_name: ZipPath<'a>,
    pub extra_field: ExtraField<'a>,
    pub comment: &'a AsciiStr,
    pub compressed_data: CompressedData<'a>,
}
