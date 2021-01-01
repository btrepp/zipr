use winstructs::timestamp::WinTimestamp;

/// PKWARE Win95/ WinNT Extra Field
/// Seems to just be timestamps for now?.
/// https://opensource.apple.com/source/zip/zip-6/unzip/unzip/proginfo/extra.fld
#[derive(Debug, PartialEq)]
pub struct NTFS {
    pub mtime: WinTimestamp,
    pub ctime: WinTimestamp,
    pub atime: WinTimestamp,
}
