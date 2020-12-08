use crate::{constants::CENTRAL_DIRECTORY_HEAD_MIN_LENGTH, data::CentralDirectoryEntry};




impl<'a> CentralDirectoryEntry<'a> {

    /// Given a local file entry, locate where the data is located
    /// Note: this is the minimum size + file size + extra data size
    pub fn  zip_size(&self) -> u32 {
        let file_length = self.file_name.as_os_str().len();
        let extra_length = crate::domain::extra_field::zip_size(&self.extra_field);
        let comment_length = self.comment.len();
        CENTRAL_DIRECTORY_HEAD_MIN_LENGTH as u32 + file_length as u32 + extra_length + comment_length as u32
    }
}    

#[cfg(test)]
mod tests {
    use std::path::Path;

    use winstructs::timestamp::{DosDate, DosTime, WinTimestamp};

    use crate::data::extra_field::{ExtraField, ntfs::NTFS};

    use super::*;
   
    #[test]
    fn test_data_offset_hellotxt() {
        let input = CentralDirectoryEntry {
            version_made_by: 63,
            version_needed: 10,
            file_modification_time: DosTime::new(41164),
            file_modification_date: DosDate::new(20867),
            crc32: 980881731,
            compressed_size: 5,
            uncompressed_size: 5,
            internal_file_attributes: 0,
            external_file_attributes: 32,
            file_name: Path::new("hello.txt"),
            comment: "",
            extra_field: ExtraField::NTFS(NTFS {
                atime: WinTimestamp::from_u64(132514708162669827),
                mtime: WinTimestamp::from_u64(132514707831351075),
                ctime: WinTimestamp::from_u64(132514707783459448),
            }),
            compression_method: crate::data::CompressionMethod::Stored,
            general_purpose: 0,
            relative_offset: 0,
        };

        let result = input.zip_size();
        assert_eq!(9, input.file_name.as_os_str().len());
        assert_eq!(36, crate::domain::extra_field::zip_size(&input.extra_field));
        assert_eq!(0, input.comment.len());
        assert_eq!(91, result);
    }

}