use self::ntfs::NTFS;

pub mod ntfs;
pub mod wintimestamp;

/// Extra fields that can be present
/// Known ones will be parsed, unknown will just store
/// the slice
/// https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT
/// 4.5 Extensible data fields
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ExtraField<'a> {
    NTFS(NTFS),
    Unknown(&'a [u8]),
}

impl<'a> Default for ExtraField<'a> {
    fn default() -> Self {
        //This isn't really the best, but does keep it all
        //easy to use
        ExtraField::Unknown(&[])
    }
}
