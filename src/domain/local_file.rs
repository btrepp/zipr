use crate::{constants::LOCAL_FILE_MIN_LENGTH, data::LocalFileEntry};


/// Given a local file entry, locate where the data is located
/// Note: this is the minimum size + file size + extra data size
pub fn  data_offset(entry:LocalFileEntry) -> u32 {
    let file_length = entry.file_name.as_os_str().len();
    let extra_length = crate::domain::extra_field::zip_size(&entry.extra_field);
    LOCAL_FILE_MIN_LENGTH as u32 + file_length as u32 + extra_length
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use winstructs::timestamp::{DosDate, DosTime};

    use crate::data::extra_field::ExtraField;

    use super::*;
   
    #[test]
    fn test_data_offset_hellotxt() {
        let input = LocalFileEntry {
            version_needed: 10,
            general_purpose: 0,
            compression_method: crate::data::CompressionMethod::Stored,
            file_modification_time: DosTime::new(41164),
            file_modification_date: DosDate::new(20867),
            crc32: 980881731,
            uncompressed_size: 5,
            file_name: Path::new("hello.txt"),
            extra_field: ExtraField::Unknown(&[]),
            bytes: &[]
        };
        let result = data_offset(input);

        assert_eq!(39, result);
    }

    #[test]
    fn test_data_offset_moretxt() {
        let input = LocalFileEntry {
            version_needed: 10,
            general_purpose: 0,
            compression_method: crate::data::CompressionMethod::Stored,
            file_modification_time: DosTime::new(41164),
            file_modification_date: DosDate::new(20867),
            crc32: 980881731,
            uncompressed_size: 5,
            file_name: Path::new("more.txt"),
            extra_field: ExtraField::Unknown(&[]),
            bytes: &[]
        };
        let result = data_offset(input);

        assert_eq!(38, result);
    }
}