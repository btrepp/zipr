use crate::CP437Str;

/// A borrow of a ascii str
///
/// This should represent a valid path for entries in the
/// zip file. Note this is a subset of path or asciistr.
/// as there are invalid states possible
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct ZipPath<'a>(CP437Str<'a>);

/// Error for when a string is not a valid zippath
#[derive(Debug)]
pub enum ZipPathError {}

impl<'a> ZipPath<'a> {
    /// The name of the file, with optional relative path.
    /// The path stored MUST NOT contain a drive or
    /// device letter, or a leading slash.  All slashes
    /// MUST be forward slashes '/' as opposed to
    /// backwards slashes '\' for compatibility with Amiga
    /// and UNIX file systems etc.
    pub fn from_cp437(string: CP437Str<'a>) -> Result<Self, ZipPathError> {
        // need to validate this in future
        Ok(ZipPath(string))
    }

    pub fn to_cp437(&self) -> CP437Str<'a> {
        self.0
    }
}
