use zipr_data::borrowed::{
    file::{CentralDirectoryEntry, LocalFileEntry},
    ZipEntry,
};

/// Creates a zip entry from a the central directory entry
/// and the localfile entry
/// Note: we treat the directory entry as the more valid one.
/// so the local entry is only used to get the zip data
/// see ZipEntry
pub fn make_zip_entry<'a>(
    central: &CentralDirectoryEntry<'a>,
    local: &LocalFileEntry<'a>,
) -> ZipEntry<'a> {
    let version_made_by = central.version_made_by;
    let version_needed = central.version_needed;
    let general_purpose = central.general_purpose;
    let file_modification_time = central.file_modification_time;
    let file_modification_date = central.file_modification_date;
    let internal_file_attributes = central.internal_file_attributes;
    let external_file_attributes = central.external_file_attributes;
    let file_name = central.file_name;
    let extra_field = central.extra_field;
    let comment = central.comment;
    let compressed_data = local.compressed_data;
    ZipEntry {
        version_made_by,
        version_needed,
        general_purpose,
        file_modification_time,
        file_modification_date,
        internal_file_attributes,
        external_file_attributes,
        file_name,
        extra_field,
        comment,
        compressed_data,
    }
}

/// Given a zip entry, convert it into
/// the local file and central directory entries.
/// Note we need to know where the localfile entry will be written as the offset from
/// the beginning of the file
pub fn zip_entry_to_files<'a>(
    relative_offset: u32,
    entry: &'a ZipEntry<'a>,
) -> (LocalFileEntry<'a>, CentralDirectoryEntry<'a>) {
    let version_made_by = entry.version_made_by;
    let version_needed = entry.version_needed;
    let general_purpose = entry.general_purpose;
    let file_modification_time = entry.file_modification_time;
    let file_modification_date = entry.file_modification_date;
    let file_name = entry.file_name;
    let extra_field = entry.extra_field;
    let compressed_data = entry.compressed_data;
    let compression_method = compressed_data.compression_method();
    let crc32 = compressed_data.crc32();
    let compressed_size = compressed_data.bytes().len() as u32;
    let uncompressed_size = compressed_data.uncompressed_size();
    let internal_file_attributes = entry.internal_file_attributes;
    let external_file_attributes = entry.external_file_attributes;
    let comment = entry.comment;

    let local = LocalFileEntry {
        version_needed,
        general_purpose,
        file_modification_time,
        file_modification_date,
        file_name,
        extra_field,
        compressed_data,
    };

    let central = CentralDirectoryEntry {
        version_made_by,
        version_needed,
        general_purpose,
        compression_method,
        file_modification_time,
        file_modification_date,
        crc32,
        compressed_size,
        uncompressed_size,
        internal_file_attributes,
        external_file_attributes,
        relative_offset,
        file_name,
        extra_field,
        comment,
    };

    (local, central)
}
