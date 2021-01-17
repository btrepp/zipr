use ascii::AsciiStr;

/// A borrow of a ascii str
///
/// This should represent a valid path for entries in the
/// zip file. Note this is a subset of path or asciistr.
/// as there are invalid states possible
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct ZipPath<'a>(&'a AsciiStr);

/// Error for when a string is not a valid zippath
#[derive(Debug)]
pub enum ZipPathError {}

impl<'a> ZipPath<'a> {
    pub fn create_from_string(string: &'a AsciiStr) -> Result<Self, ZipPathError> {
        // need to validate this in future
        Ok(ZipPath(string))
    }

    pub fn to_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    pub fn len(&self) -> usize {
        self.to_bytes().len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
