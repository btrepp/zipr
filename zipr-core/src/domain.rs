use crate::data::{
    file::{CentralDirectoryEntry, LocalFileEntry},
    ZipEntry,
};

/// Creates a zip entry from a the central directory entry
/// and the localfile entry
/// Note: we treat the directory entry as the more valid one.
/// so the local entry is only used to get the zip data
/// see ZipEntry
pub fn make_zip_entry<'a>(
    central: CentralDirectoryEntry<'a>,
    local: LocalFileEntry<'a>,
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
