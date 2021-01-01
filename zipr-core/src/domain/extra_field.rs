use crate::{constants::EXTRA_FIELD_NTFS_LENGTH, data::extra_field::ExtraField};

/// Given a local file entry, locate where the data is located
/// Note: this is the minimum size + file size + extra data size
pub fn zip_size(entry: &ExtraField) -> u32 {
    match *entry {
        ExtraField::NTFS(_) => EXTRA_FIELD_NTFS_LENGTH.into(),
        ExtraField::Unknown(x) => x.len() as u32,
    }
}
